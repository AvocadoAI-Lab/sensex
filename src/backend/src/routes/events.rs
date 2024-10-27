use axum::{
    Router,
    routing::post,
};
use crate::handlers::events::*;

pub fn routes() -> Router {
    Router::new()
        .route("/events", post(create_event))
}
