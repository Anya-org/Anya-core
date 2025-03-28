#!/bin/bash
# Anya Core Unified Installer Wrapper Script
# [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
#
# This script acts as a wrapper for the Rust-based unified installer,
# allowing for easy installation on Linux and macOS systems.

set -e

# Terminal colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script variables
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
INSTALLER_BIN="${PROJECT_ROOT}/target/release/unified_installer"
INSTALLER_SRC="${PROJECT_ROOT}/src/bin/unified_installer.rs"
DEFAULT_INSTALL_PATH="/opt/anya-core"
LOG_FILE="${PROJECT_ROOT}/logs/installer.log"

# Function to print status messages
log() {
    local level=$1
    shift
    local message="$@"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    
    case $level in
        "info")
            echo -e "${BLUE}[INFO]${NC} $message"
            ;;
        "success")
            echo -e "${GREEN}[SUCCESS]${NC} $message"
            ;;
        "warning")
            echo -e "${YELLOW}[WARNING]${NC} $message"
            ;;
        "error")
            echo -e "${RED}[ERROR]${NC} $message"
            ;;
        *)
            echo -e "$message"
            ;;
    esac
    
    echo "[${timestamp}] [${level^^}] $message" >> "$LOG_FILE"
}

# Function to display help
show_help() {
    echo "Anya Core Unified Installer"
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  -h, --help                  Show this help message"
    echo "  -p, --path PATH             Installation path (default: $DEFAULT_INSTALL_PATH)"
    echo "  -m, --mode MODE             Installation mode (development or production)"
    echo "  -c, --components COMPONENTS Comma-separated list of components to install"
    echo "  --profile PROFILE           Installation profile (minimal, standard, full, enterprise)"
    echo "  -v, --verify                Only verify system requirements"
    echo "  -r, --report                Generate detailed installation report"
    echo "  --rpc-endpoint URL          Custom Bitcoin RPC endpoint"
    echo "  --skip-dependencies         Skip dependency installation"
    echo "  --verbose                   Verbose output"
    echo ""
    echo "Examples:"
    echo "  $0 --path ~/anya-core --mode development"
    echo "  $0 --profile enterprise --components core,bitcoin,dao,web5,ml"
    echo "  $0 --verify --verbose"
    echo ""
}

# Function to ensure Rust is installed
ensure_rust() {
    if ! command -v rustc &> /dev/null; then
        log "info" "Rust not detected, installing..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        log "success" "Rust installed successfully"
    else
        log "info" "Rust already installed: $(rustc --version)"
    fi
}

# Function to build the installer if needed
build_installer() {
    if [ ! -f "$INSTALLER_BIN" ] || [ "$INSTALLER_SRC" -nt "$INSTALLER_BIN" ]; then
        log "info" "Building installer..."
        mkdir -p "$(dirname "$INSTALLER_BIN")"
        mkdir -p "$(dirname "$LOG_FILE")"
        
        # Check if we have all required dependencies
        if ! command -v cargo &> /dev/null; then
            ensure_rust
        fi
        
        # Build the installer
        (cd "$PROJECT_ROOT" && cargo build --release --bin unified_installer)
        
        if [ ! -f "$INSTALLER_BIN" ]; then
            log "error" "Failed to build the installer"
            exit 1
        fi
        
        log "success" "Installer built successfully"
    else
        log "info" "Using existing installer binary"
    fi
}

# Function to check if running as root and prompt for sudo if needed
check_permissions() {
    local install_path=$1
    
    # Check if the installation path requires root
    if [[ "$install_path" == "/opt/"* || "$install_path" == "/usr/"* ]]; then
        if [ "$EUID" -ne 0 ]; then
            log "warning" "Installing to $install_path requires root privileges"
            log "info" "Please run this script with sudo or specify a different installation path"
            exit 1
        fi
    fi
}

# Parse command line arguments
INSTALL_PATH=$DEFAULT_INSTALL_PATH
INSTALLER_ARGS=()

while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        -h|--help)
            show_help
            exit 0
            ;;
        -p|--path)
            INSTALL_PATH="$2"
            INSTALLER_ARGS+=("--path" "$2")
            shift 2
            ;;
        -m|--mode)
            INSTALLER_ARGS+=("--mode" "$2")
            shift 2
            ;;
        -c|--components)
            INSTALLER_ARGS+=("--components" "$2")
            shift 2
            ;;
        --profile)
            INSTALLER_ARGS+=("--profile" "$2")
            shift 2
            ;;
        -v|--verify)
            INSTALLER_ARGS+=("--verify-only")
            shift
            ;;
        -r|--report)
            INSTALLER_ARGS+=("--report")
            shift
            ;;
        --rpc-endpoint)
            INSTALLER_ARGS+=("--rpc-endpoint" "$2")
            shift 2
            ;;
        --skip-dependencies)
            INSTALLER_ARGS+=("--skip-dependencies")
            shift
            ;;
        --verbose)
            INSTALLER_ARGS+=("--verbose")
            shift
            ;;
        *)
            log "error" "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Check if we have permission to install to the specified path
check_permissions "$INSTALL_PATH"

# Ensure logs directory exists
mkdir -p "$(dirname "$LOG_FILE")"

# Log start of installation
log "info" "Starting Anya Core installation"
log "info" "Installation path: $INSTALL_PATH"

# Build the installer if needed
build_installer

# Run the installer
log "info" "Running installer with args: ${INSTALLER_ARGS[*]}"
"$INSTALLER_BIN" "${INSTALLER_ARGS[@]}"

exit_code=$?
if [ $exit_code -eq 0 ]; then
    log "success" "Installation completed successfully"
else
    log "error" "Installation failed with exit code $exit_code"
    exit $exit_code
fi 