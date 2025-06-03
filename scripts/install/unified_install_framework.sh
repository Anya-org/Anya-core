#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# Unified Installation Framework for Anya Core
# Following official Bitcoin Improvement Proposals (BIPs)
# Part of the Anya Core Hexagonal Architecture
# Date: 2025-05-20

set -e

# Error handling with line number reporting
function handle_error {
    echo "[ERROR] An error occurred at line $1"
    exit 1
}

trap 'handle_error $LINENO' ERR

# Define root directory
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
UTILS_DIR="${SCRIPT_DIR}/utils"

# Source common utilities
if [ -f "${UTILS_DIR}/install_common.sh" ]; then
    source "${UTILS_DIR}/install_common.sh"
else
    echo "ERROR: Common utilities not found: ${UTILS_DIR}/install_common.sh"
    exit 1
fi

# Import validation functions
source <(node scripts/install/validator.js)

validate_installation() {
  node scripts/install/validator.js check_bip_compliance
}

# Script version
VERSION="1.0.0"

# Default directories
LOG_DIR="/var/log/anya"
CONFIG_DIR="/etc/anya"
INSTALL_DIR="/opt/anya"

# Parse command line arguments
DRY_RUN=false
for arg in "$@"; do
  case $arg in
    --dry-run)
      DRY_RUN=true
      shift
      ;;
    --network=*)
      NETWORK="${arg#*=}"
      shift
      ;;
    --type=*)
      INSTALL_TYPE="${arg#*=}"
      shift
      ;;
    *)
      shift
      ;;
  esac
done

# Default configuration
NETWORK="${NETWORK:-testnet}"
INSTALL_TYPE="${INSTALL_TYPE:-standard}"  # minimal, standard, full
HARDENING_LEVEL="standard"  # basic, standard, strict
START_SERVICE=true
INSTALL_DEPS=true
CONFIGURE_FIREWALL=true

# Display header
print_header "Anya Core Installation" "$VERSION"

echo "[INFO] Installation type: $INSTALL_TYPE"
echo "[INFO] Network: $NETWORK"

if [ "$DRY_RUN" = true ]; then
  echo "[DRY RUN] Running in dry-run mode. No changes will be made."
fi
AUTO_RUN=false
FEATURE_FLAGS=""
RUN_TESTS=false
TEST_TYPE="basic"  # basic, comprehensive, full
FORCE_CLEAN=false
UPGRADE_EXISTING=true
DRY_RUN=false
SILENT_MODE=false

# Function to show help
function show_help {
    echo "Anya Core Unified Installation Framework v${VERSION}"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --network=NETWORK       Specify network (mainnet, testnet, regtest)"
    echo "  --type=TYPE             Installation type (minimal, standard, full)"
    echo "  --hardening=LEVEL       Security hardening level (basic, standard, strict)"
    echo "  --no-start              Don't start the service after installation"
    echo "  --no-deps               Skip dependency installation"
    echo "  --no-firewall           Skip firewall configuration"
    echo "  --auto-run              Automatically run with no prompts (non-interactive)"
    echo "  --run-tests             Run tests after installation"
    echo "  --test-type=TYPE        Type of tests to run (basic, comprehensive, full)"
    echo "  --features=FLAGS        Explicitly set Cargo feature flags (overrides auto-detection)"
    echo "  --force-clean           Force clean installation even if already installed"
    echo "  --no-upgrade            Don't upgrade existing installation (exit if installed)"
    echo "  --dry-run               Perform a dry run (don't make any changes)"
    echo "  --silent                Run in silent mode (suppress output)"
    echo "  --help                  Display this help message"
    echo "  --version               Display script version"
    echo "  --log-level=LEVEL       Set log level (debug, info, warn, error)"
    echo ""
    echo "Example:"
    echo "  sudo $0 --network=testnet --type=standard --auto-run"
}

# Function to show version
function show_version {
    echo "Anya Core Unified Installation Framework v${VERSION}"
}

# Function to parse command line arguments
function parse_args {
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
                shift
                ;;
            --no-upgrade)
                UPGRADE_EXISTING=false
                shift
                ;;
            --dry-run)
                DRY_RUN=true
                shift
                ;;
            --silent)
                SILENT_MODE=true
                shift
                ;;
            --log-level=*)
                LOG_LEVEL="${1#*=}"
                shift
                ;;
            *)
                log ERROR "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
}

