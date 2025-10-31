import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import * as dotenv from "dotenv";

dotenv.config({ path: "../env.example" });

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.20",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200,
      },
    },
  },
  networks: {
    "arc-testnet": {
      url: process.env.ARC_RPC_URL || "https://testnet-rpc.arcblockchain.com",
      chainId: parseInt(process.env.ARC_CHAIN_ID || "12345"),
      accounts: process.env.TEST_WALLET_PRIVATE_KEY ? [process.env.TEST_WALLET_PRIVATE_KEY] : [],
    },
    localhost: {
      url: "http://127.0.0.1:8545",
    },
  },
  paths: {
    sources: "./contracts",
    tests: "./test",
    cache: "./cache",
    artifacts: "./artifacts",
  },
};

export default config;

