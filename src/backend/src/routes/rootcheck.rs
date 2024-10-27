use axum::{
    Router,
    routing::{put, delete},
};
use crate::handlers::rootcheck::*;

pub fn routes() -> Router {
    Router::new()
        // Keep only PUT/DELETE operations
        .route("/rootcheck", put(run_rootcheck))
        .route("/rootcheck/:agent_id", delete(delete_rootcheck_agent))
}
