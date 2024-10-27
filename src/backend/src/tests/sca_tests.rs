use super::common::{BASE_URL, get_test_client};
use serde_json::Value;

#[tokio::test]
async fn test_sca_checks() {
    let (client, token) = get_test_client().await;

    // 首先獲取一個 agent ID
    let agents_url = format!("{}/agents", BASE_URL);
    let response = client.get(&agents_url, Some(&token))
        .await
        .expect("Should get agents list");
    
    let response_text = response.text().await.expect("Should get response text");
    let agents_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    
    if let Some(agent_id) = get_first_agent_id(&agents_json) {
        println!("Getting SCA checks for agent: {}", agent_id);
        let sca_url = format!("{}/sca/{}/checks", BASE_URL, agent_id);
        let response = client.get(&sca_url, Some(&token))
            .await
            .expect("Should get SCA checks");
        
        let status = response.status();
        println!("SCA checks status: {}", status);
        
        let response_text = response.text().await.expect("Should get response text");
        println!("Raw SCA checks response: {}", response_text);
        
        if status.as_u16() == 200 {
            let sca_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
            println!("SCA Checks Response: {:#?}", sca_json);
        }
    }
}

#[tokio::test]
async fn test_sca_policies() {
    let (client, token) = get_test_client().await;

    println!("Getting SCA policies");
    let policies_url = format!("{}/sca/policies", BASE_URL);
    let response = client.get(&policies_url, Some(&token))
        .await
        .expect("Should get SCA policies");
    
    let status = response.status();
    println!("SCA policies status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get policies successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw SCA policies response: {}", response_text);
    
    let policies_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("SCA Policies Response: {:#?}", policies_json);
}

#[tokio::test]
async fn test_sca_results() {
    let (client, token) = get_test_client().await;

    // 首先獲取一個 agent ID
    let agents_url = format!("{}/agents", BASE_URL);
    let response = client.get(&agents_url, Some(&token))
        .await
        .expect("Should get agents list");
    
    let response_text = response.text().await.expect("Should get response text");
    let agents_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    
    if let Some(agent_id) = get_first_agent_id(&agents_json) {
        println!("Getting SCA results for agent: {}", agent_id);
        let results_url = format!("{}/sca/{}/results", BASE_URL, agent_id);
        let response = client.get(&results_url, Some(&token))
            .await
            .expect("Should get SCA results");
        
        let status = response.status();
        println!("SCA results status: {}", status);
        
        let response_text = response.text().await.expect("Should get response text");
        println!("Raw SCA results response: {}", response_text);
        
        if status.as_u16() == 200 {
            let results_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
            println!("SCA Results Response: {:#?}", results_json);
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
