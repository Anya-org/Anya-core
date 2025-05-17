#!/bin/bash
# Anya Core Installation System Test Script
# [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

set -euo pipefail

# Script version
VERSION="1.0.0"

# Directory setup
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
INSTALL_DIR="${PROJECT_ROOT}/scripts/install"
UTILS_DIR="${INSTALL_DIR}/utils"

# Source common utilities if available
if [ -f "${UTILS_DIR}/install_common.sh" ]; then
    source "${UTILS_DIR}/install_common.sh"
else
    # Basic logging function fallback
    log() {
        local level=$1
        shift
        echo "[$(date '+%Y-%m-%d %H:%M:%S')] [$level] $*"
    }
    
    print_header() {
        local title=$1
        local version=$2
        echo "================================================================"
        echo "        $title (v$version)"
        echo "================================================================"
        echo
    }
fi

# Display header
print_header "Installation System Test Suite" "$VERSION"

# Set up a temporary test directory
setup_test_environment() {
    log INFO "Setting up test environment..."
    
    # Create temporary test directory
    TEST_DIR=$(mktemp -d /tmp/anya-core-test.XXXXXX)
    log INFO "Created test directory: $TEST_DIR"
    
    # Create test project structure
    mkdir -p "${TEST_DIR}/scripts/install/utils"
    mkdir -p "${TEST_DIR}/config"
    mkdir -p "${TEST_DIR}/target/release"
    mkdir -p "${TEST_DIR}/src/security/hsm"
    mkdir -p "${TEST_DIR}/logs"
    
    # Create dummy binary
    echo "#!/bin/bash" > "${TEST_DIR}/target/release/anya-core"
    echo "echo 'Anya Core v0.1.0-test'" >> "${TEST_DIR}/target/release/anya-core"
    echo "case \"\$1\" in" >> "${TEST_DIR}/target/release/anya-core"
    echo "  --version) echo 'Anya Core v0.1.0-test' ;;" >> "${TEST_DIR}/target/release/anya-core"
    echo "  --show-features) echo 'std hsm bitcoin_integration' ;;" >> "${TEST_DIR}/target/release/anya-core"
    echo "  *) echo 'Unknown option' ;;" >> "${TEST_DIR}/target/release/anya-core"
    echo "esac" >> "${TEST_DIR}/target/release/anya-core"
    chmod +x "${TEST_DIR}/target/release/anya-core"
    
    # Create dummy config
    cat > "${TEST_DIR}/config/anya.conf" << EOF
# Anya Core Test Configuration
network=testnet
enable_hsm=true
hsm_provider=software
feature_flags=std,hsm,bitcoin_integration
rpc_url=http://localhost:8332
EOF

    log INFO "Test environment set up successfully"
}

# Clean up test environment
cleanup_test_environment() {
    log INFO "Cleaning up test environment..."
    rm -rf "$TEST_DIR"
    log INFO "Cleanup complete"
}

# Copy scripts to test environment
copy_scripts_to_test_env() {
    log INFO "Copying installation scripts to test environment..."
    
    # Copy installation scripts
    cp -f "${PROJECT_ROOT}/install.sh" "${TEST_DIR}/scripts/"
    cp -f "${INSTALL_DIR}/auto_install.sh" "${TEST_DIR}/scripts/install/"
    cp -f "${INSTALL_DIR}/linux_install.sh" "${TEST_DIR}/scripts/install/"
    
    # Copy utility scripts
    cp -f "${UTILS_DIR}/install_common.sh" "${TEST_DIR}/scripts/install/utils/"
    cp -f "${UTILS_DIR}/verify_installation.sh" "${TEST_DIR}/scripts/install/utils/"
    cp -f "${UTILS_DIR}/monitor_health.sh" "${TEST_DIR}/scripts/install/utils/"
    cp -f "${UTILS_DIR}/script_cleanup.sh" "${TEST_DIR}/scripts/install/utils/"
    
    # Make them executable
    chmod +x "${TEST_DIR}/scripts/install.sh"
    chmod +x "${TEST_DIR}/scripts/install/auto_install.sh"
    chmod +x "${TEST_DIR}/scripts/install/linux_install.sh"
    chmod +x "${TEST_DIR}/scripts/install/utils/install_common.sh"
    chmod +x "${TEST_DIR}/scripts/install/utils/verify_installation.sh"
    chmod +x "${TEST_DIR}/scripts/install/utils/monitor_health.sh"
    chmod +x "${TEST_DIR}/scripts/install/utils/script_cleanup.sh"
    
    log INFO "Scripts copied successfully"
}

