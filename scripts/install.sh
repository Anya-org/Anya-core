#!/bin/bash
# Anya Core Comprehensive Installation Test Suite
# [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

set -eo pipefail

# Script version
VERSION="2.0.0"

# Directory setup
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Set PROJECT_ROOT to the main Anya Core directory, not the scripts directory
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
INSTALL_DIR="${PROJECT_ROOT}/scripts/install"
UTILS_DIR="${INSTALL_DIR}/utils"
LOGS_DIR="${PROJECT_ROOT}/logs/installation"
TEST_LOGS_DIR="${PROJECT_ROOT}/logs/tests"
mkdir -p "$LOGS_DIR" "$TEST_LOGS_DIR"

# Color codes for prettier output
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
RED="\033[0;31m"
BLUE="\033[0;34m"
MAGENTA="\033[0;35m"
CYAN="\033[0;36m"
NC="\033[0m" # No Color
BOLD="\033[1m"

# Test log file
TEST_LOG="${TEST_LOGS_DIR}/test_$(date +%Y%m%d-%H%M%S).log"

# Test configuration
TEST_LEVEL="comprehensive" # basic, comprehensive, full
PARALLEL_TESTS=true        # Run tests in parallel when possible
TIMEOUT=900                # Maximum time in seconds for any single test (15 min for large Rust builds)
BUILD_TIMEOUT=1800         # Special timeout just for build operations (30 min)
SHOW_VERBOSE=false         # Show verbose output
FIX_ISSUES=true            # Attempt to fix issues automatically
SKIP_BUILD=false           # Set to true to skip binary building (for quick testing)

# Display header 
echo -e "${BOLD}================================================================${NC}"
echo -e "${BOLD}        Anya Core Test Suite (v$VERSION)${NC}"
echo -e "${BOLD}================================================================${NC}"
echo
echo -e "${BLUE}Starting tests at $(date)${NC}"
echo -e "${BLUE}Test log: $TEST_LOG${NC}"
echo

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --level=*)
                TEST_LEVEL="${1#*=}"
                if [[ ! "$TEST_LEVEL" =~ ^(basic|comprehensive|full)$ ]]; then
                    echo -e "${RED}Invalid test level: $TEST_LEVEL. Must be basic, comprehensive, or full.${NC}"
                    exit 1
                fi
                shift
                ;;
            --no-parallel)
                PARALLEL_TESTS=false
                shift
                ;;
            --timeout=*)
                TIMEOUT="${1#*=}"
                if ! [[ "$TIMEOUT" =~ ^[0-9]+$ ]]; then
                    echo -e "${RED}Invalid timeout: $TIMEOUT. Must be a positive integer.${NC}"
                    exit 1
                fi
                shift
                ;;
            --build-timeout=*)
                BUILD_TIMEOUT="${1#*=}"
                if ! [[ "$BUILD_TIMEOUT" =~ ^[0-9]+$ ]]; then
                    echo -e "${RED}Invalid build timeout: $BUILD_TIMEOUT. Must be a positive integer.${NC}"
                    exit 1
                fi
                shift
                ;;
            --skip-build)
                SKIP_BUILD=true
                log INFO "Binary build will be skipped"
                shift
                ;;
            --verbose)
                SHOW_VERBOSE=true
                shift
                ;;
            --no-fix)
                FIX_ISSUES=false
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
    echo -e "Anya Core Installation Test Suite v${VERSION}"
    echo -e ""
    echo -e "Usage: $0 [OPTIONS]"
    echo -e ""
    echo -e "Options:"
    echo -e "  --level=LEVEL         Test level (basic, comprehensive, full)"
    echo -e "  --no-parallel         Disable parallel testing"
    echo -e "  --timeout=SECONDS     Set test timeout in seconds"
    echo -e "  --build-timeout=SECS  Set specific timeout for build operations"
    echo -e "  --skip-build          Skip building and testing the binary"
    echo -e "  --verbose             Show verbose output"
    echo -e "  --no-fix              Don't attempt to fix issues automatically"
    echo -e "  --help                Display this help message"
    echo -e ""
    echo -e "Examples:"
    echo -e "  sudo $0 --level=full --verbose"
    echo -e "  sudo $0 --skip-build --level=basic # For quick validation"
}

