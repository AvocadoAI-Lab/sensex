use super::common::{BASE_URL, get_test_client};
use serde_json::Value;

#[tokio::test]
async fn test_groups_list() {
    let (client, token) = get_test_client().await;

    println!("Getting groups list");
    let groups_url = format!("{}/groups", BASE_URL);
    let response = client.get(&groups_url, Some(&token))
        .await
        .expect("Should get groups list");
    
    let status = response.status();
    println!("Groups list status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get groups list successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw groups response: {}", response_text);
    
    let groups_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Groups List Response: {:#?}", groups_json);
}

#[tokio::test]
async fn test_group_files() {
    let (client, token) = get_test_client().await;

    // 1. 先獲取組列表
    let groups_url = format!("{}/groups", BASE_URL);
    let response = client.get(&groups_url, Some(&token))
        .await
        .expect("Should get groups list");
    
    let response_text = response.text().await.expect("Should get response text");
    let groups_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");

    // 2. 獲取第一個組的文件
    if let Some(group_id) = get_first_group_id(&groups_json) {
        println!("\nGetting files for group: {}", group_id);
        let files_url = format!("{}/groups/{}/files", BASE_URL, group_id);
        let response = client.get(&files_url, Some(&token))
            .await
            .expect("Should get group files");
        
        let status = response.status();
        println!("Group files status: {}", status);
        
        let response_text = response.text().await.expect("Should get response text");
        println!("Raw group files response: {}", response_text);
        
        if status.as_u16() == 200 {
            let files_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
            println!("Group Files Response: {:#?}", files_json);
        } else {
            println!("Group files not available. Status: {}", status);
        }
    } else {
        println!("No groups found in the response");
    }
}

#[tokio::test]
async fn test_group_agents() {
    let (client, token) = get_test_client().await;

    // 1. 先獲取組列表
    let groups_url = format!("{}/groups", BASE_URL);
    let response = client.get(&groups_url, Some(&token))
        .await
        .expect("Should get groups list");
    
    let response_text = response.text().await.expect("Should get response text");
    let groups_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");

    // 2. 獲取第一個組的代理列表
    if let Some(group_id) = get_first_group_id(&groups_json) {
        println!("\nGetting agents for group: {}", group_id);
        let agents_url = format!("{}/groups/{}/agents", BASE_URL, group_id);
        let response = client.get(&agents_url, Some(&token))
            .await
            .expect("Should get group agents");
        
        let status = response.status();
        println!("Group agents status: {}", status);
        
        let response_text = response.text().await.expect("Should get response text");
        println!("Raw group agents response: {}", response_text);
        
        if status.as_u16() == 200 {
            let agents_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
            println!("Group Agents Response: {:#?}", agents_json);
        } else {
            println!("Group agents not available. Status: {}", status);
        }
    } else {
        println!("No groups found in the response");
    }
}

// 輔助函數：從回應中獲取第一個 group 的 ID
fn get_first_group_id(json: &Value) -> Option<String> {
    println!("Checking JSON structure for group ID:");
    println!("{:#?}", json);

    let id = if let Some(items) = json.get("data").and_then(|data| data.get("affected_items")) {
        if let Some(array) = items.as_array() {
            if let Some(first) = array.first() {
                first.get("name").and_then(|id| id.as_str()).map(String::from)
            } else {
                println!("No items in the array");
                None
            }
        } else {
            println!("affected_items is not an array");
            None
        }
    } else {
        println!("Could not find data.affected_items path");
        None
    };

    if let Some(ref found_id) = id {
        println!("Found group ID: {}", found_id);
    } else {
        println!("No group ID found");
    }

    id
}
