use serde_json::Value;
use super::common::{BASE_URL, get_test_client};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

const BACKEND_URL: &str = "http://127.0.0.1:3001";

#[tokio::test]
async fn test_agents_flow() {
    // 獲取認證 token
    let (_, token) = get_test_client().await;
    
    // 創建 HTTP client
    let client = Client::new();

    // 創建 headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // 1. 獲取所有 agents 列表
    println!("Step 1: Getting all agents");
    let response = client
        .post(&format!("{}/agents", BACKEND_URL))
        .headers(headers.clone())
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "endpoint": BASE_URL,
            "token": token
        }))
        .send()
        .await
        .expect("Should get agents list");
    
    let status = response.status();
    println!("Agents list status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get agents list successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw agents response: {}", response_text);
    
    let agents_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Agents List Response: {:#?}", agents_json);

    // 檢查回應結構
    assert!(agents_json.is_object(), "Response should be a JSON object");
    assert!(agents_json.get("data").is_some(), "Response should have 'data' field");

    // 獲取第一個 agent ID
    let first_agent_id = get_first_agent_id(&agents_json);

    // 2. 獲取特定 agent 的詳細資訊
    if let Some(agent_id) = first_agent_id {
        println!("\nStep 2: Getting specific agent details for ID: {}", agent_id);
        let response = client
            .post(&format!("{}/agents", BACKEND_URL))
            .headers(headers.clone())
            .header("Authorization", format!("Bearer {}", token))
            .json(&serde_json::json!({
                "endpoint": BASE_URL,
                "token": token,
                "params": {
                    "agents_list": agent_id
                }
            }))
            .send()
            .await
            .expect("Should get agent details");
        
        let status = response.status();
        println!("Agent details status: {}", status);
        
        let response_text = response.text().await.expect("Should get response text");
        println!("Raw agent details: {}", response_text);
        
        let agent_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
        println!("Agent Details Response: {:#?}", agent_json);
    } else {
        println!("No valid agents found in the response");
    }
}

// 輔助函數：從回應中獲取第一個 agent 的 ID
fn get_first_agent_id(json: &Value) -> Option<String> {
    // 首先檢查並打印完整的 JSON 結構
    println!("Checking JSON structure for agent ID:");
    println!("{:#?}", json);

    // 嘗試不同的路徑來找到 agent ID
    let id = if let Some(items) = json.get("data").and_then(|data| data.get("affected_items")) {
        if let Some(array) = items.as_array() {
            if let Some(first) = array.first() {
                first.get("id").and_then(|id| id.as_str()).map(String::from)
            } else {
                println!("No items in the array");
                None
            }
        } else {
            println!("affected_items is not an array");
            None
        }
    } else {
        println!("Could not find data.affected_items path");
        None
    };

    // 打印找到的 ID
    if let Some(ref found_id) = id {
        println!("Found agent ID: {}", found_id);
    } else {
        println!("No agent ID found");
    }

    id
}
