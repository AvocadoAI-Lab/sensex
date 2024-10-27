use super::common::{BASE_URL, get_test_client};
use serde_json::Value;

#[tokio::test]
async fn test_syscheck_files() {
    let (client, token) = get_test_client().await;

    // 首先獲取一個 agent ID
    let agents_url = format!("{}/agents", BASE_URL);
    let response = client.get(&agents_url, Some(&token))
        .await
        .expect("Should get agents list");
    
    let response_text = response.text().await.expect("Should get response text");
    let agents_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    
    if let Some(agent_id) = get_first_agent_id(&agents_json) {
        println!("Getting syscheck files for agent: {}", agent_id);
        let files_url = format!("{}/syscheck/{}", BASE_URL, agent_id);
        let response = client.get(&files_url, Some(&token))
            .await
            .expect("Should get syscheck files");
        
        let status = response.status();
        println!("Syscheck files status: {}", status);
        
        let response_text = response.text().await.expect("Should get response text");
        println!("Raw syscheck files response: {}", response_text);
        
        if status.is_success() {
            let files_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
            println!("Syscheck Files Response: {:#?}", files_json);
        }
    }
}

#[tokio::test]
async fn test_syscheck_last_scan() {
    let (client, token) = get_test_client().await;

    // 首先獲取一個 agent ID
    let agents_url = format!("{}/agents", BASE_URL);
    let response = client.get(&agents_url, Some(&token))
        .await
        .expect("Should get agents list");
    
    let response_text = response.text().await.expect("Should get response text");
    let agents_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    
    if let Some(agent_id) = get_first_agent_id(&agents_json) {
        println!("Getting syscheck last scan for agent: {}", agent_id);
        let last_scan_url = format!("{}/syscheck/{}/last_scan", BASE_URL, agent_id);
        let response = client.get(&last_scan_url, Some(&token))
            .await
            .expect("Should get syscheck last scan");
        
        let status = response.status();
        println!("Syscheck last scan status: {}", status);
        
        let response_text = response.text().await.expect("Should get response text");
        println!("Raw syscheck last scan response: {}", response_text);
        
        if status.is_success() {
            let last_scan_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
            println!("Syscheck Last Scan Response: {:#?}", last_scan_json);
        }
    }
}

#[tokio::test]
async fn test_syscheck_summary() {
    let (client, token) = get_test_client().await;

    // 首先獲取一個 agent ID
    let agents_url = format!("{}/agents", BASE_URL);
    let response = client.get(&agents_url, Some(&token))
        .await
        .expect("Should get agents list");
    
    let response_text = response.text().await.expect("Should get response text");
    let agents_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    
    if let Some(agent_id) = get_first_agent_id(&agents_json) {
        println!("Getting syscheck summary for agent: {}", agent_id);
        let summary_url = format!("{}/syscheck/{}/summary", BASE_URL, agent_id);
        let response = client.get(&summary_url, Some(&token))
            .await
            .expect("Should get syscheck summary");
        
        let status = response.status();
        println!("Syscheck summary status: {}", status);
        
        let response_text = response.text().await.expect("Should get response text");
        println!("Raw syscheck summary response: {}", response_text);
        
        if status.is_success() {
            let summary_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
            println!("Syscheck Summary Response: {:#?}", summary_json);
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
