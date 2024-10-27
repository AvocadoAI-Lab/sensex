use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Basic manager information
pub async fn get_manager_status(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/status", |url| url).await
}

pub async fn get_manager_info(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/info", |url| url).await
}

// Configuration management
pub async fn get_manager_config(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/configuration", |url| url).await
}

pub async fn update_manager_config(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/configuration", |url| url).await
}

pub async fn get_manager_component_config(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/configuration/{component}/{configuration}", |url| url).await
}

pub async fn get_manager_config_validation(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/configuration/validation", |url| url).await
}

// Statistics
pub async fn get_manager_stats(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/stats", |url| url).await
}

pub async fn get_manager_stats_hourly(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/stats/hourly", |url| url).await
}

pub async fn get_manager_stats_weekly(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/stats/weekly", |url| url).await
}

pub async fn get_manager_stats_analysisd(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/stats/analysisd", |url| url).await
}

pub async fn get_manager_stats_remoted(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/stats/remoted", |url| url).await
}

pub async fn get_manager_daemon_stats(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/daemons/stats", |url| url).await
}

// Logs
pub async fn get_manager_logs(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/logs", |url| url).await
}

pub async fn get_manager_logs_summary(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/logs/summary", |url| url).await
}

// API and version
pub async fn get_manager_api_config(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/api/config", |url| url).await
}

pub async fn check_manager_version(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/version/check", |url| url).await
}

// Operations
pub async fn restart_manager(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/restart", |url| url).await
}
