use axum::{
    Router,
    routing::{get, post, put, delete},
};
use crate::handlers::{
    get_agents,
    add_agent,
    delete_agents,
    get_agent_config,
    get_agent_key,
    get_agent_no_group,
    get_agent_outdated,
    get_agent_stats,
    get_agent_fields,
    get_daemon_stats,
    get_agent_summary_status,
    get_agent_summary_os,
    restart_agent,
    upgrade_agents,
    get_agent_upgrade_result,
    get_agent_sync_status,
    // New handlers
    get_agent_by_id,
    delete_agent_by_id,
    restart_agent_by_id,
    get_agent_config_by_id,
    get_agent_stats_process,
    get_agent_stats_anomaly,
    get_agent_stats_syscollector,
};

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
        .route("/agents/upgrade_result", get(get_agent_upgrade_result))
        .route("/agents/sync", get(get_agent_sync_status))

        // New routes with {agent_id}
        .route("/agents/:agent_id", get(get_agent_by_id))
        .route("/agents/:agent_id", delete(delete_agent_by_id))
        .route("/agents/:agent_id/restart", put(restart_agent_by_id))
        .route("/agents/:agent_id/config/:component/:configuration", get(get_agent_config_by_id))
        .route("/agents/:agent_id/stats/process", get(get_agent_stats_process))
        .route("/agents/:agent_id/stats/anomaly", get(get_agent_stats_anomaly))
        .route("/agents/:agent_id/stats/syscollector", get(get_agent_stats_syscollector))
}
