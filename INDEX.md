# Anya Core Documentation

Welcome to the Anya Core documentation. This index will help you navigate the various documentation files and resources available for the Anya Core platform.

## Quick Start

- [Getting Started](docs/getting-started/README.md) - Quick setup guide
- [Installation](INSTALLATION.md) - Installation instructions
- [README](README.md) - Main project readme with overview and features

## Core Documentation

- [Documentation Index](docs/INDEX.md) - Main documentation index
- [System Architecture](SYSTEM_MAP.md) - Complete system architecture
- [DAO System](src/dao/README.md) - Comprehensive DAO documentation
- [Tokenomics System](docs/TOKENOMICS_SYSTEM.md) - Bitcoin-style tokenomics
- [Implementation Milestones](docs/IMPLEMENTATION_MILESTONES.md) - Current progress tracking
- [Testing Strategy](TESTING.md) - Sectional testing methodology
- [README.md](./README.md) - Main project documentation
- [SECURITY_CODEQL.md](./SECURITY_CODEQL.md) - Security analysis framework documentation
- [ROADMAP.md](./ROADMAP.md) - Project development roadmap
- [CHANGELOG.md](./CHANGELOG.md) - Version history and changes
- [TODO.md](./TODO.md) - Current development tasks
- [SYSTEM_MAP.md](./SYSTEM_MAP.md) - System architecture overview
- [AI Labeling System](./docs/standards/AI_LABELING.md) - AI labeling system documentation

## Installation & Deployment

- [Installation Guide](INSTALLATION.md) - Complete installation instructions
- [Advanced Deployment](docs/installation/ADVANCED_DEPLOYMENT.md) - Advanced deployment scenarios

## Architecture Documentation

- [ML System Architecture](docs/ML_SYSTEM_ARCHITECTURE.md) - Machine Learning system with Agent Checker (AIP-002)
- [Security Architecture](docs/SECURITY_ARCHITECTURE.md) - Security system with System Hardening (AIE-001)
- [Performance Architecture](docs/PERFORMANCE_ARCHITECTURE.md) - Performance system with Optimization (AIR-008)
- [Core System Integration](docs/CORE_SYSTEM_INTEGRATION.md) - Integration of all P1 components

## System Components

### Bitcoin & Lightning

- [Bitcoin Integration](src/bitcoin/README.md) - Bitcoin protocol features
  - [Bitcoin Error Handling](src/bitcoin/error.rs) - Comprehensive error types
  - [Bitcoin Interface Layer](src/bitcoin/interface/) - Clean API abstractions
    - [Block Interfaces](src/bitcoin/interface/block.rs) - Block structure and information
    - [Transaction Interfaces](src/bitcoin/interface/transaction.rs) - Transaction structure and information
    - [Network Interfaces](src/bitcoin/interface/network.rs) - Network status and operations
  - [Protocol Implementation](src/bitcoin/protocol/) - Bitcoin protocol implementation
    - [Protocol Validation](src/bitcoin/protocol/validation.rs) - Transaction and protocol validation
    - [Script Execution](src/bitcoin/protocol/script.rs) - Bitcoin script execution
    - [Address Utilities](src/bitcoin/protocol/address.rs) - Address generation and validation
  - [Simplified Payment Verification](src/bitcoin/spv.rs) - SPV implementation with constant-time operations
  - [Taproot/BIP-341](src/bitcoin/bip341.rs) - Taproot implementation
  - [Bitcoin Adapters](src/bitcoin/adapters/) - Hexagonal architecture adapters
- [BIP Implementations](core/src/bip/) - Core BIP implementations
  - [BIP-341 (Taproot)](core/src/bip/bip341.rs) - Taproot verification and utilities
  - [BIP-342 (Tapscript)](core/src/bip/bip342.rs) - Tapscript verification
  - [BIP Registry](core/src/bip/mod.rs) - BIP implementation registry and status tracking
