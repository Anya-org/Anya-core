<!-- markdownlint-disable MD013 line-length -->

# Anya DAO System Guide [AIR-3][AIS-3][AIT-3][AIP-3][BPC-3][DAO-3]

## Overview

The Anya DAO (Decentralized Autonomous Organization) is the governance layer for the Anya Core Platform, enabling token holders to collectively manage the protocol, treasury, and ecosystem development.

## Governance Token

The Anya Governance Token (AGT) is the core utility and governance token of the platform.

### Token Economics

- **Total Supply**: 21,000,000,000 AGT (Fixed)
- **Initial Block Reward**: 10,000 AGT
- **Emission Schedule**: Adaptive Bitcoin-inspired halving mechanism
  - Minimum halving interval: 105,000 blocks
  - Halving controlled by governance parameters

### Token Distribution

The AGT token is distributed according to the following model:

- **35% Protocol Treasury** (7,350,000,000 AGT)
  - 15% Strategic Reserves
  - 20% Ecosystem Development

- **25% Liquidity Provision** (5,250,000,000 AGT)
  - 15% Initial DEX Liquidity
  - 10% Ongoing Liquidity Mining

- **20% Team & Development** (4,200,000,000 AGT)
  - 4-year vesting with 1-year cliff
  - Milestone-based release triggers

- **15% Community Incentives** (3,150,000,000 AGT)
  - Governance participation rewards
  - Protocol usage incentives

- **5% Strategic Partners & Advisors** (1,050,000,000 AGT)
  - 3-year vesting schedule

*For detailed tokenomics information, see [TOKENOMICS_SYSTEM.md](TOKENOMICS_SYSTEM.md)*

## Governance Framework

### Proposal Types

The DAO supports multiple proposal types, each with specific requirements and voting parameters:

1. **Protocol Upgrades**
   - Contract upgrades
   - Parameter changes
   - Feature additions/removals

2. **Treasury Management**
   - Fund allocations
   - Investment decisions
   - Protocol-owned liquidity operations
   
3. **Emission Schedule Adjustments**
   - Halving interval modifications
   - Block reward changes
   - Special emission events

4. **Community Grants**
   - Developer grants
   - Marketing initiatives
   - Community projects

5. **Governance System Changes**
   - Voting mechanism updates
   - Proposal threshold adjustments
   - Quorum requirement changes

### Proposal Process

1. **Submission Phase**
   - Minimum 100 AGT to submit a proposal
   - 3-day discussion period
   - Technical feasibility review

2. **Voting Phase**
   - 7-day voting period
   - Token-weighted voting
   - Quadratic voting for select proposal types

3. **Execution Phase**
   - 2-day timelock before execution
   - Automatic execution for approved proposals
   - Multi-signature security for treasury operations

### Voting Power

Voting power in the DAO is determined by:

- AGT token holdings
- Governance participation history
- Reputation score (based on contribution)

## Treasury Management

The DAO treasury is managed according to the following principles:

### Treasury Composition

- **Strategic Reserves**: 15% minimum of circulating supply
- **Protocol-Owned Liquidity**: Minimum 15% of DEX allocation
- **Ecosystem Fund**: Grants and investments
- **Operations Fund**: Protocol development and maintenance

### Treasury Operations

The DAO can authorize various treasury operations:

1. **Liquidity Management**
   - Adding/removing DEX liquidity
   - Fee tier adjustments
   - Rebalancing across venues

2. **Buyback and Burn**
   - Token buybacks from market
   - Burning mechanisms
   - Supply adjustment operations

3. **Strategic Investments**
   - Protocol investments
   - Ecosystem funding
   - Partnership development

4. **Reserve Management**
   - Asset diversification
   - Yield generation
   - Risk management

### Treasury Guards

To ensure responsible treasury management:

- **Spending Limits**: Tiered approval requirements based on amount
- **Circuit Breakers**: Emergency pause during extreme conditions
- **Time Locks**: Graduated waiting periods based on impact
- **Audits**: Quarterly independent audits

## Implementation Architecture

