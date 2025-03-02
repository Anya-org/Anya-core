# Anya Tokenomics System Architecture

This document outlines the tokenomics architecture of the Anya DAO system, including the hybrid Bitcoin-style issuance model, DEX integration, and related components.

## System Overview

The Anya tokenomics system implements a hybrid token issuance model that combines:

1. An initial 6-month distribution phase releasing 45% of the total supply
2. A Bitcoin-like halving schedule after the initial period

This model provides a balanced approach that ensures sufficient initial token distribution for governance while maintaining the long-term supply controls inspired by Bitcoin's monetary policy.

## Component Architecture

```
anya-core/
├── src/contracts/
│   ├── governance_token.clar   # SIP-010 compliant governance token
│   ├── dao.clar                # Main DAO contract
│   ├── bitcoin-issuance.clar   # Bitcoin-style token issuance logic
│   ├── dex-adapter.clar        # DEX integration for liquidity and trading
│   └── token-economics.clar    # Token economic model implementation
├── dao/
│   ├── traits/
│   │   ├── dao-trait.clar           # Interface for DAO functionality
│   │   └── dex-integration-trait.clar # Interface for DEX integration
│   ├── core/
│   │   └── dao-core.clar            # Enhanced DAO core implementation
│   └── extensions/
│       └── token-economics.clar     # Advanced token economics logic
```

## Issuance Model

### 1. Initial Distribution Phase (6 Months)

- **Duration**: ~240,000 blocks (assuming 10-minute block times)
- **Allocation**: 45% of the total supply (9.45 million AGT)
- **Distribution**: Linear release over the 6-month period
- **Purpose**: Ensure sufficient tokens for early governance and ecosystem bootstrapping

### 2. Bitcoin-Style Phase (Post-Initial)

- **Starting Block Reward**: 50 AGT per block
- **Halving Interval**: Every 210,000 blocks (~4 years)
- **Supply Cap**: 21 million AGT (matching Bitcoin's supply)
- **Halving Schedule**:
  - First 210,000 blocks post-initial: 50 AGT per block
  - Next 210,000 blocks: 25 AGT per block
  - Next 210,000 blocks: 12.5 AGT per block
  - And so on...

## DEX Integration

The DEX adapter provides liquidity management and trading capabilities for the AGT token, including:

### Key Features

1. **Liquidity Provision**
   - Users can provide STX/AGT liquidity to earn trading fees
   - Liquidity providers receive LP tokens representing their share

2. **Trading Operations**
   - Swap AGT for STX and vice versa
   - Constant product market maker formula (x * y = k)
   - Fee percentage: 0.3% by default (configurable)

3. **Buyback Mechanism**
   - DAO can execute buybacks through the DEX
   - Supports DAO-controlled market stabilization

4. **Price Oracle**
   - Provides reliable on-chain price information
   - Useful for other contracts needing AGT price data

## Contract Integration Flow

```
                       ┌───────────────┐
                       │  DAO Contract │
                       └───────┬───────┘
                               │
                      Control & │
                      Governance│
                               ▼
┌────────────────┐    ┌───────────────┐    ┌───────────────┐
│  Governance    │◄─►│   DAO Core    │◄─►│  DEX Adapter   │
│    Token       │    └───────┬───────┘    └───────────────┘
└────────────────┘            │                    ▲
         ▲                    │                    │
         │                    │                    │
         │                    │                    │
         │                    ▼                    │
         │            ┌───────────────┐            │
         └────────────┤Bitcoin Issuance├────────────┘
                      └───────────────┘
```

## Key Contract Relationships

1. **DAO & DAO Core**
   - The DAO contract uses the DAO core for implementation
   - Both implement the `dao-trait` interface for consistency

2. **Issuance & Token**
   - The Bitcoin issuance contract mints tokens according to the schedule
   - It directs new tokens to the DAO treasury or other designated recipients

3. **DEX & Token**
   - The DEX adapter manages trading between STX and AGT
   - It implements the `dex-integration-trait` interface

4. **DAO & DEX Integration**
   - The DAO can issue instructions to the DEX for buybacks
   - The DAO can adjust liquidity parameters and fee settings

## Governance Controls

The DAO governance system has control over several tokenomics parameters:

1. **Issuance Controls**
   - Initialization of the issuance schedule
   - Distribution of newly issued tokens

2. **DEX Controls**
   - Fee percentage adjustment
   - Liquidity provision/removal
   - Buyback execution

3. **Treasury Management**
   - Token allocation for proposals
   - Strategic reserve management

## Security Considerations

1. **Administrator Controls**
   - All contracts have administrator checks for sensitive operations
   - The DAO itself can be an administrator of the issuance and DEX contracts

2. **Immutable Schedule**
   - The core issuance schedule (halving intervals) is hardcoded and cannot be changed
   - This provides certainty about the token's monetary policy

3. **Contract Boundaries**
   - Clear separation of concerns between contracts
   - Well-defined interfaces (traits) for component interaction

## Deployment and Initialization

To deploy and initialize the system:

1. Deploy all contracts in Clarinet.toml
2. Initialize the governance token
3. Initialize the DAO with the token address
4. Initialize the Bitcoin issuance contract with start block and token address
5. Initialize the DEX adapter with DAO and token addresses
6. Configure the DAO as an administrator of the issuance and DEX contracts

## Tokenomics Parameters

Current tokenomics parameters are set as follows:

| Parameter | Value | Description |
|-----------|-------|-------------|
| Total Supply | 21,000,000 AGT | Maximum supply cap |
| Initial Release | 45% | Percentage released in first 6 months |
| Initial Block Reward | 50 AGT | Block reward after initial period |
| Halving Interval | 210,000 blocks | ~4 years with 10-minute blocks |
| DEX Fee | 0.3% | Trading fee percentage |
| Proposal Threshold | 100 AGT | Minimum tokens to submit a proposal |
| Voting Threshold | 60% | Percentage needed to pass a proposal |
| Quorum | 30% | Minimum participation required |

## Compliance with Bitcoin Development Framework

This implementation follows the Bitcoin Development Framework v2.5 requirements:

1. **Protocol Adherence**
   - Bitcoin-like monetary policy with halving schedule
   - Use of traits for interface consistency
   - Comprehensive error handling

2. **Privacy-Preserving Architecture**
   - Constant product market maker formula for DEX
   - Predictable issuance schedule

3. **Asset Management Standards**
   - SIP-010 compliant token
   - Well-defined treasury management

## Future Enhancements

Potential future enhancements to the tokenomics system include:

1. **Staking Mechanism**
   - Time-locked staking for additional rewards
   - Staking-based voting weight

2. **Enhanced DEX Features**
   - Multi-pair trading
   - Dynamic fee adjustment based on volatility
   - Flash loan prevention mechanisms

3. **Cross-Chain Integration**
   - Wrapped AGT on other chains
   - Cross-chain governance capabilities 