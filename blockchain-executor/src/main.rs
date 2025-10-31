use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod arc_client;
mod executor;
mod types;
mod server;

use crate::config::Config;
use crate::arc_client::ArcClient;
use crate::server::start_server;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "blockchain_executor=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Load configuration
    let config = Config::from_env()?;
    info!("Configuration loaded successfully");

    // Display startup info
    info!("⛓️  ArcPilot Blockchain Executor v{}", env!("CARGO_PKG_VERSION"));
    info!("🌐 Arc RPC: {}", config.arc_rpc_url);
    info!("🔗 Chain ID: {}", config.arc_chain_id);
    info!("💵 USDC Contract: {}", config.usdc_contract_address);

    // Initialize Arc client
    let arc_client = ArcClient::new(&config).await?;
    info!("✅ Connected to Arc blockchain");

    // Get chain info
    let chain_id = arc_client.get_chain_id().await?;
    let block_number = arc_client.get_block_number().await?;
    info!("📊 Chain ID: {}, Latest block: {}", chain_id, block_number);

    // Check USDC balance (if wallet configured)
    if let Ok(balance) = arc_client.get_usdc_balance().await {
        info!("💰 USDC Balance: ${:.2}", balance);
    }

    // Start HTTP server
    let server_handle = tokio::spawn({
        let config = config.clone();
        let arc_client = arc_client.clone();
        async move {
            if let Err(e) = start_server(config, arc_client).await {
                error!("Server error: {}", e);
            }
        }
    });

    // Wait for shutdown signal
    tokio::select! {
        _ = server_handle => {
            error!("Server ended unexpectedly");
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Received shutdown signal");
        }
    }

    info!("🛑 Blockchain Executor shutting down gracefully");
    Ok(())
}