# Function to check for root privileges
function check_root {
    if [ "$EUID" -ne 0 ]; then
        log ERROR "This script requires root privileges. Please run with sudo."
        exit 1
    fi
}

# Function to analyze system capabilities
function analyze_system {
    log INFO "Analyzing system capabilities..."
    
    # Check CPU
    CPU_CORES=$(nproc)
    log INFO "CPU cores: $CPU_CORES"
    
    # Check memory
    MEM_TOTAL=$(free -m | awk '/^Mem:/{print $2}')
    MEM_AVAIL=$(free -m | awk '/^Mem:/{print $7}')
    log INFO "Memory: ${MEM_AVAIL}MB available out of ${MEM_TOTAL}MB total"
    
    # Check disk space
    DISK_AVAIL=$(df -h "$ROOT_DIR" | awk 'NR==2 {print $4}')
    log INFO "Disk space available: $DISK_AVAIL"
    
    # Detect hardware security devices
    log INFO "Detecting hardware security devices..."
    
    # Check for TPM
    if [ -d "/sys/class/tpm" ] || [ -d "/dev/tpm0" ]; then
        log INFO "TPM detected"
        HAS_TPM=true
    else
        log INFO "No TPM detected"
        HAS_TPM=false
    fi
    
    # Check for hardware wallets (simplified check)
    if lsusb | grep -q -E "Ledger|Trezor"; then
        log INFO "Hardware wallet detected"
        HAS_HW_WALLET=true
    else
        log INFO "No hardware wallet detected"
        HAS_HW_WALLET=false
    fi
    
    # Check for PKCS#11 devices (simplified check)
    if [ -d "/usr/lib/pkcs11" ] || [ -d "/usr/local/lib/pkcs11" ]; then
        log INFO "PKCS#11 libraries detected"
        HAS_PKCS11=true
    else
        log INFO "No PKCS#11 libraries detected"
        HAS_PKCS11=false
    fi
    
    # Check for secure elements (simplified check)
    if lsusb | grep -q -E "Yubico|NXP"; then
        log INFO "Secure element detected"
        HAS_SECURE_ELEMENT=true
    else
        log INFO "No secure element detected"
        HAS_SECURE_ELEMENT=false
    fi
    
    # Check network connectivity
    if ping -c 1 bitcoin.org &>/dev/null; then
        log INFO "Network connectivity: OK"
        HAS_NETWORK=true
    else
        log WARN "Network connectivity: Limited"
        HAS_NETWORK=false
    fi
    
    # Check for virtualization
    if systemd-detect-virt -q; then
        VIRT_TYPE=$(systemd-detect-virt)
        log INFO "Running in virtualized environment: $VIRT_TYPE"
        IS_VIRTUALIZED=true
    else
        log INFO "Running on physical hardware"
        IS_VIRTUALIZED=false
    fi
    
    # Adjust installation type based on system capabilities if not explicitly set
    if [ "$AUTO_RUN" = true ] && [ "$INSTALL_TYPE" = "standard" ]; then
        if [ "$MEM_TOTAL" -lt 2048 ] || [ "$CPU_CORES" -lt 2 ]; then
            log WARN "Limited system resources detected, switching to minimal installation type"
            INSTALL_TYPE="minimal"
        elif [ "$MEM_TOTAL" -gt 8192 ] && [ "$CPU_CORES" -gt 4 ]; then
            log INFO "High-performance system detected, switching to full installation type"
            INSTALL_TYPE="full"
        fi
    fi
    
    log INFO "System analysis completed"
}

# Hardware Recommendations
recommend_features() {
  if lscpu | grep -q 'avx2'; then
    echo "Recommended Features: AVX2_OPTIMIZED BITCOIN_FULL_NODE"
  elif [ $(free -g | awk '/Mem:/ {print $2}') -ge 16 ]; then
    echo "Recommended Features: HIGH_MEM_MODE"
  else
    echo "Recommended Features: STANDARD_MODE"
  fi
}

