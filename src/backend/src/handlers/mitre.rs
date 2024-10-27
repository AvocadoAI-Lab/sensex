use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

pub async fn get_mitre_metadata(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "mitre/metadata", |url| url).await
}

pub async fn get_mitre_references(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "mitre/references", |url| url).await
}

pub async fn get_mitre_techniques(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "mitre/techniques", |url| url).await
}

pub async fn get_mitre_tactics(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "mitre/tactics", |url| url).await
}
