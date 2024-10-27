use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{CorsLayer, Any};

#[derive(Debug, Deserialize)]
struct AuthRequest {
    endpoint: String,
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct AuthResponse {
    token: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct WazuhRequest {
    endpoint: String,
    token: String,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/auth", post(authenticate))
        .route("/agents", post(get_agents))
        .route("/cluster/status", post(get_cluster_status))
        .route("/manager/status", post(get_manager_status))
        .route("/manager/info", post(get_manager_info))
        .route("/cluster/local/info", post(get_cluster_local_info))
        .route("/rules", post(get_rules))
        .route("/decoders", post(get_decoders))
        .route("/manager/logs", post(get_manager_logs))
        .route("/manager/stats", post(get_manager_stats))
        .route("/health", get(health_check))
        .layer(cors);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server running on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(&addr)
            .await
            .unwrap(),
        app,
    )
    .await
    .unwrap();
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn authenticate(Json(payload): Json<AuthRequest>) -> Json<AuthResponse> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    
    let auth_url = format!("{}/security/user/authenticate", payload.endpoint);
    
    match client
        .get(&auth_url)
        .basic_auth(payload.username, Some(payload.password))
        .send()
        .await {
            Ok(response) => {
                match response.json::<serde_json::Value>().await {
                    Ok(data) => {
                        if let Some(token) = data["data"]["token"].as_str() {
                            Json(AuthResponse {
                                token: Some(token.to_string()),
                                error: None,
                            })
                        } else {
                            Json(AuthResponse {
                                token: None,
                                error: Some("Token not found in response".to_string()),
                            })
                        }
                    },
                    Err(e) => Json(AuthResponse {
                        token: None,
                        error: Some(format!("Failed to parse response: {}", e)),
                    }),
                }
            },
            Err(e) => Json(AuthResponse {
                token: None,
                error: Some(format!("Request failed: {}", e)),
            }),
        }
}

async fn get_agents(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    
    let url = format!("{}/agents", payload.endpoint);
    
    match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", payload.token))
        .send()
        .await {
            Ok(response) => {
                match response.json::<serde_json::Value>().await {
                    Ok(data) => Json(data),
                    Err(e) => Json(serde_json::json!({
                        "error": format!("Failed to parse response: {}", e)
                    })),
                }
            },
            Err(e) => Json(serde_json::json!({
                "error": format!("Request failed: {}", e)
            })),
        }
}

async fn get_cluster_status(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    
    let url = format!("{}/cluster/status", payload.endpoint);
    
    match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", payload.token))
        .send()
        .await {
            Ok(response) => {
                match response.json::<serde_json::Value>().await {
                    Ok(data) => Json(data),
                    Err(e) => Json(serde_json::json!({
                        "error": format!("Failed to parse response: {}", e)
                    })),
                }
            },
            Err(e) => Json(serde_json::json!({
                "error": format!("Request failed: {}", e)
            })),
        }
}

async fn get_manager_status(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    
    let url = format!("{}/manager/status", payload.endpoint);
    
    match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", payload.token))
        .send()
        .await {
            Ok(response) => {
                match response.json::<serde_json::Value>().await {
                    Ok(data) => Json(data),
                    Err(e) => Json(serde_json::json!({
                        "error": format!("Failed to parse response: {}", e)
                    })),
                }
            },
            Err(e) => Json(serde_json::json!({
                "error": format!("Request failed: {}", e)
            })),
        }
}

async fn get_manager_info(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    
    let url = format!("{}/manager/info", payload.endpoint);
    
    match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", payload.token))
        .send()
        .await {
            Ok(response) => {
                match response.json::<serde_json::Value>().await {
                    Ok(data) => Json(data),
                    Err(e) => Json(serde_json::json!({
                        "error": format!("Failed to parse response: {}", e)
                    })),
                }
            },
            Err(e) => Json(serde_json::json!({
                "error": format!("Request failed: {}", e)
            })),
        }
}

async fn get_cluster_local_info(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    
    let url = format!("{}/cluster/local/info", payload.endpoint);
    
    match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", payload.token))
        .send()
        .await {
            Ok(response) => {
                match response.json::<serde_json::Value>().await {
                    Ok(data) => Json(data),
                    Err(e) => Json(serde_json::json!({
                        "error": format!("Failed to parse response: {}", e)
                    })),
                }
            },
            Err(e) => Json(serde_json::json!({
                "error": format!("Request failed: {}", e)
            })),
        }
}

async fn get_rules(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    
    let url = format!("{}/rules", payload.endpoint);
    
    match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", payload.token))
        .send()
        .await {
            Ok(response) => {
                match response.json::<serde_json::Value>().await {
                    Ok(data) => Json(data),
                    Err(e) => Json(serde_json::json!({
                        "error": format!("Failed to parse response: {}", e)
                    })),
                }
            },
            Err(e) => Json(serde_json::json!({
                "error": format!("Request failed: {}", e)
            })),
        }
}

async fn get_decoders(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    
    let url = format!("{}/decoders", payload.endpoint);
    
    match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", payload.token))
        .send()
        .await {
            Ok(response) => {
                match response.json::<serde_json::Value>().await {
                    Ok(data) => Json(data),
                    Err(e) => Json(serde_json::json!({
                        "error": format!("Failed to parse response: {}", e)
                    })),
                }
            },
            Err(e) => Json(serde_json::json!({
                "error": format!("Request failed: {}", e)
            })),
        }
}

async fn get_manager_logs(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    
    let url = format!("{}/manager/logs", payload.endpoint);
    
    match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", payload.token))
        .send()
        .await {
            Ok(response) => {
                match response.json::<serde_json::Value>().await {
                    Ok(data) => Json(data),
                    Err(e) => Json(serde_json::json!({
                        "error": format!("Failed to parse response: {}", e)
                    })),
                }
            },
            Err(e) => Json(serde_json::json!({
                "error": format!("Request failed: {}", e)
            })),
        }
}

async fn get_manager_stats(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    
    let url = format!("{}/manager/stats", payload.endpoint);
    
    match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", payload.token))
        .send()
        .await {
            Ok(response) => {
                match response.json::<serde_json::Value>().await {
                    Ok(data) => Json(data),
                    Err(e) => Json(serde_json::json!({
                        "error": format!("Failed to parse response: {}", e)
                    })),
                }
            },
            Err(e) => Json(serde_json::json!({
                "error": format!("Request failed: {}", e)
            })),
        }
}