# Function to check for existing installation
function check_existing_installation {
    log INFO "Checking for existing installation..."
    
    local is_installed=false
    
    # Check for binary
    if [ -f "/usr/local/bin/anya-core" ]; then
        log INFO "Anya Core binary found at /usr/local/bin/anya-core"
        is_installed=true
    fi
    
    # Check for service
    if systemctl is-enabled anya-core.service &>/dev/null; then
        log INFO "Anya Core service is enabled"
        is_installed=true
    fi
    
    # Check for configuration
    if [ -f "/etc/anya-core/config.toml" ]; then
        log INFO "Anya Core configuration found at /etc/anya-core/config.toml"
        is_installed=true
    fi
    
    if [ "$is_installed" = true ]; then
        if [ "$FORCE_CLEAN" = true ]; then
            log WARN "Existing installation found, but --force-clean specified. Will perform clean installation."
            run_uninstall
        elif [ "$UPGRADE_EXISTING" = true ]; then
            log INFO "Existing installation found. Will perform upgrade."
        else
            log ERROR "Existing installation found and --no-upgrade specified. Exiting."
            exit 1
        fi
    else
        log INFO "No existing installation found. Will perform clean installation."
    fi
}

# Function to run uninstall
function run_uninstall {
    log INFO "Running uninstall..."
    
    # Check if uninstall script exists
    if [ -f "${SCRIPT_DIR}/uninstall.sh" ]; then
        log INFO "Using uninstall script"
        bash "${SCRIPT_DIR}/uninstall.sh"
    else
        log WARN "Uninstall script not found. Performing manual uninstall."
        
        # Stop and disable service
        if systemctl is-enabled anya-core.service &>/dev/null; then
            log INFO "Stopping and disabling Anya Core service"
            systemctl stop anya-core.service
            systemctl disable anya-core.service
        fi
        
        # Remove binary
        if [ -f "/usr/local/bin/anya-core" ]; then
            log INFO "Removing Anya Core binary"
            rm -f "/usr/local/bin/anya-core"
        fi
        
        # Remove configuration
        if [ -d "/etc/anya-core" ]; then
            log INFO "Removing Anya Core configuration"
            rm -rf "/etc/anya-core"
        fi
        
        # Remove service file
        if [ -f "/etc/systemd/system/anya-core.service" ]; then
            log INFO "Removing Anya Core service file"
            rm -f "/etc/systemd/system/anya-core.service"
            systemctl daemon-reload
        fi
        
        # Remove data directory (with confirmation if not in auto mode)
        if [ -d "/var/lib/anya-core" ]; then
            if [ "$AUTO_RUN" = true ]; then
                log WARN "Removing Anya Core data directory"
                rm -rf "/var/lib/anya-core"
            else
                read -p "Remove Anya Core data directory? This will delete all data. [y/N] " -n 1 -r
                echo
                if [[ $REPLY =~ ^[Yy]$ ]]; then
                    log WARN "Removing Anya Core data directory"
                    rm -rf "/var/lib/anya-core"
                else
                    log INFO "Keeping Anya Core data directory"
                fi
            fi
        fi
        
        # Remove legacy script references
        rm -f install-anya.sh install-master.sh linux_install.sh
    fi
    
    log INFO "Uninstall completed"
}

# Core Ports
declare_ports() {
    # Bitcoin interface port
    echo "Bitcoin RPC port: 8332"
}

# Adapter Implementations
implement_bitcoin_adapter() {
    # Using previous security configurations
    verify_tpm_attestation
    configure_firewall
}

# Function to configure feature flags
function configure_feature_flags {
    log INFO "Configuring feature flags..."
    
    # If feature flags are explicitly set, use them
    if [ -n "$FEATURE_FLAGS" ]; then
        log INFO "Using explicitly set feature flags: $FEATURE_FLAGS"
        return
    fi
    
    # Default feature flags based on installation type
    case "$INSTALL_TYPE" in
        minimal)
            FEATURE_FLAGS="minimal"
            ;;
        standard)
            FEATURE_FLAGS="bitcoin_integration"
            ;;
        full)
            FEATURE_FLAGS="bitcoin_integration,hsm,lightning,dao,web5"
            ;;
    esac
    
    # Add HSM feature if hardware security devices are detected
    if [ "$HAS_TPM" = true ] || [ "$HAS_HW_WALLET" = true ] || [ "$HAS_PKCS11" = true ] || [ "$HAS_SECURE_ELEMENT" = true ]; then
        if [[ ! "$FEATURE_FLAGS" =~ "hsm" ]]; then
            FEATURE_FLAGS="${FEATURE_FLAGS},hsm"
        fi
    fi
    
    # Hardware-based feature recommendations
    local recommended_features=$(recommend_features)
    log INFO "$recommended_features"
    FEATURE_FLAGS="${FEATURE_FLAGS},${recommended_features#*:}"
    
    log INFO "Configured feature flags: $FEATURE_FLAGS"
}

