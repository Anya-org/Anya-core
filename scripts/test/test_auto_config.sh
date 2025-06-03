#!/bin/bash
# Anya Core Auto-Configuration Test Suite
# [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

set -eo pipefail

# Script version
VERSION="1.0.0"

# Directory setup
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
INSTALL_DIR="${PROJECT_ROOT}/scripts/install"
UTILS_DIR="${INSTALL_DIR}/utils"
AUTO_CONFIG="${UTILS_DIR}/auto_config.sh"
CONFIG_DIR="${PROJECT_ROOT}/config"
CONFIG_FILE="${CONFIG_DIR}/anya.conf"

# Use a temporary directory for logs to avoid permission issues
TEST_DIR="/tmp/anya-core-tests"
mkdir -p "${TEST_DIR}" 2>/dev/null || true
TIMESTAMP=$(date +"%Y%m%d-%H%M%S")
TEST_LOG="${TEST_DIR}/auto_config_test_${TIMESTAMP}.log"
touch "$TEST_LOG" 2>/dev/null || true

# Verify that the auto_config script exists
if [ ! -f "$AUTO_CONFIG" ]; then
    echo "ERROR: Auto-config script not found at expected path: $AUTO_CONFIG"
    echo "Looking for script in common locations..."
    
    # Try to find the auto_config script
    FOUND_PATH=$(find "$PROJECT_ROOT" -name "auto_config.sh" -type f | head -n 1)
    
    if [ -n "$FOUND_PATH" ]; then
        echo "Found auto_config.sh at: $FOUND_PATH"
        AUTO_CONFIG="$FOUND_PATH"
    fi
fi

# Create test directory if it doesn't exist
mkdir -p "${TEST_DIR}"

# Color codes
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
RED="\033[0;31m"
BLUE="\033[0;34m"
MAGENTA="\033[0;35m"
CYAN="\033[0;36m"
NC="\033[0m" # No Color
BOLD="\033[1m"

# Display header
echo -e "${BOLD}================================================================${NC}"
echo -e "${BOLD}        Anya Core Auto-Configuration Test Suite (v$VERSION)${NC}"
echo -e "${BOLD}================================================================${NC}"
echo
echo -e "${BLUE}Starting tests at $(date)${NC}"
echo -e "${BLUE}Test log: $TEST_LOG${NC}"
echo

# Log function
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
    
    # Log to file (with error suppression for permission issues)
    echo "[${timestamp}] ${level}: ${message}" >> "$TEST_LOG" 2>/dev/null || true
}

