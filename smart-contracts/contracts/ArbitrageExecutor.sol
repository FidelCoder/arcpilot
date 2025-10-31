// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

/**
 * @title ArbitrageExecutor
 * @notice Executes arbitrage trades on Arc blockchain using USDC
 * @dev Built for Arc + USDC Hackathon
 */
contract ArbitrageExecutor is Ownable, ReentrancyGuard, Pausable {
    IERC20 public immutable usdc;
    
    uint256 public minProfitBps = 10; // 0.1%
    uint256 public constant BPS_DENOMINATOR = 10000;
    
    event ArbitrageExecuted(
        address indexed trader,
        uint256 amountIn,
        uint256 amountOut,
        uint256 profit,
        uint256 timestamp
    );
    
    event ProfitWithdrawn(
        address indexed owner,
        uint256 amount
    );
    
    constructor(address _usdc) {
        usdc = IERC20(_usdc);
    }
    
    /**
     * @notice Execute an arbitrage trade
     * @param amountIn Amount of USDC to trade
     * @param minAmountOut Minimum amount out (slippage protection)
     * @return profit Profit made from arbitrage
     */
    function executeArbitrage(
        uint256 amountIn,
        uint256 minAmountOut
    ) external nonReentrant whenNotPaused returns (uint256 profit) {
        require(amountIn > 0, "Amount must be > 0");
        
        // Transfer USDC from user
        require(
            usdc.transferFrom(msg.sender, address(this), amountIn),
            "Transfer failed"
        );
        
        // Simulate arbitrage execution
        // In production, this would:
        // 1. Execute buy on DEX A
        // 2. Execute sell on DEX B
        // 3. Calculate actual profit
        uint256 amountOut = simulateArbitrage(amountIn);
        
        require(amountOut >= minAmountOut, "Slippage too high");
        require(amountOut > amountIn, "No profit");
        
        profit = amountOut - amountIn;
        
        // Check minimum profit threshold
        uint256 minProfit = (amountIn * minProfitBps) / BPS_DENOMINATOR;
        require(profit >= minProfit, "Profit below threshold");
        
        // Transfer profit back to user
        require(usdc.transfer(msg.sender, amountOut), "Transfer failed");
        
        emit ArbitrageExecuted(
            msg.sender,
            amountIn,
            amountOut,
            profit,
            block.timestamp
        );
    }
    
    /**
     * @notice Simulate arbitrage for demo purposes
     * @dev In production, this would call actual DEX contracts
     */
    function simulateArbitrage(uint256 amountIn) internal pure returns (uint256) {
        // Simulate 1-2% profit
        uint256 profit = (amountIn * 150) / 10000; // 1.5% profit
        return amountIn + profit;
    }
    
    /**
     * @notice Get estimated profit for an amount
     */
    function estimateProfit(uint256 amountIn) external pure returns (uint256) {
        uint256 amountOut = (amountIn * 10150) / 10000; // 1.5% profit
        return amountOut > amountIn ? amountOut - amountIn : 0;
    }
    
    /**
     * @notice Set minimum profit threshold (only owner)
     */
    function setMinProfitBps(uint256 _minProfitBps) external onlyOwner {
        require(_minProfitBps <= 1000, "Max 10%");
        minProfitBps = _minProfitBps;
    }
    
    /**
     * @notice Pause contract (only owner)
     */
    function pause() external onlyOwner {
        _pause();
    }
    
    /**
     * @notice Unpause contract (only owner)
     */
    function unpause() external onlyOwner {
        _unpause();
    }
    
    /**
     * @notice Emergency withdraw (only owner)
     */
    function emergencyWithdraw() external onlyOwner {
        uint256 balance = usdc.balanceOf(address(this));
        require(balance > 0, "No balance");
        require(usdc.transfer(owner(), balance), "Transfer failed");
        emit ProfitWithdrawn(owner(), balance);
    }
}

