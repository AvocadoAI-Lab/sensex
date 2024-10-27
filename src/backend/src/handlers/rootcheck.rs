use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Run rootcheck scan
pub async fn run_rootcheck(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rootcheck", |url| url).await
}

// Agent-specific rootcheck operations
pub async fn get_rootcheck_agent(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rootcheck/{agent_id}", |url| url).await
}

pub async fn delete_rootcheck_agent(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rootcheck/{agent_id}", |url| url).await
}

pub async fn get_rootcheck_last_scan(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rootcheck/{agent_id}/last_scan", |url| url).await
}