# Run a test and capture result
run_test() {
    local test_name="$1"
    local test_command="$2"
    local required="${3:-true}"
    
    log TEST "Running test: $test_name"
    echo -e "${MAGENTA}TEST:${NC} $test_name"
    
    # Create temp files for output capture
    local output_file=$(mktemp)
    local exit_code_file=$(mktemp)
    
    # Run the test
    (
        # Execute the command and capture its exit code
        set +e
        eval "$test_command" > "$output_file" 2>&1
        echo $? > "$exit_code_file"
    )
    
    # Get the exit code
    local exit_code=$(cat "$exit_code_file")
    
    # Process result
    if [ "$exit_code" = "0" ]; then
        log PASS "Test '$test_name' passed"
        echo -e "${GREEN}✓ PASS:${NC} $test_name"
    else
        log ERROR "Test '$test_name' failed with exit code $exit_code"
        echo -e "${RED}✗ FAIL:${NC} $test_name (exit code: $exit_code)"
        echo -e "${YELLOW}Output (truncated):${NC}"
        head -n 10 "$output_file"
        if [ $(wc -l < "$output_file") -gt 10 ]; then
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

# Test auto-config script exists
test_auto_config_exists() {
    echo -e "\n${BOLD}${CYAN}1. Auto-Configuration Script Tests${NC}"
    log INFO "Testing auto-config script existence and permissions"
    
    # Test script exists
    run_test "Auto-config script exists" "test -f '$AUTO_CONFIG'"
    
    # Test script is executable
    run_test "Auto-config script is executable" "test -x '$AUTO_CONFIG'"
    
    log INFO "Auto-config script tests completed"
}

# Test auto-config basic functionality
test_auto_config_basic() {
    echo -e "\n${BOLD}${CYAN}2. Basic Auto-Configuration Functionality Tests${NC}"
    log INFO "Testing basic auto-config functionality"
    
    # Backup existing config if it exists
    if [ -f "$CONFIG_FILE" ]; then
        local backup_file="${CONFIG_FILE}.backup_${TIMESTAMP}"
        log INFO "Backing up existing config to $backup_file"
        cp "$CONFIG_FILE" "$backup_file"
    fi
    
    # Test help option
    run_test "Help option works" "$AUTO_CONFIG --help >/dev/null"
    
    # Test basic configuration generation
    run_test "Basic configuration generation" "$AUTO_CONFIG --force >/dev/null"
    
    # Test config file was created
    run_test "Config file was created" "test -f '$CONFIG_FILE'"
    
    # Test file has content
    run_test "Config file has content" "[ -s '$CONFIG_FILE' ]"
    
    log INFO "Basic auto-config functionality tests completed"
}

# Test auto-config with custom settings
test_auto_config_custom() {
    echo -e "\n${BOLD}${CYAN}3. Custom Configuration Tests${NC}"
    log INFO "Testing auto-config with custom settings"
    
    # Test custom network setting
    run_test "Custom network setting" "$AUTO_CONFIG --network=regtest --force >/dev/null"
    run_test "Custom network applied" "grep -q 'network_type = \"regtest\"' '$CONFIG_FILE'"
    
    # Test custom security hardening
    run_test "Custom security hardening" "$AUTO_CONFIG --hardening=strict --force >/dev/null"
    run_test "Custom hardening applied" "grep -q 'hardening_level = \"strict\"' '$CONFIG_FILE'"
    
    # Test custom feature flags
    run_test "Custom feature flags" "$AUTO_CONFIG --hsm=true --dao=true --lightning=true --bitcoin=true --force >/dev/null"
    run_test "HSM flag applied" "grep -q 'hsm = true' '$CONFIG_FILE'"
    run_test "DAO flag applied" "grep -q 'dao_governance = true' '$CONFIG_FILE'"
    run_test "Lightning flag applied" "grep -q 'lightning = true' '$CONFIG_FILE'"
    
    log INFO "Custom configuration tests completed"
}

# Test directory creation
test_directory_creation() {
    echo -e "\n${BOLD}${CYAN}4. Directory Structure Tests${NC}"
    log INFO "Testing directory creation"
    
    # Test data directories
    run_test "Data directory created" "test -d '${PROJECT_ROOT}/data'"
    run_test "ML models directory created" "test -d '${PROJECT_ROOT}/data/ml/models'"
    run_test "Web5 directory created" "test -d '${PROJECT_ROOT}/data/web5'"
    
    # Test module directories (these should have been created with the feature flags we enabled)
    run_test "HSM module directory" "test -d '${PROJECT_ROOT}/src/security/hsm'" false
    run_test "DAO module directory" "test -d '${PROJECT_ROOT}/src/dao'" false
    run_test "Lightning module directory" "test -d '${PROJECT_ROOT}/src/lightning'" false
    run_test "Bitcoin module directory" "test -d '${PROJECT_ROOT}/src/bitcoin'" false
    
    log INFO "Directory structure tests completed"
}

# Test config file content
test_config_content() {
    echo -e "\n${BOLD}${CYAN}5. Configuration Content Tests${NC}"
    log INFO "Testing configuration file content"
    
    # Test required sections
    run_test "Network section exists" "grep -q '\\[network\\]' '$CONFIG_FILE'"
    run_test "Security section exists" "grep -q '\\[security\\]' '$CONFIG_FILE'"
    run_test "Features section exists" "grep -q '\\[features\\]' '$CONFIG_FILE'"
    run_test "Performance section exists" "grep -q '\\[performance\\]' '$CONFIG_FILE'"
    
    # Test dynamic content
    run_test "CPU cores detected" "grep -q 'inference_threads = [0-9]\\+' '$CONFIG_FILE'"
    
    log INFO "Configuration content tests completed"
}

# Run all tests and report
run_all_tests() {
    log INFO "Starting auto-configuration test suite"
    
    # Create arrays to track results
    declare -a test_results
    declare -a test_names
    
    # Run all test categories
    test_auto_config_exists
    test_results+=("$?")
    test_names+=("Auto-Config Script Tests")
    
    test_auto_config_basic
    test_results+=("$?")
    test_names+=("Basic Functionality Tests")
    
    test_auto_config_custom
    test_results+=("$?")
    test_names+=("Custom Configuration Tests")
    
    test_directory_creation
    test_results+=("$?")
    test_names+=("Directory Structure Tests")
    
    test_config_content
    test_results+=("$?")
    test_names+=("Configuration Content Tests")
    
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
        log INFO "All auto-configuration tests passed successfully"
        echo -e "\n${GREEN}${BOLD}All auto-configuration tests passed successfully!${NC}"
        
        # Suggest next steps
        echo -e "\n${BOLD}Next Steps:${NC}"
        echo -e "1. ${BLUE}Review configuration:${NC} cat $CONFIG_FILE"
        echo -e "2. ${BLUE}Run full installation:${NC} sudo ./scripts/auto_install.sh"
        echo -e "3. ${BLUE}Monitor with dashboard:${NC} ./scripts/dashboard.sh"
        
        return 0
    else
        log ERROR "$fail_count test categories failed"
        echo -e "\n${RED}${BOLD}$fail_count test categories failed.${NC}"
        echo -e "${YELLOW}See log for details: $TEST_LOG${NC}"
        return 1
    fi
}

# Check if auto-config script exists
if [ ! -f "$AUTO_CONFIG" ]; then
    echo -e "${RED}ERROR: Auto-configuration script not found at $AUTO_CONFIG${NC}"
    
    # Try to find it using the full absolute path
    if [ -f "${PROJECT_ROOT}/scripts/install/utils/auto_config.sh" ]; then
        AUTO_CONFIG="${PROJECT_ROOT}/scripts/install/utils/auto_config.sh"
        echo -e "${GREEN}Found auto-config script at: $AUTO_CONFIG${NC}"
    else
        # Last resort attempt to find it
        FOUND_PATH=$(find "$PROJECT_ROOT" -name "auto_config.sh" -type f 2>/dev/null | head -n 1)
        if [ -n "$FOUND_PATH" ]; then
            AUTO_CONFIG="$FOUND_PATH"
            echo -e "${GREEN}Found auto-config script at: $AUTO_CONFIG${NC}"
        else
            log ERROR "Auto-configuration script not found"
            echo -e "${RED}ERROR: Auto-configuration script not found${NC}"
            exit 1
        fi
    fi
fi

# Run the test suite
run_all_tests
exit $?