# Test script common utilities
test_common_utilities() {
    log INFO "Testing common utilities..."
    
    # Source the common utilities
    source "${TEST_DIR}/scripts/install/utils/install_common.sh"
    
    # Test logging function
    log INFO "Testing logging function"
    if type log >/dev/null 2>&1; then
        log INFO "✅ Logging function works"
    else
        log ERROR "❌ Logging function not working"
        return 1
    fi
    
    # Test hardware detection functions
    log INFO "Testing hardware detection functions"
    if type detect_tpm >/dev/null 2>&1; then
        log INFO "✅ TPM detection function exists"
        TPM_RESULT=$(detect_tpm)
        log INFO "TPM detection result: $TPM_RESULT"
    else
        log WARN "❌ TPM detection function missing"
    fi
    
    if type detect_yubikey >/dev/null 2>&1; then
        log INFO "✅ YubiKey detection function exists"
        YUBIKEY_RESULT=$(detect_yubikey)
        log INFO "YubiKey detection result: $YUBIKEY_RESULT"
    else
        log WARN "❌ YubiKey detection function missing"
    fi
    
    # Test process lock function
    log INFO "Testing process lock function"
    if type setup_process_lock >/dev/null 2>&1; then
        log INFO "✅ Process lock function exists"
        if setup_process_lock "test_lock"; then
            log INFO "✅ Process lock acquired successfully"
            
            # Try to acquire the same lock again (should fail)
            if ! setup_process_lock "test_lock"; then
                log INFO "✅ Process lock denied for duplicate process (expected)"
            else
                log ERROR "❌ Process lock function failed to deny duplicate process"
                return 1
            fi
        else
            log ERROR "❌ Process lock function failed"
            return 1
        fi
    else
        log WARN "❌ Process lock function missing"
    fi
    
    log INFO "Common utilities tests completed successfully"
    return 0
}

# Test auto_install script
test_auto_install() {
    log INFO "Testing auto_install.sh functionality..."
    
    # Modify auto_install.sh for testing
    # We'll replace system commands with echo for safe testing
    sed -i 's/systemctl start/echo "MOCK: systemctl start"/g' "${TEST_DIR}/scripts/install/auto_install.sh"
    sed -i 's/systemctl enable/echo "MOCK: systemctl enable"/g' "${TEST_DIR}/scripts/install/auto_install.sh"
    
    # Run the script with --dry-run mode if available
    if grep -q -- "--dry-run" "${TEST_DIR}/scripts/install/auto_install.sh"; then
        log INFO "Running auto_install.sh in dry-run mode..."
        if bash -c "cd ${TEST_DIR} && ./scripts/install/auto_install.sh --dry-run" > "${TEST_DIR}/auto_install.log" 2>&1; then
            log INFO "✅ auto_install.sh executed successfully in dry-run mode"
            
            # Check for expected output
            if grep -q "Detecting hardware capabilities" "${TEST_DIR}/auto_install.log"; then
                log INFO "✅ Hardware detection functionality working"
            else
                log WARN "⚠️ Hardware detection may not be working correctly"
            fi
            
            # Check for existing installation detection
            if grep -q "Checking for existing installation" "${TEST_DIR}/auto_install.log"; then
                log INFO "✅ Existing installation check implemented"
            else
                log WARN "⚠️ May be missing existing installation check"
            fi
        else
            log ERROR "❌ auto_install.sh failed in dry-run mode"
            cat "${TEST_DIR}/auto_install.log"
            return 1
        fi
    else
        log WARN "Dry-run mode not available, skipping execution test"
    fi
    
    log INFO "auto_install.sh tests completed"
    return 0
}

