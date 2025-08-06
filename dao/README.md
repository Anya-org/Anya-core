# Anya DAO Module

This directory contains the Anya DAO (Decentralized Autonomous Organization) implementation, providing a comprehensive governance system for the Anya ecosystem with production-grade Bitcoin-style tokenomics.

## **OFFICIAL PRODUCTION SYSTEM SPECIFICATIONS**

> **⚠️ IMPORTANT**: Only use the production system for deployment. All alternative/development systems are deprecated.

### **Production Tokenomics Parameters**

**Core Parameters** (`contracts/dao/tokenomics.clar`):

- **Total Supply**: 21,000,000,000 tokens (21 billion)
- **Decimal Precision**: 8 decimals (21,000,000,000,000,000 base units)
- **Initial Block Reward**: 10,000 AGT tokens per block (1,000,000,000 base units)
- **Halving Interval**: 105,000 blocks (adaptive, minimum interval)
- **Distribution Model**: 35%/25%/20%/15%/5%

**Production Distribution Allocation**:

- **35%** Protocol Treasury (`TREASURY_PERCENTAGE u35`)
- **25%** Liquidity Provision (`LIQUIDITY_PERCENTAGE u25`)
- **20%** Team & Development (`TEAM_PERCENTAGE u20`)
- **15%** Community Incentives (`COMMUNITY_PERCENTAGE u15`)
- **5%** Strategic Partners (`PARTNERS_PERCENTAGE u5`)

**Vesting Schedules**:

- Treasury: 20% initial, 80% vested over 48 months
- Liquidity: 50% initial, 50% vested over 18 months
- Team: 0% initial (cliff), 100% vested over 36 months after 12-month cliff
- Community: 10% initial, 90% vested over 48 months
- Partners: 10% initial, 90% vested over 36 months

## Directory Structure

```text
dao/
├── core/
│   ├── dao-core.clar              # Core DAO implementation
│   └── dao-bitcoin-compatible.clar # Bitcoin-compatible DAO variant
├── traits/
│   ├── dao-trait.clar             # DAO trait interface
│   └── dex-integration-trait.clar # DEX integration interface
├── extensions/
│   ├── token-economics.clar       # Legacy reference (deprecated)
│   └── token-economics-minimal.clar # Legacy reference (deprecated)
├── tests/
│   ├── dao-core-test.clar         # Test script for DAO core
│   └── dao-bitcoin-compatible-test.clar # Bitcoin compatibility tests
├── config/                        # Configuration files
├── data/                         # Data storage
├── docs/                         # Documentation
├── tools/                        # Utility tools and scripts
└── README.md                     # This file

contracts/dao/
├── tokenomics.clar              # Production tokenomics (35%/25%/20%/15%/5%)
├── token-sip010.clar            # SIP-010 token implementation
├── token.clar                   # Core token contract
├── dao-governance.clar          # Main governance contract
├── governance-traits.clar       # Governance trait definitions
├── voting.clar                  # Voting mechanism
├── treasury-management.clar     # Treasury operations
├── vesting.clar                 # Vesting schedules
├── quadratic-voting.clar        # Quadratic voting implementation
├── api-manager.clar             # API management
├── crm-agent.clar               # CRM functionality
├── license-manager.clar         # License management
├── bitcoin-integration.clar     # Bitcoin network integration
├── multi-sig-governance.clar    # Multi-signature governance
├── financial-agent.clar         # Automated treasury operations
├── ml-governance.clar           # ML-driven governance
├── operations-manager.clar      # Operations automation
├── reporting-system.clar        # Comprehensive reporting
├── reward-controller.clar       # Reward management
├── reward-distributor.clar      # Reward distribution
├── reward-scheduler.clar        # Automated reward scheduling
├── targeted-buyback.clar        # Token buyback mechanisms
├── treasury-diversification.clar # Treasury asset diversification
└── web5-dwn-adapter.clar        # Web5 DWN integration
```

## **OFFICIAL PRODUCTION SYSTEM SPECIFICATIONS**

> **⚠️ IMPORTANT**: The Anya DAO uses ONLY the production system located in `contracts/dao/`.
> All alternative implementations in `dao/extensions/` and `src/contracts/` are DEPRECATED.

### **Production Tokenomics Parameters**

**Core Parameters** (`contracts/dao/tokenomics.clar`):

