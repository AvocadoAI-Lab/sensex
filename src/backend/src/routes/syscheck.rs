use axum::{
    Router,
    routing::{get, put, delete},
};
use crate::handlers::syscheck::*;

pub fn routes() -> Router {
    Router::new()
        // Run syscheck scan
        .route("/syscheck", put(run_syscheck))
        
        // Agent-specific syscheck operations
        .route("/syscheck/:agent_id", get(get_syscheck_agent))
        .route("/syscheck/:agent_id", delete(delete_syscheck_agent))
        .route("/syscheck/:agent_id/last_scan", get(get_syscheck_last_scan))
}
