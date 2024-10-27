use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

pub async fn get_rootcheck_info(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rootcheck/info", |url| url).await
}

pub async fn get_rootcheck_database(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rootcheck/database", |url| url).await
}

pub async fn get_rootcheck_last_scan(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rootcheck/last_scan", |url| url).await
}
