use axum::{
    Router,
    routing::{get, put, delete},
};
use crate::handlers::rootcheck::*;

pub fn routes() -> Router {
    Router::new()
        // Run rootcheck scan
        .route("/rootcheck", put(run_rootcheck))
        
        // Agent-specific rootcheck operations
        .route("/rootcheck/:agent_id", get(get_rootcheck_agent))
        .route("/rootcheck/:agent_id", delete(delete_rootcheck_agent))
        .route("/rootcheck/:agent_id/last_scan", get(get_rootcheck_last_scan))
}
