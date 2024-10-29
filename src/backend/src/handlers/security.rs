use axum::Json;
use axum::http::StatusCode;
use serde_json::Value;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};
use crate::client::WazuhClient;

// Authentication
pub async fn authenticate_user(Json(payload): Json<WazuhRequest>) -> (StatusCode, Json<Value>) {
    let client = WazuhClient::new();
    let auth_url = format!("{}/security/user/authenticate", payload.endpoint);
    
    // Extract Basic auth from token field
    let auth_parts: Vec<&str> = payload.token.split(':').collect();
    if auth_parts.len() != 2 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid authentication format"
            }))
        );
    }

    let username = auth_parts[0];
    let password = auth_parts[1];

    // Get JWT from Wazuh
    match client.get_with_auth(&auth_url, username, password).await {
        Ok(response) => {
            if response.status().as_u16() == 401 {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({
                        "error": "Invalid credentials"
                    }))
                );
            }

            match WazuhClient::handle_json_response(response).await {
                Ok(data) => {
                    if let Some(token) = data["data"]["token"].as_str() {
                        (
                            StatusCode::OK,
                            Json(serde_json::json!({
                                "data": {
                                    "token": token
                                }
                            }))
                        )
                    } else {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(serde_json::json!({
                                "error": "Token not found in response"
                            }))
                        )
                    }
                },
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": e
                    }))
                ),
            }
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Request failed: {}", e)
            }))
        ),
    }
}

// Security information endpoints
pub async fn get_security_actions(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/actions", |url| url).await
}

pub async fn get_security_resources(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/resources", |url| url).await
}

pub async fn get_security_config(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/config", |url| url).await
}
