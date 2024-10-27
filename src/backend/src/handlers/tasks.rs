use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

pub async fn get_tasks_list(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "tasks", |url| url).await
}

pub async fn get_tasks_status(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "tasks/status", |url| url).await
}

pub async fn get_task_result(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "tasks/result", |url| url).await
}
