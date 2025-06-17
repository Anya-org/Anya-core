# Anya Core Documentation [AIR-3][AIS-3][BPC-3]

[AIR-3][AIS-3][BPC-3][RES-3]

*Last Updated: June 17, 2025*

Welcome to the Anya Core documentation. This is the authoritative root index for all Anya Core documentation, following official Bitcoin Improvement Proposals (BIPs).

> **Note**: This documentation has been cleaned up, consolidated, and corrected. Outdated timestamps have been updated, duplicate structures have been removed, and inaccurate claims about "production readiness" have been corrected for improved clarity and honest assessment of development status.

## Recent Updates (June 17, 2025) [AIR-3][AIS-3][BPC-3][RES-3]

**🔄 BRANCH CONSOLIDATION COMPLETED**

Branch management and documentation accuracy improvements have been implemented:

### 📋 Documentation Cleanup & Truth Verification ✅

- **Timestamp Updates**: All documentation updated to reflect current date (June 17, 2025)
- **Truth Verification**: Corrected misleading "production-ready" claims with accurate development status
- **Project Status**: Realistic assessment of implementation progress across all components
- **Link Validation**: Ensured all internal documentation links remain functional
- **Backup Removal**: Cleaned up accumulated backup files and old configuration archives

### 🛠️ MCP Tools Integration ✅ **[NEW]**

- **Comprehensive MCP Toolbox**: Integrated 9 MCP servers for enhanced development
- **Custom Development Tools**: Created `anya-dev-tools.js` MCP server with 8 specialized tools
- **Management Scripts**: Complete MCP lifecycle management with `manage-tools.sh`
- **IDE Integration Ready**: Generated configuration files for seamless editor integration
- **Automated Workflows**: Project analysis, testing, compliance, and optimization tools

### 🚀 Enhanced Development Capabilities

- **Memory-Enhanced Development**: Context retention across development sessions
- **Automated Project Analysis**: Comprehensive repository insights and recommendations
- **GitHub Integration**: Seamless repository management and automation
- **Bitcoin Protocol Tools**: Advanced Taproot and Schnorr signature support
- **Quality Assurance**: Automated documentation validation and compliance checking

### 🔄 Branch Management & Consolidation (June 17, 2025)

- **PR #44 Merge**: Consolidated all improvements from dev and feature branches
- **GitHub Actions**: Updated all workflow files to use latest action versions
- **Rust Fixes**: Resolved compilation errors and test failures
- **DevContainer**: Enhanced configuration with comprehensive tool installation
- **Documentation**: Corrected inaccurate status claims and updated dates

### Previous System Improvements (May 29, 2025)

### Core System Improvements

- **Dependency Management**: All Cargo.toml dependencies pinned to specific versions for enhanced stability
- **Import Path Migration**: Successfully migrated from `anya_bitcoin` to `anya_core` throughout the codebase
- **Function Signature Fixes**: Removed duplicate return type annotations and malformed signatures
- **Build System Cleanup**: Streamlined build.rs configuration, removed redundant OS-specific flags

### Bitcoin Protocol Enhancements [BPC-3]

- **HSM Module Restructuring**: Fixed provider imports, corrected type definitions, resolved module conflicts
- **Secp256k1 Updates**: Upgraded to version 0.29.1 with proper API structure alignment
- **Bitcoin Protocol Testing**: Enhanced testing modules with proper error handling and BIP compliance
- **Taproot Implementation**: Improved asset creation and script generation functionality

### Security & Documentation [AIS-3]

- **AI Labeling Compliance**: Added comprehensive AI labeling tags to all documentation files
- **Documentation Enhancement**: Improved formatting and structure across all markdown files
- **Security Module Updates**: Fixed HSM provider imports and enhanced error handling
- **Trait Implementations**: Added missing trait methods and fixed trait object boxing issues

### Development Tools & APIs

- **Binary Executables**: Updated all binary files with correct import paths and enhanced error handling
- **API Structure**: Created new API handlers and server modules with proper organization
- **Tools Module**: Implemented markdown validation and commit tracking utilities
- **Module Organization**: Enhanced module structure with proper visibility and exports

All modules maintain compliance with official Bitcoin Improvement Proposals (BIPs) and include proper AI labeling according to the canonical AI Labeling System. For detailed information about these changes, see the [Changelog](CHANGELOG.md) and [Implementation Summary](docs/IMPLEMENTATION_SUMMARY.md).

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

### MCP Tools & Development [MCP-3] **[NEW]**

- [MCP Tools Overview](mcp/toolbox/README.md) - Comprehensive MCP integration guide
- [MCP Configuration](mcp/mcp.json) - Main MCP server configuration
- [Development Tools](mcp/toolbox/servers/anya-dev-tools.js) - Custom Anya development server
- [Management Scripts](scripts/mcp/manage-tools.sh) - MCP lifecycle management
- [Setup Guide](scripts/mcp/init-toolbox.sh) - MCP toolbox initialization
- [Tools Configuration](mcp/toolbox/mcp-tools-config.json) - Complete tools specification

## System Status

### Core Protocol Status [BPC-3]

- Bitcoin Core: v25.0 (integration in progress)
- Taproot: Implementation underway (BIP-341/342) - ~95% complete
- PSBT: v2 support (BIP-174/370) - ~85% complete
- HSM: v2.5 integration - in development

### Layer 2 Integration Status [BPC-3]

- Lightning Network: In development (src/lightning)
- RGB Protocol: Planned Q3 2025 (src/layer2/rgb)
- RSK Integration: Planned Q3 2025 (src/layer2/rsk)
- BOB Layer 2: In testing (src/layer2/bob)
- State Channels: Framework defined (src/layer2/state_channels)

### Security Compliance [AIS-3]

- SILENT_LEAF Implementation: In testing
- Taproot Script Validation: In development
- Schnorr Signature Support: ~90% complete
- HSM Integration: In progress

### Web5 Components [W5C-3]

- DWN Implementation: In development
- DID Support: Basic framework defined
- Verifiable Credentials: In development
- BIP-341 Integration: In progress

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

- Version: 1.2.0-dev
- Last Updated: 2025-06-17
- Framework: Bitcoin Improvement Proposals (BIPs)

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
*Last updated: 2025-06-17 14:30 UTC+2*
*All components comply with [AI Labeling Standards](docs/standards/AI_LABELING.md)*
