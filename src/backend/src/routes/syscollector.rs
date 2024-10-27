use axum::{
    Router,
    routing::get,
};
use crate::handlers::syscollector::*;

pub fn routes() -> Router {
    Router::new()
        // Hardware information
        .route("/syscollector/:agent_id/hardware", get(get_syscollector_hardware))
        
        // Hotfixes information
        .route("/syscollector/:agent_id/hotfixes", get(get_syscollector_hotfixes))
        
        // Network information
        .route("/syscollector/:agent_id/netaddr", get(get_syscollector_netaddr))
        .route("/syscollector/:agent_id/netiface", get(get_syscollector_netiface))
        .route("/syscollector/:agent_id/netproto", get(get_syscollector_netproto))
        
        // Operating system information
        .route("/syscollector/:agent_id/os", get(get_syscollector_os))
        
        // Package information
        .route("/syscollector/:agent_id/packages", get(get_syscollector_packages))
        
        // Port information
        .route("/syscollector/:agent_id/ports", get(get_syscollector_ports))
        
        // Process information
        .route("/syscollector/:agent_id/processes", get(get_syscollector_processes))
}