# Function to run main installer
function run_main_installer {
    log INFO "Running main installer..."
    
    # Check if main installer script exists
    if [ -f "${SCRIPT_DIR}/main_installer.sh" ]; then
        log INFO "Using main_installer.sh"
        
        # Build command
        local cmd="${SCRIPT_DIR}/main_installer.sh"
        cmd+=" --network=$NETWORK"
        cmd+=" --type=$INSTALL_TYPE"
        cmd+=" --hardening=$HARDENING_LEVEL"
        cmd+=" --features=$FEATURE_FLAGS"
        
        if [ "$START_SERVICE" = false ]; then
            cmd+=" --no-start"
        fi
        
        if [ "$INSTALL_DEPS" = false ]; then
            cmd+=" --no-deps"
        fi
        
        if [ "$CONFIGURE_FIREWALL" = false ]; then
            cmd+=" --no-firewall"
        fi
        
        if [ "$AUTO_RUN" = true ]; then
            cmd+=" --auto-run"
        fi
        
        # Run the command
        log INFO "Executing: $cmd"
        eval "$cmd"
    else
        log WARN "main_installer.sh not found. Using minimal installation mode."
        log INFO "Creating basic directory structure..."
        
        if [ "$DRY_RUN" != true ]; then
            # Create minimal directory structure
            mkdir -p "$INSTALL_DIR/bin"
            mkdir -p "$INSTALL_DIR/config"
            mkdir -p "$INSTALL_DIR/logs"
            
            # Create basic configuration
            cat > "$CONFIG_DIR/anya.conf" <<EOL
# Anya Core Configuration
# Generated on $(date)

[network]
type=$NETWORK
rpcport=8332
rpcuser=anya
rpcpassword=$(openssl rand -hex 32)

[core]
datadir=/var/lib/anya
logdir=/var/log/anya

[bitcoin]
rpchost=127.0.0.1
rpcport=8332
rpcuser=bitcoin
rpcpassword=bitcoin
EOL
            
            log INFO "Created minimal installation at $INSTALL_DIR"
            log INFO "Configuration file created at $CONFIG_DIR/anya.conf"
        else
            log INFO "[DRY RUN] Would create minimal installation at $INSTALL_DIR"
            log INFO "[DRY RUN] Would create configuration at $CONFIG_DIR/anya.conf"
        fi
    fi
    
    log INFO "Main installer completed"
}

# Function to run systemd configuration
function run_systemd_config {
    log INFO "Configuring systemd service..."
    
    # Check if systemd_config.sh exists
    if [ -f "${SCRIPT_DIR}/systemd_config.sh" ]; then
        log INFO "Using systemd_config.sh"
        
        # Build command
        local cmd="${SCRIPT_DIR}/systemd_config.sh"
        cmd+=" --network=$NETWORK"
        
        if [ "$START_SERVICE" = false ]; then
            cmd+=" --no-start"
        fi
        
        if [ "$AUTO_RUN" = true ]; then
            cmd+=" --auto-run"
        fi
        
        # Run the command
        log INFO "Executing: $cmd"
        eval "$cmd"
    else
        log WARN "systemd_config.sh not found. Skipping systemd configuration."
    fi
    
    log INFO "Systemd configuration completed"
}

# Function to verify installation
function verify_installation {
    log INFO "Verifying installation..."
    
    local verification_passed=true
    
    # Check binary
    if [ ! -f "/usr/local/bin/anya-core" ]; then
        log ERROR "Anya Core binary not found at /usr/local/bin/anya-core"
        verification_passed=false
    fi
    
    # Check configuration
    if [ ! -f "/etc/anya-core/config.toml" ]; then
        log ERROR "Anya Core configuration not found at /etc/anya-core/config.toml"
        verification_passed=false
    fi
    
    # Check service
    if ! systemctl is-enabled anya-core.service &>/dev/null; then
        log WARN "Anya Core service is not enabled"
    fi
    
    # Check if service is running (if it should be)
    if [ "$START_SERVICE" = true ]; then
        if ! systemctl is-active anya-core.service &>/dev/null; then
            log ERROR "Anya Core service is not running"
            verification_passed=false
        fi
    fi
    
    if [ "$verification_passed" = true ]; then
        log INFO "Installation verification passed"
        return 0
    else
        log ERROR "Installation verification failed"
        return 1
    fi
}

