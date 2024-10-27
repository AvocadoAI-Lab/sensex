use super::common::{BASE_URL, get_test_client};
use serde_json::Value;

#[tokio::test]
async fn test_manager_info() {
    let (client, token) = get_test_client().await;

    println!("Getting manager info");
    let info_url = format!("{}/manager/info", BASE_URL);
    let response = client.get(&info_url, Some(&token))
        .await
        .expect("Should get manager info");
    
    let status = response.status();
    println!("Manager info status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get manager info successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw manager info response: {}", response_text);
    
    let info_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Manager Info Response: {:#?}", info_json);
}

#[tokio::test]
async fn test_manager_status() {
    let (client, token) = get_test_client().await;

    println!("Getting manager status");
    let status_url = format!("{}/manager/status", BASE_URL);
    let response = client.get(&status_url, Some(&token))
        .await
        .expect("Should get manager status");
    
    let status = response.status();
    println!("Manager status response status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get manager status successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw manager status response: {}", response_text);
    
    let status_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Manager Status Response: {:#?}", status_json);
}

#[tokio::test]
async fn test_manager_logs() {
    let (client, token) = get_test_client().await;

    println!("Getting manager logs");
    let logs_url = format!("{}/manager/logs", BASE_URL);
    let response = client.get(&logs_url, Some(&token))
        .await
        .expect("Should get manager logs");
    
    let status = response.status();
    println!("Manager logs status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get manager logs successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw manager logs response: {}", response_text);
    
    let logs_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Manager Logs Response: {:#?}", logs_json);
}
