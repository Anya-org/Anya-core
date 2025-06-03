#!/bin/bash
# Anya Core Auto-Configuration Utility
# [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

set -eo pipefail

# Script version
VERSION="1.0.0"

# Directory setup
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
INSTALL_DIR="${PROJECT_ROOT}/scripts/install"
UTILS_DIR="${SCRIPT_DIR}"
CONFIG_DIR="${PROJECT_ROOT}/config"
CONFIG_FILE="${CONFIG_DIR}/anya.conf"
LOG_DIR="${PROJECT_ROOT}/logs"
TIMESTAMP=$(date +"%Y%m%d-%H%M%S")
LOG_FILE="${LOG_DIR}/auto_config_${TIMESTAMP}.log"

# Create log directory if it doesn't exist
mkdir -p "${LOG_DIR}"
touch "$LOG_FILE" 2>/dev/null || true

# Define our own logging function to avoid dependency issues
log() {
    local level="$1"
    local message="$2"
    local timestamp=$(date "+%Y-%m-%d %H:%M:%S")
    echo "[$timestamp] $level: $message"
    echo "[$timestamp] $level: $message" >> "$LOG_FILE" 2>/dev/null || true
}

# Color codes
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
RED="\033[0;31m"
BLUE="\033[0;34m"
NC="\033[0m" # No Color
BOLD="\033[1m"

# Display header
echo -e "${BOLD}================================================================${NC}"
echo -e "${BOLD}        Anya Core Auto-Configuration Utility (v$VERSION)${NC}"
echo -e "${BOLD}================================================================${NC}"
echo
echo -e "${BLUE}Starting auto-configuration at $(date)${NC}"
echo -e "${BLUE}Log: $LOG_FILE${NC}"
echo

# Default configuration values
CONFIG_NETWORK="testnet"
CONFIG_HARDENING="standard"
CONFIG_HSM="auto"
CONFIG_DAO="false"
CONFIG_LIGHTNING="false"
CONFIG_BITCOIN="true"
FORCE_OVERRIDE=false

# Function to check command existence
command_exists() {
    command -v "$1" &> /dev/null
}

# Parse arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --network=*)
                CONFIG_NETWORK="${1#*=}"
                shift
                ;;
            --hardening=*)
                CONFIG_HARDENING="${1#*=}"
                shift
                ;;
            --hsm=*)
                CONFIG_HSM="${1#*=}"
                shift
                ;;
            --dao=*)
                CONFIG_DAO="${1#*=}"
                shift
                ;;
            --lightning=*)
                CONFIG_LIGHTNING="${1#*=}"
                shift
                ;;
            --bitcoin=*)
                CONFIG_BITCOIN="${1#*=}"
                shift
                ;;
            --force)
                FORCE_OVERRIDE=true
                shift
                ;;
            --help)
                show_help
                exit 0
                ;;
            *)
                echo -e "${RED}Unknown option: $1${NC}"
                show_help
                exit 1
                ;;
        esac
    done
}

# Show help
show_help() {
    echo "Anya Core Auto-Configuration Utility v${VERSION}"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --network=NETWORK   Set network (mainnet, testnet, regtest)"
    echo "  --hardening=LEVEL   Set security hardening level (basic, standard, strict)"
    echo "  --hsm=MODE          Set HSM mode (true, false, auto)"
    echo "  --dao=BOOL          Enable/disable DAO governance (true, false)"
    echo "  --lightning=BOOL    Enable/disable Lightning Network (true, false)"
    echo "  --bitcoin=BOOL      Enable/disable Bitcoin integration (true, false)"
    echo "  --force             Force override existing configuration"
    echo "  --help              Display this help message"
    echo ""
    echo "Example:"
    echo "  $0 --network=testnet --hsm=auto"
}

# Detect CPU capabilities
detect_cpu() {
    log INFO "Detecting CPU capabilities..."
    
    # Get CPU information
    CPU_CORES=$(nproc 2>/dev/null || echo 4)
    CPU_MODEL=$(grep "model name" /proc/cpuinfo | head -n1 | cut -d':' -f2 | sed 's/^[ \t]*//')
    
    # Check for AES-NI support (hardware acceleration)
    if grep -q "aes" /proc/cpuinfo; then
        CPU_AES_SUPPORT=true
        log INFO "AES-NI hardware acceleration detected"
    else
        CPU_AES_SUPPORT=false
        log INFO "AES-NI hardware acceleration not detected"
    fi
    
    # Check for AVX support (useful for some crypto operations)
    if grep -q "avx" /proc/cpuinfo; then
        CPU_AVX_SUPPORT=true
        log INFO "AVX support detected"
    else
        CPU_AVX_SUPPORT=false
    fi
    
    log INFO "CPU: $CPU_MODEL with $CPU_CORES cores"
}

