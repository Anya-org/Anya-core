#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# Anya Core Installation Script
# Following official Bitcoin Improvement Proposals (BIPs)
# Date: 2025-05-21

set -euo pipefail

# Script version
VERSION="1.0.0"

# Default values
NETWORK="testnet"
INSTALL_TYPE="standard"
DRY_RUN=false

# Error handling
handle_error() {
    local line_number=$1
    local error_code=${2:-1}
    echo "[ERROR] An error occurred at line ${line_number}: ${BASH_COMMAND}"
    exit "${error_code}"
}

trap 'handle_error $LINENO' ERR

# Show help
show_help() {
    cat << EOF
Anya Core Installation Script v${VERSION}

Usage: $0 [OPTIONS]

Options:
  --network=NETWORK    Network to connect to (mainnet, testnet, regtest, signet)
  --type=TYPE         Installation type (standard, minimal, full)
  --dry-run           Simulate installation without making changes
  --help              Show this help message
  --version           Show version information

Examples:
  # Standard installation for testnet
  sudo $0 --network=testnet

  # Full installation for mainnet with dry run
  sudo $0 --network=mainnet --type=full --dry-run

  # Minimal installation for development
  sudo $0 --network=regtest --type=minimal
EOF
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --network=*)
                NETWORK="${1#*=}"
                shift
                ;;
            --type=*)
                INSTALL_TYPE="${1#*=}"
                shift
                ;;
            --dry-run)
                DRY_RUN=true
                shift
                ;;
            --help)
                show_help
                exit 0
                ;;
            --version)
                echo "Anya Core Installation Script v${VERSION}"
                exit 0
                ;;
            *)
                echo "[ERROR] Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done

    # Validate network
    case "$NETWORK" in
        mainnet|testnet|regtest|signet)
            # Valid network, continue
            ;;
        *)
            echo "[ERROR] Invalid network: $NETWORK. Must be one of: mainnet, testnet, regtest, signet"
            exit 1
            ;;
    esac

    # Validate installation type
    case "$INSTALL_TYPE" in
        minimal|standard|full)
            # Valid type, continue
            ;;
        *)
            echo "[ERROR] Invalid installation type: $INSTALL_TYPE. Must be one of: minimal, standard, full"
            exit 1
            ;;
    esac
}

# Main function
main() {
    echo "=== Anya Core Installation ==="
    echo "Version: ${VERSION}"
    echo "Network: ${NETWORK}"
    echo "Type: ${INSTALL_TYPE}"
    echo "Dry Run: ${DRY_RUN}"
    echo "============================"
    echo

    # Check if running as root
    if [ "$(id -u)" -ne 0 ]; then
        echo "[ERROR] This script must be run as root"
        exit 1
    fi

    # Define directories
    ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    INSTALL_DIR="${ROOT_DIR}/scripts/install"

    # Check if install directory exists
    if [ ! -d "$INSTALL_DIR" ]; then
        echo "[ERROR] Installation directory not found: $INSTALL_DIR"
        exit 1
    fi

    # Execute the main installer with all parameters
    if [ -f "${INSTALL_DIR}/main_installer.sh" ]; then
        echo "[INFO] Starting Anya Core installation..."
        
        # Build the command with all parameters
        local cmd="${INSTALL_DIR}/main_installer.sh"
        cmd+=" --network=${NETWORK}"
        cmd+=" --type=${INSTALL_TYPE}"
        
        if [ "$DRY_RUN" = true ]; then
            cmd+=" --dry-run"
        fi
        
        # Execute the command
        if bash -c "$cmd"; then
            echo "[SUCCESS] Installation completed successfully"
            exit 0
        else
            echo "[ERROR] Installation failed with status code $?"
            exit 1
        fi
    else
        echo "[ERROR] Main installer script not found: ${INSTALL_DIR}/main_installer.sh"
        exit 1
    fi
}

# Parse command line arguments
parse_args "$@"

# Execute main function
main

exit 0
