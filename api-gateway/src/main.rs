use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{CorsLayer, Any};
use tracing::{info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api_gateway=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    let api_port = std::env::var("API_PORT")
        .unwrap_or_else(|_| "8080".to_string());

    info!("🌐 ArcPilot API Gateway v{}", env!("CARGO_PKG_VERSION"));

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/opportunities", get(get_opportunities))
        .route("/api/v1/trades/execute", post(execute_trade))
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any));

    let addr = format!("0.0.0.0:{}", api_port);
    info!("🚀 API Gateway starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "healthy".to_string(),
            service: "api-gateway".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }),
    )
}

async fn get_opportunities() -> (StatusCode, Json<Vec<()>>) {
    (StatusCode::OK, Json(vec![]))
}

async fn execute_trade() -> (StatusCode, Json<TradeResponse>) {
    (
        StatusCode::OK,
        Json(TradeResponse {
            success: true,
            message: "Trade execution in progress".to_string(),
        }),
    )
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
}

#[derive(Serialize)]
struct TradeResponse {
    success: bool,
    message: String,
}

