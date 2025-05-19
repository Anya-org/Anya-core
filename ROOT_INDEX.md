# Anya Core Documentation [AIR-3][AIS-3][BPC-3]

[AIR-3][AIS-3][BPC-3][RES-3]

*Last Updated: May 19, 2025*

Welcome to the Anya Core documentation. This is the authoritative root index for all Anya Core documentation, following Bitcoin Development Framework v2.5 standards.

## Recent Updates (May 19, 2025) [AIR-3][AIS-3][BPC-3][RES-3]

The following modules have been updated to fix compilation errors and ensure compliance with the Bitcoin Development Framework v2.5 standards:

- **RGB Module**: Fixed duplicate implementation of the `generate_asset_id` function and ensured Taproot-compatible asset ID generation
- **Bitcoin Module**: Resolved error handling issues and fixed network configuration handling
- **ML Module**: Implemented missing `predict_proposal_metrics` method and fixed `Device` usage
- **DLC Module**: Added missing `OracleClient` implementation and fixed method signatures

All modules now include proper AI labeling according to the canonical AI Labeling System. For detailed information about these changes, see the [Changelog](CHANGELOG.md) and [Implementation Summary](docs/IMPLEMENTATION_SUMMARY.md).

## Quick Navigation [AIR-3]

### Core Documentation

- [Getting Started](README.md) - Project overview and setup
- [Installation](INSTALLATION.md) - Installation instructions
- [System Architecture](SYSTEM_MAP.md) - Complete system architecture
- [Security Framework](SECURITY.md) - AIS-3 compliance details
- [Changelog](CHANGELOG.md) - Release notes and changes
- [Roadmap](ROADMAP.md) - Development roadmap and milestones

### Bitcoin Protocol Integration [BPC-3]

- [Bitcoin Module](docs/bitcoin/README.md) - Core Bitcoin functionality
- [Layer 2 Support](docs/layer2/README.md) - Layer 2 solutions
- [Migration Guide](docs/bitcoin/migration.md) - Migration documentation

### DAO & Tokenomics [DAO-3]

- [DAO Architecture](src/dao/README.md) - Detailed DAO architecture
- [Bitcoin-Compatible DAO](dao/core/dao-bitcoin-compatible.clar) - Full Bitcoin Layer 2 compatible implementation
- [Bitcoin-Style Tokenomics](docs/TOKENOMICS_SYSTEM.md) - 21B token supply with halving
- [Implementation Status](docs/IMPLEMENTATION_MILESTONES.md) - Current progress
- [Component Reference](docs/DAO_INDEX.md) - Index of DAO components

### Web5 & Identity [W5C-3][DID-3]

- [Web5 Integration](src/web5/README.md) - Web5 implementation details
- [DID System](docs/identity/README.md) - Decentralized identity implementation

### AI & Machine Learning [AIR-3][AIS-3]

- [ML System Architecture](docs/ML_SYSTEM_ARCHITECTURE.md) - ML system design
- [AI Component Reference](src/ml/README.md) - AI component details
- [Model Management](docs/ml/models.md) - Model versioning and deployment

## System Status

### Core Protocol Status [BPC-3]

- Bitcoin Core: v25.0
- Taproot: Fully enabled (BIP-341/342)
- PSBT: v2 support (BIP-174/370)
- HSM: v2.5 integration

### Layer 2 Integration Status [BPC-3]

- Lightning Network: Enabled (src/lightning)
- RGB Protocol: Q3 2025 (src/layer2/rgb)
- RSK Integration: Q3 2025 (src/layer2/rsk)
- BOB Layer 2: Active (src/layer2/bob)
- State Channels: Enabled (src/layer2/state_channels)

### Security Compliance [AIS-3]

- SILENT_LEAF Implementation: ✅
- Taproot Script Validation: ✅
- Schnorr Signature Support: ✅
- HSM Integration: ✅

### Web5 Components [W5C-3]

- DWN Implementation: ✅
- DID Support: ✅
- Verifiable Credentials: ✅
- BIP-341 Integration: ✅

## Development Resources

### API & SDK

- [API Documentation](docs/api/README.md) - Complete API reference
- [Mobile SDK](docs/mobile/SDK.md) - React Native integration
- [Web5 SDK](src/web5/README.md) - Web5 development kit

### Testing & Validation

- [Test Framework](TESTING.md) - Testing procedures
- [Security Tests](src/security/README.md) - Security validation
- [Performance Tests](src/testing/performance/README.md) - Benchmarking

### Compliance & Standards

- [AI Labeling](docs/standards/AI_LABELING.md) - Component labeling
- [Security Standards](docs/standards/SECURITY.md) - Security requirements
- [BIP Compliance](docs/standards/BIP_COMPLIANCE.md) - Bitcoin protocol standards

## Current Version

- Version: 3.1.1
- Released: 2025-04-29
- Framework: Bitcoin Development Framework v2.5

## Support

- Time Zone: UTC+2 (Johannesburg)
- Hours: 09:00-17:00 SAST
- Enterprise Support: <enterprise@anya.org>
- Security: <security@anya.org>

## Repository Structure

The repository follows a structured organization:

- `/src` - Main source code
  - `/bitcoin` - Bitcoin protocol implementation
  - `/layer2` - Layer 2 solutions (Lightning, RGB, RSK, etc.)
  - `/web5` - Web5 implementation
  - `/dao` - DAO system
  - `/security` - Security framework
  - `/ml` - Machine learning components
- `/docs` - Documentation files
- `/tests` - Test files
- `/scripts` - Utility scripts

## Repository Links

- [GitHub Repository](https://github.com/anya-org/anya-core)
- [Issue Tracker](https://github.com/anya-org/anya-core/issues)
- [Discussions](https://github.com/anya-org/anya-core/discussions)

---
*Last updated: 2025-04-29 14:30 UTC+2*
*All components comply with [AI Labeling Standards](docs/standards/AI_LABELING.md)*