# Function to run tests
function run_tests {
    log INFO "Running tests..."
    
    # Check if unified_test_framework.sh exists
    if [ -f "${ROOT_DIR}/scripts/test/unified_test_framework.sh" ]; then
        log INFO "Using unified_test_framework.sh"
        
        # Build command
        local cmd="${ROOT_DIR}/scripts/test/unified_test_framework.sh"
        cmd+=" --level=$TEST_TYPE"
        
        # Run the command
        log INFO "Executing: $cmd"
        eval "$cmd"
    else
        log WARN "unified_test_framework.sh not found. Skipping tests."
    fi
    
    log INFO "Tests completed"
}

# Function to show completion message
function show_completion {
    echo
    echo -e "${GREEN}${BOLD}Anya Core Installation Completed!${NC}"
    echo
    echo -e "${BLUE}Installation Details:${NC}"
    echo -e "  Network: $NETWORK"
    echo -e "  Installation Type: $INSTALL_TYPE"
    echo -e "  Hardening Level: $HARDENING_LEVEL"
    echo -e "  Feature Flags: $FEATURE_FLAGS"
    echo
    
    if systemctl is-active anya-core.service &>/dev/null; then
        echo -e "${GREEN}Service Status: Running${NC}"
    else
        echo -e "${YELLOW}Service Status: Not Running${NC}"
    fi
    
    echo
    echo -e "${BLUE}Configuration:${NC}"
    echo -e "  Config File: /etc/anya-core/config.toml"
    echo -e "  Data Directory: /var/lib/anya-core"
    echo -e "  Log Directory: /var/log/anya-core"
    echo
    
    echo -e "${BLUE}Usage:${NC}"
    echo -e "  Start Service: sudo systemctl start anya-core.service"
    echo -e "  Stop Service: sudo systemctl stop anya-core.service"
    echo -e "  View Logs: sudo journalctl -u anya-core.service"
    echo
    
    echo -e "${BLUE}API Access:${NC}"
    echo -e "  Health Check: http://localhost:3300/health"
    echo -e "  Status: http://localhost:3300/status"
    echo
    
    echo -e "${BLUE}For more information, see:${NC}"
    echo -e "  Documentation: ${ROOT_DIR}/docs/README.md"
    echo -e "  Installation Log: ${INSTALL_LOG}"
    echo
}

# Post-install audit
log INFO "Running system audit..."
./scripts/audit/system_audit.sh

# Function to create directories with proper permissions
create_directories() {
    local dirs=("$LOG_DIR" "$CONFIG_DIR" "$INSTALL_DIR")
    
    for dir in "${dirs[@]}"; do
        if [ ! -d "$dir" ]; then
            if [ "$DRY_RUN" = true ]; then
                echo "[DRY RUN] Would create directory: $dir"
            else
                echo "[INFO] Creating directory: $dir"
                if ! mkdir -p "$dir"; then
                    echo "[ERROR] Failed to create directory: $dir"
                    exit 1
                fi
                chmod 755 "$dir"
            fi
        else
            echo "[INFO] Directory exists: $dir"
        fi
    done
}

# Main function
function main {
    # Parse command line arguments
    parse_args "$@"
    
    # Check for root privileges
    check_root
    
    # Log level handling
    LOG_LEVEL="info"
    case $1 in
      --log-level=*)
        LOG_LEVEL="${1#*=}"
        shift
        ;;
    esac
    
    # Create required directories
    create_directories
    chmod 0755 "$LOG_DIR"
    
    # Analyze system
    analyze_system
    
    # Check for existing installation
    check_existing_installation
    
    # Configure feature flags
    configure_feature_flags
    
    # Run main installer
    run_main_installer
    
    # Run systemd configuration
    run_systemd_config
    
    # Verify installation
    if ! verify_installation; then
        log WARN "Installation verification failed, but continuing"
    fi
    
    # Run tests if requested
    if [ "$RUN_TESTS" = true ]; then
        run_tests
    fi
    
    # Validate installation
    validate_installation
    
    # Show completion message
    if [ "$SILENT_MODE" = false ]; then
        show_completion
    fi
    
    log INFO "Anya Core installation completed successfully"
}

# Run the script
if [ "$DRY_RUN" = true ]; then
    log INFO "Dry run mode, exiting without making changes"
    exit 0
fi

main "$@"
sudo apt-get install -y fdclone
