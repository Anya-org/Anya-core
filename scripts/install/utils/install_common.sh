#!/bin/bash
# Common Utilities for Anya Core Installation Scripts
# [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

# Version
UTILS_VERSION="1.0.0"

# Common paths
PROJECT_ROOT="${PROJECT_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../" && pwd)}"
LOGS_DIR="${PROJECT_ROOT}/logs"
INSTALL_LOGS_DIR="${LOGS_DIR}/installation"
CONFIG_DIR="${PROJECT_ROOT}/config"
DATA_DIR="${PROJECT_ROOT}/data"
VAR_DIR="${PROJECT_ROOT}/var"
BACKUPS_DIR="${VAR_DIR}/backups"
VERSIONS_DIR="${VAR_DIR}/versions"

# Ensure directories exist
mkdir -p "$LOGS_DIR" "$INSTALL_LOGS_DIR" "$CONFIG_DIR" "$DATA_DIR" "$VAR_DIR" "$BACKUPS_DIR" "$VERSIONS_DIR"

# Common colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Process locking function
setup_process_lock() {
    local lock_name="$1"
    local lock_file="/tmp/anya_${lock_name}.lock"
    
    # Check if another instance is running
    if [ -e "$lock_file" ]; then
        local pid=$(cat "$lock_file")
        if ps -p "$pid" > /dev/null 2>&1; then
            echo -e "${RED}ERROR: Another instance of the $lock_name process is already running (PID: $pid).${NC}"
            echo "If this is an error, remove the lock file: $lock_file"
            return 1
        else
            echo -e "${YELLOW}WARNING: Found stale lock file. Removing and continuing.${NC}"
            rm -f "$lock_file"
        fi
    fi
    
    # Create lock file with current PID
    echo $$ > "$lock_file"
    
    # Set up cleanup on exit
    eval "remove_${lock_name}_lock() { rm -f \"$lock_file\"; }"
    trap "remove_${lock_name}_lock" EXIT INT TERM
    
    return 0
}

# Logging function with timestamp and level
log() {
    local level=$1
    shift
    local message=$*
    local timestamp
    timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    local log_file="${INSTALL_LOGS_DIR}/install_$(date +%Y%m%d).log"
    
    case $level in
        ERROR) printf "${RED}[%s] ERROR: %s${NC}\n" "$timestamp" "$message" | tee -a "$log_file" ;;
        WARN)  printf "${YELLOW}[%s] WARN: %s${NC}\n" "$timestamp" "$message" | tee -a "$log_file" ;;
        INFO)  printf "${GREEN}[%s] INFO: %s${NC}\n" "$timestamp" "$message" | tee -a "$log_file" ;;
        DEBUG) printf "${BLUE}[%s] DEBUG: %s${NC}\n" "$timestamp" "$message" | tee -a "$log_file" ;;
    esac
    
    # Also log errors and warnings to stderr
    if [[ "$level" == "ERROR" || "$level" == "WARN" ]]; then
        printf "${RED}[%s] %s: %s${NC}\n" "$timestamp" "$level" "$message" >&2
    fi
}

# Hardware detection functions
detect_tpm() {
    if [ -e "/dev/tpm0" ] || [ -e "/dev/tpmrm0" ]; then
        echo "true"
    else
        echo "false"
    fi
}

detect_yubikey() {
    if lsusb 2>/dev/null | grep -qi "yubico"; then
        echo "true"
    else
        echo "false"
    fi
}

detect_pkcs11_modules() {
    for lib in /usr/lib*/libykcs11.so* /usr/lib*/libtpm2_pkcs11.so* /usr/local/lib*/libykcs11.so*; do
        if [ -f "$lib" ]; then
            echo "true"
            return
        fi
    done
    echo "false"
}

detect_hw_wallets() {
    if lsusb 2>/dev/null | grep -qi "ledger\|trezor"; then
        echo "true"
    else
        echo "false"
    fi
}

# System resource detection
get_cpu_cores() {
    nproc
}

