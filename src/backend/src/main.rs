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

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/auth", post(authenticate))
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
    // 建立一個忽略 SSL 驗證的 client
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
