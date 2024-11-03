use crate::tests::core::TestFramework;
use tokio::time::Duration;
use std::collections::HashSet;

const MODULE_NAME: &str = "groups_list";

async fn test_agent_endpoints(framework: &TestFramework, agent_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Define all endpoints to test
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

    for endpoint_template in endpoints {
        println!("\nTesting GET {} for agent {}", endpoint_template, agent_id);
        let endpoint = framework.create_agent_endpoint(endpoint_template, agent_id);
        
        match framework.test_endpoint(endpoint).await {
            Ok(_) => println!("Successfully tested endpoint"),
            Err(e) => println!("Endpoint returned error (this may be normal if agent has no data): {}", e)
        }
        
        // Add delay between requests
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    Ok(())
}

#[tokio::test]
async fn test_groups_list() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    // Get groups
    let groups_endpoint = framework.create_endpoint("/groups");
    let groups_response = framework.test_endpoint(groups_endpoint).await?;

    // Store unique agent IDs
    let mut agent_ids = HashSet::new();

    // Extract groups and their agents
    if let Some(affected_items) = groups_response["data"]["affected_items"].as_array() {
        for group in affected_items {
            if let Some(group_name) = group["name"].as_str() {
                let agents_endpoint = framework.create_param_endpoint(
                    "/groups/{group_id}/agents",
                    "group_id",
                    group_name
                );
                
                let agents_response = framework.test_endpoint(agents_endpoint).await?;

                if let Some(affected_items) = agents_response["data"]["affected_items"].as_array() {
                    for agent in affected_items {
                        if let Some(id) = agent["id"].as_str() {
                            agent_ids.insert(id.to_string());
                        }
                    }
                }
                
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    // Test all endpoints for each unique agent
    println!("\nTesting endpoints for {} unique agents", agent_ids.len());
    for agent_id in agent_ids {
        println!("\n=== Testing endpoints for agent {} ===", agent_id);
        if let Err(e) = test_agent_endpoints(&framework, &agent_id).await {
            println!("Error testing agent {}: {}", agent_id, e);
            // Continue with next agent even if this one had errors
            continue;
        }
    }

    Ok(())
}
