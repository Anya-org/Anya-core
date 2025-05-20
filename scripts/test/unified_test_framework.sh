#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# Unified Test Framework for Anya Core
# Following Bitcoin Development Framework v2.5 standards
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
cd "$ROOT_DIR"

# Color codes for prettier output
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
RED="\033[0;31m"
BLUE="\033[0;34m"
NC="\033[0m" # No Color
BOLD="\033[1m"

# Default configuration
TEST_LEVEL="standard"  # minimal, standard, full
TEST_CATEGORY="all"    # core, bitcoin, hsm, web5, all
PARALLEL_TESTS=true    # Run tests in parallel when possible
VERBOSE=false          # Verbose output
TIMEOUT=300            # Default timeout in seconds
LOG_DIR="${ROOT_DIR}/logs/tests"
mkdir -p "$LOG_DIR"
LOG_FILE="${LOG_DIR}/test_$(date +%Y%m%d-%H%M%S).log"

# Function to log messages
function log {
    local level=$1
    local message=$2
    local timestamp=$(date "+%Y-%m-%d %H:%M:%S")
    
    echo -e "[$timestamp] [$level] $message" | tee -a "$LOG_FILE"
}

# Function to display help
function show_help {
    echo -e "${BOLD}Anya Core Unified Test Framework${NC}"
    echo -e "Following Bitcoin Development Framework v2.5 standards"
    echo
    echo -e "Usage: $0 [OPTIONS]"
    echo
    echo -e "Options:"
    echo -e "  --level=LEVEL       Test level (minimal, standard, full)"
    echo -e "  --category=CATEGORY Test category (core, bitcoin, hsm, web5, all)"
    echo -e "  --no-parallel       Disable parallel testing"
    echo -e "  --verbose           Enable verbose output"
    echo -e "  --timeout=SECONDS   Set timeout in seconds (default: 300)"
    echo -e "  --help              Display this help message"
    echo
    echo -e "Examples:"
    echo -e "  $0 --level=minimal --category=core    # Run minimal core tests"
    echo -e "  $0 --level=full --category=bitcoin    # Run full bitcoin tests"
    echo -e "  $0 --level=standard                   # Run standard tests for all categories"
}

# Function to parse command line arguments
function parse_args {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --level=*)
                TEST_LEVEL="${1#*=}"
                if [[ ! "$TEST_LEVEL" =~ ^(minimal|standard|full)$ ]]; then
                    log "ERROR" "Invalid test level: $TEST_LEVEL. Must be minimal, standard, or full."
                    exit 1
                fi
                shift
                ;;
            --category=*)
                TEST_CATEGORY="${1#*=}"
                if [[ ! "$TEST_CATEGORY" =~ ^(core|bitcoin|hsm|web5|all)$ ]]; then
                    log "ERROR" "Invalid test category: $TEST_CATEGORY. Must be core, bitcoin, hsm, web5, or all."
                    exit 1
                fi
                shift
                ;;
            --no-parallel)
                PARALLEL_TESTS=false
                shift
                ;;
            --verbose)
                VERBOSE=true
                shift
                ;;
            --timeout=*)
                TIMEOUT="${1#*=}"
                if ! [[ "$TIMEOUT" =~ ^[0-9]+$ ]]; then
                    log "ERROR" "Invalid timeout: $TIMEOUT. Must be a positive integer."
                    exit 1
                fi
                shift
                ;;
            --help)
                show_help
                exit 0
                ;;
            *)
                log "ERROR" "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
}

# Function to run a test with timeout
function run_test_with_timeout {
    local test_name=$1
    local test_command=$2
    local timeout_seconds=$3
    
    log "INFO" "Running test: $test_name (timeout: ${timeout_seconds}s)"
    
    # Create a temporary file for test output
    local temp_output=$(mktemp)
    
    # Run the test with timeout
    if timeout "$timeout_seconds" bash -c "$test_command" > "$temp_output" 2>&1; then
        echo -e "${GREEN}✓ PASS:${NC} $test_name"
        if [ "$VERBOSE" = true ]; then
            cat "$temp_output"
        fi
        rm "$temp_output"
        return 0
    else
        local exit_code=$?
        echo -e "${RED}✗ FAIL:${NC} $test_name (exit code: $exit_code)"
        echo -e "${YELLOW}Test output:${NC}"
        cat "$temp_output"
        rm "$temp_output"
        return 1
    fi
}

