use crate::tests::core::common::{get_test_client, TEST_AGENT_ID};
use crate::tests::core::test_framework::TestFramework;
use crate::endpoints_with_params;

const MODULE_NAME: &str = "agent_specific";

pub async fn test_agent_specific_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    let mut endpoints = vec![];

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

    // Add syscollector endpoints
    endpoints.extend(endpoints_with_params!(framework,
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
    ));

    // Test each endpoint
    for endpoint in endpoints {
        match framework.test_endpoint(endpoint.clone()).await {
            Ok(_) => println!("Successfully tested {}", endpoint.path),
            Err(e) => println!("Failed to test {}: {}", endpoint.path, e),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_specific() {
        test_agent_specific_endpoints().await.unwrap();
    }
}
