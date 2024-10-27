use super::common::{BASE_URL, get_test_client};
use serde_json::Value;

#[tokio::test]
async fn test_decoders_list() {
    let (client, token) = get_test_client().await;

    println!("Getting decoders list");
    let decoders_url = format!("{}/decoders", BASE_URL);
    let response = client.get(&decoders_url, Some(&token))
        .await
        .expect("Should get decoders list");
    
    let status = response.status();
    println!("Decoders list status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get decoders list successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw decoders response: {}", response_text);
    
    let decoders_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Decoders List Response: {:#?}", decoders_json);
}

#[tokio::test]
async fn test_decoders_files() {
    let (client, token) = get_test_client().await;

    println!("Getting decoders files");
    let files_url = format!("{}/decoders/files", BASE_URL);
    let response = client.get(&files_url, Some(&token))
        .await
        .expect("Should get decoders files");
    
    let status = response.status();
    println!("Decoders files status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get decoders files successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw decoders files response: {}", response_text);
    
    let files_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Decoders Files Response: {:#?}", files_json);
}

#[tokio::test]
async fn test_decoders_parents() {
    let (client, token) = get_test_client().await;

    println!("Getting decoders parents");
    let parents_url = format!("{}/decoders/parents", BASE_URL);
    let response = client.get(&parents_url, Some(&token))
        .await
        .expect("Should get decoders parents");
    
    let status = response.status();
    println!("Decoders parents status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get decoders parents successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw decoders parents response: {}", response_text);
    
    let parents_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Decoders Parents Response: {:#?}", parents_json);
}