# Detect memory
detect_memory() {
    log INFO "Detecting memory..."
    
    # Get memory information
    MEM_TOTAL=$(free -m | awk '/Mem:/ {print $2}')
    log INFO "Total memory: $MEM_TOTAL MB"
    
    # Set memory-based recommendations
    if [ "$MEM_TOTAL" -lt 2048 ]; then
        log WARN "Low memory system detected, recommending minimal installation"
        MEM_RECOMMENDATION="minimal"
    elif [ "$MEM_TOTAL" -lt 4096 ]; then
        log INFO "Medium memory system detected, recommending standard installation"
        MEM_RECOMMENDATION="standard"
    else
        log INFO "High memory system detected, can support full installation"
        MEM_RECOMMENDATION="full"
    fi
}

# Detect HSM/TPM devices
detect_hsm() {
    log INFO "Detecting hardware security modules..."
    
    HSM_AVAILABLE=false
    
    # Check if TPM device exists
    if [ -e /dev/tpm0 ] || [ -e /dev/tpmrm0 ]; then
        log INFO "TPM device detected"
        TPM_AVAILABLE=true
        HSM_AVAILABLE=true
    else
        log INFO "No TPM device detected"
        TPM_AVAILABLE=false
    fi
    
    # Check for YubiKey
    if command_exists ykinfo; then
        if ykinfo -v 2>/dev/null; then
            log INFO "YubiKey detected"
            YUBIKEY_AVAILABLE=true
            HSM_AVAILABLE=true
        else
            log INFO "No YubiKey detected"
            YUBIKEY_AVAILABLE=false
        fi
    else
        YUBIKEY_AVAILABLE=false
    fi
    
    # Check for PKCS#11 libraries
    if command_exists pkcs11-tool; then
        log INFO "PKCS#11 support available"
        PKCS11_AVAILABLE=true
    else
        log INFO "PKCS#11 support not available"
        PKCS11_AVAILABLE=false
    fi
    
    # Set HSM recommendation based on detections
    if [ "$HSM_AVAILABLE" = true ]; then
        log INFO "Hardware security devices available, recommending HSM=true"
        HSM_RECOMMENDATION="true"
    else
        log INFO "No hardware security devices detected, defaulting to software security"
        HSM_RECOMMENDATION="false"
    fi
}

# Detect network capabilities
detect_network() {
    log INFO "Detecting network capabilities..."
    
    # Check for active internet connection
    if ping -c 1 8.8.8.8 >/dev/null 2>&1; then
        log INFO "Internet connectivity verified"
        INTERNET_AVAILABLE=true
    else
        log WARN "No internet connectivity detected"
        INTERNET_AVAILABLE=false
    fi
    
    # Check for Lightning support capabilities
    if command_exists lncli || command_exists lightning-cli; then
        log INFO "Lightning client detected"
        LIGHTNING_CLIENT_AVAILABLE=true
    else
        log INFO "No Lightning client detected"
        LIGHTNING_CLIENT_AVAILABLE=false
    fi
    
    # Set Lightning recommendation
    if [ "$LIGHTNING_CLIENT_AVAILABLE" = true ]; then
        log INFO "Lightning client available, can support Lightning integration"
        LIGHTNING_RECOMMENDATION="true"
    else
        LIGHTNING_RECOMMENDATION="false"
    fi
}