The DAO is implemented using:

### On-Chain Components

- **Governance Contract**: Main DAO contract
- **Treasury Contract**: Treasury management
- **Token Contract**: AGT token implementation
- **Proposal Registry**: Tracks all proposals

### Off-Chain Components

- **DAO Dashboard**: Web interface for governance
- **Analytics Suite**: Governance metrics and insights
- **Notification System**: Alerts for proposals and votes
- **Discussion Forum**: Proposal discussion platform

## Security Measures

The DAO implements multiple security layers:

- **Multi-Signature Requirements**: For critical operations
- **Time Locks**: Delayed execution of significant changes
- **Security Council**: Emergency response capability
- **Formal Verification**: Of all governance contracts
- **Bug Bounty Program**: For vulnerability reporting

## Getting Started

### Participation Guide

1. **Acquire AGT tokens**
   - DEX trading
   - Liquidity provision
   - Community contributions

2. **Delegate Voting Power**
   - Self-delegation
   - Delegate to representatives
   - Split delegation

3. **Create Proposals**
   - Proposal templates
   - Documentation requirements
   - Technical specifications

4. **Vote on Proposals**
   - Voting interface
   - Voting strategies
   - Vote timing considerations

## Technical Reference

### Contract Addresses

- **DAO Contract**: `[CONTRACT_ADDRESS]`
- **Treasury Contract**: `[CONTRACT_ADDRESS]`
- **Token Contract**: `[CONTRACT_ADDRESS]`

### API Integration

```typescript
// Example: Creating a proposal
const proposal = await anyaDAO.createProposal({
  title: "Adjust Emission Schedule",
  description: "Modify halving interval to 115,000 blocks",
  actions: [
    {
      contract: "emission",
      method: "setHalvingInterval",
      params: ["115000"]
    }
  ]
});
```

## Governance Dashboard

