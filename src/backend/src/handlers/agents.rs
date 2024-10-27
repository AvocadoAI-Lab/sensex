use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

pub async fn get_agents(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents", |url| url).await
}

pub async fn get_agent_config(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/config", |url| url).await
}

pub async fn get_agent_key(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/key", |url| url).await
}

pub async fn get_agent_no_group(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/no_group", |url| url).await
}

pub async fn get_agent_outdated(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/outdated", |url| url).await
}

pub async fn get_agent_stats(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/stats", |url| url).await
}

pub async fn get_agent_summary_status(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/summary/status", |url| url).await
}

pub async fn get_agent_summary_os(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/summary/os", |url| url).await
}

pub async fn restart_agent(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/restart", |url| url).await
}

pub async fn delete_agents(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents", |url| url).await
}

pub async fn add_agent(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents", |url| url).await
}

pub async fn get_agent_config_component(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/config", |url| url).await
}

pub async fn get_agent_sync_status(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/sync", |url| url).await
}

pub async fn get_agent_upgrade_result(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/upgrade_result", |url| url).await
}

pub async fn upgrade_agents(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/upgrade", |url| url).await
}

pub async fn get_daemon_stats(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/daemons/stats", |url| url).await
}

pub async fn get_agent_fields(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/stats/distinct", |url| url).await
}
