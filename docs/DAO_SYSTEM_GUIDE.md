# Anya DAO System Comprehensive Guide

## Introduction

The Anya DAO (Decentralized Autonomous Organization) is a governance system built on the Bitcoin Development Framework v2.5, featuring Bitcoin-style tokenomics, integrated DEX capabilities, and advanced proposal management. This comprehensive guide consolidates information from all DAO-related documentation.

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
# On Windows
.\scripts\install-clarinet.ps1
```

### Verifying Configuration

To ensure all contracts are properly configured in Clarinet.toml:

```powershell
# On Windows
.\scripts\verify-clarinet-config.ps1
```

### Running Tests

With Clarinet installed:

```bash
# Navigate to the anya-core directory
cd anya-core

# Check contract syntax
clarinet check

# Run tests
clarinet test
```

Without Clarinet (simulation only):

```powershell
# On Windows
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
# Check DAO core syntax
clarinet check dao/core/dao-core.clar

# Run a specific test
clarinet test dao/tests/dao-core-test.clar

# Deploy to testnet
clarinet deploy --testnet

# Generate documentation
clarinet docs
