# 🚀 Getting Started with ArcPilot

Welcome! This guide will help you get ArcPilot up and running quickly.

## 📋 Prerequisites

Make sure you have these installed:

- **Rust 1.75+**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Node.js 20+**: `nvm install 20` or download from [nodejs.org](https://nodejs.org/)
- **Docker & Docker Compose**: For infrastructure services
- **Git**: For version control

## ⚙️ Initial Setup

### 1. Clone the Repository

```bash
git clone https://github.com/FidelCoder/arcpilot.git
cd arcpilot
```

### 2. Environment Configuration

```bash
# Copy the example environment file
cp env.example .env

# Edit .env with your API keys
nano .env
```

**Required API Keys:**
- `THIRDWEB_CLIENT_ID`: Get from [thirdweb.com](https://thirdweb.com/dashboard)
- `ELEVENLABS_API_KEY`: Get from [elevenlabs.io](https://elevenlabs.io) (use hackathon coupon!)
- `ARC_RPC_URL`: Arc testnet RPC endpoint
- `USDC_CONTRACT_ADDRESS`: USDC token address on Arc

### 3. Start Infrastructure

```bash
# Start PostgreSQL, Redis, Kafka
docker-compose up -d
```

Wait ~10 seconds for services to initialize.

## 🏃 Quick Start (Development)

### Option A: Use Makefile (Recommended)

```bash
# Install all dependencies
make install

# Start all services
make dev
```

### Option B: Manual Start

**Terminal 1 - AI Engine:**
```bash
cd ai-engine
cargo run
```

**Terminal 2 - Blockchain Executor:**
```bash
cd blockchain-executor
cargo run
```

**Terminal 3 - API Gateway:**
```bash
cd api-gateway
cargo run
```

**Terminal 4 - Frontend:**
```bash
cd frontend
npm install
npm run dev
```

## 🌐 Access the Application

- **Frontend**: http://localhost:3000
- **API Gateway**: http://localhost:8080
- **AI Engine**: http://localhost:8081
- **Blockchain Executor**: http://localhost:8082

## 🪙 Get Testnet USDC

1. Visit [Arc Testnet Faucet](https://faucet.circle.com/arc)
2. Connect your wallet via Thirdweb
3. Request testnet USDC
4. Start trading!

## 📜 Deploy Smart Contracts

```bash
cd smart-contracts
npm install
npm run deploy
```

Copy the deployed contract address to your `.env` file.

## 🧪 Run Tests

```bash
# All tests
make test

# Rust tests only
cargo test --workspace

# Frontend tests
cd frontend && npm test
```

## 🐳 Docker Deployment

```bash
# Build and start all services
docker-compose --profile full up -d

# View logs
docker-compose logs -f

# Stop all services
docker-compose down
```

## 🎤 ElevenLabs Voice Integration

To enable voice features:

1. Sign up at [elevenlabs.io](https://elevenlabs.io)
2. Use hackathon coupon code (check hackathon page)
3. Get your API key from dashboard
4. Add to `.env`:
```bash
ELEVENLABS_API_KEY=your_key_here
ELEVENLABS_VOICE_ID=your_voice_id
```

5. Restart frontend:
```bash
cd frontend && npm run dev
```

## 🔧 Troubleshooting

### Rust compilation errors
```bash
cargo clean
cargo build
```

### Port already in use
```bash
# Check what's using the port
lsof -i :8080
# Kill the process
kill -9 <PID>
```

### Database connection errors
```bash
# Restart PostgreSQL
docker-compose restart postgres
```

### Frontend won't start
```bash
cd frontend
rm -rf node_modules .next
npm install
npm run dev
```

## 📚 Next Steps

1. **Read the docs**: Check out [README.md](./README.md) for architecture overview
2. **Explore cursor rules**: See [.cursorrules](./.cursorrules) for development guidelines
3. **Check API docs**: Visit http://localhost:8080/health to test API
4. **Join Discord**: Get support from the community

## 🏆 Hackathon Checklist

- [ ] Get Arc testnet USDC
- [ ] Deploy smart contracts to Arc testnet
- [ ] Configure Thirdweb wallet
- [ ] Test voice commands with ElevenLabs
- [ ] Record demo video
- [ ] Prepare pitch deck
- [ ] Submit to lablab.ai

## 💡 Tips

- **Start simple**: Get one service running first, then add others
- **Check logs**: Use `docker-compose logs -f` to debug issues
- **USDC gas**: Arc uses USDC for gas - your gas fees are in dollars!
- **Voice commands**: Speak clearly and naturally to your AI co-pilot

## 🤝 Need Help?

- 📖 Documentation: [README.md](./README.md)
- 💬 Issues: [GitHub Issues](https://github.com/FidelCoder/arcpilot/issues)
- 🎮 Discord: Join the hackathon Discord
- 📧 Contact: [@FidelCoder](https://github.com/FidelCoder)

---

**Happy Building! 🚀**

