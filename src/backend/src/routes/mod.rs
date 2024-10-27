use axum::Router;
use axum::routing::get;

mod agents;
mod system;
mod security;
mod groups;
mod ciscat;
mod cluster;
mod decoders;
mod lists;
mod logtest;
mod manager;
mod mitre;
mod rootcheck;
mod rules;
mod sca;
mod syscheck;
mod syscollector;
mod tasks;
mod events;
mod experimental;

pub fn create_router() -> Router {
    Router::new()
        .merge(agents::routes())
        .merge(system::routes())
        .merge(security::routes())
        .merge(groups::routes())
        .merge(ciscat::routes())
        .merge(cluster::routes())
        .merge(decoders::routes())
        .merge(lists::routes())
        .merge(logtest::routes())
        .merge(manager::routes())
        .merge(mitre::routes())
        .merge(rootcheck::routes())
        .merge(rules::routes())
        .merge(sca::routes())
        .merge(syscheck::routes())
        .merge(syscollector::routes())
        .merge(tasks::routes())
        .merge(events::routes())
        .merge(experimental::routes())
        .route("/health", get(health_check))
}

async fn health_check() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}
