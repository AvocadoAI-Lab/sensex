mod client;
mod handlers;

use axum::{
    routing::{get, post},
    http::StatusCode,
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use handlers::*;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/auth", post(authenticate))
        .route("/agents", post(get_agents))
        .route("/cluster/status", post(get_cluster_status))
        .route("/manager/status", post(get_manager_status))
        .route("/manager/info", post(get_manager_info))
        .route("/cluster/local/info", post(get_cluster_local_info))
        .route("/rules", post(get_rules))
        .route("/decoders", post(get_decoders))
        .route("/manager/logs", post(get_manager_logs))
        .route("/manager/stats", post(get_manager_stats))
        .route("/groups", post(get_groups))
        .route("/groups/files", post(get_group_files))
        .route("/groups/agents", post(get_group_agents))
        .route("/syscollector/hardware", post(get_syscollector_hardware))
        .route("/syscollector/os", post(get_syscollector_os))
        .route("/syscollector/packages", post(get_syscollector_packages))
        .route("/syscollector/processes", post(get_syscollector_processes))
        .route("/syscollector/ports", post(get_syscollector_ports))
        .route("/syscollector/netaddr", post(get_syscollector_netaddr))
        .route("/syscollector/netproto", post(get_syscollector_netproto))
        .route("/syscollector/netiface", post(get_syscollector_netiface))
        .route("/syscheck/files", post(get_syscheck_files))
        .route("/syscheck/last_scan", post(get_syscheck_last_scan))
        .route("/syscheck/summary", post(get_syscheck_summary))
        .route("/active-response/commands", post(get_active_response_commands))
        .route("/active-response/run", post(run_active_response_command))
        .route("/events", post(get_events))
        .route("/events/stats", post(get_events_stats))
        .route("/mitre/metadata", post(get_mitre_metadata))
        .route("/mitre/references", post(get_mitre_references))
        .route("/mitre/techniques", post(get_mitre_techniques))
        .route("/mitre/tactics", post(get_mitre_tactics))
        .route("/ciscat/results", post(get_ciscat_results))
        .route("/ciscat/scan/config", post(get_ciscat_scan_config))
        .route("/ciscat/scan/results", post(get_ciscat_scan_results))
        .route("/rootcheck/info", post(get_rootcheck_info))
        .route("/rootcheck/database", post(get_rootcheck_database))
        .route("/rootcheck/last_scan", post(get_rootcheck_last_scan))
        .route("/security/users", post(get_security_users))
        .route("/security/roles", post(get_security_roles))
        .route("/security/rules", post(get_security_rules))
        .route("/security/policies", post(get_security_policies))
        .route("/security/resources", post(get_security_resources))
        .route("/tasks/status", post(get_tasks_status))
        .route("/tasks", post(get_tasks_list))
        .route("/tasks/result", post(get_task_result))
        .route("/sca/checks", post(get_sca_checks))
        .route("/sca/results", post(get_sca_results))
        .route("/sca/summary", post(get_sca_summary))
        .route("/sca/policies", post(get_sca_policies))
        .route("/health", get(health_check))
        .layer(cors);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server running on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(&addr)
            .await
            .unwrap(),
        app,
    )
    .await
    .unwrap();
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
