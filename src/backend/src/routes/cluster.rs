use axum::{
    Router,
    routing::post,
};
use crate::handlers::cluster::*;

pub fn routes() -> Router {
    Router::new()
        // Cluster status and info
        .route("/cluster/status", post(get_cluster_status))
        .route("/cluster/local/info", post(get_cluster_local_info))
        .route("/cluster/nodes", post(get_cluster_nodes))
        .route("/cluster/healthcheck", post(get_cluster_healthcheck))
}
