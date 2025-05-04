#!/bin/bash
# Anya Core Easy Installation Wrapper Script
# [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

set -euo pipefail

# Script version
VERSION="1.0.0"

# Find script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Default values
NETWORK="testnet"
INSTALL_TYPE="standard"
HARDENING_LEVEL="standard"
HELP=false
VERSION_ONLY=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --help|-h)
            HELP=true
            shift
            ;;
        --version|-v)
            VERSION_ONLY=true
            shift
            ;;
        --network=*)
            NETWORK="${1#*=}"
            if [[ ! "$NETWORK" =~ ^(mainnet|testnet|regtest)$ ]]; then
                echo "ERROR: Invalid network: $NETWORK. Must be mainnet, testnet, or regtest."
                exit 1
            fi
            shift
            ;;
        --type=*)
            INSTALL_TYPE="${1#*=}"
            if [[ ! "$INSTALL_TYPE" =~ ^(minimal|standard|full)$ ]]; then
                echo "ERROR: Invalid installation type: $INSTALL_TYPE. Must be minimal, standard, or full."
                exit 1
            fi
            shift
            ;;
        --hardening=*)
            HARDENING_LEVEL="${1#*=}"
            if [[ ! "$HARDENING_LEVEL" =~ ^(basic|standard|strict)$ ]]; then
                echo "ERROR: Invalid hardening level: $HARDENING_LEVEL. Must be basic, standard, or strict."
                exit 1
            fi
            shift
            ;;
        *)
            echo "ERROR: Unknown option: $1"
            HELP=true
            shift
            ;;
    esac
done

# Show help if requested
if [ "$HELP" = true ]; then
    echo "Anya Core Easy Installation Script v${VERSION}"
    echo ""
    echo "This script automatically installs Anya Core with system-optimized settings."
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --network=NETWORK    Specify network (mainnet, testnet, regtest)"
    echo "                       Default: $NETWORK"
    echo "  --type=TYPE          Installation type (minimal, standard, full)"
    echo "                       Default: $INSTALL_TYPE"
    echo "  --hardening=LEVEL    Security hardening level (basic, standard, strict)"
    echo "                       Default: $HARDENING_LEVEL"
    echo "  --help, -h           Display this help message"
    echo "  --version, -v        Display script version"
    echo ""
    echo "Examples:"
    echo "  sudo $0                                  # Standard installation"
    echo "  sudo $0 --network=mainnet --type=full   # Full installation for mainnet"
    echo "  sudo $0 --type=minimal                  # Minimal installation"
    echo ""
    echo "Installation Types:"
    echo "  - minimal: Basic functionality, lower resource usage"
    echo "  - standard: Balanced functionality and resource usage"
    echo "  - full: All features enabled, optimized for performance"
    echo ""
    echo "Hardening Levels:"
    echo "  - basic: Minimal security hardening"
    echo "  - standard: Balanced security measures"
    echo "  - strict: Maximum security measures"
    exit 0
fi

# Show version if requested
if [ "$VERSION_ONLY" = true ]; then
    echo "Anya Core Easy Installation Script v${VERSION}"
    exit 0
fi

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "ERROR: This script requires root privileges. Please run with sudo."
    exit 1
fi

# Print banner
echo "================================================================"
echo "        Anya Core System-Optimized Installation"
echo "================================================================"
echo
echo "Starting installation with the following settings:"
echo "- Network: $NETWORK"
echo "- Installation Type: $INSTALL_TYPE"
echo "- Security Hardening: $HARDENING_LEVEL"
echo
echo "This script will:"
echo "1. Analyze your system capabilities"
echo "2. Install dependencies"
echo "3. Build and configure Anya Core optimized for your system"
echo "4. Set up systemd service with resource limits"
echo "5. Start the service automatically"
echo
echo "The installation will run without requiring user input."
echo "Installation logs will be stored in the logs directory."
echo
echo "Press Ctrl+C now to abort, or wait 5 seconds to continue..."
sleep 5
echo "Starting installation..."
echo

# Construct and execute the auto_install command
AUTO_INSTALL_CMD="${SCRIPT_DIR}/auto_install.sh --network=${NETWORK} --type=${INSTALL_TYPE} --hardening=${HARDENING_LEVEL} --auto-run"

echo "Executing: $AUTO_INSTALL_CMD"
$AUTO_INSTALL_CMD

# Installation completed
echo
echo "================================================================"
echo "        Anya Core Installation Complete!"
echo "================================================================"
echo
echo "The system has been installed and configured based on your"
echo "hardware capabilities for optimal performance."
echo
echo "To check service status:"
echo "  sudo systemctl status anya-core"
echo
echo "To view logs:"
echo "  sudo journalctl -u anya-core -f"
echo
echo "To uninstall:"
echo "  sudo ${SCRIPT_DIR}/uninstall.sh --auto-run"
echo
echo "Thank you for installing Anya Core!"
echo "================================================================" 