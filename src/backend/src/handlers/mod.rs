pub mod agents;
pub mod auth;
pub mod cluster;
pub mod common;
pub mod decoders;
pub mod groups;
pub mod lists;
pub mod manager;
pub mod mitre;
pub mod rules;
pub mod security;
pub mod syscollector;
pub mod tasks;

// Re-export handlers that are used in routes
pub use agents::{
    get_agents,
    get_agent_config_by_id,
    get_agent_group_sync_status,
    get_daemon_stats,
    get_agent_stats_component,
    get_agents_without_group,
    get_outdated_agents,
    get_distinct_agents_stats,
    get_agents_os_summary,
    get_agents_status_summary,
};

pub use cluster::{
    get_cluster_status,
    get_cluster_local_info,
    get_cluster_nodes,
    get_cluster_healthcheck,
};

pub use decoders::{
    get_decoders,
    get_decoder_files,
    get_decoder_parents,
};

pub use groups::{
    get_groups,
    get_group_files,
    get_group_agents,
    get_group_configuration,
};

pub use lists::{
    get_lists,
    get_lists_files,
};

pub use manager::{
    get_api_info,
    get_manager_status,
    get_manager_info,
    get_manager_configuration,
    get_manager_stats,
    get_manager_hourly_stats,
    get_manager_weekly_stats,
    get_manager_logs,
    get_manager_logs_summary,
};

pub use mitre::{
    get_mitre_groups,
    get_mitre_metadata,
    get_mitre_mitigations,
    get_mitre_references,
    get_mitre_software,
    get_mitre_tactics,
    get_mitre_techniques,
};

pub use rules::{
    get_rules,
    get_rules_groups,
    get_rules_files,
};

pub use security::{
    get_security_actions,
    get_security_resources,
    get_security_config,
};

pub use syscollector::{
    get_syscollector_hardware,
    get_syscollector_hotfixes,
    get_syscollector_netaddr,
    get_syscollector_netiface,
    get_syscollector_netproto,
    get_syscollector_os,
    get_syscollector_packages,
    get_syscollector_ports,
    get_syscollector_processes,
};

pub use tasks::{
    get_tasks_status,
};
