# Anya Core Documentation

Welcome to the Anya Core documentation. This index will help you navigate the various documentation files and resources available for the Anya Core platform.

## 🔧 CURRENT DEVELOPMENT STATUS (June 10, 2025)

**The Anya-core project is actively under development with strong architectural foundations and ongoing Bitcoin protocol implementation.**

### 🎯 Verified Project Status

- **Architecture Design:** Excellent hexagonal architecture with clear separation of concerns ✅
- **Code Quality:** Professional Rust implementation with comprehensive error handling ✅
- **Bitcoin Implementation:** Active development of BIP-341, BIP-342, and BIP-174 protocols 🔄
- **Layer2 Protocols:** Mixed progress across Lightning, RGB, DLC, and other protocols 🔄
- **Documentation:** Comprehensive structure with ongoing accuracy improvements 🔄
- **Testing Framework:** Well-designed infrastructure ready for comprehensive test implementation 🔄

### 🏗️ Architecture Highlights

- **Hexagonal Architecture:** Complete port/adapter separation implemented ✅
- **BIP Compliance Framework:** Comprehensive validation and testing infrastructure ✅
- **Security Design:** Thoughtful security considerations and validation systems ✅
- **Performance Framework:** Benchmarking and monitoring infrastructure ready ✅

## Quick Start

- [Getting Started](docs/getting-started/README.md) - Quick setup guide
- [Installation](INSTALLATION.md) - Installation instructions
- [README](README.md) - Main project readme with overview and features

## Core Documentation

