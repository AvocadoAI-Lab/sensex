use axum::{
    Router,
    routing::{get, put},
};
use crate::handlers::cluster::*;

pub fn routes() -> Router {
    Router::new()
        // Local cluster information
        .route("/cluster/local/info", get(get_cluster_local_info))
        .route("/cluster/local/config", get(get_cluster_local_config))
        
        // Cluster status and health
        .route("/cluster/status", get(get_cluster_status))
        .route("/cluster/nodes", get(get_cluster_nodes))
        .route("/cluster/healthcheck", get(get_cluster_healthcheck))
        
        // Cluster configuration and validation
        .route("/cluster/configuration/validation", get(get_cluster_config_validation))
        .route("/cluster/api/config", get(get_cluster_api_config))
        
        // Cluster operations
        .route("/cluster/restart", put(restart_cluster))
        .route("/cluster/ruleset/synchronization", get(get_cluster_ruleset_sync))
        
        // Node-specific operations
        .route("/cluster/:node_id/status", get(get_node_status))
        .route("/cluster/:node_id/info", get(get_node_info))
        .route("/cluster/:node_id/configuration", get(get_node_config))
        .route("/cluster/:node_id/configuration", put(update_node_config))
        .route("/cluster/:node_id/configuration/:component/:configuration", get(get_node_component_config))
        
        // Node statistics
        .route("/cluster/:node_id/daemons/stats", get(get_node_daemon_stats))
        .route("/cluster/:node_id/stats", get(get_node_stats))
        .route("/cluster/:node_id/stats/hourly", get(get_node_stats_hourly))
        .route("/cluster/:node_id/stats/weekly", get(get_node_stats_weekly))
        .route("/cluster/:node_id/stats/analysisd", get(get_node_stats_analysisd))
        .route("/cluster/:node_id/stats/remoted", get(get_node_stats_remoted))
        
        // Node logs
        .route("/cluster/:node_id/logs", get(get_node_logs))
        .route("/cluster/:node_id/logs/summary", get(get_node_logs_summary))
}
