use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Base group operations
pub async fn get_groups(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "groups", |url| url).await
}

pub async fn create_group(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "groups", |url| url).await
}

pub async fn delete_groups(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "groups", |url| url).await
}

// Group-specific operations
pub async fn get_group_agents(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "groups/{group_id}/agents", |url| url).await
}

pub async fn get_group_configuration(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "groups/{group_id}/configuration", |url| url).await
}

pub async fn update_group_configuration(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "groups/{group_id}/configuration", |url| url).await
}

// Group files operations
pub async fn get_group_files(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "groups/{group_id}/files", |url| url).await
}

pub async fn get_group_file_content(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "groups/{group_id}/files/{file_name}", |url| url).await
}