# Function to run core tests
function run_core_tests {
    log "INFO" "Running core tests (level: $TEST_LEVEL)"
    
    local core_tests_passed=true
    
    # Minimal tests
    if [[ "$TEST_LEVEL" =~ ^(minimal|standard|full)$ ]]; then
        log "INFO" "Running minimal core tests"
        
        # Run cargo test with minimal features
        if ! run_test_with_timeout "Core minimal tests" "cd $ROOT_DIR && cargo test --no-default-features" "$TIMEOUT"; then
            core_tests_passed=false
        fi
    fi
    
    # Standard tests
    if [[ "$TEST_LEVEL" =~ ^(standard|full)$ ]]; then
        log "INFO" "Running standard core tests"
        
        # Run cargo test with default features
        if ! run_test_with_timeout "Core standard tests" "cd $ROOT_DIR && cargo test" "$TIMEOUT"; then
            core_tests_passed=false
        fi
    fi
    
    # Full tests
    if [[ "$TEST_LEVEL" == "full" ]]; then
        log "INFO" "Running full core tests"
        
        # Run cargo test with all features
        if ! run_test_with_timeout "Core full tests" "cd $ROOT_DIR && cargo test --all-features" "$TIMEOUT"; then
            core_tests_passed=false
        fi
    fi
    
    if [ "$core_tests_passed" = true ]; then
        log "INFO" "All core tests passed"
        return 0
    else
        log "ERROR" "Some core tests failed"
        return 1
    fi
}

# Function to run Bitcoin tests
function run_bitcoin_tests {
    log "INFO" "Running Bitcoin tests (level: $TEST_LEVEL)"
    
    local bitcoin_tests_passed=true
    
    # Minimal tests
    if [[ "$TEST_LEVEL" =~ ^(minimal|standard|full)$ ]]; then
        log "INFO" "Running minimal Bitcoin tests"
        
        # Run cargo test for Bitcoin module with minimal features
        if ! run_test_with_timeout "Bitcoin minimal tests" "cd $ROOT_DIR && cargo test --package anya-core --lib bitcoin" "$TIMEOUT"; then
            bitcoin_tests_passed=false
        fi
    fi
    
    # Standard tests
    if [[ "$TEST_LEVEL" =~ ^(standard|full)$ ]]; then
        log "INFO" "Running standard Bitcoin tests"
        
        # Run cargo test for Bitcoin module with taproot feature
        if ! run_test_with_timeout "Bitcoin taproot tests" "cd $ROOT_DIR && cargo test --package anya-core --lib bitcoin --features taproot" "$TIMEOUT"; then
            bitcoin_tests_passed=false
        fi
    fi
    
    # Full tests
    if [[ "$TEST_LEVEL" == "full" ]]; then
        log "INFO" "Running full Bitcoin tests"
        
        # Run cargo test for Bitcoin module with all features
        if ! run_test_with_timeout "Bitcoin full tests" "cd $ROOT_DIR && cargo test --package anya-core --lib bitcoin --all-features" "$TIMEOUT"; then
            bitcoin_tests_passed=false
        fi
    fi
    
    if [ "$bitcoin_tests_passed" = true ]; then
        log "INFO" "All Bitcoin tests passed"
        return 0
    else
        log "ERROR" "Some Bitcoin tests failed"
        return 1
    fi
}

# Function to run HSM tests
function run_hsm_tests {
    log "INFO" "Running HSM tests (level: $TEST_LEVEL)"
    
    local hsm_tests_passed=true
    
    # Minimal tests
    if [[ "$TEST_LEVEL" =~ ^(minimal|standard|full)$ ]]; then
        log "INFO" "Running minimal HSM tests"
        
        # Run cargo test for HSM module with minimal features
        if ! run_test_with_timeout "HSM minimal tests" "cd $ROOT_DIR && cargo test --package anya-core --lib security::hsm" "$TIMEOUT"; then
            hsm_tests_passed=false
        fi
    fi
    
    # Standard tests
    if [[ "$TEST_LEVEL" =~ ^(standard|full)$ ]]; then
        log "INFO" "Running standard HSM tests"
        
        # Run cargo test for HSM module with standard features
        if ! run_test_with_timeout "HSM standard tests" "cd $ROOT_DIR && cargo test --package anya-core --lib security::hsm --features hsm" "$TIMEOUT"; then
            hsm_tests_passed=false
        fi
    fi
    
    # Full tests
    if [[ "$TEST_LEVEL" == "full" ]]; then
        log "INFO" "Running full HSM tests"
        
        # Run cargo test for HSM module with all features
        if ! run_test_with_timeout "HSM full tests" "cd $ROOT_DIR && cargo test --package anya-core --lib security::hsm --all-features" "$TIMEOUT"; then
            hsm_tests_passed=false
        fi
    fi
    
    if [ "$hsm_tests_passed" = true ]; then
        log "INFO" "All HSM tests passed"
        return 0
    else
        log "ERROR" "Some HSM tests failed"
        return 1
    fi
}

