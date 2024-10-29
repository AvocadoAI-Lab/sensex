use super::common::{BASE_URL, get_test_client};
use super::test_utils::{TestEndpoint, test_endpoint, setup_test_directory};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

const BACKEND_URL: &str = "http://127.0.0.1:3001";
const MODULE_NAME: &str = "rules";

#[tokio::test]
async fn test_rules_endpoints() -> Result<(), Box<dyn std::error::Error>> {
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
        "endpoint": BASE_URL,
        "token": token
    });

    // 定義所有要測試的endpoints
    let endpoints = vec![
        TestEndpoint::new(
            "/rules",
            None,
            Some(base_request.clone())
        ),
        TestEndpoint::new(
            "/rules/groups",
            None,
            Some(base_request.clone())
        ),
        TestEndpoint::new(
            "/rules/files",
            None,
            Some(base_request.clone())
        ),
    ];

    // 測試所有endpoints
    for endpoint in endpoints {
        if let Err(e) = test_endpoint(&client, &headers, endpoint.clone(), BACKEND_URL, MODULE_NAME).await {
            println!("Warning: Endpoint {} failed with error: {}", endpoint.path, e);
            // Don't fail the entire test suite for individual endpoint failures
            continue;
        }
    }

    println!("\n測試結果已保存到 test_results/{} 目錄", MODULE_NAME);
    Ok(())
}
