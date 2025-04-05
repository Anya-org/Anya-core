#!/bin/bash
# Unified Installer Test Suite for Anya Core
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Test directories
TEST_DIR=$(mktemp -d)
REPORT_DIR="test_reports"

# Cleanup on exit
trap 'rm -rf "${TEST_DIR}"' EXIT

# Test utilities
run_test() {
    local cmd="$1"
    local desc="$2"
    echo -e "${YELLOW}Running test: $desc...${NC}"
    if eval "$cmd"; then
        echo -e "${GREEN}✓ Test passed: $desc${NC}"
        return 0
    else
        echo -e "${RED}✗ Test failed: $desc${NC}"
        return 1
    fi
}

# Test categories
test_basic_installation() {
    echo -e "\n${YELLOW}=== Testing Basic Installation ===${NC}"
    run_test "./target/release/installer --help | grep -q 'Anya Core Installer'" "Help output"
    run_test "./target/release/installer install --help | grep -q 'Install Anya Core'" "Install command help"
    run_test "./target/release/installer configure --help | grep -q 'Configure Anya Core'" "Configure command help"
}

test_network_configuration() {
    echo -e "\n${YELLOW}=== Testing Network Configuration ===${NC}"
    run_test "./target/release/installer configure --network testnet --dry-run" "Default testnet configuration"
    run_test "./target/release/installer configure --network mainnet --dry-run" "Default mainnet configuration"
    run_test "./target/release/installer configure --network regtest --dry-run" "Regtest configuration"
}

test_component_configuration() {
    echo -e "\n${YELLOW}=== Testing Component Configuration ===${NC}"
    
    # Test BDK configuration
    run_test "./target/release/installer configure --network testnet --bdk-wallet-dir /tmp/anya-wallets --dry-run" "BDK configuration"
    
    # Test LDK configuration
    run_test "./target/release/installer configure --network testnet --ldk-enabled true --ldk-auto-backup true --dry-run" "LDK configuration"
    
    # Test DLC configuration
    run_test "./target/release/installer configure --network testnet --dlc-enabled true --dlc-contract-dir /tmp/anya-dlc/contracts --dry-run" "DLC configuration"
    
    # Test RGB configuration
    run_test "./target/release/installer configure --network testnet --rgb-enabled true --rgb-asset-dir /tmp/anya-rgb/assets --dry-run" "RGB configuration"
    
    # Test Web5 configuration
    run_test "./target/release/installer configure --network testnet --web5-enabled true --web5-identity-dir /tmp/anya-web5/identities --dry-run" "Web5 configuration"
}

test_error_handling() {
    echo -e "\n${YELLOW}=== Testing Error Handling ===${NC}"
    run_test "! ./target/release/installer configure --network invalidnet --dry-run 2>/dev/null" "Invalid network fails"
    run_test "! ./target/release/installer configure --network testnet --rpc-url 'invalid-url' --dry-run 2>/dev/null" "Invalid RPC URL fails"
    run_test "! ./target/release/installer configure --network testnet --bdk-wallet-dir '/invalid/path' --dry-run 2>/dev/null" "Invalid wallet directory fails"
}

test_security_features() {
    echo -e "\n${YELLOW}=== Testing Security Features ===${NC}"
    run_test "./target/release/installer configure --network testnet --rpc-url 'https://testnet-rpc.example.com' --validate-ssl true --dry-run" "SSL validation"
    run_test "./target/release/installer configure --network testnet --web5-enabled true --web5-encryption-type aes256 --web5-key-length 256 --dry-run" "Encryption configuration"
}

test_system_integration() {
    echo -e "\n${YELLOW}=== Testing System Integration ===${NC}"
    run_test "cargo test --package anya-bitcoin" "Bitcoin compliance tests"
    run_test "cargo test --package anya-bitcoin --test dlc_tests" "DLC integration tests"
}

# Main test execution
main() {
    echo -e "${YELLOW}Starting Anya Core Installer Test Suite${NC}"
    mkdir -p "$REPORT_DIR"
    
    # Build installer
    echo -e "\n${YELLOW}Building installer...${NC}"
    run_test "cargo build --bin installer --release" "Building installer" || exit 1
    
    # Run test categories
    test_basic_installation
    test_network_configuration
    test_component_configuration
    test_error_handling
    test_security_features
    test_system_integration
    
    # Generate test report
    echo -e "\n${GREEN}All tests completed successfully!${NC}"
    echo -e "${GREEN}Installer location: ./target/release/installer${NC}"
}

# Run main if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
    main
fi