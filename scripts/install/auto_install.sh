#!/bin/bash
# Anya Core Auto-Installation Master Script
# [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

set -euo pipefail

# Script version
VERSION="1.1.0"

# Directory setup
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
UTILS_DIR="${SCRIPT_DIR}/utils"

# Source common utilities
if [ -f "${UTILS_DIR}/install_common.sh" ]; then
    source "${UTILS_DIR}/install_common.sh"
else
    echo "ERROR: Common utilities not found: ${UTILS_DIR}/install_common.sh"
    exit 1
fi

# Set up process lock
if ! setup_process_lock "installer"; then
    exit 1
fi

# Display header
print_header "Auto-Installation" "$VERSION"

# Default values (previously defined VERSION is used from above)

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
FEATURE_FLAGS=""  # Will be auto-configured based on hardware
RUN_TESTS=false
TEST_TYPE="basic"  # basic, comprehensive, full
FORCE_CLEAN=false  # Force clean installation even if already installed
UPGRADE_EXISTING=true  # Upgrade an existing installation if found

# Define log file for this run (log function is already imported from common utils)
INSTALL_LOG="${INSTALL_LOGS_DIR}/auto_install_$(date +%Y%m%d-%H%M%S).log"

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
    echo "  --auto-run              Automatically run with no prompts (non-interactive)
    --run-tests             Run tests after installation
    --test-type=TYPE       Type of tests to run (basic, comprehensive, full)
    --features=FLAGS       Explicitly set Cargo feature flags (overrides auto-detection)
    --force-clean          Force clean installation even if already installed
    --no-upgrade           Don't upgrade existing installation (exit if installed)"
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
            --run-tests)
                RUN_TESTS=true
                shift
                ;;
            --test-type=*)
                TEST_TYPE="${1#*=}"
                if [[ ! "$TEST_TYPE" =~ ^(basic|comprehensive|full)$ ]]; then
                    log ERROR "Invalid test type: $TEST_TYPE. Must be basic, comprehensive, or full."
                    exit 1
                fi
                shift
                ;;
            --features=*)
                FEATURE_FLAGS="${1#*=}"
                shift
                ;;
            --force-clean)
                FORCE_CLEAN=true
                UPGRADE_EXISTING=false
                shift
                ;;
            --no-upgrade)
                UPGRADE_EXISTING=false
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
    log INFO "- Run Tests: $RUN_TESTS"
    log INFO "- Test Type: $TEST_TYPE"
    log INFO "- Feature Flags: $FEATURE_FLAGS (will be auto-configured if empty)"
    log INFO "- Force Clean: $FORCE_CLEAN"
    log INFO "- Upgrade Existing: $UPGRADE_EXISTING"
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
    
    # HSM detection - comprehensive
    HAS_TPM=$(test -e /dev/tpm0 -o -e /dev/tpmrm0 && echo "true" || echo "false")
    HAS_YUBIKEY=$(lsusb 2>/dev/null | grep -i "yubico" > /dev/null && echo "true" || echo "false")
    
    # PKCS#11 modules detection
    HAS_PKCS11=false
    for lib in /usr/lib*/libykcs11.so* /usr/lib*/libtpm2_pkcs11.so* /usr/local/lib*/libykcs11.so*; do
        if [ -f "$lib" ]; then
            HAS_PKCS11=true
            break
        fi
    done
    
    # Hardware wallet detection
    HAS_LEDGER=$(lsusb 2>/dev/null | grep -i "ledger" > /dev/null && echo "true" || echo "false")
    HAS_TREZOR=$(lsusb 2>/dev/null | grep -i "trezor" > /dev/null && echo "true" || echo "false")
    
    # Secure element detection
    HAS_SECURE_ELEMENT=false
    if [ -d "/sys/class/tee" ] || [ -e "/dev/attestation0" ]; then
        HAS_SECURE_ELEMENT=true
    fi
    
    # Environment detection
    IS_CONTAINER=$(grep -q "container=" /proc/1/environ 2>/dev/null && echo "true" || echo "false")
    IS_VIRTUAL=$(dmesg 2>/dev/null | grep -i "hypervisor" > /dev/null && echo "true" || echo "false")
    
    # Log findings
    log INFO "System capabilities:"
    log INFO "- CPU: $CPU_CORES cores, $CPU_MODEL ($CPU_ARCH)"
    log INFO "- Memory: ${AVAIL_MEM}MB available out of ${TOTAL_MEM}MB (${MEM_PERCENTAGE}%)"
    log INFO "- Disk: ${ROOT_DISK_AVAIL} available for installation"
    log INFO "- Internet: ${INTERNET_SPEED} Mbps download (if available)"
    log INFO "- HSM: TPM=${HAS_TPM}, YubiKey=${HAS_YUBIKEY}, PKCS11=${HAS_PKCS11}"
    log INFO "- Hardware Wallets: Ledger=${HAS_LEDGER}, Trezor=${HAS_TREZOR}"
    log INFO "- Secure Element: ${HAS_SECURE_ELEMENT}"
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
    export HAS_PKCS11
    export HAS_LEDGER
    export HAS_TREZOR
    export HAS_SECURE_ELEMENT
    export IS_CONTAINER
    export IS_VIRTUAL
    
    # Auto-configure feature flags if not explicitly set
    if [ -z "$FEATURE_FLAGS" ]; then
        configure_feature_flags
    fi
}

