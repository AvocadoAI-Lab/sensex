mod client;
mod handlers;
mod routes;

use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create router with all routes
    let app = routes::create_router()
        .layer(cors);

    // Configure server address
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server running on http://{}", addr);

    // Start server
    axum::serve(
        tokio::net::TcpListener::bind(&addr)
            .await
            .unwrap(),
        app,
    )
    .await
    .unwrap();
}
