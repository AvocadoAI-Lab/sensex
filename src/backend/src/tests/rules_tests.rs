use super::common::{BASE_URL, get_test_client};
use serde_json::Value;

#[tokio::test]
async fn test_rules_list() {
    let (client, token) = get_test_client().await;

    println!("Getting rules list");
    let rules_url = format!("{}/rules", BASE_URL);
    let response = client.get(&rules_url, Some(&token))
        .await
        .expect("Should get rules list");
    
    let status = response.status();
    println!("Rules list status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get rules list successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw rules response: {}", response_text);
    
    let rules_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Rules List Response: {:#?}", rules_json);
}

#[tokio::test]
async fn test_rules_groups() {
    let (client, token) = get_test_client().await;

    println!("Getting rules groups");
    let groups_url = format!("{}/rules/groups", BASE_URL);
    let response = client.get(&groups_url, Some(&token))
        .await
        .expect("Should get rules groups");
    
    let status = response.status();
    println!("Rules groups status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get rules groups successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw rules groups response: {}", response_text);
    
    let groups_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Rules Groups Response: {:#?}", groups_json);
}

#[tokio::test]
async fn test_rules_files() {
    let (client, token) = get_test_client().await;

    println!("Getting rules files");
    let files_url = format!("{}/rules/files", BASE_URL);
    let response = client.get(&files_url, Some(&token))
        .await
        .expect("Should get rules files");
    
    let status = response.status();
    println!("Rules files status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get rules files successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw rules files response: {}", response_text);
    
    let files_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Rules Files Response: {:#?}", files_json);
}
