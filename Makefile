.PHONY: help install dev build test clean docker-up docker-down migrate fmt lint

# Default target
help:
	@echo "ArcPilot - Available Commands:"
	@echo ""
	@echo "  make install       - Install all dependencies"
	@echo "  make dev           - Start development environment"
	@echo "  make build         - Build all services"
	@echo "  make test          - Run all tests"
	@echo "  make docker-up     - Start Docker services"
	@echo "  make docker-down   - Stop Docker services"
	@echo "  make migrate       - Run database migrations"
	@echo "  make fmt           - Format code"
	@echo "  make lint          - Lint code"
	@echo "  make clean         - Clean build artifacts"
	@echo ""

# Install dependencies
install:
	@echo "📦 Installing Rust dependencies..."
	cargo fetch
	@echo "📦 Installing frontend dependencies..."
	cd frontend && npm install
	@echo "📦 Installing Python dependencies..."
	cd python-ml && pip install -r requirements.txt
	@echo "✅ All dependencies installed!"

# Start development environment
dev:
	@echo "🚀 Starting development environment..."
	docker-compose up -d postgres redis
	@echo "⏳ Waiting for services to be ready..."
	sleep 5
	@echo "🏃 Starting Rust services..."
	cargo run --bin api-gateway &
	cargo run --bin ai-engine &
	cargo run --bin blockchain-executor &
	@echo "🎨 Starting frontend..."
	cd frontend && npm run dev

# Build all services
build:
	@echo "🔨 Building Rust services..."
	cargo build --release
	@echo "🔨 Building frontend..."
	cd frontend && npm run build
	@echo "✅ Build complete!"

# Run all tests
test:
	@echo "🧪 Running Rust tests..."
	cargo test --workspace
	@echo "🧪 Running frontend tests..."
	cd frontend && npm test
	@echo "🧪 Running Python tests..."
	cd python-ml && pytest
	@echo "✅ All tests passed!"

# Start Docker services
docker-up:
	@echo "🐳 Starting Docker services..."
	docker-compose up -d

# Start all services including apps
docker-up-full:
	@echo "🐳 Starting all services..."
	docker-compose --profile full up -d

# Stop Docker services
docker-down:
	@echo "🛑 Stopping Docker services..."
	docker-compose down

# Run database migrations
migrate:
	@echo "📊 Running database migrations..."
	sqlx migrate run --source ./infrastructure/sql/migrations

# Format code
fmt:
	@echo "✨ Formatting Rust code..."
	cargo fmt --all
	@echo "✨ Formatting frontend code..."
	cd frontend && npm run format
	@echo "✨ Formatting Python code..."
	cd python-ml && black .
	@echo "✅ Code formatted!"

# Lint code
lint:
	@echo "🔍 Linting Rust code..."
	cargo clippy --all-targets --all-features -- -D warnings
	@echo "🔍 Linting frontend code..."
	cd frontend && npm run lint
	@echo "🔍 Linting Python code..."
	cd python-ml && ruff check .
	@echo "✅ Linting complete!"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	cd frontend && rm -rf .next node_modules
	find . -type d -name "__pycache__" -exec rm -rf {} +
	find . -type f -name "*.pyc" -delete
	@echo "✅ Clean complete!"

# Deploy to testnet
deploy-contracts:
	@echo "🚀 Deploying smart contracts to Arc testnet..."
	cd smart-contracts && npx hardhat run scripts/deploy.js --network arc-testnet
	@echo "✅ Contracts deployed!"

# Generate documentation
docs:
	@echo "📚 Generating documentation..."
	cargo doc --no-deps --open
	@echo "✅ Documentation generated!"

# Check code health
check:
	@echo "🔍 Checking code health..."
	cargo check --all-targets
	cargo clippy --all-targets -- -D warnings
	cargo fmt --all -- --check
	@echo "✅ Code health check complete!"

