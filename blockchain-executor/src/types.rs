use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeParams {
    pub opportunity_id: String,
    pub pair: String,
    pub amount_usdc: f64,
    pub buy_exchange: String,
    pub sell_exchange: String,
    pub expected_profit_usdc: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeResult {
    pub success: bool,
    pub tx_hash: String,
    pub gas_used: u64,
    pub gas_cost_usdc: f64,
    pub profit_usdc: f64,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasEstimate {
    pub gas_limit: u64,
    pub gas_price_usdc: f64,
    pub total_cost_usdc: f64,
}

