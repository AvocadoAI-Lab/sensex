use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Ciscat results
pub async fn get_experimental_ciscat_results(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "experimental/ciscat/results", |url| url).await
}

// Test experimental rules
pub async fn test_experimental_rules(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "experimental/rules/test", |url| url).await
}

// Test experimental decoders
pub async fn test_experimental_decoders(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "experimental/decoders/test", |url| url).await
}
