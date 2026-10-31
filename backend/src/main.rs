use axum::{
    routing::get,
    Json,
    Router,
};
use serde_json::{json, Value};
use tower_http::cors::CorsLayer;

async fn home() -> Json<Value> {
    Json(json!({
        "success": true,
        "message": "Ocean Production Rust Backend is running!",
        "version": "1.0.0"
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/api/health", get(home))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to start server");

    println!("Ocean Production Backend started!");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}