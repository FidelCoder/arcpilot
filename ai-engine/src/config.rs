use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub ai_engine_host: String,
    pub ai_engine_port: u16,
    pub redis_url: String,
    pub kafka_brokers: String,
    pub detection_interval_ms: u64,
    pub min_profit_threshold: f64,
    pub max_risk_score: f64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let config = Self {
            ai_engine_host: std::env::var("AI_ENGINE_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            ai_engine_port: std::env::var("AI_ENGINE_PORT")
                .unwrap_or_else(|_| "8081".to_string())
                .parse()?,
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            kafka_brokers: std::env::var("KAFKA_BROKERS")
                .unwrap_or_else(|_| "localhost:9092".to_string()),
            detection_interval_ms: std::env::var("DETECTION_INTERVAL_MS")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()?,
            min_profit_threshold: std::env::var("MIN_PROFIT_THRESHOLD")
                .unwrap_or_else(|_| "0.001".to_string())
                .parse()?,
            max_risk_score: std::env::var("MAX_RISK_SCORE")
                .unwrap_or_else(|_| "0.7".to_string())
                .parse()?,
        };

        Ok(config)
    }
}

