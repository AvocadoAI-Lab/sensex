use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

pub async fn get_syscollector_hardware(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/hardware", |url| url).await
}

pub async fn get_syscollector_os(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/os", |url| url).await
}

pub async fn get_syscollector_packages(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/packages", |url| url).await
}

pub async fn get_syscollector_processes(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/processes", |url| url).await
}

pub async fn get_syscollector_ports(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/ports", |url| url).await
}

pub async fn get_syscollector_netaddr(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/netaddr", |url| url).await
}

pub async fn get_syscollector_netproto(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/netproto", |url| url).await
}

pub async fn get_syscollector_netiface(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/netiface", |url| url).await
}
