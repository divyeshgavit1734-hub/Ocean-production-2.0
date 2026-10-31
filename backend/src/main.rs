use axum::{
    routing::get,
    Json,
    Router,
};
use serde_json::{json, Value};
use tower_http::cors::CorsLayer;
use std::env;

async fn health() -> Json<Value> {
    Json(json!({
        "success": true,
        "business": "OCEAN PRODUCTION",
        "message": "Rust Backend is running!",
        "version": "1.0.0"
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(health))
        .route("/api/health", get(health))
        .layer(CorsLayer::permissive());

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string());

    let address = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("Failed to start server");

    println!("Ocean Production Backend running on {}", address);

    axum::serve(listener, app)
        .await
        .expect("Server error");
}