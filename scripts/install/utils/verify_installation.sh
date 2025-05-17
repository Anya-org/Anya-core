#!/bin/bash
# Anya Core Installation Verification Script
# [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

set -euo pipefail

# Script version
VERSION="1.0.0"

# Directory setup
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PROJECT_ROOT="$(cd "$INSTALL_DIR/../.." && pwd)"
UTILS_DIR="${SCRIPT_DIR}"

# Source common utilities
if [ -f "${UTILS_DIR}/install_common.sh" ]; then
    source "${UTILS_DIR}/install_common.sh"
else
    echo "ERROR: Common utilities not found: ${UTILS_DIR}/install_common.sh"
    exit 1
fi

# Set up process lock
if ! setup_process_lock "verify_installation"; then
    exit 1
fi

# Display header
print_header "Installation Verification" "$VERSION"

# Check if Anya Core is installed
check_installation_exists() {
    local binary_path="${PROJECT_ROOT}/target/release/anya-core"
    local config_path="${PROJECT_ROOT}/config/anya.conf"
    local service_exists=$(systemctl list-unit-files | grep -q anya-core.service && echo "true" || echo "false")
    
    log INFO "Checking for Anya Core installation..."
    
    if [ -f "$binary_path" ] && [ -x "$binary_path" ]; then
        log INFO "Binary: ✅ Found at $binary_path"
        BINARY_EXISTS=true
    else
        log ERROR "Binary: ❌ Not found at $binary_path"
        BINARY_EXISTS=false
    fi
    
    if [ -f "$config_path" ]; then
        log INFO "Configuration: ✅ Found at $config_path"
        CONFIG_EXISTS=true
    else
        log ERROR "Configuration: ❌ Not found at $config_path"
        CONFIG_EXISTS=false
    fi
    
    if [ "$service_exists" = "true" ]; then
        log INFO "Service: ✅ Systemd service installed"
        SERVICE_EXISTS=true
    else
        log ERROR "Service: ❌ Systemd service not installed"
        SERVICE_EXISTS=false
    fi
    
    if [ "$BINARY_EXISTS" = "true" ] && [ "$CONFIG_EXISTS" = "true" ] && [ "$SERVICE_EXISTS" = "true" ]; then
        return 0
    else
        log ERROR "Anya Core is not fully installed"
        return 1
    fi
}

# Check installed version and features
check_version_and_features() {
    local binary_path="${PROJECT_ROOT}/target/release/anya-core"
    
    if [ ! -f "$binary_path" ] || [ ! -x "$binary_path" ]; then
        log ERROR "Cannot check version: binary not found or not executable"
        return 1
    fi
    
    log INFO "Checking Anya Core version and features..."
    
    # Get version
    VERSION_INFO=$("$binary_path" --version 2>/dev/null || echo "Unknown")
    log INFO "Version: $VERSION_INFO"
    
    # Check for feature flag support in binary
    if "$binary_path" --help 2>&1 | grep -q -- "--show-features"; then
        FEATURES=$("$binary_path" --show-features 2>/dev/null || echo "Unknown")
        log INFO "Compiled Features: $FEATURES"
        
        # Check for specific features
        if echo "$FEATURES" | grep -q "hsm"; then
            log INFO "HSM Feature: ✅ Enabled"
            HSM_ENABLED=true
        else
            log WARN "HSM Feature: ❌ Not enabled"
            HSM_ENABLED=false
        fi
        
        if echo "$FEATURES" | grep -q "bitcoin_integration"; then
            log INFO "Bitcoin Integration: ✅ Enabled"
            BITCOIN_ENABLED=true
        else
            log WARN "Bitcoin Integration: ❌ Not enabled"
            BITCOIN_ENABLED=false
        fi
    else
        log WARN "Feature detection not supported by binary"
        HSM_ENABLED=false
        BITCOIN_ENABLED=false
    fi
    
    # Check for installation type based on feature combination
    if [ "$HSM_ENABLED" = "true" ] && [ "$BITCOIN_ENABLED" = "true" ]; then
        log INFO "Installation Type: FULL"
    elif [ "$HSM_ENABLED" = "true" ]; then
        log INFO "Installation Type: STANDARD with HSM"
    elif [ "$BITCOIN_ENABLED" = "true" ]; then
        log INFO "Installation Type: STANDARD with Bitcoin"
    else
        log INFO "Installation Type: MINIMAL"
    fi
    
    return 0
}

