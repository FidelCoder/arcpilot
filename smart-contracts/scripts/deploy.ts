import { ethers } from "hardhat";

async function main() {
  console.log("🚀 Deploying ArbitrageExecutor to Arc...");

  // Get USDC address from environment
  const usdcAddress = process.env.USDC_CONTRACT_ADDRESS || "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238";
  
  console.log(`📍 USDC Address: ${usdcAddress}`);

  // Deploy ArbitrageExecutor
  const ArbitrageExecutor = await ethers.getContractFactory("ArbitrageExecutor");
  const arbitrageExecutor = await ArbitrageExecutor.deploy(usdcAddress);

  await arbitrageExecutor.waitForDeployment();

  const address = await arbitrageExecutor.getAddress();

  console.log(`✅ ArbitrageExecutor deployed to: ${address}`);
  console.log(`🔗 View on explorer: ${process.env.ARC_EXPLORER_URL}/address/${address}`);
  
  // Verify configuration
  const minProfitBps = await arbitrageExecutor.minProfitBps();
  console.log(`💰 Min profit threshold: ${minProfitBps / 100}%`);

  console.log("\n📝 Add this to your .env file:");
  console.log(`ARBITRAGE_EXECUTOR_ADDRESS=${address}`);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });

