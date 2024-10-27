use super::common::{BASE_URL, get_test_client};
use crate::client::WazuhClient;

/// 整合測試：按順序執行所有主要功能的測試
#[tokio::test]
async fn test_all_endpoints() {
    println!("\n=== Starting Integration Test Suite ===\n");
    let (client, token) = get_test_client().await;

    // 1. 測試認證
    println!("\n=== Testing Authentication ===");
    test_auth(&client, &token).await;

    // 2. 測試 Agents
    println!("\n=== Testing Agents ===");
    test_agents(&client, &token).await;

    // 3. 測試 Groups
    println!("\n=== Testing Groups ===");
    test_groups(&client, &token).await;

    // 4. 測試 Rules
    println!("\n=== Testing Rules ===");
    test_rules(&client, &token).await;

    // 5. 測試 Decoders
    println!("\n=== Testing Decoders ===");
    test_decoders(&client, &token).await;

    // 6. 測試 Manager
    println!("\n=== Testing Manager ===");
    test_manager(&client, &token).await;

    // 7. 測試 Cluster
    println!("\n=== Testing Cluster ===");
    test_cluster(&client, &token).await;

    println!("\n=== Integration Test Suite Completed ===\n");
}

async fn test_auth(client: &WazuhClient, token: &str) {
    let auth_url = format!("{}/security/user/authenticate", BASE_URL);
    let response = client.get(&auth_url, Some(token))
        .await
        .expect("Auth request failed");
    
    println!("Auth Status: {}", response.status());
    let text = response.text().await.expect("Failed to get auth response");
    println!("Auth Response: {}", text);
}

async fn test_agents(client: &WazuhClient, token: &str) {
    let agents_url = format!("{}/agents", BASE_URL);
    let response = client.get(&agents_url, Some(token))
        .await
        .expect("Agents request failed");
    
    println!("Agents Status: {}", response.status());
    let text = response.text().await.expect("Failed to get agents response");
    println!("Agents Response: {}", text);
}

async fn test_groups(client: &WazuhClient, token: &str) {
    let groups_url = format!("{}/groups", BASE_URL);
    let response = client.get(&groups_url, Some(token))
        .await
        .expect("Groups request failed");
    
    println!("Groups Status: {}", response.status());
    let text = response.text().await.expect("Failed to get groups response");
    println!("Groups Response: {}", text);
}

async fn test_rules(client: &WazuhClient, token: &str) {
    let rules_url = format!("{}/rules", BASE_URL);
    let response = client.get(&rules_url, Some(token))
        .await
        .expect("Rules request failed");
    
    println!("Rules Status: {}", response.status());
    let text = response.text().await.expect("Failed to get rules response");
    println!("Rules Response: {}", text);
}

async fn test_decoders(client: &WazuhClient, token: &str) {
    let decoders_url = format!("{}/decoders", BASE_URL);
    let response = client.get(&decoders_url, Some(token))
        .await
        .expect("Decoders request failed");
    
    println!("Decoders Status: {}", response.status());
    let text = response.text().await.expect("Failed to get decoders response");
    println!("Decoders Response: {}", text);
}

async fn test_manager(client: &WazuhClient, token: &str) {
    let manager_url = format!("{}/manager/status", BASE_URL);
    let response = client.get(&manager_url, Some(token))
        .await
        .expect("Manager request failed");
    
    println!("Manager Status: {}", response.status());
    let text = response.text().await.expect("Failed to get manager response");
    println!("Manager Response: {}", text);
}

async fn test_cluster(client: &WazuhClient, token: &str) {
    let cluster_url = format!("{}/cluster/status", BASE_URL);
    let response = client.get(&cluster_url, Some(token))
        .await
        .expect("Cluster request failed");
    
    println!("Cluster Status: {}", response.status());
    let text = response.text().await.expect("Failed to get cluster response");
    println!("Cluster Response: {}", text);
}
