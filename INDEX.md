# Anya Core Documentation

Welcome to the Anya Core documentation. This index will help you navigate the various documentation files and resources available for the Anya Core platform.

## Quick Start

- [Getting Started](docs/getting-started/README.md) - Quick setup guide
- [Installation](docs/installation/README.md) - Installation instructions
- [README](README.md) - Main project readme with overview and features

## Core Documentation

- [Documentation Index](docs/INDEX.md) - Main documentation index
- [System Architecture](docs/SYSTEM_MAP.md) - Complete system architecture
- [DAO System](docs/DAO_SYSTEM_GUIDE.md) - Comprehensive DAO documentation
- [Tokenomics System](docs/TOKENOMICS_SYSTEM.md) - Bitcoin-style tokenomics
- [Implementation Milestones](docs/IMPLEMENTATION_MILESTONES.md) - Current progress tracking
- [Testing Strategy](docs/TESTING_STRATEGY.md) - Sectional testing methodology
- [README.md](./README.md) - Main project documentation
- [SECURITY_CODEQL.md](./SECURITY_CODEQL.md) - Security analysis framework documentation
- [ROADMAP.md](./ROADMAP.md) - Project development roadmap
- [CHANGELOG.md](./CHANGELOG.md) - Version history and changes
- [TODO.md](./TODO.md) - Current development tasks
- [SYSTEM_MAP.md](./SYSTEM_MAP.md) - System architecture overview
- [docs/standards/AI_LABELING.md](./docs/standards/AI_LABELING.md) - AI labeling system documentation

## Installation & Deployment

- [Installation Guide](docs/installation/README.md) - Complete installation instructions
- [Installer Architecture](docs/installation/ARCHITECTURE.md) - Installation system design
- [Configuration Guide](docs/installation/CONFIGURATION.md) - Configuration options
- [Advanced Deployment](docs/installation/ADVANCED_DEPLOYMENT.md) - Advanced deployment scenarios

## Architecture Documentation

- [ML System Architecture](docs/ML_SYSTEM_ARCHITECTURE.md) - Machine Learning system with Agent Checker (AIP-002)
- [Security Architecture](docs/SECURITY_ARCHITECTURE.md) - Security system with System Hardening (AIE-001)
- [Performance Architecture](docs/PERFORMANCE_ARCHITECTURE.md) - Performance system with Optimization (AIR-008)
- [Core System Integration](docs/CORE_SYSTEM_INTEGRATION.md) - Integration of all P1 components

## System Components

### Bitcoin & Lightning

- [Bitcoin Integration](docs/bitcoin/README.md) - Bitcoin protocol features
- [Lightning Integration](docs/lightning/README.md) - Lightning Network features

### DAO & Tokenomics

- [DAO Architecture](docs/DAO_SYSTEM_MAP.md) - Detailed architecture of the DAO
- [Bitcoin-Style Tokenomics](docs/TOKENOMICS_SYSTEM.md) - 21B token supply with halving
- [DAO Implementation Status](docs/IMPLEMENTATION_MILESTONES.md) - Current progress
- [DAO Component Reference](docs/DAO_INDEX.md) - Index of all DAO components

### Web5 & Decentralized Identity

- [Web5 Integration](docs/web5/README.md) - Web5 implementation details
- [DID System](docs/identity/README.md) - Decentralized identity implementation

### AI & Machine Learning

- [ML System Architecture](docs/ML_SYSTEM_ARCHITECTURE.md) - ML system design with Agent Checker
- [AI Component Reference](docs/ml/README.md) - AI component details

### Installation System

- [Installer Core](docs/installation/CORE.md) - Core installation system
- [Virtual Environment](docs/installation/VENV.md) - Virtual environment management
- [Component Installation](docs/installation/COMPONENTS.md) - Component installation procedures
- [Deployment Types](docs/installation/DEPLOYMENT_TYPES.md) - Deployment configurations

## Development Resources

- [API Documentation](docs/API.md) - API reference
- [Contributing Guide](CONTRIBUTING.md) - How to contribute
- [Checkpoint System](docs/CHECKPOINT_SYSTEM.md) - Development checkpoints
- [](docs/standards/AI_LABELING.md) - Component labeling standards

## Key Features at a Glance

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

- [Security Guidelines](docs/SECURITY.md) - Security best practices
- [Compliance Framework](docs/COMPLIANCE.md) - Compliance information
- [Privacy Measures](docs/PRIVACY.md) - Privacy protection measures

## Release Information

- **Current Version**: 3.1.0
- **Last Updated**: 2025-03-12
- **Compatibility**:
  - Stacks v2.4
  - Web5 Protocol v1.0
  - Bitcoin Core 24.0+

## Support & Community

- [GitHub Repository](https://github.com/anya/anya-core)
- [Issue Tracker](https://github.com/anya/anya-core/issues)
- [GitHub Discussions](https://github.com/anya/anya-core/discussions)

*This documentation follows the [](docs/standards/AI_LABELING.md) standards based on the Bitcoin Development Framework v2.5. All components are labeled with appropriate Core and Extended category labels.*

## Bitcoin Protocol Implementation

- [scripts/bitcoin/mcp-server.js](./scripts/bitcoin/mcp-server.js) - MCP server implementation
- [scripts/bitcoin/validate-bip-compliance.js](./scripts/bitcoin/validate-bip-compliance.js) - BIP compliance validation
- [scripts/bitcoin/validate-security.js](./scripts/bitcoin/validate-security.js) - Bitcoin security validation

## Security Analysis Components

- [scripts/security/run-codeql-analysis.ps1](./scripts/security/run-codeql-analysis.ps1) - CodeQL analysis script
- [scripts/security/crypto-validation.js](./scripts/security/crypto-validation.js) - Cryptographic validation
- [scripts/security/analyze-mcp-server.js](./scripts/security/analyze-mcp-server.js) - MCP server analysis
- [scripts/security/setup-permissions.sh](./scripts/security/setup-permissions.sh) - Security permissions setup

## Development Scripts

- [scripts/README.md](./scripts/README.md) - Overview of available scripts
- [scripts/build.ps1](./scripts/build.ps1) - Build script for the project
- [scripts/dev-setup.ps1](./scripts/dev-setup.ps1) - Development environment setup

## Testing Framework

- [scripts/run-all-tests.sh](./scripts/run-all-tests.sh) - Run all test suites
- [scripts/run-integration-tests.ps1](./scripts/run-integration-tests.ps1) - Integration tests
- [scripts/run-module-tests.ps1](./scripts/run-module-tests.ps1) - Module-specific tests

## Compliance and Governance

- [GOVERNANCE.md](./GOVERNANCE.md) - Project governance structure
- [CONTRIBUTING.md](./CONTRIBUTING.md) - Contribution guidelines
- [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md) - Community code of conduct
- [LICENSE.md](./LICENSE.md) - License information

## Bitcoin Development Framework Compliance

All components conform to the Bitcoin Development Framework v2.5 guidelines and include proper AI labeling according to the following system:

- [AIR-3] - AI Readiness Level 3
- [AIS-3] - AI Security Level 3
- [BPC-3] - Bitcoin Protocol Compliance Level 3
- [AIT-3] - AI Testing Level 3
- [RES-3] - Resilience Level 3

For full details on the AI labeling system, see [docs/standards/AI_LABELING.md](./docs/standards/AI_LABELING.md).

## Version Information

- Current Version: 0.9.5
- Last Updated: March 16, 2025
- Bitcoin Development Framework: v2.5