# Check if hardware capabilities match enabled features
check_hardware_alignment() {
    log INFO "Checking hardware and feature alignment..."
    
    # Detect hardware capabilities
    local has_tpm=$(detect_tpm)
    local has_yubikey=$(detect_yubikey)
    local has_pkcs11=$(detect_pkcs11_modules)
    local has_hw_wallet=$(detect_hw_wallets)
    
    # Check if any hardware security device is available
    if [ "$has_tpm" = "true" ] || [ "$has_yubikey" = "true" ] || [ "$has_pkcs11" = "true" ] || [ "$has_hw_wallet" = "true" ]; then
        log INFO "Hardware Security: ✅ Detected"
        HW_SECURITY_AVAILABLE=true
    else
        log INFO "Hardware Security: ❌ Not detected"
        HW_SECURITY_AVAILABLE=false
    fi
    
    # Check for alignment between hardware and features
    if [ "$HW_SECURITY_AVAILABLE" = "true" ] && [ "$HSM_ENABLED" = "true" ]; then
        log INFO "Hardware-Feature Alignment: ✅ Optimal (HSM hardware detected and feature enabled)"
        ALIGNMENT="optimal"
    elif [ "$HW_SECURITY_AVAILABLE" = "true" ] && [ "$HSM_ENABLED" = "false" ]; then
        log WARN "Hardware-Feature Alignment: ⚠️ Suboptimal (HSM hardware detected but feature not enabled)"
        ALIGNMENT="suboptimal"
    elif [ "$HW_SECURITY_AVAILABLE" = "false" ] && [ "$HSM_ENABLED" = "true" ]; then
        log INFO "Hardware-Feature Alignment: ℹ️ Using software HSM (no HSM hardware detected but feature enabled)"
        ALIGNMENT="software_fallback"
    else
        log INFO "Hardware-Feature Alignment: ✅ Consistent (no HSM hardware and feature not enabled)"
        ALIGNMENT="consistent"
    fi
    
    # Print hardware details
    if [ "$has_tpm" = "true" ]; then
        log INFO "TPM: ✅ Detected"
    fi
    
    if [ "$has_yubikey" = "true" ]; then
        log INFO "YubiKey: ✅ Detected"
    fi
    
    if [ "$has_pkcs11" = "true" ]; then
        log INFO "PKCS#11 Module: ✅ Detected"
    fi
    
    if [ "$has_hw_wallet" = "true" ]; then
        log INFO "Hardware Wallet: ✅ Detected"
    fi
    
    return 0
}

