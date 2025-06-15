#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# Anya Core Installation Script (Upgraded for rootless/sandboxed install)
# Date: 2025-06-15

set -euo pipefail

# Script version
VERSION="1.1.0"

# Default values
NETWORK="testnet"
INSTALL_TYPE="standard"
DRY_RUN=false
ROOTLESS=false
AUTO_RUN=false
YES_ALL=false

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
  --rootless|--sandbox  Install in user home, no root/systemd required
  --auto-run          Auto install all dependencies and run all steps
  --yes-all           Assume yes to all prompts
  --help              Show this help message
  --version           Show version information

Examples:
  # Standard installation for testnet
  sudo $0 --network=testnet

  # Full installation for mainnet with dry run
  sudo $0 --network=mainnet --type=full --dry-run

  # Minimal installation for development
  sudo $0 --network=regtest --type=minimal

  # Rootless installation with automatic dependency installation
  $0 --network=testnet --rootless --auto-run --yes-all
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
            --rootless|--sandbox)
                ROOTLESS=true
                shift
                ;;
            --auto-run)
                AUTO_RUN=true
                shift
                ;;
            --yes-all)
                YES_ALL=true
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

# Dependency installer (rootless aware)
install_deps() {
    echo "[INFO] Checking/installing dependencies..."
    # Rust
    if ! command -v cargo >/dev/null 2>&1; then
        echo "[INFO] Installing Rust toolchain (user-local)..."
        curl https://sh.rustup.rs -sSf | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    # Node.js
    if ! command -v node >/dev/null 2>&1; then
        echo "[INFO] Installing Node.js (user-local via nvm)..."
        if ! command -v nvm >/dev/null 2>&1; then
            curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
            export NVM_DIR="$HOME/.nvm"
            [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
        fi
        nvm install --lts
    fi
    # Docker (optional, only if monitoring is requested)
    if [ "$AUTO_RUN" = true ]; then
        if ! command -v docker >/dev/null 2>&1; then
            echo "[WARN] Docker not found. Monitoring stack will not be available unless Docker is installed."
        fi
    fi
    # Python (for scripts)
    if ! command -v python3 >/dev/null 2>&1; then
        echo "[WARN] Python3 not found. Some scripts may not work."
    fi
    # npm packages (user-local)
    if [ -f "dependencies/install_dependencies.sh" ]; then
        bash dependencies/install_dependencies.sh
    fi
}

# Main function
main() {
    echo "=== Anya Core Installation (Upgraded) ==="
    echo "Version: ${VERSION}"
    echo "Network: ${NETWORK}"
    echo "Type: ${INSTALL_TYPE}"
    echo "Dry Run: ${DRY_RUN}"
    echo "Rootless: ${ROOTLESS}"
    echo "Auto Run: ${AUTO_RUN}"
    echo "Yes All: ${YES_ALL}"
    echo "============================"
    echo

    # If not root and not rootless, warn and auto-switch
    if [ "$(id -u)" -ne 0 ] && [ "$ROOTLESS" = false ]; then
        echo "[WARN] Not running as root. Switching to rootless/sandboxed install."
        ROOTLESS=true
    fi

    # Set install dir and main_installer path
    if [ "$ROOTLESS" = true ]; then
        export INSTALL_DIR="$HOME/.anya-core"
        mkdir -p "$INSTALL_DIR"
        MAIN_INSTALLER_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")/scripts/install" && pwd)/main_installer.sh"
    else
        export INSTALL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/scripts/install"
        MAIN_INSTALLER_PATH="${INSTALL_DIR}/main_installer.sh"
    fi

    # Install dependencies
    if [ "$AUTO_RUN" = true ]; then
        install_deps
    fi

    # Call main_installer.sh with all relevant flags
    local cmd="$MAIN_INSTALLER_PATH --network=${NETWORK} --type=${INSTALL_TYPE}"
    [ "$DRY_RUN" = true ] && cmd+=" --dry-run"
    [ "$ROOTLESS" = true ] && cmd+=" --rootless"
    [ "$AUTO_RUN" = true ] && cmd+=" --auto-run"
    [ "$YES_ALL" = true ] && cmd+=" --yes-all"
    echo "[INFO] Running: $cmd"
    if bash -c "$cmd"; then
        echo "[SUCCESS] Installation completed successfully"
        exit 0
    else
        echo "[ERROR] Installation failed with status code $?"
        exit 1
    fi
}

# Parse command line arguments
parse_args "$@"

# Execute main function
main

exit 0
