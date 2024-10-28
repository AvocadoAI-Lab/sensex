pub mod auth;
pub mod manager;
pub mod cluster;
pub mod agents;
pub mod rules;
pub mod decoders;
pub mod common;
pub mod groups;
pub mod syscollector;
pub mod syscheck;
pub mod active_response;
pub mod events;
pub mod mitre;
pub mod ciscat;
pub mod rootcheck;
pub mod security;
pub mod tasks;
pub mod sca;
pub mod lists;
pub mod logtest;
pub mod experimental;
pub mod vulnerability;  // Add vulnerability module

pub use cluster::{
    get_cluster_local_info,
    get_cluster_status,
};
pub use decoders::get_decoders;
// Re-export only the handlers that are actually used in routes
pub use manager::{
    get_manager_info,
    get_manager_logs,
    get_manager_stats,
    get_manager_status,
};
pub use rules::get_rules;
pub use syscheck::{
    get_syscheck_agent,
    get_syscheck_last_scan,
};
pub use syscollector::{
    get_syscollector_hardware,
    get_syscollector_os,
};