The DAO dashboard is available at [https://dao.anya-core.org](https://dao.anya-core.org) and provides:

- Active proposal overview
- Voting interface
- Treasury statistics
- Governance analytics
- Personal voting history
- Delegation management

## Version History

- **v2.0.0**: Updated tokenomics model with adaptive emission and strategic distribution
- **v1.0.0**: Initial DAO implementation with fixed Bitcoin-style emission

## Additional Resources

- [DAO Technical Documentation](./DAO_TECHNICAL.md)
- [DAO System Map](./DAO_SYSTEM_MAP.md)
- [Tokenomics System](./TOKENOMICS_SYSTEM.md)
- [Governance API Reference](./api/GOVERNANCE_API.md)

## Key Features

- **Bitcoin-Style Tokenomics**: 21 billion token supply with halving mechanism
- **Strategic Distribution**: 30% DEX, 15% development team, 55% DAO/community
- **Enhanced Governance**: Advanced proposal creation, voting, and execution
- **DEX Integration**: Built-in liquidity and trading capabilities
- **Comprehensive Logging**: Complete transparency for all operations
- **Modular Architecture**: Clear separation of interfaces and implementations

## Documentation Map

This project includes several documents covering different aspects of the DAO system:

| Document | Purpose | Location |
|----------|---------|----------|
| [DAO Index](DAO_INDEX.md) | Central entry point to all DAO documentation | `docs/DAO_INDEX.md` |
| [DAO README](../dao/README.md) | Overview of setup and usage | `dao/README.md` |
| [DAO System Map](DAO_SYSTEM_MAP.md) | Architectural overview | `docs/DAO_SYSTEM_MAP.md` |
| [Tokenomics System](TOKENOMICS_SYSTEM.md) | Token economics architecture | `docs/TOKENOMICS_SYSTEM.md` |
| [Implementation Milestones](IMPLEMENTATION_MILESTONES.md) | Progress tracking and roadmap | `docs/IMPLEMENTATION_MILESTONES.md` |
| This Guide | Comprehensive consolidated documentation | `docs/DAO_SYSTEM_GUIDE.md` |

## System Architecture

### Component Architecture

The DAO system consists of the following components:

```
anya-core/
├── dao/
│   ├── core/
│   │   └── dao-core.clar        # Enhanced Core DAO implementation
│   ├── traits/
│   │   ├── dao-trait.clar       # DAO trait interface
│   │   └── dex-integration-trait.clar # DEX integration interface
│   ├── extensions/
│   │   └── token-economics.clar # Advanced token economics implementation
│   └── tests/
│       └── dao-core-test.clar   # Test script for DAO core
└── src/
    └── contracts/
        ├── dao.clar             # Main DAO contract with full governance
        ├── governance_token.clar # Governance token contract
        ├── bitcoin-issuance.clar # Bitcoin-style token issuance
        └── dex-adapter.clar     # DEX integration for liquidity
```

### Component Relationships

The components interact with each other according to the following diagram:

```
┌─────────────────┐     implements     ┌─────────────────┐
│   dao-trait.clar │◄─────────────────┤  dao-core.clar  │
└────────┬────────┘                   └────────▲────────┘
         │                                     │
         │                                     │
         │ uses trait                          │ calls
         │                                     │
         ▼                                     │
┌─────────────────┐     interacts     ┌─────────────────┐
│    dao.clar     │◄─────────────────►│ governance_token│
└─────────┬───────┘                   └────────▲────────┘
          │                                    │
          │ controls                           │ mints
          ▼                                    │
┌─────────────────┐     provides      ┌─────────────────┐
│   dex-adapter   │◄─────────────────┤bitcoin-issuance │
└─────────────────┘     liquidity     └─────────────────┘
       ▲                                    ▲
       │           ┌─────────────────┐      │
       └───────────┤token-economics ├──────┘
                   └─────────────────┘
                         guides
```

## Bitcoin-Style Tokenomics

### Issuance Model

The Anya governance token (AGT) follows a Bitcoin-style issuance model:

- **Total Supply**: 21 billion AGT (with 8 decimal places)
- **Initial Block Reward**: 5,000 AGT per block (higher than Bitcoin)
- **Halving Interval**: Every 210,000 blocks (~4 years with 10-minute blocks)
- **Halving Schedule**:
  - First 210,000 blocks: 5,000 AGT per block
  - Next 210,000 blocks: 2,500 AGT per block
  - Next 210,000 blocks: 1,250 AGT per block
  - And so on...

### Distribution Allocation

Each block reward is distributed strategically:

- **DEX Allocation (30%)**: Automatically added to the DEX liquidity pool
- **Developer Team (15%)**: Distributed among 10 team members based on contribution
- **DAO/Community (55%)**: Allocated to the DAO for governance and community initiatives

### Developer Team Allocation

The team allocation is further distributed:

- **Top Performer**: 40% of the team allocation
- **Lowest Performer**: 5% of the team allocation
- **Other Members**: Distributed on a sliding scale between 40% and 5%

## Governance System

### Proposal Lifecycle

1. **Creation**: Any token holder with sufficient balance can submit a proposal
2. **Voting Period**: Token holders vote on the proposal (voting weight = token balance)
3. **Execution Delay**: Successful proposals go through a timelock period
4. **Execution**: Approved proposals are executed after the timelock

### Proposal Types

- **Parameter Changes**: Modify DAO settings
- **Token Actions**: Token distribution or allocation changes
- **DEX Actions**: Adjust DEX parameters or execute buybacks
- **Administrative Actions**: Add/remove administrators

### Voting Mechanism

- **Threshold**: Minimum token balance needed to submit a proposal (100 AGT default)
- **Quorum**: Minimum participation required for valid vote (30% default)
- **Approval**: Percentage needed to pass a proposal (60% default)

## DEX Integration

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

## Setup and Usage

### Prerequisites

- [Clarinet](https://github.com/hirosystems/clarinet) v2.3.0 or later

### Installation

If you don't have Clarinet installed, you can use the provided installation script:

```powershell
## On Windows
.\scripts\install-clarinet.ps1
```

### Verifying Configuration

To ensure all contracts are properly configured in Clarinet.toml:

```powershell
## On Windows
.\scripts\verify-clarinet-config.ps1
```

### Running Tests

With Clarinet installed:

```bash
## Navigate to the anya-core directory
cd anya-core

## Check contract syntax
clarinet check

## Run tests
clarinet test
```

Without Clarinet (simulation only):

```powershell
## On Windows
.\scripts\run-dao-tests.ps1
```

## Contract Usage Examples

### Integrating with the DAO

```clarity
;; Import the DAO trait
(use-trait dao-trait .dao-trait.dao-trait)

;; Function that uses the DAO
(define-public (submit-to-dao (dao-contract <dao-trait>) (title (string-ascii 256)) (description (string-utf8 4096)) (duration uint))
    (contract-call? dao-contract submit-proposal title description duration)
)
```

### Creating a Proposal

```clarity
;; Call the DAO contract to create a proposal
(contract-call? .dao-core submit-proposal "My Proposal" "This is a proposal description" u10080)
```

### Interacting with Token Economics

```clarity
;; Get current distribution phase
(contract-call? .token-economics get-current-phase)

;; Check available tokens to mint
(contract-call? .bitcoin-issuance get-available-to-mint)
```

### DEX Integration Example

```clarity
;; Get token price from DEX
(contract-call? .dex-adapter get-token-price)

;; Execute buyback through DAO
(contract-call? .dao-core execute-buyback u1000)
```

### Administrative Functions

```clarity
;; Update DAO settings (admin only)
(contract-call? .dao-core update-proposal-threshold u200)

;; Add an administrator (admin only)
(contract-call? .dao-core add-administrator 'ST2PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM)
```

## Implementation Status

Current implementation status:
- ✅ Core architecture and interfaces
- ✅ Bitcoin-style issuance model 
- 🔄 Distribution allocation mechanisms (In Progress)
- ⏳ DEX integration (Pending)
- ⏳ Advanced governance features (Pending)

For detailed progress, see the [Implementation Milestones](IMPLEMENTATION_MILESTONES.md) document.

## Bitcoin Development Framework Compliance

This implementation follows the Bitcoin Development Framework v2.5 requirements:

1. **Protocol Adherence**
   - Bitcoin-style issuance with halving schedule
   - Uses Clarity's trait system for interface consistency
   - Maintains decentralized governance principles
   - Comprehensive error handling and validation

2. **Privacy-Preserving Architecture**
   - Constant product market maker formula for DEX
   - Vote delegation through proxy patterns
   - Private proposal submission options
   - Secure admin controls with proper authorization checks

3. **Asset Management Standards**
   - Governance token uses SIP-010 standard
   - Proper token integration with mint functions
   - Token balance validation for proposal submission
   - Strategic distribution for liquidity and governance

4. **Security Measures**
   - Admin-only access for sensitive operations
   - Multi-level validation for all operations
   - Comprehensive logging for auditing
   - Clear separation of responsibilities between components

## Future Development

Planned enhancements to the DAO system include:

- **DLC Oracle Integration**: Using oracle attestations for voting
- **Cross-Chain Governance**: Integration with RSK and Liquid
- **Web5 Identity**: Using decentralized identities for member registration
- **Enhanced Voting**: Quadratic voting and delegation options
- **Advanced Execution**: Automatic execution of approved proposals
- **Extended DEX Features**: Multi-pair trading and dynamic fee adjustment

## Contributing

When extending or modifying the DAO system:

1. All new components should implement or use the appropriate traits
2. Maintain the file structure with traits in `traits/`, implementations in `core/`, and extensions in `extensions/`
3. Add appropriate tests in the `tests/` directory
4. Ensure all operations are properly logged for transparency
5. Update the documentation to reflect your changes
6. Ensure compatibility with the Bitcoin-style tokenomics model

## Reference Information

### Tokenomics Parameters

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

### Useful Commands

```bash
## Check DAO core syntax
clarinet check dao/core/dao-core.clar

## Run a specific test
clarinet test dao/tests/dao-core-test.clar

## Deploy to testnet
clarinet deploy --testnet

## Generate documentation
clarinet docs
