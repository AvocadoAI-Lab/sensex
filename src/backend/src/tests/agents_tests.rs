use super::common::{WAZUH_URL, PROXY_URL, get_test_client, TEST_AGENT_ID};
use super::test_utils::{TestEndpoint, test_endpoint, setup_test_directory};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

const MODULE_NAME: &str = "agents";

#[tokio::test]
async fn test_agents_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    // 設置測試目錄
    setup_test_directory(MODULE_NAME)?;

    // 獲取認證 token
    let (_, token) = get_test_client().await;
    
    // 創建 HTTP client
    let client = Client::new();

    // 創建 headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", token))?);

    // 基本請求結構
    let base_request = serde_json::json!({
        "endpoint": WAZUH_URL,
        "token": token
    });

    // 定義所有要測試的endpoints
    let mut endpoints = vec![
        TestEndpoint::new("/agents", None, Some(base_request.clone())),
    ];

    // Add tests for each valid agent configuration
    for config in ["buffer", "internal", "client", "labels"] {
        endpoints.push(
            TestEndpoint::new(
                &format!("/agents/{}/config/agent/{}", TEST_AGENT_ID, config),
                Some("agent_id, component, configuration"),
                Some(serde_json::json!({
                    "endpoint": WAZUH_URL,
                    "token": token,
                    "params": {
                        "agent_id": TEST_AGENT_ID,
                        "component": "agent",
                        "configuration": config
                    }
                }))
            )
        );
    }

    // Add tests for each valid stats component
    for component in ["logcollector", "agent"] {
        endpoints.push(
            TestEndpoint::new(
                &format!("/agents/{}/stats/{}", TEST_AGENT_ID, component),
                Some("agent_id, component"),
                Some(serde_json::json!({
                    "endpoint": WAZUH_URL,
                    "token": token,
                    "params": {
                        "agent_id": TEST_AGENT_ID,
                        "component": component
                    }
                }))
            )
        );
    }

    // Add remaining endpoints
    endpoints.extend(vec![
        TestEndpoint::new(
            &format!("/agents/{}/group/is_sync", TEST_AGENT_ID),
            Some("agent_id"),
            Some(serde_json::json!({
                "endpoint": WAZUH_URL,
                "token": token,
                "params": {
                    "agent_id": TEST_AGENT_ID
                }
            }))
        ),
        TestEndpoint::new(
            &format!("/agents/{}/daemons/stats", TEST_AGENT_ID),
            Some("agent_id"),
            Some(serde_json::json!({
                "endpoint": WAZUH_URL,
                "token": token,
                "params": {
                    "agent_id": TEST_AGENT_ID
                }
            }))
        ),
        TestEndpoint::new("/agents/no_group", None, Some(base_request.clone())),
        TestEndpoint::new("/agents/stats/distinct", None, Some(base_request.clone())),
        TestEndpoint::new("/agents/summary/os", None, Some(base_request.clone())),
        TestEndpoint::new("/agents/summary/status", None, Some(base_request.clone())),
    ]);

    // 測試所有endpoints
    for endpoint in endpoints {
        if let Err(e) = test_endpoint(&client, &headers, endpoint.clone(), PROXY_URL, MODULE_NAME).await {
            println!("Warning: Endpoint {} failed with error: {}", endpoint.path, e);
            continue;
        }
    }

    println!("\n測試結果和結構分析已保存到 test_results/{} 目錄", MODULE_NAME);
    Ok(())
}
