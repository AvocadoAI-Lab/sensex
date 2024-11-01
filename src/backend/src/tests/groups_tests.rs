use crate::tests::core::common::TEST_GROUP_ID;
use crate::tests::core::test_framework::TestFramework;
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

    // Test each endpoint individually
    for endpoint in endpoints {
        framework.test_endpoint(endpoint).await?;
    }
    
    Ok(())
}
