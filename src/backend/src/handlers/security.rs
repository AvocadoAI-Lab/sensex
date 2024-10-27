use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Authentication
pub async fn authenticate_user(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/user/authenticate", |url| url).await
}

pub async fn get_auth_status(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/user/authenticate", |url| url).await
}

pub async fn logout_user(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/user/authenticate", |url| url).await
}

pub async fn run_as_auth(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/user/authenticate/run_as", |url| url).await
}

pub async fn get_user_me(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/users/me", |url| url).await
}

pub async fn get_user_me_policies(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/users/me/policies", |url| url).await
}

pub async fn revoke_user_tokens(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/user/revoke", |url| url).await
}

pub async fn set_user_run_as(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/users/{user_id}/run_as", |url| url).await
}

// Actions and Resources
pub async fn get_security_actions(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/actions", |url| url).await
}

pub async fn get_security_resources(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/resources", |url| url).await
}

// User Management
pub async fn get_security_users(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/users", |url| url).await
}

pub async fn create_user(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/users", |url| url).await
}

pub async fn delete_users(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/users", |url| url).await
}

pub async fn update_user(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/users/{user_id}", |url| url).await
}

// Role Management
pub async fn get_security_roles(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/roles", |url| url).await
}

pub async fn create_role(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/roles", |url| url).await
}

pub async fn delete_roles(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/roles", |url| url).await
}

pub async fn update_role(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/roles/{role_id}", |url| url).await
}

// Rule Management
pub async fn get_security_rules(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/rules", |url| url).await
}

pub async fn create_security_rule(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/rules", |url| url).await
}

pub async fn delete_security_rules(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/rules", |url| url).await
}

pub async fn update_security_rule(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/rules/{rule_id}", |url| url).await
}

// Policy Management
pub async fn get_security_policies(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/policies", |url| url).await
}

pub async fn create_policy(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/policies", |url| url).await
}

pub async fn delete_policies(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/policies", |url| url).await
}

pub async fn update_policy(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/policies/{policy_id}", |url| url).await
}

// Relationship Management
pub async fn add_user_role(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/users/{user_id}/roles", |url| url).await
}

pub async fn remove_user_role(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/users/{user_id}/roles", |url| url).await
}

pub async fn add_role_policy(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/roles/{role_id}/policies", |url| url).await
}

pub async fn remove_role_policy(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/roles/{role_id}/policies", |url| url).await
}

pub async fn add_role_rule(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/roles/{role_id}/rules", |url| url).await
}

pub async fn remove_role_rule(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/roles/{role_id}/rules", |url| url).await
}

// Security Configuration
pub async fn get_security_config(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/config", |url| url).await
}

pub async fn update_security_config(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/config", |url| url).await
}

pub async fn delete_security_config(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/config", |url| url).await
}
