mod client;
mod handlers;

use axum::{
    routing::{get, post},
    http::StatusCode,
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use handlers::*;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/auth", post(authenticate))
        .route("/agents", post(get_agents))
        .route("/cluster/status", post(get_cluster_status))
        .route("/manager/status", post(get_manager_status))
        .route("/manager/info", post(get_manager_info))
        .route("/cluster/local/info", post(get_cluster_local_info))
        .route("/rules", post(get_rules))
        .route("/decoders", post(get_decoders))
        .route("/manager/logs", post(get_manager_logs))
        .route("/manager/stats", post(get_manager_stats))
        .route("/health", get(health_check))
        .layer(cors);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server running on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(&addr)
            .await
            .unwrap(),
        app,
    )
    .await
    .unwrap();
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
