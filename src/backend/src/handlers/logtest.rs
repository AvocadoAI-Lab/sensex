use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Run log testing
pub async fn run_logtest(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "logtest", |url| url).await
}

// Delete logtest session
pub async fn delete_logtest_session(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "logtest/sessions/{token}", |url| url).await
}
