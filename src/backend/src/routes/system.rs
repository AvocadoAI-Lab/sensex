use axum::{
    Router,
    routing::post,
};
use crate::handlers::{
    get_cluster_status,
    get_cluster_local_info,
    get_manager_status,
    get_manager_info,
    get_manager_logs,
    get_manager_stats,
    get_syscollector_hardware,
    get_syscollector_os,
    get_syscheck_agent,
    get_syscheck_last_scan,
    get_rules,
    get_decoders,
    get_groups,
    get_group_files,
    get_group_agents,
    get_tasks_status,
};

pub fn routes() -> Router {
    Router::new()
        // Cluster and manager endpoints
        .route("/cluster/status", post(get_cluster_status))
        .route("/cluster/local/info", post(get_cluster_local_info))
        .route("/manager/status", post(get_manager_status))
        .route("/manager/info", post(get_manager_info))
        .route("/manager/logs", post(get_manager_logs))
        .route("/manager/stats", post(get_manager_stats))
        
        // System monitoring
        .route("/syscollector/hardware", post(get_syscollector_hardware))
        .route("/syscollector/os", post(get_syscollector_os))
        .route("/syscheck/:agent_id", post(get_syscheck_agent))
        .route("/syscheck/:agent_id/last_scan", post(get_syscheck_last_scan))
        
        // Rules and decoders
        .route("/rules", post(get_rules))
        .route("/decoders", post(get_decoders))
        
        // Groups
        .route("/groups", post(get_groups))
        .route("/groups/files", post(get_group_files))
        .route("/groups/agents", post(get_group_agents))
        
        // Tasks
        .route("/tasks/status", post(get_tasks_status))
}