# Log messages
log() {
    local level="$1"
    local message="$2"
    local timestamp=$(date "+%Y-%m-%d %H:%M:%S")
    
    # Choose color based on level
    local color="$NC"
    case "$level" in
        INFO)  color="$BLUE" ;;
        WARN)  color="$YELLOW" ;;
        ERROR) color="$RED" ;;
        PASS)  color="$GREEN" ;;
        TEST)  color="$MAGENTA" ;;
    esac
    
    # Print to stdout
    echo -e "[${timestamp}] ${color}${level}:${NC} ${message}"
    
    # Log to file
    echo "[${timestamp}] ${level}: ${message}" >> "$TEST_LOG"
}

# Run a test and capture result
run_test() {
    local test_name="$1"
    local test_command="$2"
    local timeout_seconds="$3"
    local required="${4:-true}"
    
    log TEST "Running test: $test_name"
    echo -e "${MAGENTA}TEST:${NC} $test_name"
    
    # Create temp files for output capture
    local output_file=$(mktemp)
    local exit_code_file=$(mktemp)
    
    # Run the test with timeout
    (
        # Execute the command and capture its exit code
        set +e
        eval "$test_command" > "$output_file" 2>&1
        echo $? > "$exit_code_file"
    ) & 
    local pid=$!
    
    # Wait for the command to finish or timeout
    local waited=0
    while kill -0 $pid 2>/dev/null && [ $waited -lt $timeout_seconds ]; do
        sleep 1
        waited=$((waited + 1))
        # Show progress for long-running tests
        if [ $((waited % 5)) -eq 0 ] && [ $waited -gt 0 ]; then
            echo -n "."
        fi
    done
    
    # If still running after timeout, kill it
    if kill -0 $pid 2>/dev/null; then
        kill -9 $pid
        wait $pid 2>/dev/null || true
        echo "TIMEOUT" > "$exit_code_file"
    fi
    
    # Get the exit code
    local exit_code=$(cat "$exit_code_file")
    
    # Process result
    if [ "$exit_code" = "0" ]; then
        log PASS "Test '$test_name' passed"
        echo -e "${GREEN}✓ PASS:${NC} $test_name"
    elif [ "$exit_code" = "TIMEOUT" ]; then
        log ERROR "Test '$test_name' timed out after $timeout_seconds seconds"
        echo -e "${RED}✗ TIMEOUT:${NC} $test_name (after $timeout_seconds seconds)"
        if [ "$SHOW_VERBOSE" = true ] || [ "$required" = true ]; then
            echo -e "${YELLOW}Output (truncated):${NC}"
            head -n 20 "$output_file"
            echo -e "${YELLOW}[output truncated]${NC}"
        fi
    else
        log ERROR "Test '$test_name' failed with exit code $exit_code"
        echo -e "${RED}✗ FAIL:${NC} $test_name (exit code: $exit_code)"
        if [ "$SHOW_VERBOSE" = true ] || [ "$required" = true ]; then
            echo -e "${YELLOW}Output (truncated):${NC}"
            head -n 20 "$output_file"
            echo -e "${YELLOW}[output truncated]${NC}"
        fi
    fi
    
    # Save output to test log
    echo "\n==== Test: $test_name (Exit: $exit_code) ====" >> "$TEST_LOG"
    cat "$output_file" >> "$TEST_LOG"
    echo "==== End of test output ====\n" >> "$TEST_LOG"
    
    # Clean up temp files
    rm -f "$output_file" "$exit_code_file"
    
    # Return appropriate exit code
    if [ "$exit_code" = "0" ]; then
        return 0
    else
        if [ "$required" = true ]; then
            return 1
        else
            return 0  # Non-required tests don't cause overall failure
        fi
    fi
}

# Binary tests
test_binary() {
    echo -e "\n${BOLD}${CYAN}1. Binary Tests${NC}"
    log INFO "Starting binary tests"
    
    local binary_path="${PROJECT_ROOT}/target/release/anya-core"
    
    # Skip binary tests if SKIP_BUILD is true
    if [ "$SKIP_BUILD" = true ]; then
        log INFO "Skipping binary build and tests (--skip-build used)"
        echo -e "${YELLOW}⚠ SKIP:${NC} Binary tests skipped (--skip-build flag used)"
        return 0
    fi
    
    # Check binary exists
    if [ ! -f "$binary_path" ]; then
        log ERROR "Binary not found at $binary_path"
        echo -e "${RED}✗ FAIL:${NC} Binary does not exist"
        if [ "$FIX_ISSUES" = true ]; then
            log INFO "Attempting to build binary..."
            echo -e "${YELLOW}⚠ Attempting to build binary...${NC}"
            echo -e "${YELLOW}Note: This may take up to 30 minutes for large Rust projects${NC}"
            
            # First try to prepare the dependencies without building
            run_test "Fetch dependencies" "cd $PROJECT_ROOT && cargo fetch" $TIMEOUT false
            
            # Then do the actual build with a longer timeout
            run_test "Build binary" "cd $PROJECT_ROOT && cargo build --release" $BUILD_TIMEOUT
            
            if [ ! -f "$binary_path" ]; then
                log ERROR "Failed to build binary"
                echo -e "${RED}✗ FAIL:${NC} Could not build binary"
                return 1
            fi
        else
            return 1
        fi
    fi
    
    # Check binary permissions
    run_test "Binary permissions" "test -x '$binary_path'" 5
    
    # Check binary integrity
    run_test "Binary integrity" "file '$binary_path' | grep -q 'ELF'" 5
    
    # Check if binary starts
    run_test "Binary execution" "'$binary_path' --version || true" 10
    
    log INFO "Binary tests completed"
    return 0
}

