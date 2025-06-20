# Anya Core Installation System

# [AIR-3][AIS-3][BPC-3][RES-3][PFM-3]

*Last Updated: June 7, 2025*

**Latest Version: v0.3.0-rc2**

> **Note**: This version includes a complete refactor of the installation system following official Bitcoin Improvement Proposals (BIPs), with enhanced security features including HSM integration, Taproot support, and automated hardware detection.

This directory contains the Anya Core installation system scripts. This system has been designed to provide a unified installation experience with automatic hardware detection, appropriate feature flag configuration, and support for both fresh installations and upgrades. The system follows official Bitcoin Improvement Proposals (BIPs) and implements hexagonal architecture principles.

## System Components [AIR-3][AIS-3][BPC-3]

### Core Installation Scripts

- **main_installer.sh** - Comprehensive installation framework following BDF v2.5 standards
- **unified_install_framework.sh** - Unified installation framework with hardware detection
- **systemd_config.sh** - Systemd service configuration and management
- **dashboard.sh** - System monitoring and status dashboard
- **utils/** - Directory containing utility scripts and common functions

### Security Features [AIS-3][RES-3]

- Hardware-based security recommendations
- Automatic TPM attestation verification
- Secure key generation and management
- System hardening configurations

### Monitoring & Maintenance [RES-3]

- System health checks
- Performance monitoring
- Automatic updates and patches
- Logging and audit trails

## Installation Guide [BPC-3][UXA-2]

### Quick Start

```bash
# Clone the repository
git clone https://github.com/your-org/anya-core.git
cd anya-core

# Run the installer
./install.sh --network=testnet --type=standard
```

### Installation Options

The installation system supports various modes and configurations:

#### Installation Types

- **minimal**: Basic functionality with minimal resource usage (recommended for embedded systems)
- **standard**: Default installation with balanced features (recommended for most users)
- **full**: Complete installation with all features (for development and testing)

#### Network Options

- **mainnet**: Connect to Bitcoin mainnet (requires full node)
- **testnet**: Connect to Bitcoin testnet (recommended for development)
- **regtest**: Local regtest environment (for testing)
- **signet**: Bitcoin signet (for testing with real BTC)

### Security Hardening [AIS-3][RES-3]

#### Hardening Levels

- **basic**: Minimal security configuration (not recommended for production)
- **standard**: Balanced security with reasonable performance impact (default)
- **high**: Enhanced security with additional protections (recommended for production)
- **paranoid**: Maximum security with significant performance impact (for high-security environments)

#### Security Features

- Automatic firewall configuration
- Process isolation
- Memory protection
- Secure boot verification
- Hardware security module (HSM) integration
- **standard**: Default security configuration
- **strict**: Maximum security configuration

### Network Options

- **mainnet**: Bitcoin mainnet
- **testnet**: Bitcoin testnet (default)
- **regtest**: Bitcoin regression test network

### Hardware Detection Features

The installation system automatically detects available hardware and configures Anya Core accordingly:

- TPM modules
- Hardware Security Modules (HSM)
- YubiKeys and other PKCS#11 devices
- Hardware wallets (Ledger, Trezor)
- Secure elements

## Example Usage

```bash
# Using the unified installation framework
sudo ./scripts/install/unified_install_framework.sh

# Basic installation
sudo ./scripts/install.sh

# Minimal installation for low-resource environments
sudo ./scripts/install/unified_install_framework.sh --type=minimal

# Full installation with strict security
sudo ./scripts/install/unified_install_framework.sh --type=full --hardening=strict

# Upgrade an existing installation
sudo ./scripts/install/unified_install_framework.sh

# Force clean installation (removes existing installation)
sudo ./scripts/install/unified_install_framework.sh --force-clean

# Run tests after installation
sudo ./scripts/install/unified_install_framework.sh --run-tests
```

## Utilities

The installation system includes several utilities:

- **install_common.sh**: Common functions for all installation scripts
- **script_cleanup.sh**: Analyzes and manages redundant installation scripts

## Feature Flag Support

The installation system automatically configures feature flags based on available hardware:

- **hsm**: Enabled when hardware security devices are detected
- **bitcoin_integration**: Bitcoin integration features
- **complete**: All features (for full installations)

## Debugging and Monitoring

For debugging installation issues, use:

```bash
# Run the debug test script
./scripts/test/debug_test.sh

# Check installation system for redundant scripts
./scripts/install/utils/script_cleanup.sh
```

## Installation Logs

Installation logs are stored in:

```
./logs/installation/
```

## Hexagonal Architecture

The installation system follows the hexagonal architecture principles as defined in official Bitcoin Improvement Proposals (BIPs):

1. **Core Domain**:
   - `unified_install_framework.sh` provides the comprehensive installation framework
   - `linux_install.sh` handles the core installation process

2. **Ports**:
   - Installation interface through command-line arguments
   - System analysis interface for hardware detection
   - Configuration interface for feature flags

3. **Adapters**:
   - Network adapters for different Bitcoin networks (mainnet, testnet, regtest)
   - Hardware adapters for different security devices (TPM, HSM, hardware wallets)
   - System adapters for different environments (physical, virtualized)

4. **Utilities**:
   - Common functions are centralized in `utils/install_common.sh`
   - Script management utilities in `scripts/maintenance/script_manager.sh`

This hexagonal architecture design ensures:

- Clear separation of concerns
- Consistent behavior across different environments
- Simplified maintenance and extensibility
- Compliance with official Bitcoin Improvement Proposals (BIPs)

## AI Labeling Compliance

All installation scripts include proper AI labeling according to official Bitcoin Improvement Proposals (BIPs):

- [AIR-3] - AI Responsibility
- [AIS-3] - AI Security
- [BPC-3] - Bitcoin Protocol Compliance
- [RES-3] - Resource Efficiency
