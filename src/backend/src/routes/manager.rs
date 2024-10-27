use axum::{
    Router,
    routing::{get, put},
};
use crate::handlers::manager::*;

pub fn routes() -> Router {
    Router::new()
        // Keep only working endpoints
        .route("/manager/info", get(get_manager_info))
        .route("/manager/stats", get(get_manager_stats))
        .route("/manager/logs", get(get_manager_logs))
        
        // Keep configuration update operation
        .route("/manager/configuration", put(update_manager_config))
        .route("/manager/restart", put(restart_manager))
}
