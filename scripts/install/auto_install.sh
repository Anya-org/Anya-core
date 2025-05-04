#!/bin/bash
# Anya Core Auto-Installation Master Script
# [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

set -euo pipefail

# Script version
VERSION="1.0.0"

# Directory setup
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
LOGS_DIR="${PROJECT_ROOT}/logs"
mkdir -p "$LOGS_DIR"

# Log file
INSTALL_LOG="${LOGS_DIR}/auto_install_$(date +%Y%m%d-%H%M%S).log"

# Default values
NETWORK="testnet"
START_SERVICE=true
INSTALL_DEPS=true
CONFIGURE_FIREWALL=true
INSTALL_TYPE="standard"  # standard, minimal, full
HARDENING_LEVEL="standard"  # basic, standard, strict
AUTO_RUN=false

# Function to log messages
log() {
    local level=$1
    shift
    local message=$*
    local timestamp
    timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    
    case $level in
        ERROR) printf "\033[0;31m[%s] ERROR: %s\033[0m\n" "$timestamp" "$message" | tee -a "$INSTALL_LOG" ;;
        WARN)  printf "\033[1;33m[%s] WARN: %s\033[0m\n" "$timestamp" "$message" | tee -a "$INSTALL_LOG" ;;
        INFO)  printf "\033[0;32m[%s] INFO: %s\033[0m\n" "$timestamp" "$message" | tee -a "$INSTALL_LOG" ;;
        DEBUG) printf "\033[0;34m[%s] DEBUG: %s\033[0m\n" "$timestamp" "$message" | tee -a "$INSTALL_LOG" ;;
    esac
    
    # Also log errors and warnings to stderr
    if [[ "$level" == "ERROR" || "$level" == "WARN" ]]; then
        printf "\033[0;31m[%s] %s: %s\033[0m\n" "$timestamp" "$level" "$message" >&2
    fi
}

# Setup error handling
cleanup() {
    local error_code=$?
    if [ $error_code -ne 0 ]; then
        log ERROR "Auto-installation failed with error code $error_code. Check log at $INSTALL_LOG"
    fi
    exit $error_code
}

# Set up trap for cleanup on script exit
trap cleanup EXIT INT TERM

# Check for root privileges
check_root() {
    if [ "$EUID" -ne 0 ]; then
        log ERROR "This script requires root privileges. Please run with sudo."
        exit 1
    fi
}

# Show help
show_help() {
    echo "Anya Core Auto-Installation Script v${VERSION}"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --network=NETWORK       Specify network (mainnet, testnet, regtest)"
    echo "  --no-start              Don't start the service after installation"
    echo "  --no-deps               Skip dependency installation"
    echo "  --no-firewall           Skip firewall configuration"
    echo "  --type=TYPE             Installation type (minimal, standard, full)"
    echo "  --hardening=LEVEL       Security hardening level (basic, standard, strict)"
    echo "  --auto-run              Automatically run with no prompts (non-interactive)"
    echo "  --help                  Display this help message"
    echo "  --version               Display script version"
    echo ""
    echo "Example:"
    echo "  sudo $0 --network=testnet --type=standard --auto-run"
}

# Show version
show_version() {
    echo "Anya Core Auto-Installation Script v${VERSION}"
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help)
                show_help
                exit 0
                ;;
            --version)
                show_version
                exit 0
                ;;
            --network=*)
                NETWORK="${1#*=}"
                if [[ ! "$NETWORK" =~ ^(mainnet|testnet|regtest)$ ]]; then
                    log ERROR "Invalid network: $NETWORK. Must be mainnet, testnet, or regtest."
                    exit 1
                fi
                shift
                ;;
            --no-start)
                START_SERVICE=false
                shift
                ;;
            --no-deps)
                INSTALL_DEPS=false
                shift
                ;;
            --no-firewall)
                CONFIGURE_FIREWALL=false
                shift
                ;;
            --type=*)
                INSTALL_TYPE="${1#*=}"
                if [[ ! "$INSTALL_TYPE" =~ ^(minimal|standard|full)$ ]]; then
                    log ERROR "Invalid installation type: $INSTALL_TYPE. Must be minimal, standard, or full."
                    exit 1
                fi
                shift
                ;;
            --hardening=*)
                HARDENING_LEVEL="${1#*=}"
                if [[ ! "$HARDENING_LEVEL" =~ ^(basic|standard|strict)$ ]]; then
                    log ERROR "Invalid hardening level: $HARDENING_LEVEL. Must be basic, standard, or strict."
                    exit 1
                fi
                shift
                ;;
            --auto-run)
                AUTO_RUN=true
                shift
                ;;
            *)
                log ERROR "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    log INFO "Auto-installation configured with:"
    log INFO "- Network: $NETWORK"
    log INFO "- Start Service: $START_SERVICE"
    log INFO "- Install Dependencies: $INSTALL_DEPS"
    log INFO "- Configure Firewall: $CONFIGURE_FIREWALL"
    log INFO "- Installation Type: $INSTALL_TYPE"
    log INFO "- Hardening Level: $HARDENING_LEVEL"
    log INFO "- Auto Run: $AUTO_RUN"
}

