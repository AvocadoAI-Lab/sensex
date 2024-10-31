use super::common::TEST_AGENT_ID;
use super::test_framework::TestFramework;
use crate::endpoints;
use crate::endpoints_with_params;

const MODULE_NAME: &str = "agents";

#[tokio::test]
async fn test_agents_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    // Basic endpoints without parameters
    let mut endpoints = endpoints!(framework,
        "/agents",
        "/agents/no_group",
        "/agents/stats/distinct",
        "/agents/summary/os",
        "/agents/summary/status"
    );

    // Add endpoints with agent configuration parameters
    endpoints.extend(endpoints_with_params!(framework,
        (
            &format!("/agents/{}/group/is_sync", TEST_AGENT_ID),
            "agent_id",
            serde_json::json!({ "agent_id": TEST_AGENT_ID })
        ),
        (
            &format!("/agents/{}/daemons/stats", TEST_AGENT_ID),
            "agent_id",
            serde_json::json!({ "agent_id": TEST_AGENT_ID })
        )
    ));

    // Add configuration endpoints
    for config in ["buffer", "internal", "client", "labels"] {
        endpoints.push(framework.create_endpoint_with_params(
            &format!("/agents/{}/config/agent/{}", TEST_AGENT_ID, config),
            "agent_id, component, configuration",
            serde_json::json!({
                "agent_id": TEST_AGENT_ID,
                "component": "agent",
                "configuration": config
            })
        ));
    }

    // Add stats endpoints
    for component in ["logcollector", "agent"] {
        endpoints.push(framework.create_endpoint_with_params(
            &format!("/agents/{}/stats/{}", TEST_AGENT_ID, component),
            "agent_id, component",
            serde_json::json!({
                "agent_id": TEST_AGENT_ID,
                "component": component
            })
        ));
    }

    framework.test_endpoints(endpoints).await?;
    Ok(())
}
