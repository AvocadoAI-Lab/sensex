use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

pub async fn get_manager_status(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/status", |url| url).await
}

pub async fn get_manager_info(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/info", |url| url).await
}

pub async fn get_manager_logs(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/logs", |url| url).await
}

pub async fn get_manager_stats(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/stats", |url| url).await
}
