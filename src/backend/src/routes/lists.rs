use axum::{
    Router,
    routing::{get, put, delete},
};
use crate::handlers::lists::*;

pub fn routes() -> Router {
    Router::new()
        // Base lists endpoint
        .route("/lists", get(get_lists))
        
        // List files operations
        .route("/lists/files", get(get_list_files))
        .route("/lists/files/:filename", get(get_list_file_content))
        .route("/lists/files/:filename", put(update_list_file))
        .route("/lists/files/:filename", delete(delete_list_file))
}
