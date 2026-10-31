use axum::{
    routing::{get, post},
    Json,
    Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
struct QuotationItem {
    name: String,
    quantity: f64,
    unit_price: f64,
}

#[derive(Deserialize)]
struct QuotationRequest {
    customer_name: String,
    items: Vec<QuotationItem>,
    discount_percent: f64,
    gst_percent: f64,
}

async fn health() -> Json<Value> {
    Json(json!({
        "success": true,
        "business": "OCEAN PRODUCTION",
        "message": "Rust Backend is running!",
        "version": "1.0.0"
    }))
}

async fn calculate_quotation(
    Json(request): Json<QuotationRequest>,
) -> Result<Json<Value>, (axum::http::StatusCode, Json<Value>)> {

    let error = |message: &str| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": message
            })),
        )
    };

    if request.customer_name.trim().is_empty() {
        return Err(error("Customer name is required"));
    }

    if request.items.is_empty() || request.items.len() > 100 {
        return Err(error("Add between 1 and 100 quotation items"));
    }

    if !request.discount_percent.is_finite()
        || !(0.0..=100.0).contains(&request.discount_percent)
    {
        return Err(error("Discount must be between 0 and 100"));
    }

    if !request.gst_percent.is_finite()
        || !(0.0..=100.0).contains(&request.gst_percent)
    {
        return Err(error("GST must be between 0 and 100"));
    }

    let mut subtotal = 0.0;

    for item in &request.items {
        if item.name.trim().is_empty()
            || !item.quantity.is_finite()
            || !item.unit_price.is_finite()
            || item.quantity <= 0.0
            || item.unit_price < 0.0
        {
            return Err(error("Invalid quotation item"));
        }

        subtotal += item.quantity * item.unit_price;
    }

    if !subtotal.is_finite() {
        return Err(error("Quotation amount is too large"));
    }

    let discount =
        subtotal * request.discount_percent / 100.0;

    let taxable_amount = subtotal - discount;

    let gst =
        taxable_amount * request.gst_percent / 100.0;

    let total = taxable_amount + gst;

    let round = |value: f64| {
        (value * 100.0).round() / 100.0
    };

    Ok(Json(json!({
        "success": true,
        "business": "OCEAN PRODUCTION",
        "customer_name": request.customer_name,
        "subtotal": round(subtotal),
        "discount_percent": request.discount_percent,
        "discount_amount": round(discount),
        "taxable_amount": round(taxable_amount),
        "gst_percent": request.gst_percent,
        "gst_amount": round(gst),
        "grand_total": round(total),
        "currency": "INR"
    })))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(health))
        .route("/api/health", get(health))
        .route(
            "/api/quotations/calculate",
            post(calculate_quotation),
        )
        .layer(CorsLayer::permissive());

    let port = std::env::var("PORT")
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