# Analyze system capabilities
analyze_system() {
    log INFO "Analyzing system capabilities..."
    
    # CPU analysis
    CPU_CORES=$(nproc)
    CPU_MODEL=$(grep "model name" /proc/cpuinfo | head -1 | cut -d':' -f2 | sed 's/^[ \t]*//' || echo "Unknown")
    CPU_ARCH=$(uname -m)
    
    # Memory analysis
    TOTAL_MEM=$(free -m | awk '/^Mem:/{print $2}')
    AVAIL_MEM=$(free -m | awk '/^Mem:/{print $7}')
    MEM_PERCENTAGE=$((AVAIL_MEM * 100 / TOTAL_MEM))
    
    # Disk analysis
    ROOT_DISK_AVAIL=$(df -h "${PROJECT_ROOT}" | awk 'NR==2 {print $4}')
    ROOT_DISK_AVAIL_BYTES=$(df -B1 "${PROJECT_ROOT}" | awk 'NR==2 {print $4}')
    
    # Network analysis
    INTERNET_SPEED=$(which speedtest-cli > /dev/null && speedtest-cli --simple 2>/dev/null | grep Download | awk '{print $2}' || echo "Unknown")
    
    # HSM detection
    HAS_TPM=$(test -e /dev/tpm0 && echo "true" || echo "false")
    HAS_YUBIKEY=$(lsusb 2>/dev/null | grep -i "yubico" > /dev/null && echo "true" || echo "false")
    
    # Environment detection
    IS_CONTAINER=$(grep -q "container=" /proc/1/environ 2>/dev/null && echo "true" || echo "false")
    IS_VIRTUAL=$(dmesg 2>/dev/null | grep -i "hypervisor" > /dev/null && echo "true" || echo "false")
    
    # Log findings
    log INFO "System capabilities:"
    log INFO "- CPU: $CPU_CORES cores, $CPU_MODEL ($CPU_ARCH)"
    log INFO "- Memory: ${AVAIL_MEM}MB available out of ${TOTAL_MEM}MB (${MEM_PERCENTAGE}%)"
    log INFO "- Disk: ${ROOT_DISK_AVAIL} available for installation"
    log INFO "- Internet: ${INTERNET_SPEED} Mbps download (if available)"
    log INFO "- HSM: TPM=${HAS_TPM}, YubiKey=${HAS_YUBIKEY}"
    log INFO "- Environment: Container=${IS_CONTAINER}, VM=${IS_VIRTUAL}"
    
    # Check for minimum requirements
    if [ "$TOTAL_MEM" -lt 2048 ]; then
        log WARN "Less than 2GB RAM available. Installation may be slow."
        if [ "$AUTO_RUN" = false ]; then
            read -p "Continue with low memory? (y/N): " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                log ERROR "Installation cancelled due to low memory"
                exit 1
            fi
        fi
    fi
    
    if [ "$ROOT_DISK_AVAIL_BYTES" -lt 5368709120 ]; then # 5GB
        log WARN "Less than 5GB disk space available. Installation may fail."
        if [ "$AUTO_RUN" = false ]; then
            read -p "Continue with low disk space? (y/N): " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                log ERROR "Installation cancelled due to low disk space"
                exit 1
            fi
        fi
    fi
    
    # Export system variables
    export CPU_CORES
    export TOTAL_MEM
    export AVAIL_MEM
    export HAS_TPM
    export HAS_YUBIKEY
    export IS_CONTAINER
    export IS_VIRTUAL
}