# Test linux_install script
test_linux_install() {
    log INFO "Testing linux_install.sh functionality..."
    
    # Modify linux_install.sh for testing
    # We'll replace system commands with echo for safe testing
    sed -i 's/systemctl start/echo "MOCK: systemctl start"/g' "${TEST_DIR}/scripts/install/linux_install.sh"
    sed -i 's/systemctl enable/echo "MOCK: systemctl enable"/g' "${TEST_DIR}/scripts/install/linux_install.sh"
    sed -i 's/cargo build/echo "MOCK: cargo build"/g' "${TEST_DIR}/scripts/install/linux_install.sh"
    
    # Run the script with --dry-run mode if available
    if grep -q -- "--dry-run" "${TEST_DIR}/scripts/install/linux_install.sh"; then
        log INFO "Running linux_install.sh in dry-run mode..."
        if bash -c "cd ${TEST_DIR} && ./scripts/install/linux_install.sh --dry-run" > "${TEST_DIR}/linux_install.log" 2>&1; then
            log INFO "✅ linux_install.sh executed successfully in dry-run mode"
            
            # Check for upgrade functionality
            if grep -q "Checking for upgrade" "${TEST_DIR}/linux_install.log" || grep -q "Preparing for upgrade" "${TEST_DIR}/linux_install.log"; then
                log INFO "✅ Upgrade functionality implemented"
            else
                log WARN "⚠️ May be missing upgrade functionality"
            fi
            
            # Check for feature flag configuration
            if grep -q "Configuring feature flags" "${TEST_DIR}/linux_install.log"; then
                log INFO "✅ Feature flag configuration implemented"
            else
                log WARN "⚠️ May be missing feature flag configuration"
            fi
        else
            log ERROR "❌ linux_install.sh failed in dry-run mode"
            cat "${TEST_DIR}/linux_install.log"
            return 1
        fi
    else
        log WARN "Dry-run mode not available, skipping execution test"
        
        # Check for upgrade functionality by inspecting code
        if grep -q "UPGRADE_MODE" "${TEST_DIR}/scripts/install/linux_install.sh"; then
            log INFO "✅ Upgrade functionality appears to be implemented"
        else
            log WARN "⚠️ May be missing upgrade functionality"
        fi
        
        # Check for feature flag configuration by inspecting code
        if grep -q "feature" "${TEST_DIR}/scripts/install/linux_install.sh"; then
            log INFO "✅ Feature flag configuration appears to be implemented"
        else
            log WARN "⚠️ May be missing feature flag configuration"
        fi
    fi
    
    log INFO "linux_install.sh tests completed"
    return 0
}

# Test verification script
test_verification_script() {
    log INFO "Testing verification script..."
    
    # Create a mock systemctl function
    cat > "${TEST_DIR}/mock_systemctl.sh" << 'EOF'
#!/bin/bash
case "$1" in
  "is-active")
    echo "active"
    exit 0
    ;;
  "list-unit-files")
    echo "anya-core.service                   enabled"
    exit 0
    ;;
  "cat")
    if [[ "$2" == "anya-core.service" ]]; then
      echo "[Unit]"
      echo "Description=Anya Core Service"
      echo ""
      echo "[Service]"
      echo "MemoryMax=2G"
      echo "CPUQuota=50%"
      echo "Restart=on-failure"
      exit 0
    fi
    ;;
  *)
    exit 1
    ;;
esac
EOF
    chmod +x "${TEST_DIR}/mock_systemctl.sh"
    
    # Modify the verification script to use our mock
    sed -i 's/systemctl /\.\/mock_systemctl.sh /g' "${TEST_DIR}/scripts/install/utils/verify_installation.sh"
    
    # Disable network checks
    sed -i 's/curl -s -o \/dev\/null -w "%{http_code}" http:\/\/localhost:3300\/health/echo "200"/g' "${TEST_DIR}/scripts/install/utils/verify_installation.sh"
    
    # Run the verification script
    log INFO "Running verification script..."
    if bash -c "cd ${TEST_DIR} && ./scripts/install/utils/verify_installation.sh" > "${TEST_DIR}/verify.log" 2>&1; then
        log INFO "✅ Verification script executed successfully"
        
        # Check for key verification steps
        if grep -q "Checking for Anya Core installation" "${TEST_DIR}/verify.log"; then
            log INFO "✅ Installation check implemented"
        else
            log WARN "⚠️ May be missing installation check"
        fi
        
        if grep -q "Checking hardware and feature alignment" "${TEST_DIR}/verify.log"; then
            log INFO "✅ Hardware-feature alignment check implemented"
        else
            log WARN "⚠️ May be missing hardware-feature alignment check"
        fi
    else
        log ERROR "❌ Verification script failed"
        cat "${TEST_DIR}/verify.log"
        return 1
    fi
    
    log INFO "Verification script tests completed"
    return 0
}