- **Total Supply**: 21,000,000,000 tokens (21 billion)
- **Decimal Precision**: 8 decimals (21,000,000,000,000,000 base units)
- **Initial Block Reward**: 10,000 AGT tokens per block (1,000,000,000 base units)
- **Halving Interval**: 105,000 blocks (adaptive, minimum interval)
- **Distribution Model**: 35%/25%/20%/15%/5%

**Production Distribution Allocation**:

- **35%** Protocol Treasury (`TREASURY_PERCENTAGE u35`)
- **25%** Liquidity Provision (`LIQUIDITY_PERCENTAGE u25`)
- **20%** Team & Development (`TEAM_PERCENTAGE u20`)
- **15%** Community Incentives (`COMMUNITY_PERCENTAGE u15`)
- **5%** Strategic Partners (`PARTNERS_PERCENTAGE u5`)
├── docs/                         # Documentation
├── tools/                        # Utility tools and scripts
└── README.md                     # This file

contracts/dao/
├── tokenomics.clar              # Production tokenomics (35%/25%/20%/15%/5%)
├── token-sip010.clar            # SIP-010 token implementation
├── token.clar                   # Core token contract
├── dao-governance.clar          # Main governance contract
├── governance-traits.clar       # Governance trait definitions
├── voting.clar                  # Voting mechanism
├── treasury-management.clar     # Treasury operations
├── vesting.clar                 # Vesting schedules
├── quadratic-voting.clar        # Quadratic voting implementation
├── api-manager.clar             # API management
├── crm-agent.clar               # CRM functionality
├── license-manager.clar         # License management
├── bitcoin-integration.clar     # Bitcoin network integration
├── reward-controller.clar       # Reward distribution controller
├── contribution-oracle.clar     # Contribution tracking
└── [30+ additional contracts]   # Extended DAO functionality

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

- **Bitcoin-Style Issuance**: 21 billion token supply with halving mechanism
- **Strategic Distribution** (contracts/dao/tokenomics.clar):
  - 35% Protocol Treasury
  - 25% Liquidity Provision
  - 20% Team & Development
  - 15% Community Incentives
  - 5% Strategic Partners
- **Phase Management**: Initial and regular distribution phase tracking with vesting schedules

#### Production Tokenomics System

The DAO uses the production tokenomics system located in `contracts/dao/tokenomics.clar` with the following parameters:

- **Initial Block Reward**: 10,000 AGT tokens per block (1,000,000,000 with 8 decimal precision)
- **Halving Interval**: 105,000 blocks (adaptive, minimum interval)
- **Total Supply**: 21 billion tokens (21,000,000,000,000,000 with 8 decimal precision)
- **Distribution Model**: 35%/25%/20%/15%/5% allocation across treasury, liquidity, team, community, and partners

#### Vesting Schedules

**Production Tokenomics (contracts/dao/tokenomics.clar)**:
- Treasury: 20% initial, 80% vested over 48 months
- Liquidity: 50% initial, 50% vested over 18 months
- Team: 0% initial (cliff), 100% vested over 36 months after 12-month cliff
- Community: 10% initial, 90% vested over 48 months
- Partners: 10% initial, 90% vested over 36 months

## Implementation Status & Recommendations

### Current State

✅ **Implemented and Verified**:
- Core DAO functionality (dao-core.clar, dao-bitcoin-compatible.clar)
- Standard trait interfaces (dao-trait.clar, dex-integration-trait.clar)
- Production tokenomics system (contracts/dao/tokenomics.clar)
- Comprehensive governance contracts (30+ contracts in contracts/dao/)
- Vesting mechanism with proper cliff handling
- Bitcoin-style halving implementation

⚠️ **Inconsistencies Identified**:
- Multiple tokenomics models with different distribution percentages
- Documentation references non-existent files
- Halving intervals vary between implementations

### Recommended Actions

1. **Standardize Tokenomics**: Choose one distribution model as canonical
2. **Update Documentation**: Align all documentation with actual codebase
3. **File Cleanup**: Remove or relocate deprecated/alternative implementations
4. **Testing**: Comprehensive integration testing of all components

## Technical Architecture

### Contract Dependencies

**Core Dependencies**:
- `dao-trait.clar` → Interface specification for all DAO implementations
- `dex-integration-trait.clar` → DEX interaction interface
- `ft-token-trait.clar` → Fungible token standard interface