# Check script dependencies
check_scripts() {
    log INFO "Checking for required installation scripts..."
    
    # Check main installation script
    if [ ! -f "${SCRIPT_DIR}/linux_install.sh" ]; then
        log ERROR "Main installation script not found: ${SCRIPT_DIR}/linux_install.sh"
        exit 1
    fi
    
    # Check systemd configuration script
    if [ ! -f "${SCRIPT_DIR}/systemd_config.sh" ]; then
        log ERROR "Systemd configuration script not found: ${SCRIPT_DIR}/systemd_config.sh"
        exit 1
    fi
    
    # Make scripts executable
    chmod +x "${SCRIPT_DIR}/linux_install.sh"
    chmod +x "${SCRIPT_DIR}/systemd_config.sh"
    if [ -f "${SCRIPT_DIR}/uninstall.sh" ]; then
        chmod +x "${SCRIPT_DIR}/uninstall.sh"
    fi
    
    log INFO "All required scripts found and made executable"
}

# Configure installation based on type and system analysis
configure_installation_type() {
    log INFO "Configuring installation type: $INSTALL_TYPE"
    
    case "$INSTALL_TYPE" in
        minimal)
            # Minimal installation: basic components only
            START_SERVICE=false
            INSTALL_DEPS=true
            CONFIGURE_FIREWALL=false
            HARDENING_LEVEL="basic"
            # Low memory configuration
            export RUST_FLAGS="--cfg minimal"
            export MEMORY_LIMIT=$((TOTAL_MEM / 2))
            log INFO "Configuring for minimal installation with ${MEMORY_LIMIT}MB memory limit"
            ;;
        standard)
            # Standard installation: default settings
            START_SERVICE=true
            INSTALL_DEPS=true
            CONFIGURE_FIREWALL=true
            HARDENING_LEVEL="standard"
            
            # Memory-based optimization
            if [ "$TOTAL_MEM" -lt 4096 ]; then
                export RUST_FLAGS="--cfg memory_optimized"
                export MEMORY_LIMIT=$((TOTAL_MEM * 2 / 3))
                log INFO "Configuring for memory-optimized build with ${MEMORY_LIMIT}MB limit"
            else
                export MEMORY_LIMIT=$((TOTAL_MEM * 3 / 4))
                log INFO "Configuring with standard memory settings: ${MEMORY_LIMIT}MB limit"
            fi
            ;;
        full)
            # Full installation: all components with highest security
            START_SERVICE=true
            INSTALL_DEPS=true
            CONFIGURE_FIREWALL=true
            
            # Set hardening based on detected hardware security
            if [ "$HAS_TPM" = "true" ] || [ "$HAS_YUBIKEY" = "true" ]; then
                HARDENING_LEVEL="strict"
                export HSM_TYPE="hardware"
                log INFO "Hardware security module detected, using strict security profile"
            else
                HARDENING_LEVEL="standard"
                export HSM_TYPE="software"
                log INFO "No hardware security module detected, using software HSM"
            fi
            
            # CPU-based optimization
            if [ "$CPU_CORES" -gt 4 ]; then
                export PARALLEL_JOBS=$((CPU_CORES - 1))
                log INFO "Setting build to use $PARALLEL_JOBS parallel jobs"
            fi
            
            # Memory configuration for full installation
            export MEMORY_LIMIT=$((TOTAL_MEM * 3 / 4))
            log INFO "Configuring with full memory settings: ${MEMORY_LIMIT}MB limit"
            ;;
    esac
}

# Run the main installation script
run_main_installer() {
    log INFO "Running main installation script..."
    
    # Build command with appropriate arguments
    INSTALLER_CMD="${SCRIPT_DIR}/linux_install.sh --network=${NETWORK}"
    
    # Add optional arguments based on settings
    if [ "$AUTO_RUN" = true ]; then
        INSTALLER_CMD="${INSTALLER_CMD} --non-interactive"
    fi
    
    if [ "$START_SERVICE" = false ]; then
        # Don't add auto-start to the installer since we'll handle it in systemd config
        :
    else
        INSTALLER_CMD="${INSTALLER_CMD} --auto-start"
    fi
    
    if [ "$INSTALL_DEPS" = false ]; then
        INSTALLER_CMD="${INSTALLER_CMD} --skip-deps"
    fi
    
    if [ "$CONFIGURE_FIREWALL" = false ]; then
        INSTALLER_CMD="${INSTALLER_CMD} --skip-firewall"
    fi
    
    # Execute the installer
    log INFO "Executing: $INSTALLER_CMD"
    if ! $INSTALLER_CMD >> "$INSTALL_LOG" 2>&1; then
        log ERROR "Main installation script failed. Check log for details."
        exit 1
    fi
    
    log INFO "Main installation completed successfully"
}