# Check if service is running and properly configured
check_service_health() {
    log INFO "Checking service health..."
    
    # Check if service is active
    if systemctl is-active --quiet anya-core.service; then
        log INFO "Service Status: ✅ Running"
        SERVICE_RUNNING=true
    else
        log ERROR "Service Status: ❌ Not running"
        SERVICE_RUNNING=false
        return 1
    fi
    
    # Check service configuration
    if systemctl cat anya-core.service &>/dev/null; then
        # Check memory limits
        MEMORY_LIMIT=$(systemctl cat anya-core.service | grep -i "MemoryMax" | awk '{print $2}')
        if [ -n "$MEMORY_LIMIT" ]; then
            log INFO "Memory Limit: $MEMORY_LIMIT"
        else
            log WARN "Memory Limit: Not configured"
        fi
        
        # Check CPU limits
        CPU_LIMIT=$(systemctl cat anya-core.service | grep -i "CPUQuota" | awk '{print $2}')
        if [ -n "$CPU_LIMIT" ]; then
            log INFO "CPU Quota: $CPU_LIMIT"
        else
            log WARN "CPU Quota: Not configured"
        fi
        
        # Check restart policy
        RESTART_POLICY=$(systemctl cat anya-core.service | grep -i "Restart=" | awk -F= '{print $2}')
        if [ -n "$RESTART_POLICY" ]; then
            log INFO "Restart Policy: $RESTART_POLICY"
        else
            log WARN "Restart Policy: Not configured"
        fi
    fi
    
    # Check API accessibility
    if curl -s -o /dev/null -w "%{http_code}" http://localhost:3300/health --connect-timeout 5 &>/dev/null; then
        STATUS_CODE=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:3300/health --connect-timeout 5)
        if [ "$STATUS_CODE" = "200" ]; then
            log INFO "API Health: ✅ Responding ($STATUS_CODE)"
            API_HEALTHY=true
        else
            log WARN "API Health: ⚠️ Responding with error ($STATUS_CODE)"
            API_HEALTHY=false
        fi
    else
        log ERROR "API Health: ❌ Not responding"
        API_HEALTHY=false
    fi
    
    return 0
}

# Check configuration settings
check_configuration() {
    local config_file="${PROJECT_ROOT}/config/anya.conf"
    
    if [ ! -f "$config_file" ]; then
        log ERROR "Configuration file not found: $config_file"
        return 1
    fi
    
    log INFO "Checking configuration..."
    
    # Check network setting
    if grep -q "^network=" "$config_file" || grep -q "^\[network\]" "$config_file"; then
        if grep -q "^network=" "$config_file"; then
            NETWORK=$(grep "^network=" "$config_file" | awk -F= '{print $2}')
            log INFO "Network: $NETWORK"
        elif grep -q "^\[network\]" "$config_file"; then
            # Handle INI-style config
            NETWORK=$(awk '/^\[network\]/{flag=1;next}/^\[/{flag=0}flag&&/network=/{print $0}' "$config_file" | awk -F= '{print $2}')
            if [ -n "$NETWORK" ]; then
                log INFO "Network: $NETWORK"
            else
                log WARN "Network not specified in configuration"
            fi
        fi
    else
        log WARN "Network configuration not found"
    fi
    
    # Check for HSM configuration
    if grep -q "enable_hsm=true" "$config_file" || grep -q "hsm_enabled=true" "$config_file"; then
        log INFO "HSM: ✅ Enabled in configuration"
        CONFIG_HSM_ENABLED=true
        
        # Check HSM provider
        if grep -q "hsm_provider=" "$config_file"; then
            HSM_PROVIDER=$(grep "hsm_provider=" "$config_file" | awk -F= '{print $2}')
            log INFO "HSM Provider: $HSM_PROVIDER"
        else
            log WARN "HSM Provider not specified"
        fi
    else
        log INFO "HSM: ❌ Not enabled in configuration"
        CONFIG_HSM_ENABLED=false
    fi
    
    # Check feature flags in configuration
    if grep -q "feature_flags=" "$config_file"; then
        CONFIG_FEATURES=$(grep "feature_flags=" "$config_file" | awk -F= '{print $2}')
        log INFO "Configured Feature Flags: $CONFIG_FEATURES"
    fi
    
    return 0
}

# Check for critical files
check_critical_files() {
    log INFO "Checking for critical files..."
    
    # Define critical files
    declare -a CRITICAL_FILES=(
        "${PROJECT_ROOT}/target/release/anya-core"
        "${PROJECT_ROOT}/config/anya.conf"
    )
    
    # Define critical directories
    declare -a CRITICAL_DIRS=(
        "${PROJECT_ROOT}/data"
        "${PROJECT_ROOT}/logs"
        "${PROJECT_ROOT}/var"
    )
    
    # Check files
    for file in "${CRITICAL_FILES[@]}"; do
        if [ -f "$file" ]; then
            log INFO "File: ✅ $file"
        else
            log ERROR "File: ❌ $file (missing)"
        fi
    done
    
    # Check directories
    for dir in "${CRITICAL_DIRS[@]}"; do
        if [ -d "$dir" ]; then
            log INFO "Directory: ✅ $dir"
        else
            log WARN "Directory: ❌ $dir (missing)"
        fi
    done
    
    return 0
}

