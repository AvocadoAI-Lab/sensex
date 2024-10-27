use axum::{
    Router,
    routing::{put, delete},
};
use crate::handlers::logtest::*;

pub fn routes() -> Router {
    Router::new()
        .route("/logtest", put(run_logtest))
        .route("/logtest/sessions/:token", delete(delete_logtest_session))
}
