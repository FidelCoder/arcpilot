-- ArcPilot Database Initialization

-- Create tables for ArcPilot

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    wallet_address VARCHAR(42) UNIQUE NOT NULL,
    email VARCHAR(255),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Trades table
CREATE TABLE IF NOT EXISTS trades (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    opportunity_id VARCHAR(255),
    pair VARCHAR(50),
    amount_usdc DECIMAL(18, 6),
    profit_usdc DECIMAL(18, 6),
    gas_cost_usdc DECIMAL(18, 6),
    tx_hash VARCHAR(66),
    status VARCHAR(50),
    executed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Opportunities table (for analytics)
CREATE TABLE IF NOT EXISTS opportunities (
    id SERIAL PRIMARY KEY,
    opportunity_id VARCHAR(255) UNIQUE,
    pair VARCHAR(50),
    buy_exchange VARCHAR(100),
    sell_exchange VARCHAR(100),
    profit_margin DECIMAL(10, 6),
    risk_score DECIMAL(5, 4),
    ai_score DECIMAL(5, 4),
    detected_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_users_wallet ON users(wallet_address);
CREATE INDEX IF NOT EXISTS idx_trades_user ON trades(user_id);
CREATE INDEX IF NOT EXISTS idx_trades_executed ON trades(executed_at);
CREATE INDEX IF NOT EXISTS idx_opportunities_detected ON opportunities(detected_at);

-- Initial data (optional)
INSERT INTO users (wallet_address, email) VALUES 
('0x0000000000000000000000000000000000000000', 'demo@arcpilot.io')
ON CONFLICT (wallet_address) DO NOTHING;

