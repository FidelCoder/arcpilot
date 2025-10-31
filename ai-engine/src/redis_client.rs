use anyhow::Result;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, RedisError};
use tracing::{debug, error};
use serde_json;

use crate::opportunities::{Opportunity, MarketData};

#[derive(Clone)]
pub struct RedisClient {
    conn: ConnectionManager,
}

impl RedisClient {
    pub async fn new(redis_url: &str) -> Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let conn = ConnectionManager::new(client).await?;

        Ok(Self { conn })
    }

    /// Store an opportunity in Redis with TTL
    pub async fn store_opportunity(&self, opportunity: &Opportunity) -> Result<()> {
        let mut conn = self.conn.clone();
        let key = format!("opportunity:{}", opportunity.id);
        let value = serde_json::to_string(opportunity)?;

        // Store with 60 second expiry
        conn.set_ex(&key, value, 60).await?;

        // Add to opportunities list
        conn.zadd("opportunities:active", &opportunity.id, opportunity.timestamp.clone()).await?;

        debug!("Stored opportunity {} in Redis", opportunity.id);
        Ok(())
    }

    /// Get market data from Redis
    pub async fn get_market_data(&self) -> Result<Vec<MarketData>> {
        let mut conn = self.conn.clone();

        // Get all market data keys
        let keys: Vec<String> = conn.keys("market:*").await?;

        let mut market_data = Vec::new();

        for key in keys {
            let value: String = conn.get(&key).await?;
            if let Ok(data) = serde_json::from_str::<MarketData>(&value) {
                market_data.push(data);
            }
        }

        Ok(market_data)
    }

    /// Get active opportunities from Redis
    pub async fn get_opportunities(&self, limit: usize) -> Result<Vec<Opportunity>> {
        let mut conn = self.conn.clone();

        // Get recent opportunity IDs
        let ids: Vec<String> = conn.zrange("opportunities:active", 0, limit as isize - 1).await?;

        let mut opportunities = Vec::new();

        for id in ids {
            let key = format!("opportunity:{}", id);
            if let Ok(value) = conn.get::<_, String>(&key).await {
                if let Ok(opp) = serde_json::from_str::<Opportunity>(&value) {
                    opportunities.push(opp);
                }
            }
        }

        Ok(opportunities)
    }
}

