use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Get all decoders
pub async fn get_decoders(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "decoders", |url| url).await
}

// Get decoder files
pub async fn get_decoder_files(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "decoders/files", |url| url).await
}

// Get decoder file content
pub async fn get_decoder_file_content(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "decoders/files/{filename}", |url| url).await
}

// Update decoder file content
pub async fn update_decoder_file(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "decoders/files/{filename}", |url| url).await
}

// Delete decoder file
pub async fn delete_decoder_file(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "decoders/files/{filename}", |url| url).await
}
