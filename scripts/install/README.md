# Anya Core Installation System

# [AIR-3][AIS-3][BPC-3][RES-3]

Date: 2025-05-20

This directory contains the Anya Core installation system scripts. This system has been designed to provide a unified installation experience with automatic hardware detection, appropriate feature flag configuration, and support for both fresh installations and upgrades. The system follows the Bitcoin Development Framework v2.5 standards and implements hexagonal architecture principles.

## System Components

- **unified_install_framework.sh** - Comprehensive installation framework following BDF v2.5 standards
- **install.sh** - Main entry point for the installation system (at project root level)
- **auto_install.sh** - Automated installation with hardware detection and system analysis
- **linux_install.sh** - Core installation script for Linux systems
- **systemd_config.sh** - Systemd service configuration
- **uninstall.sh** - Clean uninstallation
- **utils/** - Directory containing utility scripts and common functions

## Installation Options

The installation system supports various modes and configurations:

### Installation Types

- **minimal**: Basic functionality with minimal resource usage
- **standard**: Default installation with balanced features
- **full**: Complete installation with all features

### Security Hardening Levels

- **basic**: Minimal security configuration
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

The installation system follows the hexagonal architecture principles as defined in the Bitcoin Development Framework v2.5:

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
- Compliance with Bitcoin Development Framework v2.5 standards

## AI Labeling Compliance

All installation scripts include proper AI labeling according to the Bitcoin Development Framework v2.5 standards:

- [AIR-3] - AI Responsibility
- [AIS-3] - AI Security
- [BPC-3] - Bitcoin Protocol Compliance
- [RES-3] - Resource Efficiency