# Run the systemd configuration script
run_systemd_config() {
    log INFO "Running systemd configuration script..."
    
    # Build command with appropriate arguments
    SYSTEMD_CMD="${SCRIPT_DIR}/systemd_config.sh --hardening=${HARDENING_LEVEL}"
    
    # Add auto-run if needed
    if [ "$AUTO_RUN" = true ]; then
        SYSTEMD_CMD="${SYSTEMD_CMD} --non-interactive"
    fi
    
    # Add start option if needed
    if [ "$START_SERVICE" = true ]; then
        SYSTEMD_CMD="${SYSTEMD_CMD} --start"
    fi
    
    # Execute the systemd config script
    log INFO "Executing: $SYSTEMD_CMD"
    if ! $SYSTEMD_CMD >> "$INSTALL_LOG" 2>&1; then
        log ERROR "Systemd configuration script failed. Check log for details."
        exit 1
    fi
    
    log INFO "Systemd configuration completed successfully"
}

# Verify installation
verify_installation() {
    log INFO "Verifying installation..."
    
    # Check if binary exists
    if [ ! -f "${PROJECT_ROOT}/target/release/anya-core" ]; then
        log ERROR "Binary not found: ${PROJECT_ROOT}/target/release/anya-core"
        return 1
    fi
    
    # Check if config file exists
    if [ ! -f "${PROJECT_ROOT}/config/anya.conf" ]; then
        log ERROR "Configuration file not found: ${PROJECT_ROOT}/config/anya.conf"
        return 1
    fi
    
    # Check if service is enabled
    if ! systemctl is-enabled anya-core.service &>/dev/null; then
        log ERROR "Service is not enabled: anya-core.service"
        return 1
    fi
    
    # Check if service is running (if it's supposed to be)
    if [ "$START_SERVICE" = true ] && ! systemctl is-active anya-core.service &>/dev/null; then
        log ERROR "Service is not running: anya-core.service"
        return 1
    fi
    
    log INFO "Installation verified successfully"
    return 0
}

# Show completion message with installation details
show_completion() {
    echo 
    echo "================================================================"
    echo "        Anya Core Auto-Installation Complete"
    echo "================================================================"
    echo
    echo "Installation type: $INSTALL_TYPE"
    echo "Network: $NETWORK"
    echo "Security hardening: $HARDENING_LEVEL"
    echo
    echo "System configuration:"
    echo "- CPU: $CPU_CORES cores"
    echo "- Memory: ${MEMORY_LIMIT}MB allocated"
    echo "- HSM type: ${HSM_TYPE:-software}"
    echo
    echo "Service status: $(systemctl is-active anya-core.service 2>/dev/null || echo "not running")"
    echo "Service enabled on boot: $(systemctl is-enabled anya-core.service 2>/dev/null || echo "not enabled")"
    echo
    echo "Configuration file:"
    echo "  ${PROJECT_ROOT}/config/anya.conf"
    echo
    echo "Binary location:"
    echo "  ${PROJECT_ROOT}/target/release/anya-core"
    echo
    echo "Installation log:"
    echo "  $INSTALL_LOG"
    echo
    echo "To manage the service:"
    echo "  sudo systemctl {start|stop|restart|status} anya-core.service"
    echo
    echo "To view logs:"
    echo "  sudo journalctl -u anya-core.service -f"
    echo
    echo "To uninstall:"
    echo "  sudo ${SCRIPT_DIR}/uninstall.sh"
    echo
    echo "================================================================"
}

# Main function
main() {
    log INFO "Starting Anya Core auto-installation (version $VERSION)..."
    
    # Parse command line arguments
    parse_args "$@"
    
    # Check for root privileges
    check_root
    
    # Check for required scripts
    check_scripts
    
    # Analyze system
    analyze_system
    
    # Configure installation based on type and system analysis
    configure_installation_type
    
    # Run main installation script
    run_main_installer
    
    # Run systemd configuration script
    run_systemd_config
    
    # Verify installation
    if ! verify_installation; then
        log WARN "Installation verification failed, but continuing"
    fi
    
    # Show completion message
    show_completion
    
    log INFO "Anya Core auto-installation completed successfully"
}

# Run the script
main "$@" 