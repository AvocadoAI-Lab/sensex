use super::common::{WAZUH_URL, PROXY_URL, get_test_client, TEST_AGENT_ID};
use super::test_utils::{TestEndpoint, test_endpoint, setup_test_directory};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

const MODULE_NAME: &str = "syscollector";

#[tokio::test]
async fn test_syscollector_endpoints() -> Result<(), Box<dyn std::error::Error>> {
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

    // 基本請求結構，包含agent_id參數
    let base_request = serde_json::json!({
        "endpoint": WAZUH_URL,
        "token": token,
        "params": {
            "agent_id": TEST_AGENT_ID
        }
    });

    // 定義所有要測試的endpoints
    let endpoints = vec![
        TestEndpoint::new(
            &format!("/syscollector/{}/hardware", TEST_AGENT_ID),
            Some("agent_id"),
            Some(base_request.clone())
        ),
        TestEndpoint::new(
            &format!("/syscollector/{}/hotfixes", TEST_AGENT_ID),
            Some("agent_id"),
            Some(base_request.clone())
        ),
        TestEndpoint::new(
            &format!("/syscollector/{}/netaddr", TEST_AGENT_ID),
            Some("agent_id"),
            Some(base_request.clone())
        ),
        TestEndpoint::new(
            &format!("/syscollector/{}/netiface", TEST_AGENT_ID),
            Some("agent_id"),
            Some(base_request.clone())
        ),
        TestEndpoint::new(
            &format!("/syscollector/{}/netproto", TEST_AGENT_ID),
            Some("agent_id"),
            Some(base_request.clone())
        ),
        TestEndpoint::new(
            &format!("/syscollector/{}/os", TEST_AGENT_ID),
            Some("agent_id"),
            Some(base_request.clone())
        ),
        TestEndpoint::new(
            &format!("/syscollector/{}/packages", TEST_AGENT_ID),
            Some("agent_id"),
            Some(base_request.clone())
        ),
        TestEndpoint::new(
            &format!("/syscollector/{}/ports", TEST_AGENT_ID),
            Some("agent_id"),
            Some(base_request.clone())
        ),
        TestEndpoint::new(
            &format!("/syscollector/{}/processes", TEST_AGENT_ID),
            Some("agent_id"),
            Some(base_request.clone())
        ),
    ];

    // 測試所有endpoints
    for endpoint in endpoints {
        if let Err(e) = test_endpoint(&client, &headers, endpoint.clone(), PROXY_URL, MODULE_NAME).await {
            println!("Warning: Endpoint {} failed with error: {}", endpoint.path, e);
            continue;
        }
    }

    println!("\n測試結果已保存到 test_results/{} 目錄", MODULE_NAME);
    Ok(())
}
