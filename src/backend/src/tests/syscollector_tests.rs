use super::common::{BASE_URL, get_test_client};
use serde_json::Value;

#[tokio::test]
async fn test_syscollector_hardware() {
    let (client, token) = get_test_client().await;

    // 首先獲取一個 agent ID
    let agents_url = format!("{}/agents", BASE_URL);
    let response = client.get(&agents_url, Some(&token))
        .await
        .expect("Should get agents list");
    
    let response_text = response.text().await.expect("Should get response text");
    let agents_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    
    if let Some(agent_id) = get_first_agent_id(&agents_json) {
        println!("Getting hardware info for agent: {}", agent_id);
        let hardware_url = format!("{}/syscollector/{}/hardware", BASE_URL, agent_id);
        let response = client.get(&hardware_url, Some(&token))
            .await
            .expect("Should get hardware info");
        
        let status = response.status();
        println!("Hardware info status: {}", status);
        
        let response_text = response.text().await.expect("Should get response text");
        println!("Raw hardware response: {}", response_text);
        
        if status.as_u16() == 200 {
            let hardware_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
            println!("Hardware Info Response: {:#?}", hardware_json);
        }
    }
}

#[tokio::test]
async fn test_syscollector_os() {
    let (client, token) = get_test_client().await;

    // 首先獲取一個 agent ID
    let agents_url = format!("{}/agents", BASE_URL);
    let response = client.get(&agents_url, Some(&token))
        .await
        .expect("Should get agents list");
    
    let response_text = response.text().await.expect("Should get response text");
    let agents_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    
    if let Some(agent_id) = get_first_agent_id(&agents_json) {
        println!("Getting OS info for agent: {}", agent_id);
        let os_url = format!("{}/syscollector/{}/os", BASE_URL, agent_id);
        let response = client.get(&os_url, Some(&token))
            .await
            .expect("Should get OS info");
        
        let status = response.status();
        println!("OS info status: {}", status);
        
        let response_text = response.text().await.expect("Should get response text");
        println!("Raw OS response: {}", response_text);
        
        if status.as_u16() == 200 {
            let os_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
            println!("OS Info Response: {:#?}", os_json);
        }
    }
}

#[tokio::test]
async fn test_syscollector_packages() {
    let (client, token) = get_test_client().await;

    // 首先獲取一個 agent ID
    let agents_url = format!("{}/agents", BASE_URL);
    let response = client.get(&agents_url, Some(&token))
        .await
        .expect("Should get agents list");
    
    let response_text = response.text().await.expect("Should get response text");
    let agents_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    
    if let Some(agent_id) = get_first_agent_id(&agents_json) {
        println!("Getting packages info for agent: {}", agent_id);
        let packages_url = format!("{}/syscollector/{}/packages", BASE_URL, agent_id);
        let response = client.get(&packages_url, Some(&token))
            .await
            .expect("Should get packages info");
        
        let status = response.status();
        println!("Packages info status: {}", status);
        
        let response_text = response.text().await.expect("Should get response text");
        println!("Raw packages response: {}", response_text);
        
        if status.as_u16() == 200 {
            let packages_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
            println!("Packages Info Response: {:#?}", packages_json);
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
