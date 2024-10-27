use axum::{
    Router,
    routing::post,
};
use crate::handlers::syscollector::*;

pub fn routes() -> Router {
    Router::new()
        // Keep only the endpoints we're testing
        .route("/syscollector/:agent_id/hardware", post(get_syscollector_hardware))
        .route("/syscollector/:agent_id/os", post(get_syscollector_os))
        .route("/syscollector/:agent_id/packages", post(get_syscollector_packages))
}