- [Lightning Integration](src/lightning/README.md) - Lightning Network features

### DAO & Tokenomics

- [DAO Architecture](src/dao/README.md) - Detailed architecture of the DAO
- [Bitcoin-Style Tokenomics](docs/TOKENOMICS_SYSTEM.md) - 21B token supply with halving
- [DAO Implementation Status](docs/IMPLEMENTATION_MILESTONES.md) - Current progress
- [DAO Component Reference](docs/DAO_INDEX.md) - Index of all DAO components

### Web5 & Decentralized Identity

- [Web5 Integration](src/web5/README.md) - Web5 implementation details
- [DID System](docs/identity/README.md) - Decentralized identity implementation

### AI & Machine Learning

- [ML System Architecture](docs/ML_SYSTEM_ARCHITECTURE.md) - ML system design with Agent Checker
- [AI Component Reference](src/ml/README.md) - AI component details

### Installation System

- [Installer Core](install/README.md) - Core installation system

## Development Resources

- [API Documentation](docs/api/README.md) - API reference
- [Contributing Guide](CONTRIBUTING.md) - How to contribute
- [Checkpoint System](docs/CHECKPOINT_SYSTEM.md) - Development checkpoints
- [AI Labeling Standards](docs/standards/AI_LABELING.md) - Component labeling standards

## Key Features at a Glance

### Bitcoin Module Architecture (Hexagonal Pattern)

- **Interface Layer**: Clean ports/interfaces for external services
  - Block interfaces (`interface/block.rs`)
  - Transaction interfaces (`interface/transaction.rs`)
  - Network interfaces (`interface/network.rs`)
- **Adapter Layer**: Adapters to external dependencies
  - Protocol adapters (`adapters/protocols/`)
  - RPC adapters (`adapters/rpc/`)
  - Storage adapters (`adapters/storage/`)
- **Core Domain Logic**: Bitcoin-specific business logic
  - Consensus rules (`core/consensus/`)
  - Mempool management (`core/mempool/`)
  - Transaction validation (`core/transaction/`)
  - Protocol implementation (`protocol/`)
  - BIP implementations (`core/bip/`)
- **Error Handling**: Comprehensive error types and propagation
  - Bitcoin-specific errors (`error.rs`)
  - Custom error types for all subsystems
  - Constant-time operations for security-sensitive code

### DAO & Tokenomics System

- **Bitcoin-Style Tokenomics**: 21 billion token supply with halving mechanism
- **Strategic Distribution**: 30% DEX, 15% development team, 55% DAO/community
- **Enhanced Governance**: Advanced proposal creation, voting, and execution
- **DEX Integration**: Built-in liquidity and trading capabilities
- **Comprehensive Logging**: Complete transparency for all operations

### Hexagonal Architecture

- Clean separation of concerns with ports and adapters
- Domain-driven design principles
- Advanced error handling and telemetry
- Circuit breaker pattern implementation

### Bitcoin & Lightning Integration

- Bitcoin Core support
- Lightning Network capabilities
- DLC (Discreet Log Contracts)
- Taproot/Schnorr signatures

### Web5 Integration

- Decentralized Web Nodes (DWN)
- Decentralized Identity (DID)
- Protocol-based data management
- Encrypted storage

### AI & Machine Learning

- Model management and execution
- Real-time inference
- Performance monitoring
- Model A/B testing

## Security & Compliance

- [Security Guidelines](SECURITY.md) - Security best practices
- [Compliance Framework](docs/COMPLIANCE.md) - Compliance information
- [Privacy Measures](docs/PRIVACY.md) - Privacy protection measures

## Release Information

- **Current Version**: 3.1.2
- **Last Updated**: May 1, 2025
- **Compatibility**:
  - Stacks v2.4
  - Web5 Protocol v1.0
  - Bitcoin Core 24.0+

## Support & Community

