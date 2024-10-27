use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Run syscheck scan
pub async fn run_syscheck(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscheck", |url| url).await
}

// Agent-specific syscheck operations
pub async fn get_syscheck_agent(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscheck/{agent_id}", |url| url).await
}

pub async fn delete_syscheck_agent(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscheck/{agent_id}", |url| url).await
}

pub async fn get_syscheck_last_scan(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscheck/{agent_id}/last_scan", |url| url).await
}
