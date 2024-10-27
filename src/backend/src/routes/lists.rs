use axum::{
    Router,
    routing::{put, delete},
};
use crate::handlers::lists::*;

pub fn routes() -> Router {
    Router::new()
        // Keep only PUT/DELETE operations
        .route("/lists/files/:filename", put(update_list_file))
        .route("/lists/files/:filename", delete(delete_list_file))
}
