use qdrant_client::prelude::*;
use qdrant_client::qdrant::{
    SearchPoints, SearchParams, Filter, FieldCondition, Match
};
use dotenv::dotenv;
use reqwest;
use serde_json::Value as JsonValue;
use std::sync::Arc;
use std::io::{self, Write};
use qdrant_client::qdrant::r#match::MatchValue;

const VECTOR_SIZE: u64 = 1536;
const EMBEDDING_MODEL: &str = "text-embedding-ada-002";
const OPENAI_API_URL: &str = "https://api.openai.com/v1/embeddings";

async fn init_qdrant_client() -> Result<Arc<QdrantClient>, Box<dyn std::error::Error>> {
    let client = QdrantClient::from_url("http://localhost:6334").build()?;
    Ok(Arc::new(client))
}

fn sanitize_collection_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect()
}

async fn generate_embedding(text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| "OPENAI_API_KEY environment variable not set")?;

    let client = reqwest::Client::new();
    let response = client
        .post(OPENAI_API_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "input": text,
            "model": EMBEDDING_MODEL
        }))
        .send()
        .await?;

    let response_json: serde_json::Value = response.json().await?;
    let embedding = response_json["data"][0]["embedding"]
        .as_array()
        .ok_or("Invalid embedding response")?
        .iter()
        .map(|v| v.as_f64().unwrap_or_default() as f32)
        .collect();

    Ok(embedding)
}

async fn search_collection(
    client: &QdrantClient,
    collection_name: &str,
    query: &str,
    group_id: Option<&str>,
    agent_id: Option<&str>,
    limit: u64,
) -> Result<Vec<(f32, JsonValue)>, Box<dyn std::error::Error>> {
    let sanitized_collection = sanitize_collection_name(collection_name);
    let query_vector = generate_embedding(query).await?;

    let mut must_conditions = Vec::new();
    
    if let Some(group_id) = group_id {
        must_conditions.push(FieldCondition {
            key: "group_id".to_string(),
            r#match: Some(Match {
                match_value: Some(MatchValue::Keyword(group_id.to_string())),
            }),
            range: None,
            geo_bounding_box: None,
            geo_radius: None,
            values_count: None,
            geo_polygon: None,
            datetime_range: None,
        }.into());
    }

    if let Some(agent_id) = agent_id {
        must_conditions.push(FieldCondition {
            key: "agent_id".to_string(),
            r#match: Some(Match {
                match_value: Some(MatchValue::Keyword(agent_id.to_string())),
            }),
            range: None,
            geo_bounding_box: None,
            geo_radius: None,
            values_count: None,
            geo_polygon: None,
            datetime_range: None,
        }.into());
    }

    let filter = if !must_conditions.is_empty() {
        Some(Filter {
            should: vec![],
            must: must_conditions,
            must_not: vec![],
            min_should: None,
        })
    } else {
        None
    };

    let search_response = client.search_points(&SearchPoints {
        collection_name: sanitized_collection,
        vector: query_vector,
        filter,
        limit,
        with_payload: Some(true.into()),
        params: Some(SearchParams {
            hnsw_ef: Some(128),
            exact: Some(false),
            ..Default::default()
        }),
        ..Default::default()
    }).await?;

    let results = search_response.result
        .into_iter()
        .map(|point| {
            let score = point.score;
            let data = point.payload.get("data")
                .and_then(|v| v.as_str())
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or(JsonValue::Null);
            (score, data)
        })
        .collect();

    Ok(results)
}

fn read_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();

    let client = init_qdrant_client().await?;
    println!("Connected to Qdrant successfully!");

    println!("\nAvailable collections:");
    println!("  - syscollector_hardware");
    println!("  - syscollector_os");
    println!("  - syscollector_netproto");
    println!("  - syscollector_packages");
    println!("  - syscollector_processes");
    println!("  - groups");
    println!("\nType 'exit' to quit the program");
    println!("Type 'help' to see this message again");
    println!("Press Enter after each input\n");

    loop {
        println!("\n=== New Query ===");
        
        let collection = match read_line("Collection (or command): ")? {
            cmd if cmd.to_lowercase() == "exit" => break,
            cmd if cmd.to_lowercase() == "help" => {
                println!("\nAvailable collections:");
                println!("  - syscollector_hardware");
                println!("  - syscollector_os");
                println!("  - syscollector_netproto");
                println!("  - syscollector_packages");
                println!("  - syscollector_processes");
                println!("  - groups");
                println!("\nType 'exit' to quit the program");
                println!("Type 'help' to see this message again");
                println!("Press Enter after each input\n");
                continue;
            }
            collection => collection,
        };

        let query = read_line("Query: ")?;
        
        let group_id = read_line("Group ID (optional, press Enter to skip): ")?;
        let group_id = if group_id.is_empty() { None } else { Some(group_id.as_str()) };

        let agent_id = read_line("Agent ID (optional, press Enter to skip): ")?;
        let agent_id = if agent_id.is_empty() { None } else { Some(agent_id.as_str()) };

        let limit_str = read_line("Number of results (press Enter for default 3): ")?;
        let limit = limit_str.parse().unwrap_or(3);

        println!("\nSearching...\n");

        match search_collection(
            &client,
            &collection,
            &query,
            group_id,
            agent_id,
            limit
        ).await {
            Ok(results) => {
                if results.is_empty() {
                    println!("No results found.");
                } else {
                    for (score, data) in results {
                        println!("Score: {:.4}", score);
                        println!("Data: {}", serde_json::to_string_pretty(&data)?);
                        println!("---");
                    }
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("Goodbye!");
    Ok(())
}
