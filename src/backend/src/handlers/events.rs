use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Create event
pub async fn create_event(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "events", |url| url).await
}
