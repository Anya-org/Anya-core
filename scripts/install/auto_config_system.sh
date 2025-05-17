#!/bin/bash
# Anya Core Auto-Configuration System
# Main entry point for auto-configuration and testing

set -eo pipefail

# Script version
VERSION="1.0.0"

# Directory setup
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
UTILS_DIR="${SCRIPT_DIR}/utils"
AUTO_CONFIG="${UTILS_DIR}/auto_config.sh"
TEST_SCRIPT="${PROJECT_ROOT}/scripts/test/test_auto_config.sh"

# Color codes
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
RED="\033[0;31m"
BLUE="\033[0;34m"
CYAN="\033[0;36m"
NC="\033[0m" # No Color
BOLD="\033[1m"

# Display header
echo -e "${BOLD}================================================================${NC}"
echo -e "${BOLD}        Anya Core Auto-Configuration System (v$VERSION)${NC}"
echo -e "${BOLD}================================================================${NC}"
echo

# Function to check if a command exists
command_exists() {
    command -v "$1" &> /dev/null
}

# Show usage information
show_usage() {
    echo -e "${BOLD}Usage:${NC}"
    echo -e "  $0 [OPTIONS] [COMMAND]"
    echo
    echo -e "${BOLD}Commands:${NC}"
    echo -e "  config          Run auto-configuration"
    echo -e "  test            Run test suite for auto-configuration"
    echo -e "  verify          Verify current configuration"
    echo -e "  install         Run auto-configuration and installation"
    echo
    echo -e "${BOLD}Options:${NC}"
    echo -e "  --network=TYPE  Set network type (mainnet, testnet, regtest)"
    echo -e "  --hsm=BOOL      Enable/disable HSM support (true, false, auto)"
    echo -e "  --dao=BOOL      Enable/disable DAO governance (true, false)"
    echo -e "  --force         Force overwrite existing configuration"
    echo -e "  --help          Show this help message"
    echo
    echo -e "${BOLD}Examples:${NC}"
    echo -e "  $0 config --network=testnet --hsm=auto"
    echo -e "  $0 test"
    echo -e "  $0 verify"
    echo -e "  $0 install --network=mainnet"
}

# Run auto-configuration
run_config() {
    echo -e "${CYAN}Running auto-configuration...${NC}"
    
    # Check if auto-config script exists
    if [ ! -f "$AUTO_CONFIG" ]; then
        echo -e "${RED}Auto-configuration script not found at $AUTO_CONFIG${NC}"
        exit 1
    fi
    
    # Run auto-configuration script with all arguments passed to this script
    "$AUTO_CONFIG" "$@"
    
    local exit_code=$?
    if [ $exit_code -eq 0 ]; then
        echo -e "${GREEN}Auto-configuration completed successfully.${NC}"
    else
        echo -e "${RED}Auto-configuration failed with exit code $exit_code.${NC}"
        exit $exit_code
    fi
}

# Run test suite
run_tests() {
    echo -e "${CYAN}Running auto-configuration test suite...${NC}"
    
    # Check if test script exists
    if [ ! -f "$TEST_SCRIPT" ]; then
        echo -e "${RED}Test script not found at $TEST_SCRIPT${NC}"
        exit 1
    fi
    
    # Run test script
    "$TEST_SCRIPT"
    
    local exit_code=$?
    if [ $exit_code -eq 0 ]; then
        echo -e "${GREEN}All tests passed successfully.${NC}"
    else
        echo -e "${RED}Tests failed with exit code $exit_code.${NC}"
        exit $exit_code
    fi
}

# Verify current configuration
verify_config() {
    local config_file="${PROJECT_ROOT}/config/anya.conf"
    
    echo -e "${CYAN}Verifying current configuration...${NC}"
    
    if [ ! -f "$config_file" ]; then
        echo -e "${RED}Configuration file not found at $config_file${NC}"
        echo -e "${YELLOW}Run auto-configuration first: $0 config${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}Configuration file exists at $config_file${NC}"
    
    # Check essential sections
    local missing_sections=0
    
    # Array of required sections
    local sections=("network" "security" "features")
    
    for section in "${sections[@]}"; do
        if ! grep -q "\\[$section\\]" "$config_file"; then
            echo -e "${RED}Missing required section: [$section]${NC}"
            missing_sections=$((missing_sections + 1))
        else
            echo -e "${GREEN}Found required section: [$section]${NC}"
        fi
    done
    
    # Check required configuration values
    local network_type=$(grep "network_type" "$config_file" | cut -d "=" -f2 | tr -d " \"")
    local hardening_level=$(grep "hardening_level" "$config_file" | cut -d "=" -f2 | tr -d " \"")
    local hsm_enabled=$(grep "hsm = " "$config_file" | cut -d "=" -f2 | tr -d " ")
    
    echo -e "${BLUE}Network Type:${NC} $network_type"
    echo -e "${BLUE}Security Hardening:${NC} $hardening_level"
    echo -e "${BLUE}HSM Enabled:${NC} $hsm_enabled"
    
    if [ $missing_sections -gt 0 ]; then
        echo -e "${RED}Configuration is missing $missing_sections required sections${NC}"
        echo -e "${YELLOW}Consider running auto-configuration again: $0 config --force${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}Configuration verification completed successfully${NC}"
}

# Run installation with auto-configuration
run_install() {
    echo -e "${CYAN}Running installation with auto-configuration...${NC}"
    
    # First run auto-configuration
    run_config "$@"
    
    # Then run installation
    local install_script="${PROJECT_ROOT}/scripts/install/auto_install.sh"
    
    if [ ! -f "$install_script" ]; then
        echo -e "${RED}Installation script not found at $install_script${NC}"
        exit 1
    fi
    
    echo -e "${BLUE}Starting installation process...${NC}"
    "$install_script"
    
    local exit_code=$?
    if [ $exit_code -eq 0 ]; then
        echo -e "${GREEN}Installation completed successfully.${NC}"
        
        # Suggest next steps
        echo -e "\n${BOLD}Next Steps:${NC}"
        echo -e "1. ${BLUE}Run tests:${NC} ./scripts/test/test_installation.sh"
        echo -e "2. ${BLUE}Open dashboard:${NC} ./scripts/dashboard.sh"
    else
        echo -e "${RED}Installation failed with exit code $exit_code.${NC}"
        exit $exit_code
    fi
}

# Parse arguments
parse_args() {
    local command=""
    local args=()
    
    # Extract command and pass remaining args to the execution functions
    for arg in "$@"; do
        case $arg in
            config|test|verify|install)
                command="$arg"
                ;;
            --help)
                show_usage
                exit 0
                ;;
            *)
                args+=("$arg")
                ;;
        esac
    done
    
    # Execute command with remaining arguments
    case $command in
        config)
            run_config "${args[@]}"
            ;;
        test)
            run_tests
            ;;
        verify)
            verify_config
            ;;
        install)
            run_install "${args[@]}"
            ;;
        *)
            # No command specified, show usage
            show_usage
            exit 1
            ;;
    esac
}

# Main execution
parse_args "$@"
exit $?
