<!-- markdownlint-disable MD013 line-length -->

# Anya DAO Documentation Index

Welcome to the Anya DAO documentation. This index provides a comprehensive overview of all DAO-related documentation and components.

## Core Documentation

| Document | Description | Last Updated |
|----------|-------------|--------------|
| [DAO README](../dao/README.md) | Overview of the DAO module, setup, and usage | 2025-03-02 |
| [DAO System Map](DAO_SYSTEM_MAP.md) | Architectural overview of the DAO system | 2025-03-02 |
| [Tokenomics System](TOKENOMICS_SYSTEM.md) | Token economics architecture and Bitcoin-style issuance model | 2025-03-02 |
| [Implementation Milestones](IMPLEMENTATION_MILESTONES.md) | Implementation progress and roadmap | 2025-03-02 |

## Technical Components

### Contracts

| Contract | Purpose | Path |
|----------|---------|------|
| DAO Trait | Interface definition for DAO functionality | `dao/traits/dao-trait.clar` |
| DAO Core | Enhanced implementation of the DAO trait | `dao/core/dao-core.clar` |
| Main DAO | Governance contract that integrates with DAO Core | `src/contracts/dao.clar` |
| Governance Token | SIP-010 compliant AGT implementation | `src/contracts/governance_token.clar` |
| Bitcoin Issuance | Bitcoin-style token issuance with special distribution | `src/contracts/bitcoin-issuance.clar` |
| DEX Adapter | Decentralized exchange integration | `src/contracts/dex-adapter.clar` |
| DEX Integration Trait | Interface for DEX interaction | `dao/traits/dex-integration-trait.clar` |
| Token Economics | Advanced token economics implementation | `dao/extensions/token-economics.clar` |

### Test Scripts

| Script | Purpose | Path |
|--------|---------|------|
| DAO Core Test | Comprehensive test suite for DAO Core | `dao/tests/dao-core-test.clar` |

### Utility Scripts

| Script | Purpose | Path |
|--------|---------|------|
| Install Clarinet | PowerShell script to install Clarinet | `scripts/install-clarinet.ps1` |
| Verify Clarinet Config | Script to check and fix Clarinet configuration | `scripts/verify-clarinet-config.ps1` |
| Run DAO Tests | Script to simulate running DAO tests | `scripts/run-dao-tests.ps1` |

## Architecture Overview

The Anya DAO system follows a hierarchical architecture:

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
└─────────────────┘                   └─────────────────┘
         │                                     ▲
         │                                     │
         │ controls                            │ mints
         ▼                                     │
┌─────────────────┐     provides      ┌─────────────────┐
│   dex-adapter   │◄─────────────────┤bitcoin-issuance │
└─────────────────┘     liquidity     └─────────────────┘
```

## Tokenomics Integration

The DAO is tightly integrated with the tokenomics system through:

1. **Bitcoin-Style Issuance**: 21 billion token supply with halvings every 210,000 blocks
2. **Strategic Distribution**: 
   - 30% to DEX for liquidity
   - 15% to development team
   - 55% to DAO/community
3. **Governance Control**: DAO proposals can adjust tokenomics parameters

## Bitcoin Development Framework Compliance

All DAO components adhere to the Bitcoin Development Framework v2.5 requirements:

- Protocol adherence through trait-based design
- Privacy-preserving architecture
- Asset management standards
- Comprehensive security measures

## Implementation Status

Current implementation status:
- ✅ Core architecture and interfaces
- ✅ Bitcoin-style issuance model 
- 🔄 Distribution allocation mechanisms (In Progress)
- ⏳ DEX integration (Pending)
- ⏳ Advanced governance features (Pending)

For detailed progress, see the [Implementation Milestones](IMPLEMENTATION_MILESTONES.md) document.