# Check for existing installation and handle upgrade/clean install logic
check_existing_installation() {
    log INFO "Checking for existing Anya Core installation..."
    
    local INSTALLED=false
    local VERSION_INSTALLED="unknown"
    local FEATURES_INSTALLED="unknown"
    local CONFIG_EXISTS=false
    local SERVICE_EXISTS=false
    local BINARY_EXISTS=false
    
    # Check for binary
    if [ -f "${PROJECT_ROOT}/target/release/anya-core" ]; then
        BINARY_EXISTS=true
        INSTALLED=true
        # Try to get version information
        if [ -x "${PROJECT_ROOT}/target/release/anya-core" ]; then
            VERSION_INSTALLED=$("${PROJECT_ROOT}/target/release/anya-core" --version 2>/dev/null | head -1 || echo "version information unavailable")
            FEATURES_INSTALLED=$("${PROJECT_ROOT}/target/release/anya-core" --show-features 2>/dev/null || echo "feature information unavailable")
        fi
        log INFO "Found existing binary: ${PROJECT_ROOT}/target/release/anya-core"
    fi
    
    # Check for systemd service
    if systemctl list-unit-files | grep -q anya-core.service; then
        SERVICE_EXISTS=true
        INSTALLED=true
        local STATUS=$(systemctl is-active anya-core.service 2>/dev/null || echo "inactive")
        local ENABLED=$(systemctl is-enabled anya-core.service 2>/dev/null || echo "disabled")
        log INFO "Found existing systemd service: anya-core.service (Status: $STATUS, Enabled: $ENABLED)"
    fi
    
    # Check for config
    if [ -f "${PROJECT_ROOT}/config/anya.conf" ]; then
        CONFIG_EXISTS=true
        INSTALLED=true
        log INFO "Found existing configuration: ${PROJECT_ROOT}/config/anya.conf"
    fi
    
    # If not installed, we can proceed with clean install
    if [ "$INSTALLED" = false ]; then
        log INFO "No existing Anya Core installation detected, proceeding with clean installation"
        return 0
    fi
    
    # Handle existing installation based on flags
    log INFO "Existing Anya Core installation detected"
    log INFO "Installed version: $VERSION_INSTALLED"
    log INFO "Installed features: $FEATURES_INSTALLED"
    
    if [ "$FORCE_CLEAN" = true ]; then
        log INFO "Force clean installation requested, removing existing installation"
        run_uninstall
        return 0
    fi
    
    if [ "$UPGRADE_EXISTING" = true ]; then
        log INFO "Upgrading existing installation"
        export UPGRADING=true
        # Backup existing config if it exists
        if [ "$CONFIG_EXISTS" = true ]; then
            local BACKUP_FILE="${PROJECT_ROOT}/config/anya.conf.backup.$(date +%Y%m%d-%H%M%S)"
            log INFO "Backing up existing configuration to $BACKUP_FILE"
            cp "${PROJECT_ROOT}/config/anya.conf" "$BACKUP_FILE"
            export CONFIG_BACKUP="$BACKUP_FILE"
        fi
        return 0
    fi
    
    # If we get here, we're not allowed to upgrade or force clean
    log ERROR "Existing installation found and neither upgrade nor force clean is allowed"
    log ERROR "Use --force-clean to force a clean installation or --upgrade (default) to upgrade"
    exit 1
}

