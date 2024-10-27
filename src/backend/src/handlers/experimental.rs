use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Rootcheck and Syscheck
pub async fn delete_experimental_rootcheck(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "experimental/rootcheck", |url| url).await
}

pub async fn delete_experimental_syscheck(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "experimental/syscheck", |url| url).await
}

// Ciscat results
pub async fn get_experimental_ciscat_results(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "experimental/ciscat/results", |url| url).await
}

// Syscollector information
pub async fn get_experimental_syscollector_hardware(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "experimental/syscollector/hardware", |url| url).await
}

pub async fn get_experimental_syscollector_netaddr(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "experimental/syscollector/netaddr", |url| url).await
}

pub async fn get_experimental_syscollector_netiface(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "experimental/syscollector/netiface", |url| url).await
}

pub async fn get_experimental_syscollector_netproto(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "experimental/syscollector/netproto", |url| url).await
}

pub async fn get_experimental_syscollector_os(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "experimental/syscollector/os", |url| url).await
}

pub async fn get_experimental_syscollector_packages(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "experimental/syscollector/packages", |url| url).await
}

pub async fn get_experimental_syscollector_ports(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "experimental/syscollector/ports", |url| url).await
}

pub async fn get_experimental_syscollector_processes(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "experimental/syscollector/processes", |url| url).await
}

pub async fn get_experimental_syscollector_hotfixes(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "experimental/syscollector/hotfixes", |url| url).await
}
