use axum::{
    Router,
    routing::{get, put, delete},
};
use crate::handlers::rules::*;

pub fn routes() -> Router {
    Router::new()
        // Base rules endpoints
        .route("/rules", get(get_rules))
        .route("/rules/groups", get(get_rule_groups))
        .route("/rules/requirement/:requirement", get(get_rules_by_requirement))
        
        // Rules files operations
        .route("/rules/files", get(get_rule_files))
        .route("/rules/files/:filename", get(get_rule_file_content))
        .route("/rules/files/:filename", put(update_rule_file))
        .route("/rules/files/:filename", delete(delete_rule_file))
}
