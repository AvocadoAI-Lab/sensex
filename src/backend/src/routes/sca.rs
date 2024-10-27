use axum::{
    Router,
    routing::get,
};
use crate::handlers::sca::*;

pub fn routes() -> Router {
    Router::new()
        .route("/sca/:agent_id", get(get_agent_sca))
        .route("/sca/:agent_id/checks/:policy_id", get(get_agent_sca_checks))
}
