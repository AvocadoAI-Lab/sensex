use axum::{
    Router,
    routing::{get, post, put, delete},
};
use crate::handlers::agents::*;

pub fn routes() -> Router {
    Router::new()
        // Base agents endpoints
        .route("/agents", get(get_agents))
        .route("/agents", post(add_agent))
        .route("/agents", delete(delete_agents))
        
        // Agent configuration and status
        .route("/agents/config", get(get_agent_config))
        .route("/agents/key", get(get_agent_key))
        .route("/agents/no_group", get(get_agent_no_group))
        .route("/agents/outdated", get(get_agent_outdated))
        
        // Agent statistics
        .route("/agents/stats", get(get_agent_stats))
        .route("/agents/stats/distinct", get(get_agent_fields))
        .route("/agents/daemons/stats", get(get_daemon_stats))
        
        // Agent summaries
        .route("/agents/summary/status", get(get_agent_summary_status))
        .route("/agents/summary/os", get(get_agent_summary_os))
        
        // Agent operations
        .route("/agents/restart", put(restart_agent))
        .route("/agents/upgrade", put(upgrade_agents))
        .route("/agents/upgrade_custom", put(upgrade_agents_custom))
        .route("/agents/upgrade_result", get(get_agent_upgrade_result))
        .route("/agents/sync", get(get_agent_sync_status))
        .route("/agents/reconnect", get(reconnect_agents))

        // Group operations
        .route("/agents/group", put(put_agents_group))
        .route("/agents/group", delete(delete_agents_group))
        .route("/agents/group/:group_id/restart", put(restart_agents_by_group))

        // Agent insertion
        .route("/agents/insert", post(insert_agent))
        .route("/agents/insert/quick", post(insert_agent_quick))

        // Node operations
        .route("/agents/node/:node_id/restart", put(restart_agents_by_node))

        // Individual agent operations
        .route("/agents/:agent_id", get(get_agent_by_id))
        .route("/agents/:agent_id", delete(delete_agent_by_id))
        .route("/agents/:agent_id/restart", put(restart_agent_by_id))
        .route("/agents/:agent_id/config/:component/:configuration", get(get_agent_config_by_id))
        .route("/agents/:agent_id/group", delete(delete_agent_group))
        .route("/agents/:agent_id/group/is_sync", get(get_agent_group_sync_status))
        .route("/agents/:agent_id/group/:group_id", delete(delete_agent_from_group))
        .route("/agents/:agent_id/group/:group_id", put(put_agent_group))
        .route("/agents/:agent_id/stats/process", get(get_agent_stats_process))
        .route("/agents/:agent_id/stats/anomaly", get(get_agent_stats_anomaly))
        .route("/agents/:agent_id/stats/syscollector", get(get_agent_stats_syscollector))
}
