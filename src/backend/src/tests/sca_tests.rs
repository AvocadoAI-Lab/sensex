use super::common::{BASE_URL, get_test_client, validate_response};
use serde_json::Value;

#[tokio::test]
async fn test_sca_policies() {
    let (client, token) = get_test_client().await;

    // 首先獲取一個 agent ID
    let agents_url = format!("{}/agents", BASE_URL);
    let response = client.get(&agents_url, Some(&token))
        .await
        .expect("Should get agents list");
    
    let response_text = response.text().await.expect("Should get response text");
    let agents_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    
    if let Some(agent_id) = get_first_agent_id(&agents_json) {
        println!("Getting SCA policies for agent: {}", agent_id);
        let policies_url = format!("{}/sca/{}/policies", BASE_URL, agent_id);
        let response = client.get(&policies_url, Some(&token))
            .await
            .expect("Should get SCA policies");
        
        let status = response.status();
        let response_text = response.text().await.expect("Should get response text");
        
        assert!(validate_response(
            "SCA Policies Test",
            status.as_u16(),
            &response_text
        ));
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
