use crate::tests::core::test_framework::TestFramework;
use crate::endpoints_with_params;

const MODULE_NAME: &str = "groups/with_agents";

#[tokio::test]
async fn test_groups_with_agents() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    // First get all groups
    let groups_endpoint = framework.create_endpoint("/groups");
    let groups_response = framework.test_endpoint(groups_endpoint).await?;

    // Create endpoints for each group's agents
    let mut all_endpoints = Vec::new();
    if let Some(affected_items) = groups_response["data"]["affected_items"].as_array() {
        for group in affected_items {
            if let Some(group_name) = group["name"].as_str() {
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

    // Test each endpoint individually
    for endpoint in all_endpoints {
        framework.test_endpoint(endpoint).await?;
    }

    Ok(())
}
