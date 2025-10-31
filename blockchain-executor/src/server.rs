use axum::{
    routing::{get, post},
    Router,
    Json,
    http::StatusCode,
    extract::State,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{CorsLayer, Any};
use tracing::info;
use anyhow::Result;
use std::sync::Arc;

use crate::config::Config;
use crate::arc_client::ArcClient;
use crate::types::{TradeParams, TradeResult, GasEstimate};

#[derive(Clone)]
struct AppState {
    config: Config,
    arc_client: ArcClient,
}

pub async fn start_server(config: Config, arc_client: ArcClient) -> Result<()> {
    let state = AppState {
        config: config.clone(),
        arc_client,
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/execute", post(execute_trade))
        .route("/api/v1/gas/estimate", post(estimate_gas))
        .route("/api/v1/gas/price", get(get_gas_price))
        .route("/api/v1/balance/usdc", get(get_usdc_balance))
        .route("/api/v1/chain/info", get(get_chain_info))
        .with_state(Arc::new(state))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any));

    let addr = format!("{}:{}", config.executor_host, config.executor_port);
    info!("🌐 Blockchain Executor HTTP server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "healthy".to_string(),
            service: "blockchain-executor".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }),
    )
}

async fn execute_trade(
    State(state): State<Arc<AppState>>,
    Json(params): Json<TradeParams>,
) -> (StatusCode, Json<TradeResult>) {
    match state.arc_client.execute_arbitrage(params).await {
        Ok(result) => (StatusCode::OK, Json(result)),
        Err(e) => {
            let error_result = TradeResult {
                success: false,
                tx_hash: String::new(),
                gas_used: 0,
                gas_cost_usdc: 0.0,
                profit_usdc: 0.0,
                execution_time_ms: 0,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_result))
        }
    }
}

async fn estimate_gas(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<GasEstimate>) {
    let gas_limit = 50000u64;
    let gas_cost = state.arc_client.estimate_gas_cost(gas_limit).await.unwrap_or(0.05);

    (
        StatusCode::OK,
        Json(GasEstimate {
            gas_limit,
            gas_price_usdc: gas_cost / gas_limit as f64,
            total_cost_usdc: gas_cost,
        }),
    )
}

async fn get_gas_price(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<GasPriceResponse>) {
    let gas_price = state.arc_client.get_gas_price_usdc().await.unwrap_or(0.000001);

    (
        StatusCode::OK,
        Json(GasPriceResponse {
            gas_price_usdc,
            unit: "USDC per gas".to_string(),
        }),
    )
}

async fn get_usdc_balance(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<BalanceResponse>) {
    let balance = state.arc_client.get_usdc_balance().await.unwrap_or(0.0);

    (
        StatusCode::OK,
        Json(BalanceResponse {
            balance_usdc: balance,
        }),
    )
}

async fn get_chain_info(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<ChainInfoResponse>) {
    let chain_id = state.arc_client.get_chain_id().await.unwrap_or_default();
    let block_number = state.arc_client.get_block_number().await.unwrap_or_default();

    (
        StatusCode::OK,
        Json(ChainInfoResponse {
            chain_id: chain_id.as_u64(),
            block_number: block_number.as_u64(),
            network: "Arc Testnet".to_string(),
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
struct GasPriceResponse {
    gas_price_usdc: f64,
    unit: String,
}

#[derive(Serialize)]
struct BalanceResponse {
    balance_usdc: f64,
}

#[derive(Serialize)]
struct ChainInfoResponse {
    chain_id: u64,
    block_number: u64,
    network: String,
}

