use super::common::{BASE_URL, get_test_client, validate_response};

#[tokio::test]
async fn test_events_list() {
    let (client, token) = get_test_client().await;

    // 首先獲取一個 agent ID
    let agents_url = format!("{}/agents", BASE_URL);
    let response = client.get(&agents_url, Some(&token))
        .await
        .expect("Should get agents list");
    
    let response_text = response.text().await.expect("Should get response text");
    let agents_json: serde_json::Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    
    if let Some(agent_id) = get_first_agent_id(&agents_json) {
        println!("Getting syscheck events for agent: {}", agent_id);
        let events_url = format!("{}/syscheck/{}/last_scan", BASE_URL, agent_id);
        let response = client.get(&events_url, Some(&token))
            .await
            .expect("Should get events");
        
        let status = response.status();
        let response_text = response.text().await.expect("Should get response text");
        
        assert!(validate_response(
            "Events List Test",
            status.as_u16(),
            &response_text
        ));
    } else {
        println!("No agents found, skipping events test");
    }
}

#[tokio::test]
async fn test_events_summary() {
    let (client, token) = get_test_client().await;

    println!("Getting events summary");
    let summary_url = format!("{}/overview/events", BASE_URL);
    let response = client.get(&summary_url, Some(&token))
        .await
        .expect("Should get events summary");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "Events Summary Test",
        status.as_u16(),
        &response_text
    ));
}

#[tokio::test]
async fn test_events_alerts() {
    let (client, token) = get_test_client().await;

    println!("Getting events alerts");
    let alerts_url = format!("{}/alerts", BASE_URL);
    let response = client.get(&alerts_url, Some(&token))
        .await
        .expect("Should get events alerts");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "Events Alerts Test",
        status.as_u16(),
        &response_text
    ));
}

// 輔助函數：從回應中獲取第一個 agent 的 ID
fn get_first_agent_id(json: &serde_json::Value) -> Option<String> {
    json.get("data")?
        .get("affected_items")?
        .as_array()?
        .first()?
        .get("id")?
        .as_str()
        .map(String::from)
}
