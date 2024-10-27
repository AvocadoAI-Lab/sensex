use super::common::{BASE_URL, get_test_client};
use serde_json::Value;

#[tokio::test]
async fn test_cluster_status() {
    let (client, token) = get_test_client().await;

    println!("Getting cluster status");
    let status_url = format!("{}/cluster/status", BASE_URL);
    let response = client.get(&status_url, Some(&token))
        .await
        .expect("Should get cluster status");
    
    let status = response.status();
    println!("Cluster status response status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get cluster status successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw cluster status response: {}", response_text);
    
    let status_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Cluster Status Response: {:#?}", status_json);
}

#[tokio::test]
async fn test_cluster_nodes() {
    let (client, token) = get_test_client().await;

    println!("Getting cluster nodes");
    let nodes_url = format!("{}/cluster/nodes", BASE_URL);
    let response = client.get(&nodes_url, Some(&token))
        .await
        .expect("Should get cluster nodes");
    
    let status = response.status();
    println!("Cluster nodes status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get cluster nodes successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw cluster nodes response: {}", response_text);
    
    let nodes_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Cluster Nodes Response: {:#?}", nodes_json);
}

#[tokio::test]
async fn test_cluster_health() {
    let (client, token) = get_test_client().await;

    println!("Getting cluster health");
    let health_url = format!("{}/cluster/healthcheck", BASE_URL);
    let response = client.get(&health_url, Some(&token))
        .await
        .expect("Should get cluster health");
    
    let status = response.status();
    println!("Cluster health status: {}", status);
    assert_eq!(status.as_u16(), 200, "Should get cluster health successfully");
    
    let response_text = response.text().await.expect("Should get response text");
    println!("Raw cluster health response: {}", response_text);
    
    let health_json: Value = serde_json::from_str(&response_text).expect("Should parse JSON");
    println!("Cluster Health Response: {:#?}", health_json);
}
