use super::common::{BASE_URL, get_test_client};
use serde_json::Value;

#[tokio::test]
async fn test_mitre_metadata() {
    let (client, token) = get_test_client().await;

    println!("Getting MITRE metadata");
    let metadata_url = format!("{}/mitre/metadata", BASE_URL);
    let response = client.get(&metadata_url, Some(&token))
        .await
        .expect("Should get MITRE metadata");
    
    let status = response.status();
    println!("MITRE metadata status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get metadata successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw MITRE metadata response: {}", response_text);
    
    let metadata_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("MITRE Metadata Response: {:#?}", metadata_json);
}

#[tokio::test]
async fn test_mitre_tactics() {
    let (client, token) = get_test_client().await;

    println!("Getting MITRE tactics");
    let tactics_url = format!("{}/mitre/tactics", BASE_URL);
    let response = client.get(&tactics_url, Some(&token))
        .await
        .expect("Should get MITRE tactics");
    
    let status = response.status();
    println!("MITRE tactics status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get tactics successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw MITRE tactics response: {}", response_text);
    
    let tactics_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("MITRE Tactics Response: {:#?}", tactics_json);
}

#[tokio::test]
async fn test_mitre_techniques() {
    let (client, token) = get_test_client().await;

    println!("Getting MITRE techniques");
    let techniques_url = format!("{}/mitre/techniques", BASE_URL);
    let response = client.get(&techniques_url, Some(&token))
        .await
        .expect("Should get MITRE techniques");
    
    let status = response.status();
    println!("MITRE techniques status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get techniques successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw MITRE techniques response: {}", response_text);
    
    let techniques_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("MITRE Techniques Response: {:#?}", techniques_json);
}

#[tokio::test]
async fn test_mitre_references() {
    let (client, token) = get_test_client().await;

    println!("Getting MITRE references");
    let references_url = format!("{}/mitre/references", BASE_URL);
    let response = client.get(&references_url, Some(&token))
        .await
        .expect("Should get MITRE references");
    
    let status = response.status();
    println!("MITRE references status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get references successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw MITRE references response: {}", response_text);
    
    let references_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("MITRE References Response: {:#?}", references_json);
}
