use axum::{
    Router,
    routing::{get, put, delete},
};
use crate::handlers::rules::*;

pub fn routes() -> Router {
    Router::new()
        // Base rules endpoints - keep as GET since it's working
        .route("/rules", get(get_rules))
        
        // Keep PUT/DELETE operations as they are
        .route("/rules/files/:filename", put(update_rule_file))
        .route("/rules/files/:filename", delete(delete_rule_file))
}
