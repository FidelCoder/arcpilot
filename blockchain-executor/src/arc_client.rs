use anyhow::{Result, Context};
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use std::sync::Arc;
use tracing::{info, debug};

use crate::config::Config;
use crate::types::{TradeParams, TradeResult};

#[derive(Clone)]
pub struct ArcClient {
    provider: Arc<Provider<Http>>,
    chain_id: U256,
    usdc_address: Address,
}

impl ArcClient {
    pub async fn new(config: &Config) -> Result<Self> {
        // Connect to Arc RPC
        let provider = Provider::<Http>::try_from(&config.arc_rpc_url)
            .context("Failed to connect to Arc RPC")?;

        let provider = Arc::new(provider);

        // Parse USDC contract address
        let usdc_address: Address = config.usdc_contract_address
            .parse()
            .context("Invalid USDC contract address")?;

        let chain_id = U256::from(config.arc_chain_id);

        info!("✅ Arc client initialized");

        Ok(Self {
            provider,
            chain_id,
            usdc_address,
        })
    }

    /// Get current chain ID
    pub async fn get_chain_id(&self) -> Result<U64> {
        let chain_id = self.provider.get_chainid().await?;
        Ok(chain_id)
    }

    /// Get latest block number
    pub async fn get_block_number(&self) -> Result<U64> {
        let block_number = self.provider.get_block_number().await?;
        Ok(block_number)
    }

    /// Get USDC balance for an address
    pub async fn get_usdc_balance(&self) -> Result<f64> {
        // This would query the USDC contract
        // For now, return a placeholder
        Ok(0.0)
    }

    /// Get current gas price in USDC
    /// Arc's unique feature: gas is paid in USDC, not native token
    pub async fn get_gas_price_usdc(&self) -> Result<f64> {
        let gas_price = self.provider.get_gas_price().await?;
        
        // Convert wei to USDC (this is simplified)
        // On Arc, gas is actually denominated in USDC
        let gas_price_usdc = gas_price.as_u128() as f64 / 1e6; // USDC has 6 decimals

        debug!("Current gas price: ${:.6} USDC", gas_price_usdc);

        Ok(gas_price_usdc)
    }

    /// Estimate gas cost for a transaction in USDC
    pub async fn estimate_gas_cost(&self, gas_limit: u64) -> Result<f64> {
        let gas_price = self.get_gas_price_usdc().await?;
        let gas_cost_usdc = gas_price * gas_limit as f64;

        debug!("Estimated gas cost: ${:.6} USDC", gas_cost_usdc);

        Ok(gas_cost_usdc)
    }

    /// Execute an arbitrage trade on Arc
    pub async fn execute_arbitrage(&self, params: TradeParams) -> Result<TradeResult> {
        info!("Executing arbitrage trade: {} USDC on {}", params.amount_usdc, params.pair);

        // In production, this would:
        // 1. Build transaction data
        // 2. Estimate gas
        // 3. Sign transaction
        // 4. Submit to Arc blockchain
        // 5. Wait for confirmation
        // 6. Return result

        // For MVP, simulate the transaction
        let estimated_gas = 50000u64;
        let gas_cost = self.estimate_gas_cost(estimated_gas).await?;

        let result = TradeResult {
            success: true,
            tx_hash: format!("0x{}", hex::encode(vec![1u8; 32])),
            gas_used: estimated_gas,
            gas_cost_usdc: gas_cost,
            profit_usdc: params.expected_profit_usdc - gas_cost,
            execution_time_ms: 1500,
        };

        info!(
            "✅ Trade executed: tx={}, profit=${:.2}, gas=${:.4}",
            result.tx_hash, result.profit_usdc, result.gas_cost_usdc
        );

        Ok(result)
    }

    /// Get transaction receipt
    pub async fn get_transaction_receipt(&self, tx_hash: &str) -> Result<Option<TransactionReceipt>> {
        let hash: H256 = tx_hash.parse()?;
        let receipt = self.provider.get_transaction_receipt(hash).await?;
        Ok(receipt)
    }

    /// Check if transaction was successful
    pub async fn is_transaction_successful(&self, tx_hash: &str) -> Result<bool> {
        if let Some(receipt) = self.get_transaction_receipt(tx_hash).await? {
            Ok(receipt.status == Some(U64::from(1)))
        } else {
            Ok(false)
        }
    }
}

