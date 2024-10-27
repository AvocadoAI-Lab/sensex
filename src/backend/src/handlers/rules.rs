use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Get all rules
pub async fn get_rules(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rules", |url| url).await
}

// Get rule groups
pub async fn get_rule_groups(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rules/groups", |url| url).await
}

// Get rules by requirement
pub async fn get_rules_by_requirement(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rules/requirement/{requirement}", |url| url).await
}

// Rules files operations
pub async fn get_rule_files(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rules/files", |url| url).await
}

pub async fn get_rule_file_content(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rules/files/{filename}", |url| url).await
}

pub async fn update_rule_file(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rules/files/{filename}", |url| url).await
}

pub async fn delete_rule_file(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rules/files/{filename}", |url| url).await
}