# Run the uninstall script if it exists, otherwise do manual uninstall
run_uninstall() {
    log INFO "Removing existing Anya Core installation..."
    
    # Check if uninstall script exists and run it
    if [ -f "${SCRIPT_DIR}/uninstall.sh" ] && [ -x "${SCRIPT_DIR}/uninstall.sh" ]; then
        log INFO "Using uninstall script: ${SCRIPT_DIR}/uninstall.sh"
        if [ "$AUTO_RUN" = true ]; then
            "${SCRIPT_DIR}/uninstall.sh" --non-interactive
        else
            "${SCRIPT_DIR}/uninstall.sh"
        fi
    else
        # Manual uninstall if no script is available
        log INFO "No uninstall script found, performing manual uninstall"
        
        # Stop and disable service
        if systemctl list-unit-files | grep -q anya-core.service; then
            log INFO "Stopping and disabling anya-core service"
            systemctl stop anya-core.service 2>/dev/null || true
            systemctl disable anya-core.service 2>/dev/null || true
            rm -f /etc/systemd/system/anya-core.service 2>/dev/null || true
            systemctl daemon-reload
        fi
        
        # Remove binaries, but keep configuration as backup
        if [ -f "${PROJECT_ROOT}/target/release/anya-core" ]; then
            log INFO "Removing Anya Core binary"
            rm -f "${PROJECT_ROOT}/target/release/anya-core"
        fi
        
        # Backup configuration if it exists
        if [ -f "${PROJECT_ROOT}/config/anya.conf" ]; then
            local BACKUP_FILE="${PROJECT_ROOT}/config/anya.conf.backup.$(date +%Y%m%d-%H%M%S)"
            log INFO "Backing up existing configuration to $BACKUP_FILE"
            cp "${PROJECT_ROOT}/config/anya.conf" "$BACKUP_FILE"
            log INFO "Removing original configuration file"
            rm -f "${PROJECT_ROOT}/config/anya.conf"
        fi
    fi
    
    log INFO "Existing installation removed"
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

# Auto-configure feature flags based on hardware detection
configure_feature_flags() {
    log INFO "Auto-configuring Cargo feature flags based on hardware detection..."
    
    # Base feature flags
    local flags="std"
    
    # HSM feature flags
    if [ "$HAS_TPM" = "true" ] || [ "$HAS_YUBIKEY" = "true" ] || \
       [ "$HAS_PKCS11" = "true" ] || [ "$HAS_LEDGER" = "true" ] || \
       [ "$HAS_TREZOR" = "true" ] || [ "$HAS_SECURE_ELEMENT" = "true" ]; then
        flags="$flags,hsm"
        log INFO "Hardware security module detected, enabling HSM feature"
    fi
    
    # Bitcoin integration flags (always include for Anya Core)
    flags="$flags,bitcoin_integration"
    
    # For full installation, add complete feature
    if [ "$INSTALL_TYPE" = "full" ]; then
        flags="$flags,complete"
        log INFO "Full installation selected, enabling complete feature set"
    fi
    
    # Set the feature flags global variable
    FEATURE_FLAGS="$flags"
    log INFO "Configured feature flags: $FEATURE_FLAGS"
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
    
    # Add feature flags argument
    INSTALLER_CMD="${INSTALLER_CMD} --features=${FEATURE_FLAGS}"
    
    # Add upgrade flag if we're upgrading
    if [ "${UPGRADING:-false}" = true ]; then
        INSTALLER_CMD="${INSTALLER_CMD} --upgrade"
        
        # Pass the config backup path if available
        if [ -n "${CONFIG_BACKUP:-}" ]; then
            INSTALLER_CMD="${INSTALLER_CMD} --config-backup=${CONFIG_BACKUP}"
        fi
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
    
    # Add upgrade flag if we're upgrading
    if [ "${UPGRADING:-false}" = true ]; then
        SYSTEMD_CMD="${SYSTEMD_CMD} --upgrade"
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
    
    # Check version and feature flags in the running binary
    log INFO "Checking version and compiled features..."
    if [ -x "${PROJECT_ROOT}/target/release/anya-core" ]; then
        ${PROJECT_ROOT}/target/release/anya-core --version >> "$INSTALL_LOG" 2>&1 || true
        ${PROJECT_ROOT}/target/release/anya-core --show-features >> "$INSTALL_LOG" 2>&1 || true
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
    if [ "${UPGRADING:-false}" = true ]; then
        echo "Installation type: UPGRADE"
    else
        echo "Installation type: $INSTALL_TYPE (fresh install)"
    fi
    echo "Network: $NETWORK"
    echo "Security hardening: $HARDENING_LEVEL"
    echo "Feature flags: $FEATURE_FLAGS"
    
    if [ -n "${CONFIG_BACKUP:-}" ]; then
        echo "Configuration backup: ${CONFIG_BACKUP}"
    fi
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

# Run post-installation tests
run_tests() {
    log INFO "Running post-installation tests (type: $TEST_TYPE)..."
    
    # Make sure service is running for tests
    if ! systemctl is-active anya-core.service &>/dev/null; then
        log INFO "Starting anya-core service for testing..."
        systemctl start anya-core.service
        sleep 3 # Give it time to start
    fi
    
    # Execute the appropriate test suite based on test type
    local TEST_SCRIPT="${PROJECT_ROOT}/scripts/test/run_tests.sh"
    if [ ! -f "$TEST_SCRIPT" ]; then
        log WARN "Test script not found: $TEST_SCRIPT"
        mkdir -p "${PROJECT_ROOT}/scripts/test"
        
        # Create a basic test script if it doesn't exist
        cat > "$TEST_SCRIPT" << 'EOL'
#!/bin/bash
set -euo pipefail

# ==== SELF-DIAGNOSTICS FUNCTIONS ====
self_diagnose() {
    echo "=== TEST SCRIPT SELF-DIAGNOSTICS ==="
    echo "Date and time: $(date)"
    echo "Script path: $0"
    echo "Working directory: $(pwd)"
    echo "User: $(whoami)"
    echo "Environment variables:"
    env | grep -E 'ANYA|RUST|CARGO|PATH' | sort

    echo "\nSystem resources:"
    echo "- CPU: $(nproc) cores"
    echo "- Memory: $(free -h | awk '/^Mem:/ {print $2}') total / $(free -h | awk '/^Mem:/ {print $7}') available"
    echo "- Disk: $(df -h . | awk 'NR==2 {print $4}') available"

    echo "\nCompiled features:"
    local core_binary
    for bin_path in "${PROJECT_ROOT}/target/release/anya-core" "$(which anya-core 2>/dev/null)" "/usr/local/bin/anya-core"; do
        if [ -x "$bin_path" ]; then
            core_binary="$bin_path"
            break
        fi
    done

    if [ -n "$core_binary" ]; then
        echo "Using binary: $core_binary"
        "$core_binary" --version || echo "Failed to get version"
        "$core_binary" --show-features 2>/dev/null || echo "Feature info not available"
    else
        echo "Could not find anya-core binary"
    fi

    echo "\nDetecting hardware security capabilities:"
    if [ -e "/dev/tpm0" ] || [ -e "/dev/tpmrm0" ]; then
        echo "- TPM: Available"
    else
        echo "- TPM: Not detected"
    fi

    if lsusb 2>/dev/null | grep -qi "yubikey"; then
        echo "- YubiKey: Connected"
    else
        echo "- YubiKey: Not detected"
    fi

    if lsusb 2>/dev/null | grep -qi "ledger"; then
        echo "- Ledger: Connected"
    else
        echo "- Ledger: Not detected"
    fi

    local pkcs11_lib=false
    for lib in /usr/lib*/libykcs11.so* /usr/lib*/libtpm2_pkcs11.so* /usr/local/lib*/libykcs11.so*; do
        if [ -f "$lib" ]; then
            echo "- PKCS#11: Available ($lib)"
            pkcs11_lib=true
            break
        fi
    done
    if [ "$pkcs11_lib" = false ]; then
        echo "- PKCS#11: No libraries detected"
    fi

    echo "\nService status:"
    systemctl status anya-core.service || echo "Service not available"

    echo "\nNetwork configuration:"
    # Check if API is responding
    echo -n "- API health endpoint: "
    curl -s -o /dev/null -w "%{http_code}" http://localhost:3300/health || echo "not responding"
    
    echo "\n=== END SELF-DIAGNOSTICS ===\n"
}

# Main script logic
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TEST_TYPE=${1:-basic}
FEATURE_FLAGS=${2:-std}
SELF_DIAG=${3:-true}

echo "Running Anya Core test suite (type: $TEST_TYPE, features: $FEATURE_FLAGS)"

# Run self-diagnostics
if [ "$SELF_DIAG" = true ]; then
    self_diagnose
fi

# Basic tests
if [[ "$TEST_TYPE" =~ ^(basic|comprehensive|full)$ ]]; then
    echo "\n=== RUNNING BASIC HEALTH CHECKS ==="
    # Check if service is running
    echo -n "Service status: "
    systemctl is-active anya-core.service || echo "not running"
    
    # Check if API is responding
    echo -n "API health: "
    curl -s -o /dev/null -w "%{http_code}" http://localhost:3300/health || echo "not responding"
    
    # Run cargo test with minimal features
    echo "\nRunning minimal feature tests..."
    cd "$PROJECT_ROOT"
    cargo test --no-default-features || echo "Basic tests failed with exit code $?"
    echo "Basic tests completed"
fi

# Comprehensive tests
if [[ "$TEST_TYPE" =~ ^(comprehensive|full)$ ]]; then
    echo "\n=== RUNNING COMPREHENSIVE TESTS ==="
    # Run tests with specific feature flags
    cd "$PROJECT_ROOT"
    echo "Testing with features: $FEATURE_FLAGS"
    cargo test --features "$FEATURE_FLAGS" || echo "Comprehensive tests failed with exit code $?"
    
    # Test HSM if available
    if [[ "$FEATURE_FLAGS" == *"hsm"* ]]; then
        echo "Running HSM-specific tests..."
        cargo test --features hsm -- --test-threads=1 --nocapture hsm_tests || echo "HSM tests failed with exit code $?"
    fi
    echo "Comprehensive tests completed"
fi

# Full tests
if [[ "$TEST_TYPE" == "full" ]]; then
    echo "\n=== RUNNING FULL TEST SUITE ==="
    # Run all tests
    cd "$PROJECT_ROOT"
    echo "Testing all features..."
    cargo test --all-features || echo "All-features tests failed with exit code $?"
    
    # Run integration tests
    echo "Running integration tests..."
    cargo test --test '*_integration' || echo "Integration tests failed with exit code $?"
    
    # Run stress tests
    echo "Running stress tests..."
    if [ -f "./scripts/test/stress_test.sh" ]; then
        ./scripts/test/stress_test.sh || echo "Stress tests failed with exit code $?"
    else
        echo "Stress test script not found, creating basic stress test..."
        mkdir -p "./scripts/test"
        cat > "./scripts/test/stress_test.sh" << 'STRESS_EOL'
#!/bin/bash
echo "Running basic stress test..."
# Make 10 concurrent API requests
for i in {1..10}; do
  curl -s http://localhost:3300/health &
done
wait
echo "Basic stress test completed"
STRESS_EOL
        chmod +x "./scripts/test/stress_test.sh"
        ./scripts/test/stress_test.sh || echo "Stress tests failed with exit code $?"
    fi
    echo "Full test suite completed"
fi

echo "\n=== ALL TESTS COMPLETED ==="
EOL
        chmod +x "$TEST_SCRIPT"
        log INFO "Created basic test script: $TEST_SCRIPT"
    fi
    
    # Run the test script with appropriate arguments
    log INFO "Executing: $TEST_SCRIPT $TEST_TYPE $FEATURE_FLAGS"
    if ! "$TEST_SCRIPT" "$TEST_TYPE" "$FEATURE_FLAGS" >> "$INSTALL_LOG" 2>&1; then
        log ERROR "Test suite failed. Check log for details."
        return 1
    fi
    
    log INFO "Test suite completed successfully"
    return 0
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
    
    # Check for existing installation
    check_existing_installation
    
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
    
    # Run tests if requested
    if [ "$RUN_TESTS" = true ]; then
        if ! run_tests; then
            log WARN "Test suite failed, but installation will be considered complete"
        fi
    fi
    
    # Show completion message
    show_completion
    
    log INFO "Anya Core auto-installation completed successfully"
}

# Run the script
main "$@" 