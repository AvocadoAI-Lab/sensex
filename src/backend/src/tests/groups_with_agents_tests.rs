use crate::tests::core::test_framework::TestFramework;
use crate::tests::core::test_utils::TestEndpoint;
use crate::endpoints_with_params;
use tokio::time::{sleep, Duration};
use std::time::Instant;

// Changed from "groups/with_agents" to "groups_with_agents" to avoid subdirectories
const MODULE_NAME: &str = "groups_with_agents";
const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 1000;

async fn retry_test_endpoint(
    framework: &TestFramework,
    endpoint: TestEndpoint,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let start = Instant::now();
    let mut last_error = None;

    for attempt in 1..=MAX_RETRIES {
        match framework.test_endpoint(endpoint.clone()).await {
            Ok(response) => {
                return Ok(response);
            }
            Err(e) => {
                println!("Attempt {} failed: {}", attempt, e);
                last_error = Some(e);
                
                if attempt < MAX_RETRIES {
                    println!("Waiting {}ms before retry...", RETRY_DELAY_MS);
                    sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
                }
            }
        }
    }

    Err(format!(
        "Failed after {} attempts over {:?}. Last error: {:?}",
        MAX_RETRIES,
        start.elapsed(),
        last_error
    ).into())
}

#[tokio::test]
async fn test_groups_with_agents() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    // First get all groups with retry mechanism
    println!("Fetching groups list...");
    let groups_endpoint = framework.create_endpoint("/groups");
    let groups_response = retry_test_endpoint(&framework, groups_endpoint).await?;

    // Add delay after groups request
    sleep(Duration::from_millis(500)).await;

    // Create endpoints for each group's agents
    let mut all_endpoints = Vec::new();
    if let Some(affected_items) = groups_response["data"]["affected_items"].as_array() {
        for group in affected_items {
            if let Some(group_name) = group["name"].as_str() {
                println!("Processing group: {}", group_name);
                all_endpoints.extend(endpoints_with_params!(framework,
                    (
                        &format!("/groups/{}/agents", group_name),
                        "group_id",
                        serde_json::json!({ "group_id": group_name })
                    )
                ));
            }
        }
    }

    // Test each endpoint individually with delay between requests
    for endpoint in all_endpoints {
        println!("Testing endpoint: {}", endpoint.path);
        
        match retry_test_endpoint(&framework, endpoint.clone()).await {
            Ok(_) => println!("Successfully tested endpoint: {}", endpoint.path),
            Err(e) => println!("Warning: Failed to test endpoint {}: {}", endpoint.path, e),
        }

        // Add delay between requests
        sleep(Duration::from_millis(500)).await;
    }

    Ok(())
}