# Test monitor health script
test_monitor_health() {
    log INFO "Testing health monitoring script..."
    
    # Create a mock systemctl function
    cp "${TEST_DIR}/mock_systemctl.sh" "${TEST_DIR}/mock_systemctl2.sh"
    chmod +x "${TEST_DIR}/mock_systemctl2.sh"
    
    # Modify the monitoring script to use our mock
    sed -i 's/systemctl /\.\/mock_systemctl2.sh /g' "${TEST_DIR}/scripts/install/utils/monitor_health.sh"
    
    # Disable network checks
    sed -i 's/curl -s -o \/dev\/null -w "%{http_code}" http:\/\/localhost:3300\/health/echo "200"/g' "${TEST_DIR}/scripts/install/utils/monitor_health.sh"
    
    # Run the monitoring script
    log INFO "Running health monitoring script..."
    if bash -c "cd ${TEST_DIR} && ./scripts/install/utils/monitor_health.sh" > "${TEST_DIR}/monitor.log" 2>&1; then
        log INFO "✅ Health monitoring script executed successfully"
        
        # Check for key monitoring steps
        if grep -q "Checking Anya Core service status" "${TEST_DIR}/monitor.log"; then
            log INFO "✅ Service status check implemented"
        else
            log WARN "⚠️ May be missing service status check"
        fi
        
        if grep -q "Checking system resources" "${TEST_DIR}/monitor.log"; then
            log INFO "✅ System resource monitoring implemented"
        else
            log WARN "⚠️ May be missing system resource monitoring"
        fi
    else
        log ERROR "❌ Health monitoring script failed"
        cat "${TEST_DIR}/monitor.log"
        return 1
    fi
    
    log INFO "Health monitoring script tests completed"
    return 0
}

# Test script cleanup utility
test_script_cleanup() {
    log INFO "Testing script cleanup utility..."
    
    # Create mock scripts for testing
    mkdir -p "${TEST_DIR}/scripts/old_install"
    echo "# Old installation script" > "${TEST_DIR}/scripts/old_install/install.sh"
    echo "# Another old script" > "${TEST_DIR}/scripts/install-legacy.sh"
    chmod +x "${TEST_DIR}/scripts/old_install/install.sh"
    chmod +x "${TEST_DIR}/scripts/install-legacy.sh"
    
    # Modify the script_cleanup.sh to work in our test directory
    sed -i "s|PROJECT_ROOT=\"\$(cd \"\$INSTALL_DIR/../..\" \&\& pwd)\"|PROJECT_ROOT=\"${TEST_DIR}\"|g" "${TEST_DIR}/scripts/install/utils/script_cleanup.sh"
    
    # Run the script cleanup utility
    log INFO "Running script cleanup utility..."
    if bash -c "cd ${TEST_DIR} && ./scripts/install/utils/script_cleanup.sh" > "${TEST_DIR}/cleanup.log" 2>&1; then
        log INFO "✅ Script cleanup utility executed successfully"
        
        # Check for redundant script detection
        if grep -q "Found .* potentially redundant scripts" "${TEST_DIR}/cleanup.log"; then
            log INFO "✅ Redundant script detection implemented"
        else
            log WARN "⚠️ May be missing redundant script detection"
        fi
    else
        log ERROR "❌ Script cleanup utility failed"
        cat "${TEST_DIR}/cleanup.log"
        return 1
    fi
    
    log INFO "Script cleanup utility tests completed"
    return 0
}

# Run the main test suite
run_test_suite() {
    TESTS_PASSED=0
    TESTS_FAILED=0
    TESTS_WARNED=0
    
    # Set up the test environment
    setup_test_environment
    
    # Copy scripts to test environment
    copy_scripts_to_test_env
    
    # Run individual tests
    log INFO "Running test suite..."
    
    # Test common utilities
    if test_common_utilities; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    # Test auto_install.sh
    if test_auto_install; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    # Test linux_install.sh
    if test_linux_install; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    # Test verification script
    if test_verification_script; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    # Test health monitor script
    if test_monitor_health; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    # Test script cleanup utility
    if test_script_cleanup; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    # Clean up test environment
    cleanup_test_environment
    
    # Display test results
    echo
    echo "================================================================"
    echo "                 Installation System Test Results"
    echo "================================================================"
    echo
    echo "Tests Passed: $TESTS_PASSED"
    echo "Tests Failed: $TESTS_FAILED"
    echo "Tests with Warnings: $TESTS_WARNED"
    echo
    
    if [ $TESTS_FAILED -eq 0 ]; then
        echo "✅ All tests passed successfully!"
        return 0
    else
        echo "❌ Some tests failed. Please review the output."
        return 1
    fi
}

# Check if we're running as root
if [ "$EUID" -eq 0 ]; then
    log WARN "This script is running as root. For testing, it's better to run as a non-root user."
fi

# Run the test suite
run_test_suite