# Function to run Web5 tests
function run_web5_tests {
    log "INFO" "Running Web5 tests (level: $TEST_LEVEL)"
    
    local web5_tests_passed=true
    
    # Minimal tests
    if [[ "$TEST_LEVEL" =~ ^(minimal|standard|full)$ ]]; then
        log "INFO" "Running minimal Web5 tests"
        
        # Run cargo test for Web5 module with minimal features
        if ! run_test_with_timeout "Web5 minimal tests" "cd $ROOT_DIR && cargo test --package anya-core --lib web5" "$TIMEOUT"; then
            web5_tests_passed=false
        fi
    fi
    
    # Standard tests
    if [[ "$TEST_LEVEL" =~ ^(standard|full)$ ]]; then
        log "INFO" "Running standard Web5 tests"
        
        # Run cargo test for Web5 module with standard features
        if ! run_test_with_timeout "Web5 standard tests" "cd $ROOT_DIR && cargo test --package anya-core --lib web5 --features web5" "$TIMEOUT"; then
            web5_tests_passed=false
        fi
    fi
    
    # Full tests
    if [[ "$TEST_LEVEL" == "full" ]]; then
        log "INFO" "Running full Web5 tests"
        
        # Run cargo test for Web5 module with all features
        if ! run_test_with_timeout "Web5 full tests" "cd $ROOT_DIR && cargo test --package anya-core --lib web5 --all-features" "$TIMEOUT"; then
            web5_tests_passed=false
        fi
    fi
    
    if [ "$web5_tests_passed" = true ]; then
        log "INFO" "All Web5 tests passed"
        return 0
    else
        log "ERROR" "Some Web5 tests failed"
        return 1
    fi
}

# Function to run all tests
function run_all_tests {
    log "INFO" "Running all tests (level: $TEST_LEVEL)"
    
    local all_tests_passed=true
    
    # Run tests based on category
    case "$TEST_CATEGORY" in
        core)
            if ! run_core_tests; then
                all_tests_passed=false
            fi
            ;;
        bitcoin)
            if ! run_bitcoin_tests; then
                all_tests_passed=false
            fi
            ;;
        hsm)
            if ! run_hsm_tests; then
                all_tests_passed=false
            fi
            ;;
        web5)
            if ! run_web5_tests; then
                all_tests_passed=false
            fi
            ;;
        all)
            if ! run_core_tests; then
                all_tests_passed=false
            fi
            
            if ! run_bitcoin_tests; then
                all_tests_passed=false
            fi
            
            if ! run_hsm_tests; then
                all_tests_passed=false
            fi
            
            if ! run_web5_tests; then
                all_tests_passed=false
            fi
            ;;
    esac
    
    if [ "$all_tests_passed" = true ]; then
        log "INFO" "All tests passed"
        return 0
    else
        log "ERROR" "Some tests failed"
        return 1
    fi
}

# Main function
function main {
    # Parse command line arguments
    parse_args "$@"
    
    # Display header
    echo -e "${BOLD}================================================================${NC}"
    echo -e "${BOLD}        Anya Core Unified Test Framework${NC}"
    echo -e "${BOLD}================================================================${NC}"
    echo
    echo -e "${BLUE}Starting tests at $(date)${NC}"
    echo -e "${BLUE}Test level: $TEST_LEVEL${NC}"
    echo -e "${BLUE}Test category: $TEST_CATEGORY${NC}"
    echo -e "${BLUE}Parallel tests: $PARALLEL_TESTS${NC}"
    echo -e "${BLUE}Verbose output: $VERBOSE${NC}"
    echo -e "${BLUE}Timeout: $TIMEOUT seconds${NC}"
    echo -e "${BLUE}Log file: $LOG_FILE${NC}"
    echo
    
    # Run tests
    if run_all_tests; then
        echo -e "\n${GREEN}${BOLD}All tests passed!${NC}"
        exit 0
    else
        echo -e "\n${RED}${BOLD}Some tests failed. Check log for details.${NC}"
        exit 1
    fi
}

# Run the script
main "$@"
