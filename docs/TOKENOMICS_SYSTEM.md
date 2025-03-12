<!-- markdownlint-disable MD013 line-length -->

# Anya Tokenomics System Architecture

This document outlines the tokenomics architecture of the Anya DAO system, including the Bitcoin-style issuance model, DEX integration, and related components.

## System Overview

The Anya tokenomics system implements a pure Bitcoin-style token issuance model with specialized distribution rules:

1. Bitcoin-style issuance from genesis with higher initial rewards
2. Strategic token distribution to DEX (30%), dev team (15%), and DAO/community (55%)
3. Varying developer team allocation based on work contribution

This model ensures proper market liquidity through DEX allocation, rewards contributors fairly, and maintains the long-term supply control mechanism inspired by Bitcoin's monetary policy.

> **Implementation Status**: For detailed information about the implementation progress and milestones, see [IMPLEMENTATION_MILESTONES.md](IMPLEMENTATION_MILESTONES.md).

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

### Bitcoin-Style Issuance with Specialized Distribution

- **Total Supply**: 21 billion AGT (with 8 decimal places)
- **Initial Block Reward**: 5,000 AGT per block (higher than Bitcoin)
- **Halving Interval**: Every 210,000 blocks (~4 years with 10-minute blocks)
- **Halving Schedule**:
  - First 210,000 blocks: 5,000 AGT per block
  - Next 210,000 blocks: 2,500 AGT per block
  - Next 210,000 blocks: 1,250 AGT per block
  - And so on...

### Distribution Allocation

For each block reward:

- **DEX Allocation**: 30% of issuance for liquidity provision
- **Developer Team**: 15% of issuance distributed to 10 team members
- **DAO/Community**: 55% of issuance for governance and community

### Developer Team Allocation

The 15% allocation to the development team is distributed among 10 team members based on work contribution:

- **Top Performer**: 40% of the team allocation
- **Lowest Performer**: 5% of the team allocation
- **Other Members**: Distributed on a sliding scale between 40% and 5%

Team member allocations are configurable by administrators but must always sum to 100% of the team allocation.

## DEX Integration

The DEX adapter provides liquidity management and trading capabilities for the AGT token, including:

### Key Features

1. **Liquidity Provision**
   - DEX receives 30% of all token issuance
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
   - It directs new tokens to the DEX, team members, and DAO as per allocation rules

3. **DEX & Token**
   - The DEX receives 30% of all newly minted tokens for liquidity
   - It implements the `dex-integration-trait` interface

4. **DAO & DEX Integration**
   - The DAO can issue instructions to the DEX for buybacks
   - The DAO can adjust liquidity parameters and fee settings

## Governance Controls

The DAO governance system has control over several tokenomics parameters:

1. **Issuance Controls**
   - Initialization of the issuance schedule
   - Setting team member allocations
   - Configuration of contract addresses

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
4. Initialize the Bitcoin issuance contract with start block, token, DAO, and DEX addresses
5. Set the team member allocations using the `set-team-allocations` function
6. Configure the DAO as an administrator of the issuance and DEX contracts

## Tokenomics Parameters

Current tokenomics parameters are set as follows:

| Parameter | Value | Description |
|-----------|-------|-------------|
| Total Supply | 21,000,000,000 AGT | Maximum supply cap |
| Initial Block Reward | 5,000 AGT | Block reward with 8 decimal places |
| Halving Interval | 210,000 blocks | ~4 years with 10-minute blocks |
| DEX Allocation | 30% | Percentage of block rewards allocated to DEX |
| Team Allocation | 15% | Percentage of block rewards allocated to dev team |
| DAO Allocation | 55% | Percentage of block rewards allocated to DAO/community |
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
