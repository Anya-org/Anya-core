# Anya Core Installation System

This directory contains the Anya Core installation system scripts. This system has been designed to provide a unified installation experience with automatic hardware detection, appropriate feature flag configuration, and support for both fresh installations and upgrades.

## System Components

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
# Basic installation
sudo ./scripts/install.sh

# Minimal installation for low-resource environments
sudo ./scripts/install.sh --type=minimal

# Full installation with strict security
sudo ./scripts/install.sh --type=full --hardening=strict

# Upgrade an existing installation
sudo ./scripts/install.sh

# Force clean installation (removes existing installation)
sudo ./scripts/install.sh --force-clean

# Run tests after installation
sudo ./scripts/install.sh --run-tests
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

## Architecture

The installation system follows a modular design:

1. **Entry Point**: `install.sh` provides a clean, unified entry point
2. **System Analysis**: `auto_install.sh` analyzes the system and configures appropriately
3. **Core Installation**: `linux_install.sh` handles the core installation process
4. **Service Configuration**: `systemd_config.sh` configures the system service
5. **Utilities**: Common functions are centralized in `utils/install_common.sh`

This design ensures consistent behavior across different environments and simplifies maintenance.
