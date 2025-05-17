#!/bin/bash
# Anya Core Health Monitoring Utility
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
if ! setup_process_lock "health_monitor"; then
    exit 1
fi

# Display header
print_header "Health Monitoring Utility" "$VERSION"

# Check if Anya Core service is running
check_service_status() {
    log INFO "Checking Anya Core service status..."
    
    if systemctl is-active --quiet anya-core.service; then
        log INFO "Service Status: ✅ RUNNING"
        return 0
    else
        log ERROR "Service Status: ❌ NOT RUNNING"
        return 1
    fi
}

# Check service configuration
check_service_config() {
    log INFO "Checking service configuration..."
    
    if systemctl cat anya-core.service &>/dev/null; then
        log INFO "Service Configuration: ✅ PRESENT"
        
        # Check memory limits
        memory_limit=$(systemctl cat anya-core.service | grep -i "MemoryMax" | awk '{print $2}')
        if [ -n "$memory_limit" ]; then
            log INFO "Memory Limit: $memory_limit"
        else
            log WARN "Memory Limit: ❌ NOT CONFIGURED"
        fi
        
        # Check CPU limits
        cpu_limit=$(systemctl cat anya-core.service | grep -i "CPUQuota" | awk '{print $2}')
        if [ -n "$cpu_limit" ]; then
            log INFO "CPU Quota: $cpu_limit"
        else
            log WARN "CPU Quota: ❌ NOT CONFIGURED"
        fi
        
        return 0
    else
        log ERROR "Service Configuration: ❌ NOT FOUND"
        return 1
    fi
}

# Check binary existence and features
check_binary() {
    local binary_path="${PROJECT_ROOT}/target/release/anya-core"
    log INFO "Checking Anya Core binary..."
    
    if [ -x "$binary_path" ]; then
        log INFO "Binary Status: ✅ PRESENT"
        
        # Get version information
        version_info=$("$binary_path" --version 2>/dev/null || echo "Unable to determine version")
        log INFO "Version: $version_info"
        
        # Get feature information if available
        if "$binary_path" --show-features &>/dev/null; then
            features=$("$binary_path" --show-features 2>/dev/null)
            log INFO "Features: $features"
        else
            log WARN "Feature information not available"
        fi
        
        return 0
    else
        log ERROR "Binary Status: ❌ NOT FOUND"
        return 1
    fi
}

# Check API health
check_api_health() {
    log INFO "Checking API health..."
    
    # Try to connect to health endpoint
    if curl -s -o /dev/null -w "%{http_code}" http://localhost:3300/health --connect-timeout 5 &>/dev/null; then
        status_code=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:3300/health --connect-timeout 5)
        if [ "$status_code" = "200" ]; then
            log INFO "API Status: ✅ HEALTHY ($status_code)"
            return 0
        else
            log WARN "API Status: ⚠️ RESPONDING WITH ERROR ($status_code)"
            return 1
        fi
    else
        log ERROR "API Status: ❌ NOT RESPONDING"
        return 1
    fi
}

# Check system resources
check_system_resources() {
    log INFO "Checking system resources..."
    
    # Check CPU usage
    cpu_usage=$(top -bn1 | grep "Cpu(s)" | awk '{print $2 + $4}')
    log INFO "CPU Usage: ${cpu_usage}%"
    
    # Check memory usage
    mem_total=$(free -m | awk '/^Mem:/{print $2}')
    mem_used=$(free -m | awk '/^Mem:/{print $3}')
    mem_usage=$((mem_used * 100 / mem_total))
    log INFO "Memory Usage: ${mem_usage}% (${mem_used}MB / ${mem_total}MB)"
    
    # Check disk usage
    disk_usage=$(df -h "${PROJECT_ROOT}" | awk 'NR==2 {print $5}')
    disk_avail=$(df -h "${PROJECT_ROOT}" | awk 'NR==2 {print $4}')
    log INFO "Disk Usage: ${disk_usage} (${disk_avail} available)"
    
    # Check for resource constraints
    if [ "$mem_usage" -gt 90 ]; then
        log WARN "Memory usage is very high (${mem_usage}%)"
    fi
    
    if [ "${disk_usage%\%}" -gt 90 ]; then
        log WARN "Disk usage is very high (${disk_usage})"
    fi
}

# Check service logs for errors
check_service_logs() {
    log INFO "Checking service logs for errors..."
    
    # Get the last 50 lines of logs
    if journalctl -u anya-core.service -n 50 --no-pager &>/dev/null; then
        error_count=$(journalctl -u anya-core.service -n 50 --no-pager | grep -i "error\|fatal\|panic" | wc -l)
        warn_count=$(journalctl -u anya-core.service -n 50 --no-pager | grep -i "warn\|warning" | wc -l)
        
        if [ "$error_count" -gt 0 ]; then
            log WARN "Found $error_count error messages in recent logs"
            log INFO "Sample errors:"
            journalctl -u anya-core.service -n 50 --no-pager | grep -i "error\|fatal\|panic" | head -3
        else
            log INFO "No errors found in recent logs"
        fi
        
        if [ "$warn_count" -gt 0 ]; then
            log INFO "Found $warn_count warning messages in recent logs"
        fi
    else
        log WARN "Could not access service logs"
    fi
}

# Check if system has HSM hardware
check_hsm_hardware() {
    log INFO "Checking HSM hardware availability..."
    
    # Check for TPM
    if [ "$(detect_tpm)" = "true" ]; then
        log INFO "TPM: ✅ DETECTED"
    else
        log INFO "TPM: ❌ NOT DETECTED"
    fi
    
    # Check for YubiKey
    if [ "$(detect_yubikey)" = "true" ]; then
        log INFO "YubiKey: ✅ DETECTED"
    else
        log INFO "YubiKey: ❌ NOT DETECTED"
    fi
    
    # Check for PKCS#11 modules
    if [ "$(detect_pkcs11_modules)" = "true" ]; then
        log INFO "PKCS#11 Modules: ✅ DETECTED"
    else
        log INFO "PKCS#11 Modules: ❌ NOT DETECTED"
    fi
    
    # Check for hardware wallets
    if [ "$(detect_hw_wallets)" = "true" ]; then
        log INFO "Hardware Wallets: ✅ DETECTED"
    else
        log INFO "Hardware Wallets: ❌ NOT DETECTED"
    fi
}

# Run all health checks
run_health_checks() {
    check_service_status
    check_service_config
    check_binary
    check_api_health
    check_system_resources
    check_service_logs
    check_hsm_hardware
}

# Enable continuous monitoring if requested
if [ $# -gt 0 ] && [ "$1" = "--monitor" ]; then
    log INFO "Starting continuous monitoring mode (press Ctrl+C to exit)..."
    interval=${2:-60}  # Default refresh interval: 60 seconds
    
    while true; do
        clear
        print_header "Health Monitoring Utility (Continuous Mode)" "$VERSION"
        log INFO "Refresh interval: ${interval}s"
        echo
        
        run_health_checks
        
        echo
        log INFO "Next check in ${interval}s (press Ctrl+C to exit)..."
        sleep $interval
    done
else
    # Run a single health check
    run_health_checks
    
    # Print summary
    echo
    echo "================================================================"
    echo "                      Health Check Summary"
    echo "================================================================"
    echo
    echo "To monitor continuously, run: $0 --monitor [interval_seconds]"
    echo "For more detailed diagnostics, run: ${PROJECT_ROOT}/scripts/test/debug_test.sh"
    echo
fi
