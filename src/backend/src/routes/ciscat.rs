use axum::{
    Router,
    routing::get,
};
use crate::handlers::ciscat::*;

pub fn routes() -> Router {
    Router::new()
        .route("/ciscat/:agent_id/results", get(get_agent_ciscat_results))
}
