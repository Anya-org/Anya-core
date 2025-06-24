# Anya DAO Module

This directory contains the Anya DAO (Decentralized Autonomous Organization) implementation, which provides a comprehensive governance system for the Anya ecosystem with Bitcoin-style tokenomics.

## Directory Structure

```
dao/
├── core/
│   └── dao-core.clar        # Enhanced Core DAO implementation
├── traits/
│   ├── dao-trait.clar       # DAO trait interface
│   └── dex-integration-trait.clar # DEX integration interface
├── extensions/
│   └── token-economics.clar # Advanced token economics implementation
├── tools/
│   ├── contribution-tracker.js  # GitHub activity tracker
│   ├── track-contributions.sh   # Tracker shell wrapper
│   ├── dao-reward-engine.js     # Bitcoin-style reward engine
│   └── run-dao-rewards.sh       # Reward engine shell wrapper
├── data/
│   ├── contribution_history.json # Contribution history data
│   └── reward_distribution.json  # Reward distribution records
├── docs/
│   └── REWARD_SYSTEM_GUIDE.md    # Reward system documentation
├── tests/
│   └── dao-core-test.clar   # Test script for DAO core
└── README.md                # This file
```

## Components

### DAO Trait (`traits/dao-trait.clar`)

The trait defines the standard interface that all DAO implementations must follow:

- **Token Management**: Functions for minting and burning governance tokens
- **Token Economics**: Distribution phase management and allocation tracking
- **DEX Integration**: Functions for DEX interaction and buyback execution
- **Proposal Management**: Functions for submitting, voting on, and executing proposals
- **Administrative Functions**: Admin controls and settings management
- **Queries**: Functions for retrieving DAO information and proposals
- **Financial Intelligence**: Metrics reporting and analysis

### Enhanced DAO Core (`core/dao-core.clar`)

The core implementation provides the following features:

1. **Token Integration**: Full integration with SIP-010 compliant tokens and Bitcoin-style issuance
2. **Enhanced Proposal Validation**: Comprehensive validation for proposals
3. **Administrative Functions**: Advanced admin controls and settings
4. **Comprehensive Logging**: Transparent logging of all significant actions

### DEX Integration Trait (`traits/dex-integration-trait.clar`)

Interface for DEX interaction with the DAO:

- **Liquidity Management**: Providing and removing liquidity
- **Trading Operations**: Token swapping functions
- **Price Oracle**: Token price discovery
- **Market Making**: Fee management and AMM configuration
- **Analytics**: Volume and trading metrics

### Token Economics (`extensions/token-economics.clar`)

Advanced token economics implementation:

- **Bitcoin-Style Issuance**: 21 billion token supply with halving every 210,000 blocks
- **Strategic Distribution**: 
  - 30% to DEX for liquidity
  - 15% to development team (variable allocation based on contribution)
  - 55% to DAO/community
- **Phase Management**: Initial and regular distribution phase tracking

### Contribution & Reward System (`tools/`)

GitHub contribution tracking and Bitcoin-style reward distribution:

- **Contribution Tracker**: Tracks GitHub activity and assigns points
- **DAO Reward Engine**: Calculates and distributes rewards based on tokenomics
- **Bitcoin-Style Rewards**: Reward calculation based on block height and halving schedule
- **Strategic Distribution**: Follows the 15% community allocation from tokenomics

## Bitcoin-Style Reward System

The reward system implements Anya Core's tokenomics with:

- **Total Supply**: 21 billion AGT tokens (Bitcoin parity)
- **Block Reward**: Initial 10,000 AGT tokens per block
- **Halving Schedule**: Every 210,000 blocks (following Bitcoin model)
- **Community Allocation**: 15% of total supply for contributor rewards

### Usage: Tracking Contributions

Track GitHub contributions with:

```bash
# Track last 30 days by default
./track-contributions.sh

# Track a specific time period
./track-contributions.sh --period=quarter

# Full contribution history
./track-contributions.sh --full-history
```

Options:

- `--period=PERIOD`: Time period to track (all-time, year, quarter, month, week)
- `--full-history`: Track complete repository history
- `--auto-run`: Run in non-interactive mode
- `--yes-all`: Answer yes to all prompts

### Usage: Processing Rewards

Calculate and distribute rewards using:

```bash
# Simulate reward distribution
./run-dao-rewards.sh

# Calculate rewards without distribution
./run-dao-rewards.sh --audit-only

# Execute actual token transfers
./run-dao-rewards.sh --mainnet
```

Options:

- `--simulate`: Run in simulation mode (default)
- `--mainnet`: Execute actual token transfers
- `--audit-only`: Calculate rewards without distribution
- `--force`: Override safety checks
- `--block=NUMBER`: Set current block height for calculations

### Test Script (`tests/dao-core-test.clar`)

Comprehensive test suite covering all aspects of the DAO Core implementation:

- Administrator management
- DAO settings management
- Proposal creation and validation
- Logging system
- Token integration

## Setup and Testing

### Prerequisites

- [Clarinet](https://github.com/hirosystems/clarinet) v2.3.0 or later
- [Node.js](https://nodejs.org/) v14 or later (for contribution tracking and rewards)

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

## Usage

### Integrating with the DAO

To use the DAO in your contract:

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

## Documentation

For more detailed information about the DAO system, see:

- [DAO Reward System Guide](docs/REWARD_SYSTEM_GUIDE.md) - Bitcoin-style reward system details
- [DAO Documentation Index](../docs/DAO_INDEX.md) - Central entry point to all DAO documentation
- [DAO System Map](../docs/DAO_SYSTEM_MAP.md) - Architectural overview
- [Tokenomics System](../docs/TOKENOMICS_SYSTEM.md) - Token economics details
- [Implementation Milestones](../docs/IMPLEMENTATION_MILESTONES.md) - Current status and roadmap

## Bitcoin Development Framework Compliance

This implementation follows official Bitcoin Improvement Proposals (BIPs) standards, including:

- **Protocol Adherence**: Bitcoin-style issuance with halving schedule
- **Privacy-Preserving Architecture**: Constant product market maker for DEX
- **Asset Management Standards**: SIP-010 compliant token with specialized distribution
- **Security Validation**: Comprehensive validation for all operations
- **Hexagonal Architecture**: Clear separation of interfaces and implementations

## Contributing

When extending or modifying the DAO system:

1. All new components should implement or use the appropriate traits
2. Maintain the file structure with traits in `traits/`, implementations in `core/`, and extensions in `extensions/`
3. Add appropriate tests in the `tests/` directory
4. Ensure all operations are properly logged for transparency
5. Update the documentation to reflect your changes
6. Ensure compatibility with the Bitcoin-style tokenomics model