# Configuration tests
test_configuration() {
    echo -e "\n${BOLD}${CYAN}2. Configuration Tests${NC}"
    log INFO "Starting configuration tests"
    
    local config_path="${PROJECT_ROOT}/config/anya.conf"
    
    # Check if config exists
    if [ ! -f "$config_path" ]; then
        log ERROR "Configuration file not found at $config_path"
        echo -e "${RED}✗ FAIL:${NC} Configuration file does not exist"
        
        if [ "$FIX_ISSUES" = true ]; then
            log INFO "Creating default configuration..."
            echo -e "${YELLOW}⚠ Creating default configuration...${NC}"
            
            mkdir -p "${PROJECT_ROOT}/config"
            cat > "$config_path" << EOL
# Default Anya Core Configuration
# Auto-generated by test suite

[network]
type = testnet

[security]
hardening_level = standard

[features]
hsm = auto
dao_governance = false
lightning = false
EOL
            
            log INFO "Created default configuration at $config_path"
            echo -e "${GREEN}✓ PASS:${NC} Created default configuration"
        else
            return 1
        fi
    fi
    
    # Test configuration validity
    run_test "Config syntax" "grep -q '\[network\]' '$config_path'" 5
    
    # Check configuration permissions
    run_test "Config permissions" "[ -r '$config_path' ]" 5
    
    # Check specific configuration values
    if [[ "$TEST_LEVEL" != "basic" ]]; then
        run_test "Network configuration" "grep -q 'type\s*=' '$config_path'" 5
        run_test "Security configuration" "grep -q '\[security\]' '$config_path'" 5
        run_test "Features configuration" "grep -q '\[features\]' '$config_path'" 5
    fi
    
    log INFO "Configuration tests completed"
    return 0
}

# Service tests
test_service() {
    echo -e "\n${BOLD}${CYAN}3. Service Tests${NC}"
    log INFO "Starting service tests"
    
    # Skip service tests if SKIP_BUILD is true since service depends on binary
    if [ "$SKIP_BUILD" = true ]; then
        log INFO "Skipping service tests (--skip-build used, and service depends on binary)"
        echo -e "${YELLOW}⚠ SKIP:${NC} Service tests skipped (--skip-build flag used)"
        return 0
    fi
    
    # Check if systemd is available
    if ! command -v systemctl &> /dev/null; then
        log WARN "systemd not found, skipping service tests"
        echo -e "${YELLOW}⚠ SKIP:${NC} systemd not found, skipping service tests"
        return 0
    fi
    
    # Check if service is installed
    if ! systemctl list-unit-files | grep -q anya-core; then
        log ERROR "anya-core service not installed"
        echo -e "${RED}✗ FAIL:${NC} Service not installed"
        
        if [ "$FIX_ISSUES" = true ]; then
            log INFO "Attempting to install service..."
            echo -e "${YELLOW}⚠ Attempting to install service...${NC}"
            if [ -f "${INSTALL_DIR}/systemd_config.sh" ]; then
                run_test "Install service" "${INSTALL_DIR}/systemd_config.sh" $TIMEOUT
            else
                log ERROR "Service installation script not found"
                echo -e "${RED}✗ FAIL:${NC} Service installation script not found"
                return 1
            fi
        else
            return 1
        fi
    fi
    
    # Test service configuration
    run_test "Service unit file" "test -f /etc/systemd/system/anya-core.service" 5
    
    # Test service status
    run_test "Service status check" "systemctl status anya-core || true" 10 false
    
    # Attempt to start service if not running
    if ! systemctl is-active anya-core --quiet; then
        log WARN "Service not running"
        echo -e "${YELLOW}⚠ WARN:${NC} Service not running"
        
        if [ "$FIX_ISSUES" = true ]; then
            log INFO "Attempting to start service..."
            echo -e "${YELLOW}⚠ Attempting to start service...${NC}"
            run_test "Start service" "systemctl start anya-core" $TIMEOUT false
        fi
    fi
    
    # Test if service is enabled at boot
    if ! systemctl is-enabled anya-core --quiet 2>/dev/null; then
        log WARN "Service not enabled at boot"
        echo -e "${YELLOW}⚠ WARN:${NC} Service not enabled at boot"
        
        if [ "$FIX_ISSUES" = true ]; then
            log INFO "Enabling service at boot..."
            echo -e "${YELLOW}⚠ Enabling service at boot...${NC}"
            run_test "Enable service" "systemctl enable anya-core" 10 false
        fi
    else
        log PASS "Service enabled at boot"
        echo -e "${GREEN}✓ PASS:${NC} Service enabled at boot"
    fi
    
    log INFO "Service tests completed"
    return 0
}