- [Documentation Index](docs/INDEX.md) - Main documentation index
- [System Architecture](docs/SYSTEM_MAP.md#system-architecture) - Complete system architecture (with visual map)
- [DAO System](src/dao/README.md) - Comprehensive DAO documentation
- [Tokenomics System](docs/TOKENOMICS_SYSTEM.md) - Bitcoin-style tokenomics
- [Implementation Milestones](docs/IMPLEMENTATION_MILESTONES.md) - Current progress tracking
- [Testing Strategy](TESTING.md) - Sectional testing methodology
- [SECURITY_CODEQL.md](./SECURITY_CODEQL.md) - Security analysis framework documentation
- [ROADMAP.md](./ROADMAP.md) - Project development roadmap
- [CHANGELOG.md](./CHANGELOG.md) - Version history and changes
- [TODO.md](./TODO.md) - Current development tasks
- [AI Labeling System](./docs/standards/AI_LABELING.md) - AI labeling system documentation

## Installation & Deployment

- [Installation Guide](INSTALLATION.md) - Complete installation instructions
- [Production Setup](docs/deployment/production-setup.md) - Advanced deployment scenarios

## Architecture Documentation

- [ML System Architecture](docs/ML_SYSTEM_ARCHITECTURE.md) - Machine Learning system with Agent Checker (AIP-002)
- [Security Architecture](docs/SECURITY_ARCHITECTURE.md) - Security system with System Hardening (AIE-001)
- [Performance Architecture](docs/PERFORMANCE_ARCHITECTURE.md) - Performance system with Optimization (AIR-008)
- [Core System Integration](docs/CORE_SYSTEM_INTEGRATION.md) - Integration of all P1 components

## System Components

### Bitcoin & Lightning 🔄 IN DEVELOPMENT

- [Bitcoin Integration](anya-bitcoin/) - **ACTIVE DEVELOPMENT** Bitcoin protocol implementation
  - 🔄 **Core Modules**: Well-structured implementation in progress
  - 🔄 **P2P Networking**: Network infrastructure development
  - 🔄 **Mempool Management**: Transaction pool implementation ongoing
  - 🔄 **Consensus Validation**: BIP-341 (Taproot) development active
  - ✅ **Error Handling**: Comprehensive AnyaError system implemented
  - [Bitcoin Error Types](anya-bitcoin/src/core/error.rs) - Comprehensive error handling
  - [P2P Implementation](anya-bitcoin/src/core/network/) - Networking development
  - [Mempool System](anya-bitcoin/src/core/mempool/) - Transaction pool development
  - [Consensus Engine](anya-bitcoin/src/core/consensus/) - Validation development
  - [Script Interpreter](anya-bitcoin/src/core/script/) - Bitcoin script execution
  - [Taproot Support](anya-bitcoin/src/core/taproot.rs) - BIP-341 implementation
- [Layer2 Protocols](anya-bitcoin/src/layer2/) - **MIXED PROGRESS**
  - 🔄 [BOB Protocol](anya-bitcoin/src/layer2/bob/) - Interface defined, implementation ongoing
  - 🔄 [Lightning Network](anya-bitcoin/src/layer2/lightning/) - Basic structure implemented
  - 🔄 [RSK Integration](anya-bitcoin/src/layer2/rsk/) - Bridge interface design
  - 🔄 [RGB Protocol](anya-bitcoin/src/layer2/rgb/) - Asset management framework
  - 🔄 [DLC Implementation](anya-bitcoin/src/layer2/dlc/) - Oracle interface defined
  - 🔄 [Taproot Assets](anya-bitcoin/src/layer2/taproot_assets/) - Asset issuance planning
- [Security Framework](anya-bitcoin/src/security/) - Validation and monitoring

### DAO & Tokenomics

- [DAO Architecture](src/dao/README.md) - Detailed architecture of the DAO
- [Bitcoin-Style Tokenomics](docs/TOKENOMICS_SYSTEM.md) - 21B token supply with halving
- [DAO Implementation Status](docs/IMPLEMENTATION_MILESTONES.md) - Current progress
- [DAO Component Reference](docs/DAO_INDEX.md) - Index of all DAO components

### Web5 & Decentralized Identity

- [Web5 Integration](src/web5/README.md) - Web5 implementation details
- [DID System](docs/identity/README.md) - Decentralized identity implementation

### AI & Machine Learning Components

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
  - Block interfaces (`src/ports/block.rs`)
  - Transaction interfaces (`src/ports/transaction.rs`)
  - Network interfaces (`src/ports/network.rs`)
- **Adapter Layer**: Adapters to external dependencies
  - Protocol adapters (`src/adapters/protocols/`)
  - RPC adapters (`src/adapters/rpc/`)
  - Storage adapters (`src/adapters/storage/`)
- **Core Domain Logic**: Bitcoin-specific business logic
  - Consensus rules (`src/core/consensus/`)
  - Mempool management (`src/core/mempool/`)
  - Transaction validation (`src/core/transaction/`)
  - Protocol implementation (`src/protocols/`)
  - BIP implementations (`src/bip/`)
- **Error Handling**: Comprehensive error types and propagation
  - Bitcoin-specific errors (`src/core/error.rs`)
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

- Bitcoin Core support (in development)
- Lightning Network capabilities (in development)
- DLC (Discreet Log Contracts) (in development)
- Taproot/Schnorr signatures (in development)

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
- **Last Updated**: June 10, 2025
- **Compatibility**:
  - Stacks v2.4
  - Web5 Protocol v1.0
  - Bitcoin Core 24.0+

## Support & Community

- [GitHub Repository](https://github.com/anya/anya-core)
- [Issue Tracker](https://github.com/anya/anya-core/issues)
- [GitHub Discussions](https://github.com/anya/anya-core/discussions)

*This documentation follows the [AI Labeling Standards](docs/standards/AI_LABELING.md) based on official Bitcoin Improvement Proposals (BIPs). All components are labeled with appropriate Core and Extended category labels.*

## Bitcoin Protocol Implementation

- [Bitcoin Implementation](src/bitcoin/) - Bitcoin protocol implementation
- [BIP Compliance Matrix](docs/BIP_COMPLIANCE.md) - BIP-341/342 implementation status
  - [BIP-341 (Taproot)](core/src/bip/bip341.rs) - Taproot implementation in progress
  - [BIP-342 (Tapscript)](core/src/bip/bip342.rs) - Tapscript implementation in progress
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

## Official Bitcoin Improvement Proposals (BIPs) Compliance

All components conform to official Bitcoin Improvement Proposals (BIPs) and include proper AI labeling according to the following system:

- [AIR-3] - AI Readiness Level 3
- [AIS-3] - AI Security Level 3
- [BPC-3] - Bitcoin Protocol Compliance Level 3
- [AIT-3] - AI Testing Level 3
- [RES-3] - Resilience Level 3

For full details on the AI labeling system, see [docs/standards/AI_LABELING.md](./docs/standards/AI_LABELING.md).

## Version Information

- Current Version: 3.1.2
- Last Updated: June 10, 2025
- Bitcoin Improvement Proposals: Official BIPs

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

*Last updated: June 10, 2025*

<!-- AI Labeling references -->
[AIR-3]: ./docs/standards/AI_LABELING.md
[AIS-3]: ./docs/standards/AI_LABELING.md
[BPC-3]: ./docs/standards/AI_LABELING.md
[AIT-3]: ./docs/standards/AI_LABELING.md
[RES-3]: ./docs/standards/AI_LABELING.md
