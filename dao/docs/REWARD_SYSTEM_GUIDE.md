# DAO Reward System: Bitcoin-Style Implementation Guide

This guide explains the Bitcoin-style reward system implementation for Anya Core's DAO.

## Overview

The DAO reward system follows Anya Core's Bitcoin-inspired tokenomics with the following features:

1. **Bitcoin-Style Supply**: 21 billion token supply with halving mechanism
2. **Strategic Distribution**:
   - 15% allocated for community incentives (developer rewards)
   - Rewards calculated proportionally based on contribution points
3. **Block-Based Issuance**: Rewards follow Bitcoin-style halving schedule
   - Initial block reward: 10,000 AGT tokens
   - Halvings every 210,000 blocks
4. **Transparent Distribution**: Full audit trail of all rewards
5. **Security Controls**: Configurable mainnet checkpoints

## Files and Tools

- **dao-reward-engine.js**: Main reward calculation and distribution engine
- **run-dao-rewards.sh**: CLI interface for reward processing
- **contribution-tracker.js**: Tracks contributor activity from GitHub
- **contribution_history.json**: Historical contribution data
- **reward_distribution.json**: Record of calculated rewards

## How It Works

1. **Contribution Tracking**: The `contribution-tracker.js` script collects GitHub activity and assigns points
2. **Reward Calculation**: The `dao-reward-engine.js` calculates rewards using:
   - Current block number
   - Bitcoin-style halving schedule
   - 15% community allocation from total supply
   - Proportional distribution based on contribution points
3. **Distribution**: Tokens are distributed to contributors based on their points
4. **Validation**: Double-payment prevention and security checks

## Block Reward System

The reward calculation follows Bitcoin's model:

- **Initial Block Reward**: 10,000 AGT per block
- **Halving Schedule**:
  - Blocks 0-105,000: 10,000 AGT per block
  - Blocks 105,001-210,000: 5,000 AGT per block
  - Blocks 210,001-315,000: 2,500 AGT per block
  - Blocks 420,001-630,000: 2,500 AGT per block
  - And so on...

## Usage

### Basic Commands

```bash
# Calculate rewards and simulate distribution
./run-dao-rewards.sh --simulate

# Calculate rewards only (no distribution)
./run-dao-rewards.sh --audit-only

# Set specific block height for calculations
./run-dao-rewards.sh --block=105000

# Production mode - execute actual transfers
./run-dao-rewards.sh --mainnet
```

### Options

- `--simulate`: Run in simulation mode without actual transfers (default)
- `--audit-only`: Calculate rewards without distributing tokens
- `--force`: Override safety checks (use with caution)
- `--block=NUMBER`: Set the current block height for calculations
- `--mainnet`: Execute actual token transfers

## Production Deployment

For production deployment:

1. Configure proper blockchain connection (update the engine to use ethers.js/web3.js)
2. Setup DAO contract integration for on-chain transfers
3. Update security controls for mainnet interactions
4. Configure monitoring and alerts for failed transactions

## Safety Controls

The reward engine implements several safety measures:

1. **Double-Payment Prevention**: Tracks rewarded periods to prevent duplicates
2. **Mainnet Verification**: Verifies blockchain connection before executing transfers
3. **Contract Verification**: Ensures the DAO contract is operational
4. **Audit Mode**: Allows review of reward calculations without distribution
5. **Simulation Mode**: Tests distribution logic without executing transfers

## Integration with Anya Core

This implementation aligns with Anya Core's tokenomics by:

1. Following the 21 billion token supply cap
2. Implementing the Bitcoin-style halving mechanism
3. Respecting the 15% community allocation
4. Supporting transparent and auditable reward distribution

## Troubleshooting

- **Missing contribution data**: Run `track-contributions.sh` to generate history
- **Connection errors**: Check blockchain connection or use `--simulate` mode
- **Already rewarded periods**: Use `--force` to override (with caution)
