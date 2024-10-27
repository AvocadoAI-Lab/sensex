use super::common::{BASE_URL, get_test_client, validate_response};

#[tokio::test]
async fn test_cluster_status() {
    let (client, token) = get_test_client().await;

    println!("Getting cluster status");
    let status_url = format!("{}/cluster/status", BASE_URL);
    let response = client.get(&status_url, Some(&token))
        .await
        .expect("Should get cluster status");
    
    let status = response.status();
    let response_text = response.text().await.expect("Should get response text");
    
    assert!(validate_response(
        "Cluster Status Test",
        status.as_u16(),
        &response_text
    ));
}

#[tokio::test]
async fn test_cluster_nodes() {
    let (client, token) = get_test_client().await;

    // 首先檢查集群狀態
    let status_url = format!("{}/cluster/status", BASE_URL);
    let response = client.get(&status_url, Some(&token))
        .await
        .expect("Should get cluster status");
    
    let status_text = response.text().await.expect("Should get response text");
    let status_json: serde_json::Value = serde_json::from_str(&status_text).expect("Should parse JSON");
    
    // 只有當集群啟用時才測試節點
    if let Some(enabled) = status_json["data"]["enabled"].as_bool() {
        if enabled {
            println!("Getting cluster nodes");
            let nodes_url = format!("{}/cluster/nodes", BASE_URL);
            let response = client.get(&nodes_url, Some(&token))
                .await
                .expect("Should get cluster nodes");
            
            let status = response.status();
            let response_text = response.text().await.expect("Should get response text");
            
            assert!(validate_response(
                "Cluster Nodes Test",
                status.as_u16(),
                &response_text
            ));
        } else {
            println!("Cluster is disabled, skipping nodes test");
        }
    }
}

#[tokio::test]
async fn test_cluster_health() {
    let (client, token) = get_test_client().await;

    // 首先檢查集群狀態
    let status_url = format!("{}/cluster/status", BASE_URL);
    let response = client.get(&status_url, Some(&token))
        .await
        .expect("Should get cluster status");
    
    let status_text = response.text().await.expect("Should get response text");
    let status_json: serde_json::Value = serde_json::from_str(&status_text).expect("Should parse JSON");
    
    // 只有當集群啟用時才測試健康狀態
    if let Some(enabled) = status_json["data"]["enabled"].as_bool() {
        if enabled {
            println!("Getting cluster health");
            let health_url = format!("{}/cluster/healthcheck", BASE_URL);
            let response = client.get(&health_url, Some(&token))
                .await
                .expect("Should get cluster health");
            
            let status = response.status();
            let response_text = response.text().await.expect("Should get response text");
            
            assert!(validate_response(
                "Cluster Health Test",
                status.as_u16(),
                &response_text
            ));
        } else {
            println!("Cluster is disabled, skipping health test");
        }
    }
}
