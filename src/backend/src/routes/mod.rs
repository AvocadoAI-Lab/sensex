use axum::Router;
use axum::routing::get;

mod agents;
mod auth;
mod cluster;
mod decoders;
mod groups;
mod lists;
mod manager;
mod mitre;
mod rules;
mod security;
mod syscollector;
mod tasks;

pub fn create_router() -> Router {
    Router::new()
        .merge(agents::routes())
        .merge(auth::routes())
        .merge(cluster::routes())
        .merge(decoders::routes())
        .merge(groups::routes())
        .merge(lists::routes())
        .merge(manager::routes())
        .merge(mitre::routes())
        .merge(rules::routes())
        .merge(security::routes())
        .merge(syscollector::routes())
        .merge(tasks::routes())
        .route("/health", get(health_check))
}

async fn health_check() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}
