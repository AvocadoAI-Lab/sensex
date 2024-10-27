use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Get all lists
pub async fn get_lists(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "lists", |url| url).await
}

// Get all CDB lists files
pub async fn get_list_files(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "lists/files", |url| url).await
}

// Get content of a CDB list file
pub async fn get_list_file_content(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "lists/files/{filename}", |url| url).await
}

// Update content of a CDB list file
pub async fn update_list_file(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "lists/files/{filename}", |url| url).await
}

// Delete a CDB list file
pub async fn delete_list_file(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "lists/files/{filename}", |url| url).await
}
