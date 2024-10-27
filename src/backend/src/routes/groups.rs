use axum::{
    Router,
    routing::{get, post, put, delete},
};
use crate::handlers::groups::*;

pub fn routes() -> Router {
    Router::new()
        // Keep only working endpoints
        .route("/groups", get(get_groups))
        .route("/groups", post(create_group))
        .route("/groups", delete(delete_groups))
        
        // Keep configuration update
        .route("/groups/:group_id/configuration", put(update_group_configuration))
}
