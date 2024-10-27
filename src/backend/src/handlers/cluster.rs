use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Local cluster information
pub async fn get_cluster_local_info(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/local/info", |url| url).await
}

pub async fn get_cluster_local_config(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/local/config", |url| url).await
}

// Cluster status and health
pub async fn get_cluster_status(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/status", |url| url).await
}

pub async fn get_cluster_nodes(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/nodes", |url| url).await
}

pub async fn get_cluster_healthcheck(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/healthcheck", |url| url).await
}

// Cluster configuration and validation
pub async fn get_cluster_config_validation(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/configuration/validation", |url| url).await
}

pub async fn get_cluster_api_config(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/api/config", |url| url).await
}

// Cluster operations
pub async fn restart_cluster(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/restart", |url| url).await
}

pub async fn get_cluster_ruleset_sync(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/ruleset/synchronization", |url| url).await
}

// Node-specific operations
pub async fn get_node_status(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/{node_id}/status", |url| url).await
}

pub async fn get_node_info(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/{node_id}/info", |url| url).await
}

pub async fn get_node_config(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/{node_id}/configuration", |url| url).await
}

pub async fn update_node_config(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/{node_id}/configuration", |url| url).await
}

pub async fn get_node_component_config(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/{node_id}/configuration/{component}/{configuration}", |url| url).await
}

// Node statistics
pub async fn get_node_daemon_stats(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/{node_id}/daemons/stats", |url| url).await
}

pub async fn get_node_stats(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/{node_id}/stats", |url| url).await
}

pub async fn get_node_stats_hourly(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/{node_id}/stats/hourly", |url| url).await
}

pub async fn get_node_stats_weekly(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/{node_id}/stats/weekly", |url| url).await
}

pub async fn get_node_stats_analysisd(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/{node_id}/stats/analysisd", |url| url).await
}

pub async fn get_node_stats_remoted(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/{node_id}/stats/remoted", |url| url).await
}

// Node logs
pub async fn get_node_logs(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/{node_id}/logs", |url| url).await
}

pub async fn get_node_logs_summary(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "cluster/{node_id}/logs/summary", |url| url).await
}
