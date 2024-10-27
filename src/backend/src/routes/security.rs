use axum::{
    Router,
    routing::{get, post, put, delete},
};
use crate::handlers::security::*;

pub fn routes() -> Router {
    Router::new()
        // Authentication
        .route("/security/user/authenticate", post(authenticate_user))
        .route("/security/user/authenticate", get(get_auth_status))
        .route("/security/user/authenticate", delete(logout_user))
        .route("/security/user/authenticate/run_as", post(run_as_auth))
        .route("/security/users/me", get(get_user_me))
        .route("/security/users/me", post(get_user_me))  // Add POST support
        .route("/security/users/me/policies", get(get_user_me_policies))
        .route("/security/users/me/policies", post(get_user_me_policies))  // Add POST support
        .route("/security/user/revoke", put(revoke_user_tokens))
        .route("/security/users/:user_id/run_as", put(set_user_run_as))
        
        // Actions and Resources
        .route("/security/actions", get(get_security_actions))
        .route("/security/actions", post(get_security_actions))  // Add POST support
        .route("/security/resources", get(get_security_resources))
        .route("/security/resources", post(get_security_resources))  // Add POST support
        
        // User Management
        .route("/security/users", get(get_security_users))
        .route("/security/users", post(create_user))
        .route("/security/users", delete(delete_users))
        .route("/security/users/:user_id", put(update_user))
        
        // Role Management
        .route("/security/roles", get(get_security_roles))
        .route("/security/roles", post(create_role))
        .route("/security/roles", delete(delete_roles))
        .route("/security/roles/:role_id", put(update_role))
        
        // Rule Management
        .route("/security/rules", get(get_security_rules))
        .route("/security/rules", post(create_security_rule))
        .route("/security/rules", delete(delete_security_rules))
        .route("/security/rules/:rule_id", put(update_security_rule))
        
        // Policy Management
        .route("/security/policies", get(get_security_policies))
        .route("/security/policies", post(create_policy))
        .route("/security/policies", delete(delete_policies))
        .route("/security/policies/:policy_id", put(update_policy))
        
        // Relationship Management
        .route("/security/users/:user_id/roles", post(add_user_role))
        .route("/security/users/:user_id/roles", delete(remove_user_role))
        .route("/security/roles/:role_id/policies", post(add_role_policy))
        .route("/security/roles/:role_id/policies", delete(remove_role_policy))
        .route("/security/roles/:role_id/rules", post(add_role_rule))
        .route("/security/roles/:role_id/rules", delete(remove_role_rule))
        
        // Security Configuration
        .route("/security/config", get(get_security_config))
        .route("/security/config", post(get_security_config))  // Add POST support
        .route("/security/config", put(update_security_config))
        .route("/security/config", delete(delete_security_config))
}
