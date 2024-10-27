use axum::{
    Router,
    routing::{get, put},
};
use crate::handlers::manager::*;

pub fn routes() -> Router {
    Router::new()
        // Basic manager information
        .route("/manager/status", get(get_manager_status))
        .route("/manager/info", get(get_manager_info))
        
        // Configuration management
        .route("/manager/configuration", get(get_manager_config))
        .route("/manager/configuration", put(update_manager_config))
        .route("/manager/configuration/:component/:configuration", get(get_manager_component_config))
        .route("/manager/configuration/validation", get(get_manager_config_validation))
        
        // Statistics
        .route("/manager/stats", get(get_manager_stats))
        .route("/manager/stats/hourly", get(get_manager_stats_hourly))
        .route("/manager/stats/weekly", get(get_manager_stats_weekly))
        .route("/manager/stats/analysisd", get(get_manager_stats_analysisd))
        .route("/manager/stats/remoted", get(get_manager_stats_remoted))
        .route("/manager/daemons/stats", get(get_manager_daemon_stats))
        
        // Logs
        .route("/manager/logs", get(get_manager_logs))
        .route("/manager/logs/summary", get(get_manager_logs_summary))
        
        // API and version
        .route("/manager/api/config", get(get_manager_api_config))
        .route("/manager/version/check", get(check_manager_version))
        
        // Operations
        .route("/manager/restart", put(restart_manager))
}
