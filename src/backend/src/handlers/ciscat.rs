use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

pub async fn get_ciscat_results(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "ciscat/results", |url| url).await
}

pub async fn get_ciscat_scan_config(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "ciscat/scan/config", |url| url).await
}

pub async fn get_ciscat_scan_results(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "ciscat/scan/results", |url| url).await
}
