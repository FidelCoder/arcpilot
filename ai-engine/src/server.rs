use axum::{
    routing::{get, post},
    Router,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{CorsLayer, Any};
use tracing::info;
use anyhow::Result;

use crate::config::Config;

pub async fn start_server(config: Config) -> Result<()> {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/detect", post(manual_detection))
        .route("/api/v1/models/status", get(model_status))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any));

    let addr = format!("{}:{}", config.ai_engine_host, config.ai_engine_port);
    info!("🌐 AI Engine HTTP server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "healthy".to_string(),
            service: "ai-engine".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }),
    )
}

async fn manual_detection() -> (StatusCode, Json<DetectionResponse>) {
    // Placeholder for manual detection trigger
    (
        StatusCode::OK,
        Json(DetectionResponse {
            message: "Detection triggered".to_string(),
            opportunities_found: 0,
        }),
    )
}

async fn model_status() -> (StatusCode, Json<ModelStatusResponse>) {
    (
        StatusCode::OK,
        Json(ModelStatusResponse {
            loaded: true,
            model_version: "1.0.0".to_string(),
            last_training: None,
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
struct DetectionResponse {
    message: String,
    opportunities_found: usize,
}

#[derive(Serialize)]
struct ModelStatusResponse {
    loaded: bool,
    model_version: String,
    last_training: Option<String>,
}

