use super::common::{BASE_URL, get_test_client};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

const BACKEND_URL: &str = "http://127.0.0.1:3001";

#[tokio::test]
async fn test_manager_endpoints() {
    // 獲取認證 token
    let (_, token) = get_test_client().await;
    
    // 創建 HTTP client
    let client = Client::new();

    // 創建 headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());

    // 基本請求結構
    let base_request = serde_json::json!({
        "endpoint": BASE_URL,
        "token": token
    });

    // 1. 測試 GET / (API基本資訊)
    println!("\nTesting GET /");
    let response = client
        .post(&format!("{}/", BACKEND_URL))
        .headers(headers.clone())
        .json(&base_request)
        .send()
        .await
        .expect("Failed to get API info");
    assert_eq!(response.status().as_u16(), 200);

    // 2. 測試 GET /manager/status
    println!("\nTesting GET /manager/status");
    let response = client
        .post(&format!("{}/manager/status", BACKEND_URL))
        .headers(headers.clone())
        .json(&base_request)
        .send()
        .await
        .expect("Failed to get manager status");
    assert_eq!(response.status().as_u16(), 200);

    // 3. 測試 GET /manager/info
    println!("\nTesting GET /manager/info");
    let response = client
        .post(&format!("{}/manager/info", BACKEND_URL))
        .headers(headers.clone())
        .json(&base_request)
        .send()
        .await
        .expect("Failed to get manager info");
    assert_eq!(response.status().as_u16(), 200);

    // 4. 測試 GET /manager/configuration
    println!("\nTesting GET /manager/configuration");
    let response = client
        .post(&format!("{}/manager/configuration", BACKEND_URL))
        .headers(headers.clone())
        .json(&serde_json::json!({
            "endpoint": BASE_URL,
            "token": token,
            "params": {
                "section": "global",  // 測試特定配置段落
                "field": "alerts_log" // 測試特定配置欄位
            }
        }))
        .send()
        .await
        .expect("Failed to get manager configuration");
    assert_eq!(response.status().as_u16(), 200);

    // 5. 測試 GET /manager/stats
    println!("\nTesting GET /manager/stats");
    let response = client
        .post(&format!("{}/manager/stats", BACKEND_URL))
        .headers(headers.clone())
        .json(&serde_json::json!({
            "endpoint": BASE_URL,
            "token": token,
            "params": {
                "date": "now" // 使用now來獲取當前統計
            }
        }))
        .send()
        .await
        .expect("Failed to get manager stats");
    assert_eq!(response.status().as_u16(), 200);

    // 6. 測試 GET /manager/stats/hourly
    println!("\nTesting GET /manager/stats/hourly");
    let response = client
        .post(&format!("{}/manager/stats/hourly", BACKEND_URL))
        .headers(headers.clone())
        .json(&base_request)
        .send()
        .await
        .expect("Failed to get manager hourly stats");
    assert_eq!(response.status().as_u16(), 200);

    // 7. 測試 GET /manager/stats/weekly
    println!("\nTesting GET /manager/stats/weekly");
    let response = client
        .post(&format!("{}/manager/stats/weekly", BACKEND_URL))
        .headers(headers.clone())
        .json(&base_request)
        .send()
        .await
        .expect("Failed to get manager weekly stats");
    assert_eq!(response.status().as_u16(), 200);

    // 8. 測試 GET /manager/logs
    println!("\nTesting GET /manager/logs");
    let response = client
        .post(&format!("{}/manager/logs", BACKEND_URL))
        .headers(headers.clone())
        .json(&serde_json::json!({
            "endpoint": BASE_URL,
            "token": token,
            "params": {
                "offset": 0,
                "limit": 10,
                "q": "level=info",  // 測試日誌過濾
                "sort": "-timestamp" // 測試日誌排序
            }
        }))
        .send()
        .await
        .expect("Failed to get manager logs");
    assert_eq!(response.status().as_u16(), 200);

    // 9. 測試 GET /manager/logs/summary
    println!("\nTesting GET /manager/logs/summary");
    let response = client
        .post(&format!("{}/manager/logs/summary", BACKEND_URL))
        .headers(headers.clone())
        .json(&base_request)
        .send()
        .await
        .expect("Failed to get manager logs summary");
    assert_eq!(response.status().as_u16(), 200);
}
