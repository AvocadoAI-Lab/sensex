use super::common::{BASE_URL, get_test_client};
use serde_json::Value;

#[tokio::test]
async fn test_rootcheck_database() {
    let (client, token) = get_test_client().await;

    // 首先獲取一個 agent ID
    let agents_url = format!("{}/agents", BASE_URL);
    let response = client.get(&agents_url, Some(&token))
        .await
        .expect("Should get agents list");
    
    let response_text = response.text().await.expect("Should get response text");
    let agents_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    
    if let Some(agent_id) = get_first_agent_id(&agents_json) {
        println!("Getting rootcheck database for agent: {}", agent_id);
        let database_url = format!("{}/rootcheck/{}", BASE_URL, agent_id);
        let response = client.get(&database_url, Some(&token))
            .await
            .expect("Should get rootcheck database");
        
        let status = response.status();
        println!("Rootcheck database status: {}", status);
        
        let response_text = response.text().await.expect("Should get response text");
        println!("Raw rootcheck database response: {}", response_text);
        
        if status.is_success() {
            let database_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
            println!("Rootcheck Database Response: {:#?}", database_json);
        }
    }
}

#[tokio::test]
async fn test_rootcheck_last_scan() {
    let (client, token) = get_test_client().await;

    // 首先獲取一個 agent ID
    let agents_url = format!("{}/agents", BASE_URL);
    let response = client.get(&agents_url, Some(&token))
        .await
        .expect("Should get agents list");
    
    let response_text = response.text().await.expect("Should get response text");
    let agents_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    
    if let Some(agent_id) = get_first_agent_id(&agents_json) {
        println!("Getting rootcheck last scan for agent: {}", agent_id);
        let last_scan_url = format!("{}/rootcheck/{}/last_scan", BASE_URL, agent_id);
        let response = client.get(&last_scan_url, Some(&token))
            .await
            .expect("Should get rootcheck last scan");
        
        let status = response.status();
        println!("Rootcheck last scan status: {}", status);
        
        let response_text = response.text().await.expect("Should get response text");
        println!("Raw rootcheck last scan response: {}", response_text);
        
        if status.is_success() {
            let last_scan_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
            println!("Rootcheck Last Scan Response: {:#?}", last_scan_json);
        }
    }
}

// 輔助函數：從回應中獲取第一個 agent 的 ID
fn get_first_agent_id(json: &Value) -> Option<String> {
    json.get("data")?
        .get("affected_items")?
        .as_array()?
        .first()?
        .get("id")?
        .as_str()
        .map(String::from)
}