# Verify permissions
check_permissions() {
    log INFO "Checking file permissions..."
    
    # Check binary permissions
    local binary="${PROJECT_ROOT}/target/release/anya-core"
    if [ -f "$binary" ]; then
        if [ -x "$binary" ]; then
            log INFO "Binary permissions: ✅ Executable"
        else
            log ERROR "Binary permissions: ❌ Not executable"
            chmod +x "$binary"
            log INFO "Fixed binary permissions"
        fi
        
        # Check ownership
        OWNER=$(stat -c '%U' "$binary")
        GROUP=$(stat -c '%G' "$binary")
        log INFO "Binary ownership: $OWNER:$GROUP"
    fi
    
    # Check config directory permissions
    local config_dir="${PROJECT_ROOT}/config"
    if [ -d "$config_dir" ]; then
        PERMS=$(stat -c '%a' "$config_dir")
        if [ "$PERMS" -le "755" ]; then
            log INFO "Config directory permissions: ✅ $PERMS (secure)"
        else
            log WARN "Config directory permissions: ⚠️ $PERMS (too open)"
        fi
    fi
    
    # Check data directory permissions
    local data_dir="${PROJECT_ROOT}/data"
    if [ -d "$data_dir" ]; then
        PERMS=$(stat -c '%a' "$data_dir")
        if [ "$PERMS" -le "755" ]; then
            log INFO "Data directory permissions: ✅ $PERMS (secure)"
        else
            log WARN "Data directory permissions: ⚠️ $PERMS (too open)"
        fi
    fi
    
    return 0
}

# Check core components based on feature flags
check_core_components() {
    log INFO "Checking core components..."
    
    # Check for Bitcoin integration
    if [ "$BITCOIN_ENABLED" = "true" ]; then
        log INFO "Checking Bitcoin components..."
        
        # Check Bitcoin RPC configuration
        if grep -q "rpc_url=" "${PROJECT_ROOT}/config/anya.conf"; then
            BITCOIN_RPC=$(grep "rpc_url=" "${PROJECT_ROOT}/config/anya.conf" | awk -F= '{print $2}')
            log INFO "Bitcoin RPC URL: $BITCOIN_RPC"
            
            # Simple check if Bitcoin RPC is accessible
            if curl -s -o /dev/null -w "%{http_code}" "$BITCOIN_RPC" --connect-timeout 2 &>/dev/null; then
                STATUS_CODE=$(curl -s -o /dev/null -w "%{http_code}" "$BITCOIN_RPC" --connect-timeout 2)
                log INFO "Bitcoin RPC: ✅ Accessible ($STATUS_CODE)"
            else
                log WARN "Bitcoin RPC: ❌ Not accessible"
            fi
        else
            log WARN "Bitcoin RPC URL not configured"
        fi
    fi
    
    # Check for HSM components
    if [ "$HSM_ENABLED" = "true" ]; then
        log INFO "Checking HSM components..."
        
        # Check HSM provider directory exists
        if [ -d "${PROJECT_ROOT}/src/security/hsm/providers" ]; then
            log INFO "HSM Providers: ✅ Directory exists"
            
            # Try to determine which provider is in use
            if [ "$HW_SECURITY_AVAILABLE" = "true" ]; then
                if [ "$(detect_tpm)" = "true" ]; then
                    log INFO "Active HSM: Likely using TPM provider"
                elif [ "$(detect_yubikey)" = "true" ]; then
                    log INFO "Active HSM: Likely using PKCS#11 or YubiKey provider"
                elif [ "$(detect_hw_wallets)" = "true" ]; then
                    log INFO "Active HSM: Likely using hardware wallet provider"
                fi
            else
                log INFO "Active HSM: Likely using software provider"
            fi
        else
            log WARN "HSM Providers directory not found"
        fi
    fi
    
    # Check DAO Governance (if installed)
    if [ -d "${PROJECT_ROOT}/src/dao" ]; then
        log INFO "DAO Governance: ✅ Component installed"
    fi
    
    # Check Lightning Integration (if installed)
    if [ -d "${PROJECT_ROOT}/src/layer2/lightning" ]; then
        log INFO "Lightning Integration: ✅ Component installed"
    fi
    
    return 0
}

