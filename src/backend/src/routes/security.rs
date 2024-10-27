use axum::{
    Router,
    routing::post,
};
use crate::handlers::{
    authenticate,
    get_security_users,
    get_security_roles,
    get_security_policies,
    get_mitre_metadata,
    get_mitre_techniques,
    get_mitre_tactics,
    get_active_response_commands,
    run_active_response_command,
};

pub fn routes() -> Router {
    Router::new()
        // Authentication
        .route("/auth", post(authenticate))
        
        // Security management
        .route("/security/users", post(get_security_users))
        .route("/security/roles", post(get_security_roles))
        .route("/security/policies", post(get_security_policies))
        
        // MITRE ATT&CK
        .route("/mitre/metadata", post(get_mitre_metadata))
        .route("/mitre/techniques", post(get_mitre_techniques))
        .route("/mitre/tactics", post(get_mitre_tactics))
        
        // Active response
        .route("/active-response/commands", post(get_active_response_commands))
        .route("/active-response/run", post(run_active_response_command))
}
