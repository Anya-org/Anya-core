# Cross-Chain Bridge Fee Structure Guide

## Overview

This document outlines the standardized fee structure implemented for all cross-chain bridge operations in the Anya-core DAO reward system.

## Standardized Fee Structure

Based on DAO governance decisions, a uniform fee structure has been established across all bridge operations:

| Bridge Direction | Fee Rate | Minimum Amount | Confirmations |
|-----------------|----------|----------------|---------------|
| Stacks -> Bitcoin | 5% | 1000 tokens | 6 confirmations |
| Stacks -> Ethereum | 5% | 500 tokens | 12 confirmations |
| Bitcoin -> Stacks | 5% | 1000 tokens | 3 confirmations |
| Ethereum -> Stacks | 5% | 500 tokens | 12 confirmations |

### Fee Distribution

All collected fees are distributed as follows:

- **80%** to DAO Treasury
- **20%** to Community Incentives

## Implementation Details

### On-Chain Implementation

The bridge fee structure is implemented in the following smart contracts:

1. `reward-scheduler.clar`: Manages fee collection and handles distribution to treasury and community contracts
2. `cross-chain-bridge.clar`: Manages the actual bridge operations and confirmation requirements

Key parameters in `reward-scheduler.clar`:

```clarity
;; Cross-chain bridge fee configuration
;; Using standardized 5% fee across all bridges for simplicity and profitability
(define-data-var bridge-fee-rate uint u50000) ;; 5% (denominator is 1,000,000)
(define-data-var bridge-fee-treasury-share uint u800000) ;; 80% of fees to treasury
(define-data-var bridge-fee-community-share uint u200000) ;; 20% of fees to community incentives
```

### Off-Chain Implementation

The JavaScript integration utilities maintain the same fee structure:

1. `blockchain-integrations.js`: Provides utilities for calculating and applying fees across all token standards
2. `bridge_config.json`: Configures bridge parameters and fee rates

Example fee calculation:

```javascript
function calculateBridgeFee(amount, bridgeType, networkType = 'testnet') {
    // Load bridge config
    const bridgeConfig = loadBridgeConfig()[networkType];
    const bridgeSettings = bridgeConfig.bridgeSettings;
    const feeDistribution = bridgeConfig.feeDistribution;
    
    // Get specific bridge settings
    const bridgeTypeSettings = bridgeSettings[bridgeType];
    
    // Calculate fees (standardized at 5% for all bridges)
    const feeRate = bridgeTypeSettings.feeRate; // Should be 0.05
    const feeAmount = amount * feeRate;
    const netAmount = amount - feeAmount;
    
    // Calculate fee distribution (80% treasury, 20% community)
    const treasuryFee = feeAmount * feeDistribution.treasuryShare;
    const communityFee = feeAmount * feeDistribution.communityShare;
    
    return {
        originalAmount: amount,
        feeAmount: feeAmount,
        netAmount: netAmount,
        treasuryFee: treasuryFee,
        communityFee: communityFee,
        feeRate: feeRate,
        treasuryShare: feeDistribution.treasuryShare,
        communityShare: feeDistribution.communityShare
    };
}
```

## Fee Management Principles

1. **Uniform Rate**: A single 5% fee applies to all bridge operations, regardless of direction or token standard.

2. **Batch Optimization**: The DAO manages differences in network fees by keeping any operational surplus, which can be used for:
   - Reducing batch transaction costs
   - Covering gas price fluctuations
   - Managing network congestion periods

3. **DAO Treasury Allocation**: 80% of all bridge fees go directly to the DAO treasury, providing sustainable funding for operations and growth.

4. **Community Rewards**: 20% of all bridge fees are allocated to community incentives, ensuring alignment between DAO growth and community participation.

5. **Fee Transparency**: All fee operations are logged and traceable for full transparency and auditing.

## Auditing and Reporting

Bridge fee operations are logged to `/dao/logs/fee_operations.log` with the following information:

- Timestamp
- Operation type
- Source and destination standards
- Original and net amounts
- Fee breakdown (treasury and community portions)
- Bridge type and transaction ID

This enables full transparency and facilitates regular reporting to DAO governance.

## Conclusion

This standardized bridge fee structure ensures that the Anya-core DAO can sustainably operate and maintain cross-chain compatibility while providing clear value to token holders and the community. The uniform 5% fee with 80/20 treasury/community split represents a balanced approach that supports both operational requirements and continued community growth.