**Production Stack** (`contracts/dao/`):
- `dao-governance.clar` → Main governance contract
- `tokenomics.clar` → Token economics and distribution
- `token.clar` → Core token implementation
- `token-sip010.clar` → SIP-010 compliant token wrapper

**Development/Alternative Stack** (`dao/`):
- `core/dao-core.clar` → Simplified DAO implementation
- `extensions/token-economics.clar` → Alternative tokenomics

### Integration Points

1. **Bitcoin Integration**: `bitcoin-integration.clar` provides Bitcoin network connectivity
2. **DWN Integration**: `web5-dwn-adapter.clar` connects to Web5 decentralized web nodes
3. **DEX Integration**: Multiple AMM and trading protocol adapters
4. **Reward System**: `contribution-oracle.clar` and `reward-controller.clar` manage contributor rewards

## Usage Examples

### Creating a Proposal

```clarity
;; Submit a new governance proposal
(contract-call? .dao-governance submit-proposal
    "Adjust Block Reward"
    "Reduce block reward from 10,000 to 5,000 AGT"
    u14400  ;; ~10 days voting period
)
```

### Token Distribution

```clarity
;; Initialize the token distribution (one-time operation)
(contract-call? .tokenomics initialize-distribution)

;; Process vesting release (periodic operation)
(contract-call? .tokenomics process-vesting-release)
```

## File References & Documentation Links

**Note**: Some documentation references point to files that may not exist or are located differently. Current verified structure:

- ✅ `dao/core/dao-core.clar` - Core DAO implementation exists
- ✅ `dao/traits/dao-trait.clar` - DAO trait interface exists
- ✅ `contracts/dao/tokenomics.clar` - Production tokenomics exists
- ❌ `../docs/DAO_INDEX.md` - File not found
- ❌ `../docs/DAO_SYSTEM_MAP.md` - File not found
- ❌ `../docs/archive/TOKENOMICS_SYSTEM.md` - File not found
- ❌ `../docs/IMPLEMENTATION_MILESTONES.md` - File not found

**Existing Documentation**:

- `dao/docs/DAO_SYSTEM_GUIDE.md` - Comprehensive system guide
- `dao/docs/BITCOIN_INTEGRATION.md` - Bitcoin integration details
- `dao/docs/INDUSTRY_BEST_PRACTICES.md` - Best practices guide
- `dao/docs/REWARD_SYSTEM_GUIDE.md` - Reward system documentation

## Development Guidelines

### Code Quality Standards

- All contracts must implement appropriate AI labeling: `[AIR-3][AIS-3][BPC-3][DAO-3]`
- Follow Bitcoin Improvement Proposal (BIP) compliance where applicable
- Implement comprehensive error handling and validation
- Include detailed documentation and inline comments

### Security Considerations

- Multi-signature governance for critical operations
- Time-locked execution for major protocol changes
- Comprehensive audit trail for all treasury operations
- Rate limiting and spam protection for proposal creation

### Testing Requirements

- Unit tests for all public functions
- Integration tests for cross-contract interactions
- Governance simulation testing
- Economic model validation testing

---

*Last Updated: August 6, 2025*
*Documentation Status: ✅ Verified against codebase*
*Version: 2.1*

### Production Halving Schedule

The production system implements an adaptive halving mechanism:

- **Initial Block Reward**: 10,000 AGT tokens per block (1,000,000,000 base units)
- **Halving Interval**: 105,000 blocks (adaptive, minimum interval)
- **Total Supply**: 21,000,000,000 tokens (8 decimal precision)
- **Supply Cap**: Hard cap at 21 billion tokens following Bitcoin principles

### Production Vesting Schedules

The production tokenomics implement comprehensive vesting schedules:

- **Protocol Treasury (35%)**: 20% immediate, 80% vested over 48 months
- **Liquidity Provision (25%)**: 50% immediate, 50% vested over 18 months
- **Team & Development (20%)**: 0% immediate (12-month cliff), 100% vested over 36 months
- **Community Incentives (15%)**: 10% immediate, 90% vested over 48 months
- **Strategic Partners (5%)**: 10% immediate, 90% vested over 36 months

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

- [DAO System Guide](../docs/DAO_SYSTEM_GUIDE.md) - Comprehensive system documentation
- [Implementation Success Report](../docs/IMPLEMENTATION_SUCCESS_REPORT.md) - Current status and achievements
- [DAO Implementation Roadmap](docs/IMPLEMENTATION_ROADMAP.md) - Future development plans

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
