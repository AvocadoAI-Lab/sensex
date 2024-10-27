use axum::Router;
use axum::routing::get;

mod agents;
mod system;
mod security;

pub fn create_router() -> Router {
    Router::new()
        .merge(agents::routes())
        .merge(system::routes())
        .merge(security::routes())
        .route("/health", get(health_check))
}

async fn health_check() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}
