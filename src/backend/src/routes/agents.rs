use axum::{
    Router,
    routing::{post, put, delete},
};
use crate::handlers::agents::*;

pub fn routes() -> Router {
    Router::new()
        // Base agents endpoints
        .route("/agents", post(get_agents))
        .route("/agents/insert", post(insert_agent))
        .route("/agents/insert/quick", post(insert_agent_quick))
        .route("/agents", delete(delete_agents))
        
        // Agent operations
        .route("/agents/restart", put(restart_agent))
        .route("/agents/upgrade", put(upgrade_agents))
        .route("/agents/upgrade_custom", put(upgrade_agents_custom))
        
        // Group operations
        .route("/agents/group", put(put_agents_group))
        .route("/agents/group", delete(delete_agents_group))
        .route("/agents/group/:group_id/restart", put(restart_agents_by_group))

        // Node operations
        .route("/agents/node/:node_id/restart", put(restart_agents_by_node))

        // Individual agent operations
        .route("/agents/:agent_id", post(get_agent_by_id))
        .route("/agents/:agent_id", delete(delete_agent_by_id))
        .route("/agents/:agent_id/restart", put(restart_agent_by_id))
        .route("/agents/:agent_id/group", delete(delete_agent_group))
        .route("/agents/:agent_id/group/:group_id", delete(delete_agent_from_group))
        .route("/agents/:agent_id/group/:group_id", put(put_agent_group))
}
