use crate::tests::core::TestFramework;
use tokio::time::Duration;
use std::collections::HashSet;
use futures::stream::{self, StreamExt};
use tokio::sync::Semaphore;
use std::sync::Arc;

const MODULE_NAME: &str = "groups_list";
const MAX_CONCURRENT_REQUESTS: usize = 5; // Adjust this value based on server capacity

async fn test_agent_endpoints(framework: &TestFramework, agent_id: &str, semaphore: Arc<Semaphore>) -> Result<(), Box<dyn std::error::Error>> {
    let endpoints = vec![
        "/syscollector/{agent_id}/hardware",
        "/syscollector/{agent_id}/hotfixes",
        "/syscollector/{agent_id}/netaddr",
        "/syscollector/{agent_id}/netiface",
        "/syscollector/{agent_id}/netproto",
        "/syscollector/{agent_id}/os",
        "/syscollector/{agent_id}/packages",
        "/syscollector/{agent_id}/ports",
        "/syscollector/{agent_id}/processes",
        "/syscheck/{agent_id}",
        "/syscheck/{agent_id}/last_scan",
        "/sca/{agent_id}",
        "/rootcheck/{agent_id}",
        "/rootcheck/{agent_id}/last_scan",
        "/ciscat/{agent_id}/results"
    ];

    // Process endpoints concurrently with controlled parallelism
    let futures = stream::iter(endpoints)
        .map(|endpoint_template| {
            let framework = framework.clone();
            let agent_id = agent_id.to_string();
            let semaphore = Arc::clone(&semaphore);
            
            async move {
                let _permit = semaphore.acquire().await.unwrap();
                println!("\nTesting GET {} for agent {}", endpoint_template, agent_id);
                let endpoint = framework.create_agent_endpoint(endpoint_template, &agent_id);
                
                match framework.test_endpoint(endpoint).await {
                    Ok(_) => println!("Successfully tested endpoint"),
                    Err(e) => println!("Endpoint returned error (this may be normal if agent has no data): {}", e)
                }
                
                // Reduced delay between requests
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        })
        .buffer_unordered(MAX_CONCURRENT_REQUESTS);

    futures.collect::<Vec<_>>().await;
    Ok(())
}

#[tokio::test]
async fn test_groups_list() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));

    // Get groups
    let groups_endpoint = framework.create_endpoint("/groups");
    let groups_response = framework.test_endpoint(groups_endpoint).await?;

    // Store unique agent IDs
    let mut agent_ids = HashSet::new();

    // Create a longer-lived empty Vec for the unwrap_or
    let empty_vec = Vec::new();
    
    // Get the affected items array or use empty vec
    let affected_items = groups_response["data"]["affected_items"]
        .as_array()
        .unwrap_or(&empty_vec);

    // Extract groups and their agents concurrently
    let group_futures = stream::iter(
        affected_items
            .iter()
            .filter_map(|group| group["name"].as_str().map(String::from))
    )
    .map(|group_name| {
        let framework = framework.clone();
        let semaphore = Arc::clone(&semaphore);
        
        async move {
            let _permit = semaphore.acquire().await.unwrap();
            let agents_endpoint = framework.create_param_endpoint(
                "/groups/{group_id}/agents",
                "group_id",
                &group_name
            );
            
            match framework.test_endpoint(agents_endpoint).await {
                Ok(agents_response) => {
                    if let Some(affected_items) = agents_response["data"]["affected_items"].as_array() {
                        affected_items
                            .iter()
                            .filter_map(|agent| agent["id"].as_str().map(String::from))
                            .collect::<Vec<_>>()
                    } else {
                        Vec::new()
                    }
                }
                Err(e) => {
                    println!("Error getting agents for group {}: {}", group_name, e);
                    Vec::new()
                }
            }
        }
    })
    .buffer_unordered(MAX_CONCURRENT_REQUESTS);

    // Collect all agent IDs
    let agent_ids_vec: Vec<Vec<String>> = group_futures.collect().await;
    for ids in agent_ids_vec {
        agent_ids.extend(ids);
    }

    // Test all endpoints for each unique agent concurrently
    println!("\nTesting endpoints for {} unique agents", agent_ids.len());
    
    let agent_futures = stream::iter(agent_ids)
        .map(|agent_id| {
            let framework = framework.clone();
            let semaphore = Arc::clone(&semaphore);
            
            async move {
                println!("\n=== Testing endpoints for agent {} ===", agent_id);
                if let Err(e) = test_agent_endpoints(&framework, &agent_id, semaphore).await {
                    println!("Error testing agent {}: {}", agent_id, e);
                }
            }
        })
        .buffer_unordered(MAX_CONCURRENT_REQUESTS);

    agent_futures.collect::<Vec<_>>().await;
    Ok(())
}
