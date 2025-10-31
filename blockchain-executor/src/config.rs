use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub executor_host: String,
    pub executor_port: u16,
    pub arc_rpc_url: String,
    pub arc_chain_id: u64,
    pub usdc_contract_address: String,
    pub gas_price_multiplier: f64,
    pub max_gas_price_gwei: u64,
    pub slippage_tolerance: f64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let config = Self {
            executor_host: std::env::var("EXECUTOR_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            executor_port: std::env::var("EXECUTOR_PORT")
                .unwrap_or_else(|_| "8082".to_string())
                .parse()?,
            arc_rpc_url: std::env::var("ARC_RPC_URL")
                .unwrap_or_else(|_| "https://testnet-rpc.arcblockchain.com".to_string()),
            arc_chain_id: std::env::var("ARC_CHAIN_ID")
                .unwrap_or_else(|_| "12345".to_string())
                .parse()?,
            usdc_contract_address: std::env::var("USDC_CONTRACT_ADDRESS")
                .unwrap_or_else(|_| "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238".to_string()),
            gas_price_multiplier: std::env::var("GAS_PRICE_MULTIPLIER")
                .unwrap_or_else(|_| "1.2".to_string())
                .parse()?,
            max_gas_price_gwei: std::env::var("MAX_GAS_PRICE_GWEI")
                .unwrap_or_else(|_| "100".to_string())
                .parse()?,
            slippage_tolerance: std::env::var("SLIPPAGE_TOLERANCE")
                .unwrap_or_else(|_| "0.005".to_string())
                .parse()?,
        };

        Ok(config)
    }
}