# Generate configuration file
generate_config() {
    log INFO "Generating configuration based on system detection..."
    
    # Create config directory if it doesn't exist
    mkdir -p "$CONFIG_DIR"
    
    # Check if config already exists
    if [ -f "$CONFIG_FILE" ] && [ "$FORCE_OVERRIDE" != true ]; then
        log WARN "Configuration file already exists at $CONFIG_FILE"
        log WARN "Use --force to override existing configuration"
        return 1
    fi
    
    # Apply auto-detection results if set to auto
    if [ "$CONFIG_HSM" = "auto" ]; then
        CONFIG_HSM=$HSM_RECOMMENDATION
        log INFO "Auto-setting HSM to $CONFIG_HSM based on hardware detection"
    fi
    
    if [ "$CONFIG_LIGHTNING" = "auto" ]; then
        CONFIG_LIGHTNING=$LIGHTNING_RECOMMENDATION
        log INFO "Auto-setting Lightning to $CONFIG_LIGHTNING based on system detection"
    fi
    
    # Generate the configuration file
    cat > "$CONFIG_FILE" << EOL
# Anya Core Configuration - Auto-generated on $(date)
# [BPC-3][DAO-4][AIS-3]

[network]
network_type = "$CONFIG_NETWORK"  # Options: "mainnet", "testnet", "regtest"
# Default public RPC endpoints
bitcoin_mainnet_rpc_url = "https://bitcoin-rpc.publicnode.com"
bitcoin_testnet_rpc_url = "https://bitcoin-testnet-rpc.publicnode.com"
# Optional: Override with custom RPC endpoint (if empty, uses appropriate default based on network_type)
bitcoin_custom_rpc_url = ""

[security]
hardening_level = "$CONFIG_HARDENING"  # Options: "basic", "standard", "strict"
keys_rotation_days = 90
encryption_algorithm = "AES256-GCM"
brute_force_protection = true
request_rate_limit = 100
ip_whitelist = ["127.0.0.1", "::1"]

[wallet]
enable_taproot = true
bip370_support = true
coin_selection_strategy = "efficient"

[dao]
quadratic_voting = true
dao_level = "DAO4"
proposal_threshold = 100
voting_period_days = 7
execution_delay_hours = 24

[web5]
did_method = "ion"
dwn_endpoint = "http://localhost:3000"
storage_location = "data/web5"

[ml]
model_path = "data/ml/models"
inference_threads = $CPU_CORES
telemetry_enabled = true

[system_awareness]
mempool_alert_threshold_kb = 100
fee_spike_threshold = 200.0
attack_threshold = 60.0

[performance]
cache_size_mb = 20
batch_size = 100
use_prepared_statements = true

[features]
hsm = $CONFIG_HSM
dao_governance = $CONFIG_DAO
lightning = $CONFIG_LIGHTNING
bitcoin_integration = $CONFIG_BITCOIN
EOL

    log INFO "Configuration file generated at $CONFIG_FILE"
    log INFO "Network: $CONFIG_NETWORK"
    log INFO "Security hardening: $CONFIG_HARDENING"
    log INFO "HSM support: $CONFIG_HSM"
    log INFO "DAO governance: $CONFIG_DAO"
    log INFO "Lightning Network: $CONFIG_LIGHTNING"
    log INFO "Bitcoin integration: $CONFIG_BITCOIN"
    
    echo -e "${GREEN}✓ Configuration successfully generated at $CONFIG_FILE${NC}"
}

# Create necessary directories
create_dirs() {
    log INFO "Creating necessary directories..."
    
    # Create data directories
    mkdir -p "${PROJECT_ROOT}/data/ml/models"
    mkdir -p "${PROJECT_ROOT}/data/web5"
    
    # Create module directories if features enabled
    if [ "$CONFIG_HSM" = "true" ]; then
        mkdir -p "${PROJECT_ROOT}/src/security/hsm"
        log INFO "Created HSM module directory"
    fi
    
    if [ "$CONFIG_DAO" = "true" ]; then
        mkdir -p "${PROJECT_ROOT}/src/dao"
        log INFO "Created DAO module directory"
    fi
    
    if [ "$CONFIG_LIGHTNING" = "true" ]; then
        mkdir -p "${PROJECT_ROOT}/src/lightning"
        log INFO "Created Lightning module directory"
    fi
    
    if [ "$CONFIG_BITCOIN" = "true" ]; then
        mkdir -p "${PROJECT_ROOT}/src/bitcoin"
        log INFO "Created Bitcoin module directory"
    fi
}

# Main function
main() {
    log INFO "Starting Anya Core auto-configuration (version $VERSION)..."
    
    # Parse command line arguments
    parse_args "$@"
    
    # Detect system capabilities
    detect_cpu
    detect_memory
    detect_hsm
    detect_network
    
    # Generate configuration file
    generate_config
    
    # Create necessary directories
    create_dirs
    
    log INFO "Auto-configuration completed successfully"
    echo -e "\n${GREEN}${BOLD}Auto-configuration completed successfully!${NC}"
    echo -e "${BLUE}Configuration file: $CONFIG_FILE${NC}"
    echo -e "${BLUE}Run tests with: sudo ./scripts/test/test_installation.sh --skip-build${NC}"
}

# Run the script
main "$@"
