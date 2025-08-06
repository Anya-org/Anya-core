# On-Chain DAO Reward System Guide

This document provides a comprehensive overview of the fully automated, decentralized, on-chain reward system implemented for the Anya-core DAO.

## System Architecture

The on-chain reward system is built with a modular, secure, and scalable architecture:

```
┌─────────────────────────────────────────────────────────────┐
│                     Off-Chain Components                    │
├───────────────────────┬─────────────────┬──────────────────┤
│  Contribution Tracker │  Reward Bridge  │  System Manager  │
└─────────────┬─────────┴────────┬────────┴────────┬─────────┘
              │                  │                 │
              ▼                  ▼                 ▼
┌─────────────────────────────────────────────────────────────┐
│                      On-Chain Contracts                     │
├───────────────┬──────────────┬────────────┬────────────────┤
│ Contribution  │   Reward     │  Reward    │    Reward      │
│    Oracle     │ Controller   │Distributor │   Scheduler    │
└───────┬───────┴──────┬───────┴─────┬──────┴────────┬───────┘
        │              │             │               │
        └──────────────┼─────────────┼───────────────┘
                       │             │
                       ▼             ▼
┌─────────────────────────────────────────────────────────────┐
│                     Token Contracts                         │
├───────────────────────┬─────────────────────────────────────┤
│     Token Contract    │        Tokenomics Contract          │
└───────────────────────┴─────────────────────────────────────┘
```

## Key Components

### Off-Chain Components

#### 1. Contribution Tracker (`contribution-tracker.js`)

- Tracks contributions from GitHub and other sources
- Assigns contribution points based on activity
- Generates detailed contribution history

#### 2. Reward Bridge (`on-chain-reward-bridge.js`)

- Securely bridges off-chain contribution data to on-chain contracts
- Handles batch processing for gas efficiency
- Provides transaction monitoring and confirmation

#### 3. System Manager (`reward-system-manager.js`)

- Orchestrates the entire reward lifecycle
- Manages period scheduling and tracking
- Provides automation and monitoring capabilities

### On-Chain Contracts

#### 1. Contribution Oracle (`contribution-oracle.clar`)

- Receives contribution data from trusted sources
- Securely stores contribution points on-chain
- Provides verification and access controls

#### 2. Reward Controller (`reward-controller.clar`)

- Implements Bitcoin-style tokenomics (21B supply with halving)
- Calculates rewards based on contribution points
- Manages reward periods and prevents double payments

#### 3. Reward Distributor (`reward-distributor.clar`)

- Handles the distribution of tokens to contributors
- Provides claim mechanism for self-service rewards
- Maintains audit trail of all distributions

#### 4. Reward Scheduler (`reward-scheduler.clar`)

- Manages automated scheduling of reward distributions
- Integrates with blockchain events for timing
- Provides governance mechanisms for parameter updates

## Tokenomics

The reward system implements a Bitcoin-inspired token model:

- **Total Supply**: 21 billion tokens (with 8 decimals precision)
- **Halving Mechanism**: Rewards halve every 105,000 blocks (adaptive minimum)
- **Initial Block Reward**: 10,000 tokens per block
- **Community Allocation**: 15% of total supply dedicated to community incentives

## Reward Flow

1. **Contribution Tracking**
   - Off-chain GitHub activities are tracked and assigned points
   - Contributions are recorded with timestamps and metadata

2. **On-Chain Submission**
   - Contribution data is formatted and batched
   - Data is submitted to the Contribution Oracle contract
   - Oracle authenticates the source and stores the data

3. **Reward Calculation**
   - Reward Controller calculates rewards based on points
   - Bitcoin-style algorithm determines total available rewards
   - Points are converted to token amounts

4. **Reward Distribution**
   - Tokens are distributed according to calculated amounts
   - Contributors can claim rewards directly
   - All distributions are recorded for transparency

5. **Period Management**
   - Reward periods are tracked to prevent double payments
   - Automated scheduling ensures regular distributions
   - Governance mechanisms allow parameter adjustments

## Setup and Usage

### Initial Setup

1. Deploy all contracts to the blockchain:

   ```
   contribution-oracle.clar
   reward-controller.clar
   reward-distributor.clar
   reward-scheduler.clar
   ```

2. Configure the connection between off-chain tools and on-chain contracts:

   ```
   dao/config/bridge_config.json
   dao/config/reward_system_config.json
   ```

### Automated Operation

The system can run fully automated with the following components:

1. **Cron Jobs**:
   - Set up the cron script to run automatically: `cron-reward-system.sh`
   - Recommended schedule: contribution tracking on the 1st, distribution on the 5th

2. **Manual Triggers**:
   - Run the reward system manager script: `reward-system-manager.js`
   - Options for dry-run, simulation, and manual period selection

### Security Features

The system includes multiple security layers:

1. **Access Control**:
   - Only authorized addresses can submit contribution data
   - Only contract owner can modify critical parameters

2. **Transaction Safety**:
   - Simulation mode for testing without mainnet transactions
   - Required confirmations for transaction finality
   - Timelock for sensitive operations

3. **Audit Trail**:
   - Comprehensive logging of all operations
   - On-chain storage of all reward distributions
   - Transparent verification of reward calculations

## Governance Integration

The reward system integrates with DAO governance:

1. **Parameter Updates**:
   - Governance can vote to update reward parameters
   - Changes implemented through on-chain voting

2. **Transparency**:
   - All reward calculations and distributions are publicly visible
   - Contributors can verify their rewards on-chain

3. **Decentralization**:
   - No central authority controls the reward distribution
   - All processes follow predetermined rules encoded in contracts

## Monitoring and Maintenance

To monitor and maintain the system:

1. **Logs**:
   - Check log files in `dao/logs/` for system activity
   - Transaction logs provide detailed audit trail

2. **Status Checks**:
   - Run `reward-system-manager.js --check-status` to verify system health
   - Monitor blockchain events for contract activity

3. **Updates**:
   - Contract upgrades must follow governance process
   - Backward compatibility maintained for historical rewards

## Conclusion

This fully automated, decentralized, on-chain reward system provides a transparent, secure, and efficient mechanism for distributing rewards based on contributions. By leveraging blockchain technology, it eliminates central points of control and creates a trustless environment for DAO operations.

The system's modular architecture allows for future expansions and improvements while maintaining the core Bitcoin-style tokenomics that are central to the Anya-core vision.
