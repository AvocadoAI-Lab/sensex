use serde_json::Value;
use super::common::{BASE_URL, get_test_client};

#[tokio::test]
async fn test_agents_flow() {
    let (client, token) = get_test_client().await;

    // 1. 獲取所有 agents 列表
    println!("Step 1: Getting all agents");
    let agents_url = format!("{}/agents", BASE_URL);
    let response = client.get(&agents_url, Some(&token))
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

    // 2. 獲取特定 agent 的詳細資訊
    if let Some(agent_id) = get_first_agent_id(&agents_json) {
        println!("\nStep 2: Getting specific agent details for ID: {}", agent_id);
        let agent_url = format!("{}/agents/{}", BASE_URL, agent_id);
        let response = client.get(&agent_url, Some(&token))
            .await
            .expect("Should get agent details");
        
        let status = response.status();
        println!("Agent details status: {}", status);
        
        let response_text = response.text().await.expect("Should get response text");
        println!("Raw agent details: {}", response_text);
        
        if status.as_u16() == 200 {
            let agent_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
            println!("Agent Details Response: {:#?}", agent_json);

            // 3. 獲取 agent 的配置
            println!("\nStep 3: Getting agent configuration");
            let config_url = format!("{}/agents/{}/config/syscollector", BASE_URL, agent_id);
            let response = client.get(&config_url, Some(&token))
                .await
                .expect("Should get agent config");
            
            let status = response.status();
            println!("Agent config status: {}", status);
            
            let response_text = response.text().await.expect("Should get response text");
            println!("Raw agent config: {}", response_text);
            
            if status.as_u16() == 200 {
                let config_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
                println!("Agent Config Response: {:#?}", config_json);
            } else {
                println!("Agent config not available. Status: {}", status);
            }
        } else {
            println!("Agent details not available. Status: {}", status);
        }
    } else {
        println!("No valid agents found in the response");
    }

    // 4. 測試 agents 統計資訊
    println!("\nStep 4: Getting agents summary");
    let summary_url = format!("{}/agents/summary/status", BASE_URL);
    let response = client.get(&summary_url, Some(&token))
        .await
        .expect("Should get agents summary");
    
    let status = response.status();
    println!("Summary status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get summary successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw summary: {}", response_text);
    
    let summary_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Agents Summary Response: {:#?}", summary_json);
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