# Display verification results
display_results() {
    echo
    echo "================================================================"
    echo "                 Installation Verification Results"
    echo "================================================================"
    echo
    echo "Binary: ${BINARY_EXISTS:-false}"
    echo "Configuration: ${CONFIG_EXISTS:-false}"
    echo "Service: ${SERVICE_EXISTS:-false}"
    echo "Service Running: ${SERVICE_RUNNING:-false}"
    echo "API Healthy: ${API_HEALTHY:-false}"
    echo
    echo "HSM Enabled (Binary): ${HSM_ENABLED:-false}"
    echo "HSM Enabled (Config): ${CONFIG_HSM_ENABLED:-false}"
    echo "Hardware Security Available: ${HW_SECURITY_AVAILABLE:-false}"
    echo "Hardware-Feature Alignment: ${ALIGNMENT:-unknown}"
    echo
    echo "Version: ${VERSION_INFO:-unknown}"
    echo "Features: ${FEATURES:-unknown}"
    echo
    echo "Next Steps:"
    if [ "${BINARY_EXISTS:-false}" = "true" ] && [ "${SERVICE_RUNNING:-false}" = "true" ] && [ "${API_HEALTHY:-false}" = "true" ]; then
        echo "✅ Installation appears to be functioning correctly"
        
        if [ "${ALIGNMENT:-unknown}" = "suboptimal" ]; then
            echo "⚠️ Consider reinstalling with HSM support to leverage available hardware:"
            echo "   sudo ./scripts/install.sh --features=std,hsm,bitcoin_integration"
        fi
    else
        echo "❌ Installation has issues that need to be addressed"
        
        if [ "${BINARY_EXISTS:-false}" = "false" ]; then
            echo "   - Binary missing, try reinstalling"
        fi
        
        if [ "${SERVICE_RUNNING:-false}" = "false" ]; then
            echo "   - Service not running, try: sudo systemctl start anya-core.service"
        fi
        
        if [ "${API_HEALTHY:-false}" = "false" ]; then
            echo "   - API not responding, check logs: journalctl -u anya-core.service"
        fi
    fi
    echo
    echo "For detailed health monitoring, run:"
    echo "  sudo ./scripts/install/utils/monitor_health.sh"
    echo
    echo "================================================================"
}

# Run all checks
run_verification() {
    # Initialize variables with default values
    BINARY_EXISTS=false
    CONFIG_EXISTS=false
    SERVICE_EXISTS=false
    SERVICE_RUNNING=false
    API_HEALTHY=false
    HSM_ENABLED=false
    BITCOIN_ENABLED=false
    CONFIG_HSM_ENABLED=false
    HW_SECURITY_AVAILABLE=false
    ALIGNMENT="unknown"
    VERSION_INFO="unknown"
    FEATURES="unknown"
    
    # Run all verification checks
    check_installation_exists
    check_version_and_features
    check_hardware_alignment
    check_service_health || true  # Continue even if service check fails
    check_configuration || true   # Continue even if config check fails
    check_critical_files
    check_permissions
    check_core_components
    
    # Display results
    display_results
}

# Run verification with output formatting
echo
log INFO "Starting Anya Core installation verification..."
echo

run_verification
