use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod opportunities;
mod risk;
mod models;
mod server;
mod kafka;
mod redis_client;

use crate::config::Config;
use crate::server::start_server;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ai_engine=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Load configuration
    let config = Config::from_env()?;
    info!("Configuration loaded successfully");

    // Display startup info
    info!("🤖 ArcPilot AI Engine v{}", env!("CARGO_PKG_VERSION"));
    info!("🎯 Detection interval: {}ms", config.detection_interval_ms);
    info!("📊 Min profit threshold: {}", config.min_profit_threshold);
    info!("🛡️ Max risk score: {}", config.max_risk_score);

    // Initialize Redis connection
    let redis_client = redis_client::RedisClient::new(&config.redis_url).await?;
    info!("✅ Redis connected");

    // Initialize Kafka producer
    let kafka_producer = kafka::KafkaProducer::new(&config.kafka_brokers)?;
    info!("✅ Kafka producer initialized");

    // Initialize AI models
    let opportunity_detector = models::OpportunityDetector::new()?;
    let risk_scorer = risk::RiskScorer::new()?;
    info!("✅ AI models loaded");

    // Start opportunity detection task
    let detector_handle = tokio::spawn({
        let redis_client = redis_client.clone();
        let kafka_producer = kafka_producer.clone();
        let opportunity_detector = opportunity_detector.clone();
        let risk_scorer = risk_scorer.clone();
        let config = config.clone();
        
        async move {
            opportunities::detection_loop(
                redis_client,
                kafka_producer,
                opportunity_detector,
                risk_scorer,
                config,
            ).await
        }
    });

    // Start HTTP server
    let server_handle = tokio::spawn(async move {
        if let Err(e) = start_server(config).await {
            error!("Server error: {}", e);
        }
    });

    // Wait for tasks
    tokio::select! {
        _ = detector_handle => {
            error!("Detection loop ended unexpectedly");
        }
        _ = server_handle => {
            error!("Server ended unexpectedly");
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Received shutdown signal");
        }
    }

    info!("🛑 AI Engine shutting down gracefully");
    Ok(())
}

