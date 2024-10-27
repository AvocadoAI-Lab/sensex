use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Get agent SCA policies
pub async fn get_agent_sca_policies(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "sca/{agent_id}/policies", |url| url).await
}

// Get agent SCA checks for a specific policy
pub async fn get_agent_sca_checks(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "sca/{agent_id}/checks/{policy_id}", |url| url).await
}

// Get agent SCA results
pub async fn get_agent_sca_results(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "sca/{agent_id}/results", |url| url).await
}