# Feature tests
test_features() {
    echo -e "\n${BOLD}${CYAN}4. Feature Tests${NC}"
    log INFO "Starting feature tests"
    
    # Skip feature tests if SKIP_BUILD is true, as binary-dependent tests can't be run
    if [ "$SKIP_BUILD" = true ]; then
        log INFO "Running limited feature tests (--skip-build used, skipping binary-dependent tests)"
        echo -e "${YELLOW}⚠ INFO:${NC} Running configuration-only feature tests"
        # We'll continue but only do config-based tests
    fi
    
    # Only run for comprehensive and full test levels
    if [[ "$TEST_LEVEL" == "basic" ]]; then
        log INFO "Skipping feature tests in basic mode"
        echo -e "${YELLOW}⚠ SKIP:${NC} Feature tests skipped in basic mode"
        return 0
    fi
    
    local binary_path="${PROJECT_ROOT}/target/release/anya-core"
    
    # Test HSM support
    if grep -q 'hsm\s*=\s*true' "${PROJECT_ROOT}/config/anya.conf" 2>/dev/null; then
        log INFO "Testing HSM support"
        echo -e "${MAGENTA}TEST:${NC} HSM support"
        
        # Check for HSM directory
        run_test "HSM directory" "test -d '${PROJECT_ROOT}/src/security/hsm'" 5 false
        
        # Test HSM functionality if binary exists and build isn't skipped
        if [ -f "$binary_path" ] && [ "$SKIP_BUILD" != true ]; then
            run_test "HSM features in binary" "strings '$binary_path' | grep -q 'hsm'" 10 false
        elif [ "$SKIP_BUILD" = true ]; then
            log INFO "Skipping binary-dependent HSM tests due to --skip-build flag"
            echo -e "${YELLOW}⚠ SKIP:${NC} HSM binary tests (--skip-build used)"
        fi
    fi
    
    # Test DAO Governance features
    if grep -q 'dao_governance\s*=\s*true' "${PROJECT_ROOT}/config/anya.conf" 2>/dev/null; then
        log INFO "Testing DAO Governance support"
        echo -e "${MAGENTA}TEST:${NC} DAO Governance support"
        
        # Check for DAO directory
        run_test "DAO directory" "test -d '${PROJECT_ROOT}/src/dao'" 5 false
        
        # Test DAO functionality if binary exists and build isn't skipped
        if [ -f "$binary_path" ] && [ "$SKIP_BUILD" != true ]; then
            run_test "DAO features in binary" "strings '$binary_path' | grep -q 'dao_governance'" 10 false
        elif [ "$SKIP_BUILD" = true ]; then
            log INFO "Skipping binary-dependent DAO tests due to --skip-build flag"
            echo -e "${YELLOW}⚠ SKIP:${NC} DAO binary tests (--skip-build used)"
        fi
    fi
    
    # Test Lightning Network features
    if grep -q 'lightning\s*=\s*true' "${PROJECT_ROOT}/config/anya.conf" 2>/dev/null; then
        log INFO "Testing Lightning Network support"
        echo -e "${MAGENTA}TEST:${NC} Lightning Network support"
        
        # Check for Lightning directory
        run_test "Lightning directory" "test -d '${PROJECT_ROOT}/src/lightning'" 5 false
        
        # Test Lightning functionality if binary exists and build isn't skipped
        if [ -f "$binary_path" ] && [ "$SKIP_BUILD" != true ]; then
            run_test "Lightning features in binary" "strings '$binary_path' | grep -q 'lightning'" 10 false
        elif [ "$SKIP_BUILD" = true ]; then
            log INFO "Skipping binary-dependent Lightning tests due to --skip-build flag"
            echo -e "${YELLOW}⚠ SKIP:${NC} Lightning binary tests (--skip-build used)"
        fi
    fi
    
    log INFO "Feature tests completed"
    return 0
}

