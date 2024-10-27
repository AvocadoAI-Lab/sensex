use super::common::{BASE_URL, get_test_client};
use serde_json::Value;

#[tokio::test]
async fn test_lists_files() {
    let (client, token) = get_test_client().await;

    println!("Getting CDB lists files");
    let files_url = format!("{}/lists/files", BASE_URL);
    let response = client.get(&files_url, Some(&token))
        .await
        .expect("Should get CDB lists files");
    
    let status = response.status();
    println!("CDB lists files status: {}", status);
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw CDB lists files response: {}", response_text);
    
    assert!(status.is_success(), "Should successfully get CDB lists files");
}

#[tokio::test]
async fn test_lists_content() {
    let (client, token) = get_test_client().await;

    // 首先獲取列表文件
    let files_url = format!("{}/lists/files", BASE_URL);
    let response = client.get(&files_url, Some(&token))
        .await
        .expect("Should get CDB lists files");
    
    let response_text = response.text().await.expect("Should get response text");
    let files_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    
    if let Some(filename) = get_first_list_filename(&files_json) {
        println!("Getting content for list: {}", filename);
        let content_url = format!("{}/lists/files/{}", BASE_URL, filename);
        let response = client.get(&content_url, Some(&token))
            .await
            .expect("Should get list content");
        
        let status = response.status();
        println!("List content status: {}", status);
        
        let response_text = response.text().await.expect("Should get response text");
        println!("Raw list content response: {}", response_text);
        
        assert!(status.is_success(), "Should successfully get list content");
    }
}

// 輔助函數：從回應中獲取第一個列表文件名
fn get_first_list_filename(json: &Value) -> Option<String> {
    json.get("data")?
        .get("affected_items")?
        .as_array()?
        .first()?
        .get("filename")?
        .as_str()
        .map(String::from)
}
