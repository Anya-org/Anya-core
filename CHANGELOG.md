# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-02-27

### Added

- **Liquid Support**: Added comprehensive support for the Liquid sidechain
  - Bitcoin-Liquid bridge with SPV proofs
  - Asset issuance and management
  - Confidential transactions support
  - Integration with the cross-chain module
- **Web5 Module**: Complete implementation of the Web5 protocol
  - DID (Decentralized Identity) management
  - DWN (Decentralized Web Node) integration
  - Protocol handling for various Web5 protocols
  - Credential management
  - Secure messaging between DIDs
- **Configuration Updates**:
  - Added Liquid configuration options
  - Added Web5 configuration options
  - Enhanced feature flags system

### Changed

- **Cross-Chain Module**: Refactored to support multiple sidechains
  - Added support for RSK and Liquid
  - Improved transaction status tracking
  - Enhanced SPV proof verification
- **Bitcoin Module**: Updated to align with Bitcoin Development Framework v2.5
  - Added Taproot support
  - Enhanced DLC implementation
  - Improved cross-chain functionality

### Fixed

- Fixed compatibility issues with the latest Bitcoin Core
- Resolved issues with transaction signing
- Fixed configuration loading from environment variables

## [0.1.0] - 2025-02-20

### Added

- Initial release of the Anya Core framework
- Bitcoin module with basic functionality
- Lightning Network support using LDK
- DLC (Discrete Log Contracts) implementation
- Cross-chain functionality with RSK
- Wallet management with BIP39/44/84/86 support
- Hexagonal architecture with adapters and ports 