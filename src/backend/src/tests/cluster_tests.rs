use super::test_framework::TestFramework;
use crate::endpoints;
use serde_json::Value;

const MODULE_NAME: &str = "cluster";

#[tokio::test]
async fn test_cluster_status() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    let endpoints = endpoints!(framework,
        "/cluster/status"
    );

    framework.test_endpoints(endpoints).await?;
    Ok(())
}

#[tokio::test]
async fn test_cluster_nodes() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    // First check cluster status through proxy
    let status_endpoint = framework.create_endpoint("/cluster/status");
    let response = framework.client
        .post(&format!("{}/cluster/status", framework.proxy_url))
        .headers(framework.headers.clone())
        .json(&status_endpoint.request_body.unwrap_or(serde_json::json!({})))
        .send()
        .await?
        .json::<Value>()
        .await?;

    // Only test nodes if cluster is enabled
    if let Some(data) = response.get("data") {
        if let Some(enabled) = data.get("enabled") {
            if enabled.as_bool().unwrap_or(false) {
                let endpoints = endpoints!(framework,
                    "/cluster/nodes"
                );
                framework.test_endpoints(endpoints).await?;
            } else {
                println!("Cluster is disabled, skipping nodes test");
            }
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_cluster_health() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    // First check cluster status through proxy
    let status_endpoint = framework.create_endpoint("/cluster/status");
    let response = framework.client
        .post(&format!("{}/cluster/status", framework.proxy_url))
        .headers(framework.headers.clone())
        .json(&status_endpoint.request_body.unwrap_or(serde_json::json!({})))
        .send()
        .await?
        .json::<Value>()
        .await?;

    // Only test health if cluster is enabled
    if let Some(data) = response.get("data") {
        if let Some(enabled) = data.get("enabled") {
            if enabled.as_bool().unwrap_or(false) {
                let endpoints = endpoints!(framework,
                    "/cluster/healthcheck"
                );
                framework.test_endpoints(endpoints).await?;
            } else {
                println!("Cluster is disabled, skipping health test");
            }
        }
    }

    Ok(())
}
