use axum::{
    Router,
    routing::{get, delete},
};
use crate::handlers::experimental::*;

pub fn routes() -> Router {
    Router::new()
        // Rootcheck and Syscheck
        .route("/experimental/rootcheck", delete(delete_experimental_rootcheck))
        .route("/experimental/syscheck", delete(delete_experimental_syscheck))
        
        // Ciscat results
        .route("/experimental/ciscat/results", get(get_experimental_ciscat_results))
        
        // Syscollector information
        .route("/experimental/syscollector/hardware", get(get_experimental_syscollector_hardware))
        .route("/experimental/syscollector/netaddr", get(get_experimental_syscollector_netaddr))
        .route("/experimental/syscollector/netiface", get(get_experimental_syscollector_netiface))
        .route("/experimental/syscollector/netproto", get(get_experimental_syscollector_netproto))
        .route("/experimental/syscollector/os", get(get_experimental_syscollector_os))
        .route("/experimental/syscollector/packages", get(get_experimental_syscollector_packages))
        .route("/experimental/syscollector/ports", get(get_experimental_syscollector_ports))
        .route("/experimental/syscollector/processes", get(get_experimental_syscollector_processes))
        .route("/experimental/syscollector/hotfixes", get(get_experimental_syscollector_hotfixes))
}
