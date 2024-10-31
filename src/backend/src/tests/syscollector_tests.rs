use super::common::TEST_AGENT_ID;
use super::test_framework::TestFramework;
use crate::endpoints_with_params;

const MODULE_NAME: &str = "syscollector";

#[tokio::test]
async fn test_syscollector_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    let endpoints = endpoints_with_params!(framework,
        (
            &format!("/syscollector/{}/hardware", TEST_AGENT_ID),
            "agent_id",
            serde_json::json!({ "agent_id": TEST_AGENT_ID })
        ),
        (
            &format!("/syscollector/{}/hotfixes", TEST_AGENT_ID),
            "agent_id",
            serde_json::json!({ "agent_id": TEST_AGENT_ID })
        ),
        (
            &format!("/syscollector/{}/netaddr", TEST_AGENT_ID),
            "agent_id",
            serde_json::json!({ "agent_id": TEST_AGENT_ID })
        ),
        (
            &format!("/syscollector/{}/netiface", TEST_AGENT_ID),
            "agent_id",
            serde_json::json!({ "agent_id": TEST_AGENT_ID })
        ),
        (
            &format!("/syscollector/{}/netproto", TEST_AGENT_ID),
            "agent_id",
            serde_json::json!({ "agent_id": TEST_AGENT_ID })
        ),
        (
            &format!("/syscollector/{}/os", TEST_AGENT_ID),
            "agent_id",
            serde_json::json!({ "agent_id": TEST_AGENT_ID })
        ),
        (
            &format!("/syscollector/{}/packages", TEST_AGENT_ID),
            "agent_id",
            serde_json::json!({ "agent_id": TEST_AGENT_ID })
        ),
        (
            &format!("/syscollector/{}/ports", TEST_AGENT_ID),
            "agent_id",
            serde_json::json!({ "agent_id": TEST_AGENT_ID })
        ),
        (
            &format!("/syscollector/{}/processes", TEST_AGENT_ID),
            "agent_id",
            serde_json::json!({ "agent_id": TEST_AGENT_ID })
        )
    );

    framework.test_endpoints(endpoints).await?;
    Ok(())
}
