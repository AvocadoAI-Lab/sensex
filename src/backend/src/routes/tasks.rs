use axum::{
    Router,
    routing::get,
};
use crate::handlers::tasks::*;

pub fn routes() -> Router {
    Router::new()
        .route("/tasks/status", get(get_tasks_status))
}
