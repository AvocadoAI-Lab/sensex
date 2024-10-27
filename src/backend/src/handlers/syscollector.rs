use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Hardware information
pub async fn get_syscollector_hardware(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/{agent_id}/hardware", |url| url).await
}

// Operating system information
pub async fn get_syscollector_os(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/{agent_id}/os", |url| url).await
}

// Package information
pub async fn get_syscollector_packages(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/{agent_id}/packages", |url| url).await
}
