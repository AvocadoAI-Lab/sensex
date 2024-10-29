use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Cluster status and info
pub async fn get_cluster_status(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/status", |url| url).await
}

pub async fn get_cluster_local_info(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/local/info", |url| url).await
}

pub async fn get_cluster_nodes(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/nodes", |url| url).await
}

pub async fn get_cluster_healthcheck(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/healthcheck", |url| url).await
}