get_total_memory_mb() {
    free -m | awk '/^Mem:/{print $2}'
}

get_available_disk_gb() {
    df -BG "$PROJECT_ROOT" | awk 'NR==2 {gsub("G", "", $4); print $4}'
}

# Version control functions
save_version_info() {
    local feature_flags=$1
    local network=$2
    
    log INFO "Saving version information for future upgrades..."
    
    # Create version info directory if it doesn't exist
    mkdir -p "${VERSIONS_DIR}"
    
    # Get git information if available
    local git_commit="unknown"
    local git_branch="unknown"
    if command -v git >/dev/null && [ -d "${PROJECT_ROOT}/.git" ]; then
        git_commit=$(git -C "${PROJECT_ROOT}" rev-parse HEAD 2>/dev/null || echo "unknown")
        git_branch=$(git -C "${PROJECT_ROOT}" rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
    fi
    
    # Get binary version if available
    local binary_version="unknown"
    if [ -x "${PROJECT_ROOT}/target/release/anya-core" ]; then
        binary_version=$("${PROJECT_ROOT}/target/release/anya-core" --version 2>/dev/null | head -1 || echo "unknown")
    fi
    
    # Create version file
    local version_file="${VERSIONS_DIR}/version_$(date +%Y%m%d-%H%M%S).json"
    cat > "$version_file" << EOF
{
    "installation_date": "$(date -Iseconds)",
    "git_commit": "$git_commit",
    "git_branch": "$git_branch",
    "binary_version": "$binary_version",
    "feature_flags": "$feature_flags",
    "network": "$network",
    "system": {
        "cpu_cores": "$(get_cpu_cores)",
        "memory_mb": "$(get_total_memory_mb)",
        "disk_gb": "$(get_available_disk_gb)",
        "tpm_available": "$(detect_tpm)",
        "yubikey_available": "$(detect_yubikey)",
        "pkcs11_available": "$(detect_pkcs11_modules)",
        "hw_wallets_available": "$(detect_hw_wallets)"
    }
}
EOF
    
    log INFO "Version information saved to $version_file"
    
    # Create a symlink to the latest version file
    ln -sf "$version_file" "${VERSIONS_DIR}/latest.json"
}

# Check for root privileges
check_root() {
    if [ "$EUID" -ne 0 ]; then
        log ERROR "This script requires root privileges. Please run with sudo."
        return 1
    fi
    return 0
}

# Display script or process information
print_header() {
    local script_name=$1
    local version=$2
    
    echo "================================================================"
    echo "        Anya Core $script_name (v$version)"
    echo "================================================================"
    echo
    echo "Date: $(date)"
    echo "System: $(uname -a)"
    echo "User: $(whoami)"
    echo
}

# Export all functions
export -f setup_process_lock
export -f log
export -f detect_tpm
export -f detect_yubikey
export -f detect_pkcs11_modules
export -f detect_hw_wallets
export -f get_cpu_cores
export -f get_total_memory_mb
export -f get_available_disk_gb
export -f save_version_info
export -f check_root
export -f print_header

# Print utility information if script is run directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    print_header "Installation Utilities" "$UTILS_VERSION"
    echo "This script provides common utilities for the Anya Core installation process."
    echo "It is meant to be sourced by other installation scripts, not run directly."
    echo
    echo "Available functions:"
    echo "- setup_process_lock: Prevent multiple instances of a script"
    echo "- log: Unified logging with level and timestamp"
    echo "- detect_tpm/yubikey/pkcs11/hw_wallets: Hardware detection functions"
    echo "- get_cpu_cores/total_memory_mb/available_disk_gb: System resource detection"
    echo "- save_version_info: Track installation versions"
    echo "- check_root: Verify root privileges"
    echo "- print_header: Display script information"
    echo
    echo "To use these utilities, source this script in your installation script:"
    echo "  source \"$(readlink -f "${BASH_SOURCE[0]}")\""
    echo
    echo "================================================================"
fi
