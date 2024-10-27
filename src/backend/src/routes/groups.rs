use axum::{
    Router,
    routing::{get, post, put, delete},
};
use crate::handlers::groups::*;

pub fn routes() -> Router {
    Router::new()
        // Base group operations
        .route("/groups", get(get_groups))
        .route("/groups", post(create_group))
        .route("/groups", delete(delete_groups))
        
        // Group-specific operations
        .route("/groups/:group_id/agents", get(get_group_agents))
        .route("/groups/:group_id/configuration", get(get_group_configuration))
        .route("/groups/:group_id/configuration", put(update_group_configuration))
        
        // Group files operations
        .route("/groups/:group_id/files", get(get_group_files))
        .route("/groups/:group_id/files/:file_name", get(get_group_file_content))
}
