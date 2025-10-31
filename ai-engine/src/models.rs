use anyhow::Result;
use std::sync::Arc;
use tracing::info;

use crate::opportunities::{MarketData, RawOpportunity};

/// AI model for detecting arbitrage opportunities
#[derive(Clone)]
pub struct OpportunityDetector {
    // In a real implementation, this would contain trained ML models
    // For the hackathon MVP, we'll use rule-based detection with AI-like scoring
}

impl OpportunityDetector {
    pub fn new() -> Result<Self> {
        info!("Initializing OpportunityDetector model");
        Ok(Self {})
    }

    /// Detect arbitrage opportunities from market data
    pub fn detect(&self, market_data: &[MarketData]) -> Result<Vec<RawOpportunity>> {
        let mut opportunities = Vec::new();

        // Group market data by pair
        let mut pairs: std::collections::HashMap<String, Vec<&MarketData>> = 
            std::collections::HashMap::new();

        for data in market_data {
            pairs.entry(data.pair.clone())
                .or_insert_with(Vec::new)
                .push(data);
        }

        // Look for price differences across exchanges
        for (pair, exchanges) in pairs {
            if exchanges.len() < 2 {
                continue;
            }

            for i in 0..exchanges.len() {
                for j in (i + 1)..exchanges.len() {
                    let exchange_a = exchanges[i];
                    let exchange_b = exchanges[j];

                    // Calculate potential arbitrage
                    if exchange_a.price < exchange_b.price {
                        let opp = self.calculate_opportunity(
                            &pair,
                            exchange_a,
                            exchange_b,
                        );
                        if let Some(opp) = opp {
                            opportunities.push(opp);
                        }
                    } else if exchange_b.price < exchange_a.price {
                        let opp = self.calculate_opportunity(
                            &pair,
                            exchange_b,
                            exchange_a,
                        );
                        if let Some(opp) = opp {
                            opportunities.push(opp);
                        }
                    }
                }
            }
        }

        Ok(opportunities)
    }

    fn calculate_opportunity(
        &self,
        pair: &str,
        buy_exchange: &MarketData,
        sell_exchange: &MarketData,
    ) -> Option<RawOpportunity> {
        let profit_margin = (sell_exchange.price - buy_exchange.price) / buy_exchange.price;

        // Minimum threshold check
        if profit_margin < 0.0005 {
            return None;
        }

        // Estimate gas cost in USDC (Arc advantage: predictable)
        let estimated_gas_usd = 0.05; // ~$0.05 per transaction on Arc

        // Calculate net profit on a $1000 trade
        let trade_amount = 1000.0;
        let gross_profit = trade_amount * profit_margin;
        let net_profit = gross_profit - (estimated_gas_usd * 2.0); // buy + sell

        if net_profit <= 0.0 {
            return None;
        }

        // Calculate AI confidence score based on multiple factors
        let liquidity_factor = (buy_exchange.liquidity.min(sell_exchange.liquidity) / 100000.0).min(1.0);
        let volume_factor = (buy_exchange.volume.min(sell_exchange.volume) / 1000000.0).min(1.0);
        let profit_factor = (profit_margin * 100.0).min(1.0);

        let ai_score = (liquidity_factor * 0.4 + volume_factor * 0.3 + profit_factor * 0.3).min(1.0);

        let liquidity_score = liquidity_factor;

        Some(RawOpportunity {
            pair: pair.to_string(),
            buy_exchange: buy_exchange.exchange.clone(),
            buy_price: buy_exchange.price,
            sell_exchange: sell_exchange.exchange.clone(),
            sell_price: sell_exchange.price,
            profit_margin,
            net_profit,
            ai_score,
            liquidity_score,
            estimated_gas_usd,
        })
    }
}

