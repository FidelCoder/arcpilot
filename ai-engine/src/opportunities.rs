use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{info, error, debug};
use uuid::Uuid;
use chrono::Utc;
use std::time::Duration;

use crate::config::Config;
use crate::models::OpportunityDetector;
use crate::risk::RiskScorer;
use crate::kafka::KafkaProducer;
use crate::redis_client::RedisClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opportunity {
    pub id: String,
    pub opportunity_type: String,
    pub pair: String,
    pub buy_exchange: String,
    pub buy_price: f64,
    pub sell_exchange: String,
    pub sell_price: f64,
    pub profit_margin: f64,
    pub net_profit: f64,
    pub ai_score: f64,
    pub risk_score: f64,
    pub liquidity_score: f64,
    pub estimated_gas_usd: f64,
    pub expires_at: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub exchange: String,
    pub pair: String,
    pub price: f64,
    pub volume: f64,
    pub liquidity: f64,
    pub timestamp: String,
}

/// Main detection loop that runs continuously
pub async fn detection_loop(
    redis_client: RedisClient,
    kafka_producer: KafkaProducer,
    opportunity_detector: OpportunityDetector,
    risk_scorer: RiskScorer,
    config: Config,
) -> Result<()> {
    info!("🔍 Starting opportunity detection loop");

    let mut interval = tokio::time::interval(Duration::from_millis(config.detection_interval_ms));

    loop {
        interval.tick().await;

        match detect_opportunities(
            &redis_client,
            &kafka_producer,
            &opportunity_detector,
            &risk_scorer,
            &config,
        ).await {
            Ok(count) => {
                if count > 0 {
                    debug!("Found {} opportunities in this cycle", count);
                }
            }
            Err(e) => {
                error!("Error in detection cycle: {}", e);
            }
        }
    }
}

/// Detect arbitrage opportunities from market data
async fn detect_opportunities(
    redis_client: &RedisClient,
    kafka_producer: &KafkaProducer,
    opportunity_detector: &OpportunityDetector,
    risk_scorer: &RiskScorer,
    config: &Config,
) -> Result<usize> {
    // Fetch market data from Redis
    let market_data = redis_client.get_market_data().await?;

    if market_data.is_empty() {
        return Ok(0);
    }

    // Detect opportunities using AI model
    let raw_opportunities = opportunity_detector.detect(&market_data)?;

    let mut valid_opportunities = Vec::new();

    for opp in raw_opportunities {
        // Calculate risk score
        let risk_score = risk_scorer.calculate_risk(&opp, &market_data)?;

        // Filter by risk threshold
        if risk_score > config.max_risk_score {
            debug!("Opportunity filtered by risk: {:.2}", risk_score);
            continue;
        }

        // Filter by profit threshold
        if opp.profit_margin < config.min_profit_threshold {
            debug!("Opportunity filtered by profit: {:.4}", opp.profit_margin);
            continue;
        }

        // Create opportunity with risk score
        let opportunity = Opportunity {
            id: format!("opp_{}", Uuid::new_v4()),
            opportunity_type: "arbitrage".to_string(),
            pair: opp.pair.clone(),
            buy_exchange: opp.buy_exchange.clone(),
            buy_price: opp.buy_price,
            sell_exchange: opp.sell_exchange.clone(),
            sell_price: opp.sell_price,
            profit_margin: opp.profit_margin,
            net_profit: opp.net_profit,
            ai_score: opp.ai_score,
            risk_score,
            liquidity_score: opp.liquidity_score,
            estimated_gas_usd: opp.estimated_gas_usd,
            expires_at: (Utc::now() + chrono::Duration::seconds(15)).to_rfc3339(),
            timestamp: Utc::now().to_rfc3339(),
        };

        // Store in Redis
        redis_client.store_opportunity(&opportunity).await?;

        // Publish to Kafka
        kafka_producer.publish_opportunity(&opportunity).await?;

        info!(
            "💰 Found opportunity: {} on {} -> {}, profit: ${:.2}, risk: {:.2}",
            opportunity.pair,
            opportunity.buy_exchange,
            opportunity.sell_exchange,
            opportunity.net_profit,
            opportunity.risk_score
        );

        valid_opportunities.push(opportunity);
    }

    Ok(valid_opportunities.len())
}

#[derive(Debug, Clone)]
pub struct RawOpportunity {
    pub pair: String,
    pub buy_exchange: String,
    pub buy_price: f64,
    pub sell_exchange: String,
    pub sell_price: f64,
    pub profit_margin: f64,
    pub net_profit: f64,
    pub ai_score: f64,
    pub liquidity_score: f64,
    pub estimated_gas_usd: f64,
}

