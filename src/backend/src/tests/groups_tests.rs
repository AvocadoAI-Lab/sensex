use super::common::TEST_GROUP_ID;
use super::test_framework::TestFramework;
use crate::endpoints;
use crate::endpoints_with_params;

const MODULE_NAME: &str = "groups";

#[tokio::test]
async fn test_groups_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    // Basic endpoints
    let mut endpoints = endpoints!(framework,
        "/groups"
    );

    // Add group-specific endpoints
    endpoints.extend(endpoints_with_params!(framework,
        (
            &format!("/groups/{}/files", TEST_GROUP_ID),
            "group_id",
            serde_json::json!({ "group_id": TEST_GROUP_ID })
        ),
        (
            &format!("/groups/{}/agents", TEST_GROUP_ID),
            "group_id",
            serde_json::json!({ "group_id": TEST_GROUP_ID })
        ),
        (
            &format!("/groups/{}/configuration", TEST_GROUP_ID),
            "group_id",
            serde_json::json!({ "group_id": TEST_GROUP_ID })
        )
    ));

    framework.test_endpoints(endpoints).await?;
    Ok(())
}

#[tokio::test]
async fn test_groups_with_agents() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    // First get all groups
    let groups_endpoint = framework.create_endpoint("/groups");
    let groups_response = super::test_utils::test_endpoint(
        &framework.client,
        &framework.headers,
        groups_endpoint,
        &framework.proxy_url,
        &framework.module_name
    ).await?;

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

    // Test all agent endpoints
    if !all_endpoints.is_empty() {
        framework.test_endpoints(all_endpoints).await?;
    }

    Ok(())
}