- [GitHub Repository](https://github.com/anya/anya-core)
- [Issue Tracker](https://github.com/anya/anya-core/issues)
- [GitHub Discussions](https://github.com/anya/anya-core/discussions)

*This documentation follows the [AI Labeling Standards](docs/standards/AI_LABELING.md) based on the Bitcoin Development Framework v2.5. All components are labeled with appropriate Core and Extended category labels.*

## Bitcoin Protocol Implementation

- [Bitcoin Implementation](src/bitcoin/) - Bitcoin protocol implementation
- [BIP Compliance Matrix](docs/BIP_COMPLIANCE.md) - Full BIP-341/342 implementation status
  - [BIP-341 (Taproot)](core/src/bip/bip341.rs) - Complete Taproot implementation
  - [BIP-342 (Tapscript)](core/src/bip/bip342.rs) - Complete Tapscript implementation
  - [BIP Registry](core/src/bip/mod.rs) - Implementation status tracking
- [DLC Implementation](src/layer2/dlc/) - Discrete Log Contracts

## Security Analysis Components

- [Security Framework](src/security/) - Security framework implementation
- [Crypto Implementation](src/security/crypto/) - Cryptographic implementation

## Development Scripts

- [Scripts Overview](scripts/README.md) - Overview of available scripts
- [Build Scripts](scripts/build.ps1) - Build script for the project

## Testing Framework

- [Test Framework](tests/) - Test framework
- [Integration Tests](tests/integration/) - Integration tests
- [Module Tests](tests/modules/) - Module-specific tests

## Compliance and Governance

- [Governance](GOVERNANCE.md) - Project governance structure
- [Contributing](CONTRIBUTING.md) - Contribution guidelines
- [Code of Conduct](CODE_OF_CONDUCT.md) - Community code of conduct
- [License](LICENSE.md) - License information

## Bitcoin Development Framework Compliance

All components conform to the Bitcoin Development Framework v2.5 guidelines and include proper AI labeling according to the following system:

- [AIR-3] - AI Readiness Level 3
- [AIS-3] - AI Security Level 3
- [BPC-3] - Bitcoin Protocol Compliance Level 3
- [AIT-3] - AI Testing Level 3
- [RES-3] - Resilience Level 3

For full details on the AI labeling system, see [docs/standards/AI_LABELING.md](./docs/standards/AI_LABELING.md).

## Version Information

- Current Version: 3.1.2
- Last Updated: May 1, 2025
- Bitcoin Development Framework: v2.5

## Directory Structure

The repository is organized into the following main directories:

- `/src` - Main source code
  - `/adapters` - Hexagonal architecture adapters
  - `/ai` - AI components
  - `/api` - API implementations
  - `/audit` - Audit tools
  - `/bin` - Binary executables
  - `/bip` - BIP implementations
  - `/bitcoin` - Bitcoin protocol implementation
  - `/compliance` - Compliance tools
  - `/components` - Reusable components
  - `/config` - Configuration files
  - `/contracts` - Smart contracts
  - `/core` - Core functionality
  - `/crypto` - Cryptographic implementations
  - `/dao` - DAO implementation
  - `/dashboard` - Dashboard UI
  - `/layer2` - Layer 2 solutions
  - `/lightning` - Lightning Network implementation
  - `/ml` - Machine learning components
  - `/security` - Security framework
  - `/web5` - Web5 implementation
- `/docs` - Documentation
- `/tests` - Tests
- `/scripts` - Utility scripts

## React Native Components

- [React Native Components](https://reactnative.directory/anya-mobile)

## Mobile Integration

- [React Native SDK](https://github.com/anya-org/anya-mobile) - Bitcoin-compliant mobile components
- [Mobile Security Guide](docs/mobile/SECURITY.md) - Secure mobile implementation
- [Taproot Mobile Demo](docs/mobile/TAPROOT_DEMO.md) - Mobile Taproot examples

*Last updated: 2025-04-28 18:05 UTC+2*