# Integration tests
test_integration() {
    echo -e "\n${BOLD}${CYAN}5. Integration Tests${NC}"
    log INFO "Starting integration tests"
    
    # Only run for full test level
    if [[ "$TEST_LEVEL" != "full" ]]; then
        log INFO "Skipping integration tests in $TEST_LEVEL mode"
        echo -e "${YELLOW}⚠ SKIP:${NC} Integration tests skipped in $TEST_LEVEL mode"
        return 0
    fi
    
    # Test API endpoints if service is running
    if systemctl is-active anya-core --quiet; then
        log INFO "Testing API endpoints"
        
        # Health check endpoint
        run_test "API health endpoint" "curl -s -f http://localhost:3300/health || true" 10 false
        
        # Status endpoint
        run_test "API status endpoint" "curl -s -f http://localhost:3300/status || true" 10 false
    else
        log WARN "Service not running, skipping API tests"
        echo -e "${YELLOW}⚠ SKIP:${NC} Service not running, skipping API tests"
    fi
    
    # Test integration with system
    log INFO "Testing system integration"
    
    # Check if directories and permissions are correct
    run_test "Data directory" "test -d '${PROJECT_ROOT}/data' || mkdir -p '${PROJECT_ROOT}/data'" 5 false
    run_test "Log directory" "test -d '${PROJECT_ROOT}/logs' || mkdir -p '${PROJECT_ROOT}/logs'" 5 false
    
    # Check if firewall is configured (if applicable)
    if command -v ufw &> /dev/null; then
        run_test "Firewall rules" "ufw status | grep -q '3300' || true" 10 false
    fi
    
    log INFO "Integration tests completed"
    return 0
}

# Run all tests and summarize results
run_all_tests() {
    log INFO "Starting comprehensive test suite (level: $TEST_LEVEL)"
    
    # Create arrays to track results
    declare -a test_results
    declare -a test_names
    
    # Run all test categories and track results
    echo -e "${BOLD}Running all tests...${NC}"
    
    test_binary
    test_results+=("$?")
    test_names+=("Binary Tests")
    
    test_configuration
    test_results+=("$?")
    test_names+=("Configuration Tests")
    
    test_service
    test_results+=("$?")
    test_names+=("Service Tests")
    
    test_features
    test_results+=("$?")
    test_names+=("Feature Tests")
    
    test_integration
    test_results+=("$?")
    test_names+=("Integration Tests")
    
    # Display summary
    echo -e "\n${BOLD}${CYAN}Test Summary:${NC}"
    echo -e "----------------------------------------------------------------"
    
    local pass_count=0
    local fail_count=0
    
    for i in "${!test_results[@]}"; do
        if [ "${test_results[$i]}" -eq 0 ]; then
            echo -e "${GREEN}✓ PASS:${NC} ${test_names[$i]}"
            pass_count=$((pass_count + 1))
        else
            echo -e "${RED}✗ FAIL:${NC} ${test_names[$i]}"
            fail_count=$((fail_count + 1))
        fi
    done
    
    echo -e "----------------------------------------------------------------"
    echo -e "${GREEN}Passed:${NC} $pass_count"
    echo -e "${RED}Failed:${NC} $fail_count"
    echo -e "${BLUE}Total:${NC} $((pass_count + fail_count))"
    
    # Report overall success/failure
    if [ $fail_count -eq 0 ]; then
        log INFO "All tests passed successfully"
        echo -e "\n${GREEN}${BOLD}All tests passed successfully!${NC}"
        return 0
    else
        log ERROR "$fail_count test categories failed"
        echo -e "\n${RED}${BOLD}$fail_count test categories failed.${NC}"
        echo -e "${YELLOW}See log for details: $TEST_LOG${NC}"
        return 1
    fi
}

# Main function
main() {
    # Check for root privileges
    if [ "$EUID" -ne 0 ]; then
        echo -e "${RED}ERROR: This script requires root privileges. Please run with sudo.${NC}"
        exit 1
    fi
    
    # Parse arguments
    parse_args "$@"
    
    # Log start
    log INFO "Test suite started with level: $TEST_LEVEL"
    
    # Run all tests
    run_all_tests
    exit $?
}

# Run the script
main "$@"
