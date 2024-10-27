use super::common::{BASE_URL, get_test_client, validate_response};
use serde_json::Value;

#[tokio::test]
async fn test_tasks_list() {
    let (client, token) = get_test_client().await;

    println!("Getting tasks list");
    let list_url = format!("{}/tasks", BASE_URL);
    let response = client.get(&list_url, Some(&token))
        .await
        .expect("Should get tasks list");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "Tasks List Test",
        status.as_u16(),
        &response_text
    ));
}

#[tokio::test]
async fn test_tasks_status() {
    let (client, token) = get_test_client().await;

    println!("Getting tasks status");
    let status_url = format!("{}/tasks/status", BASE_URL);
    let response = client.get(&status_url, Some(&token))
        .await
        .expect("Should get tasks status");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "Tasks Status Test",
        status.as_u16(),
        &response_text
    ));
}
