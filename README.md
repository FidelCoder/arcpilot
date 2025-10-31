# 🚀 ArcPilot

**AI-powered voice agent for autonomous USDC arbitrage and DeFi trading on Arc blockchain**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Built for Arc Hackathon](https://img.shields.io/badge/Built%20for-Arc%20Hackathon-blue)](https://lablab.ai/event/ai-agents-on-arc-with-usdc)

> *"Talk to your AI co-pilot to trade, arbitrage, and earn with USDC on Arc - no complex commands, just conversation"*

---

## 🎯 What is ArcPilot?

ArcPilot is an intelligent trading agent that combines:
- 🗣️ **Voice AI** (ElevenLabs) - Natural conversation, no complex UI
- 🤖 **AI Decision Engine** - ML-powered arbitrage detection
- ⛓️ **Arc Blockchain** - USDC-native gas fees, instant settlement
- 🔐 **Smart Wallets** (Thirdweb) - One-click onboarding, no seed phrases
- 🌉 **Cross-Chain** (CCTP) - Move USDC seamlessly across chains

## ✨ Key Features

### 1. Voice-First Trading
```
You: "Hey ArcPilot, find me safe arbitrage opportunities"
AI: "I found 3 opportunities. The safest one is 1.2% profit on USDC/ETH with 98% confidence"
You: "Execute it with $100"
AI: "Done! You earned $1.18 profit, gas cost was $0.03 USDC"
```

### 2. AI-Powered Opportunity Detection
- Real-time scanning of DEX pools on Arc
- ML-based risk scoring (0-100)
- Profitability calculation after gas costs
- Historical learning from past trades

### 3. Arc-Native Integration
- All transactions settle on Arc testnet
- USDC used for gas fees (predictable dollar costs)
- Sub-second transaction finality
- Dollar-denominated everything

### 4. Smart Wallet Onboarding
- Email/social login (no wallet needed)
- Gasless transactions with session keys
- Built on Thirdweb infrastructure
- Non-custodial security

### 5. Cross-Chain USDC Arbitrage
- Detect opportunities across chains (Ethereum, Polygon, Base, Arc)
- Use Circle's CCTP for instant USDC transfers
- AI selects optimal chain for execution

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Frontend (Next.js)                      │
│                  Thirdweb + ElevenLabs Voice                 │
└───────────────────────────┬─────────────────────────────────┘
                            │
                    ┌───────▼────────┐
                    │  API Gateway   │
                    │     (Rust)     │
                    └───┬────────┬───┘
                        │        │
            ┌───────────▼─┐  ┌──▼──────────────┐
            │  AI Engine  │  │  Blockchain     │
            │   (Rust)    │  │  Executor       │
            │             │  │   (Rust)        │
            └─────────────┘  └────────┬────────┘
                 │                    │
            ┌────▼─────┐         ┌────▼──────────┐
            │  Python  │         │  Arc Testnet  │
            │ ML Models│         │  + USDC       │
            └──────────┘         └───────────────┘
```

## 🚀 Quick Start

### Prerequisites
- Rust 1.75+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Node.js 20+ (`nvm install 20`)
- Python 3.11+ (`pyenv install 3.11`)
- Docker & Docker Compose
- Git

### Installation

```bash
# Clone the repository
git clone https://github.com/FidelCoder/arcpilot.git
cd arcpilot

# Copy environment variables
cp .env.example .env
# Edit .env with your API keys

# Install dependencies
make install

# Start infrastructure (PostgreSQL, Redis)
docker-compose up -d

# Run database migrations
make migrate

# Start all services
make dev
```

The application will be available at:
- Frontend: http://localhost:3000
- API Gateway: http://localhost:8080
- AI Engine: http://localhost:8081

### Get Testnet USDC
1. Visit [Arc Testnet Faucet](https://faucet.circle.com/arc)
2. Connect your wallet
3. Request testnet USDC

## 📋 Environment Setup

Create a `.env` file with the following:

```bash
# Arc Blockchain
ARC_RPC_URL=https://testnet-rpc.arcblockchain.com
ARC_CHAIN_ID=12345
USDC_CONTRACT_ADDRESS=0x...

# Thirdweb
THIRDWEB_CLIENT_ID=your_client_id
THIRDWEB_SECRET_KEY=your_secret_key

# ElevenLabs
ELEVENLABS_API_KEY=your_api_key
ELEVENLABS_VOICE_ID=your_voice_id

# Circle CCTP
CCTP_CONTRACT_ADDRESS=0x...

# Database
DATABASE_URL=postgresql://arcpilot:arcpilot@localhost:5432/arcpilot

# API
API_PORT=8080
JWT_SECRET=your_jwt_secret
```

## 🛠️ Development

### Project Structure
```
arcpilot/
├── ai-engine/              # Rust: AI decision engine
├── voice-agent/            # TypeScript: ElevenLabs integration
├── blockchain-executor/    # Rust: Arc blockchain operations
├── api-gateway/            # Rust: API layer
├── frontend/               # Next.js + Thirdweb
├── smart-contracts/        # Solidity contracts
├── python-ml/              # Python ML models
└── docs/                   # Documentation
```

### Run Individual Services

```bash
# AI Engine
cd ai-engine
cargo run

# Blockchain Executor
cd blockchain-executor
cargo run

# API Gateway
cd api-gateway
cargo run

# Frontend
cd frontend
npm run dev

# Python ML
cd python-ml
python -m uvicorn main:app --reload
```

### Run Tests

```bash
# Rust tests
cargo test --workspace

# Frontend tests
cd frontend && npm test

# Python tests
cd python-ml && pytest

# E2E tests
npm run test:e2e
```

## 🎮 Usage

### Voice Commands

ArcPilot supports natural language commands:

- **Find opportunities**: "Show me arbitrage opportunities" / "Find profitable trades"
- **Execute trades**: "Execute that trade" / "Trade $100 USDC on that opportunity"
- **Check balance**: "What's my balance?" / "How much USDC do I have?"
- **View history**: "Show my trade history" / "What did I earn today?"
- **Risk settings**: "Set risk level to conservative" / "Only show low-risk trades"

### Dashboard

Use the web dashboard at `http://localhost:3000` to:
- Monitor live arbitrage opportunities
- View AI risk scores and profit predictions
- Track portfolio performance
- Manage risk settings
- Review trade history

## 🔐 Security

- ✅ Non-custodial: You control your funds via Thirdweb smart wallets
- ✅ No private keys: Email/social login with MPC key management
- ✅ Risk limits: Set daily loss limits and max trade sizes
- ✅ Circuit breakers: Auto-pause on unusual activity
- ✅ Open source: Auditable smart contracts

## 📊 Hackathon Tracks

ArcPilot competes in:

1. **On-chain Actions** ✅
   - Autonomous AI agents interacting with DeFi protocols
   - Arbitrage, swaps, liquidity rebalancing

2. **Best use of Arc** ✅
   - USDC-native gas fees
   - Sub-second settlement
   - Dollar-denominated costs

3. **Best use of ElevenLabs** ✅
   - Voice-first trading interface
   - Conversational AI agent

## 🏆 Innovation Highlights

1. **Voice-First DeFi** - First voice-controlled trading agent on Arc
2. **AI Learning** - Models improve from historical trade data
3. **Predictable Costs** - USDC gas means no ETH volatility
4. **Zero Friction** - Email login, no seed phrases, just conversation
5. **Cross-Chain Intelligence** - AI picks optimal chain for each trade

## 📚 Documentation

- [Architecture Overview](./docs/ARCHITECTURE.md)
- [API Reference](./docs/API.md)
- [Smart Contracts](./docs/CONTRACTS.md)
- [Deployment Guide](./docs/DEPLOYMENT.md)
- [Voice Commands](./docs/VOICE_COMMANDS.md)

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## 📄 License

MIT License - see [LICENSE](./LICENSE) file for details.

## 🙏 Acknowledgments

Built for the **AI Agents on Arc with USDC** hackathon powered by:
- [Circle](https://www.circle.com/) - Arc blockchain & USDC
- [ElevenLabs](https://elevenlabs.io/) - Voice AI
- [Thirdweb](https://thirdweb.com/) - Smart wallet infrastructure
- [lablab.ai](https://lablab.ai/) - Hackathon platform

## 🔗 Links

- 🌐 [Live Demo](https://arcpilot-demo.vercel.app)
- 📺 [Video Demo](https://youtube.com/watch?v=...)
- 📊 [Pitch Deck](./docs/pitch-deck.pdf)
- 💬 [Discord](https://discord.gg/...)
- 🐦 [Twitter](https://twitter.com/arcpilot)

---

**Built with ❤️ by a solo hacker for the Arc + USDC Hackathon**

*Questions? Reach out: [@FidelCoder](https://github.com/FidelCoder)*
