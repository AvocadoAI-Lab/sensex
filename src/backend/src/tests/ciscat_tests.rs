use super::common::{BASE_URL, get_test_client};
use serde_json::Value;

#[tokio::test]
async fn test_ciscat_results() {
    let (client, token) = get_test_client().await;

    // 首先獲取一個 agent ID
    let agents_url = format!("{}/agents", BASE_URL);
    let response = client.get(&agents_url, Some(&token))
        .await
        .expect("Should get agents list");
    
    let response_text = response.text().await.expect("Should get response text");
    let agents_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    
    if let Some(agent_id) = get_first_agent_id(&agents_json) {
        println!("Getting CIS-CAT results for agent: {}", agent_id);
        let results_url = format!("{}/ciscat/{}/results", BASE_URL, agent_id);
        let response = client.get(&results_url, Some(&token))
            .await
            .expect("Should get CIS-CAT results");
        
        let status = response.status();
        println!("CIS-CAT results status: {}", status);
        
        let response_text = response.text().await.expect("Should get response text");
        println!("Raw CIS-CAT results response: {}", response_text);
        
        if status.is_success() {
            let results_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
            println!("CIS-CAT Results Response: {:#?}", results_json);
        }
    }
}

#[tokio::test]
async fn test_ciscat_scan_info() {
    let (client, token) = get_test_client().await;

    // 首先獲取一個 agent ID
    let agents_url = format!("{}/agents", BASE_URL);
    let response = client.get(&agents_url, Some(&token))
        .await
        .expect("Should get agents list");
    
    let response_text = response.text().await.expect("Should get response text");
    let agents_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    
    if let Some(agent_id) = get_first_agent_id(&agents_json) {
        println!("Getting CIS-CAT scan info for agent: {}", agent_id);
        let scan_url = format!("{}/ciscat/{}/scan_info", BASE_URL, agent_id);
        let response = client.get(&scan_url, Some(&token))
            .await
            .expect("Should get CIS-CAT scan info");
        
        let status = response.status();
        println!("CIS-CAT scan info status: {}", status);
        
        let response_text = response.text().await.expect("Should get response text");
        println!("Raw CIS-CAT scan info response: {}", response_text);
        
        if status.is_success() {
            let scan_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
            println!("CIS-CAT Scan Info Response: {:#?}", scan_json);
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
