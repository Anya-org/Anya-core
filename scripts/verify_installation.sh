#!/bin/bash

# Anya Core Installation Verification Script
# This script verifies the installation of Anya Core and its dependencies
# Compliant with BIP 341/342 (Taproot) verification requirements
# Version: 1.0.0

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print success messages
success() {
    echo -e "${GREEN}✅ [SUCCESS]${NC} $1"
}

# Function to print warning messages
warning() {
    echo -e "${YELLOW}⚠️ [WARNING]${NC} $1"
}

# Function to print error messages
error() {
    echo -e "${RED}❌ [ERROR]${NC} $1"
}

# Function to check if a command is available
check_command() {
    if command -v $1 &> /dev/null; then
        echo -e "${GREEN}✓ $1 is installed${NC}"
        return 0
    else
        echo -e "${RED}✗ $1 is not installed${NC}"
        return 1
    fi
}

# Function to check if a service is active
check_service() {
    local service=$1
    if systemctl is-active --quiet $service; then
        success "$service is running"
        return 0
    else
        error "$service is not running"
        return 1
    fi
}

# Function to check if a port is open
network_check() {
    local host=$1
    local port=$2
    local service=$3
    
    if nc -z $host $port &>/dev/null; then
        echo -e "${GREEN}✓ $service port $port is open${NC}"
        return 0
    else
        echo -e "${RED}✗ $service port $port is closed${NC}"
        return 1
    fi
}

# Check Bitcoin RPC connection
check_bitcoin_rpc() {
    echo -e "Checking connection to Bitcoin testnet RPC..."
    if curl -s --user mainnet_8731:c8f7af8a-c33c-4f49-954c-a997a50b9a22 \
        -H 'Content-Type: application/json' \
        -d '{"jsonrpc":"1.0","id":"anya-test","method":"getblockchaininfo","params":[]}' \
        https://testnet.getblock.io/3333/ | grep -q "testnet"; then
        echo -e "${GREEN}✓ Connection to Bitcoin testnet RPC successful${NC}"
        return 0
    else
        echo -e "${RED}✗ Failed to connect to Bitcoin testnet RPC${NC}"
        return 1
    fi
}

# Check if the script is run as root
if [ "$EUID" -ne 0 ]; then
    warning "This script is not being run as root. Some checks may fail."
fi

echo "==== Anya Core Installation Verification ===="
echo "This script checks for required components and services"
echo "Using Bitcoin testnet for all tests"
echo

# Check system requirements
echo "Checking system requirements..."
check_command "curl"
check_command "nodejs"
check_command "npm"
check_command "jq"
check_command "nc"
echo

# Check for necessary services
echo "Checking services..."
check_service "web5-dwn"
check_service "prometheus"
echo

# Check Bitcoin RPC connection
echo "Checking Bitcoin testnet connection..."
check_bitcoin_rpc
echo

# Check network ports
echo "Checking network ports..."
network_check "localhost" 3000 "Web5 DWN API"
network_check "localhost" 9090 "Prometheus"
network_check "testnet.getblock.io" 3333 "Bitcoin Testnet RPC" || warning "Could not connect directly to testnet.getblock.io:3333 (expected if behind firewall)"
echo

# Check Bitcoin configuration
echo "Checking Bitcoin configuration..."
if [ -f "/home/anya/.bitcoin/bitcoin.conf" ]; then
    echo -e "${GREEN}✓ Bitcoin configuration file exists${NC}"
    
    # Check for testnet configuration
    if grep -q "testnet=1" /home/anya/.bitcoin/bitcoin.conf; then
        echo -e "${GREEN}✓ Testnet is enabled in configuration${NC}"
    else
        echo -e "${RED}✗ Testnet is not enabled in configuration${NC}"
    fi
    
    # Check for Bitcoin RPC endpoint configuration
    if grep -q "rpcconnect=testnet.getblock.io" /home/anya/.bitcoin/bitcoin.conf; then
        echo -e "${GREEN}✓ Public Bitcoin RPC endpoint is configured${NC}"
    else
        echo -e "${RED}✗ Public Bitcoin RPC endpoint is not configured${NC}"
    fi
else
    echo -e "${RED}✗ Bitcoin configuration file not found${NC}"
fi
echo

# Check Web5 DWN configuration
echo "Checking Web5 DWN configuration..."
if [ -f "/home/anya/web5/dwn-server/.env" ]; then
    echo -e "${GREEN}✓ Web5 DWN configuration file exists${NC}"
    
    # Check for Bitcoin testnet configuration
    if grep -q "BITCOIN_NETWORK=testnet" /home/anya/web5/dwn-server/.env; then
        echo -e "${GREEN}✓ Bitcoin testnet is configured for Web5 DWN${NC}"
    else
        echo -e "${RED}✗ Bitcoin testnet is not configured for Web5 DWN${NC}"
    fi
else
    echo -e "${RED}✗ Web5 DWN configuration file not found${NC}"
fi
echo

# Check firewall configuration
echo "Checking firewall configuration..."
if sudo ufw status | grep -q "Status: active"; then
    echo -e "${GREEN}✓ Firewall is active${NC}"
    
    # Check if required ports are allowed
    if sudo ufw status | grep -q "3000"; then
        echo -e "${GREEN}✓ Web5 DWN API port is allowed through firewall${NC}"
    else
        echo -e "${RED}✗ Web5 DWN API port is not allowed through firewall${NC}"
    fi
    
    if sudo ufw status | grep -q "9090"; then
        echo -e "${GREEN}✓ Prometheus port is allowed through firewall${NC}"
    else
        echo -e "${RED}✗ Prometheus port is not allowed through firewall${NC}"
    fi
else
    echo -e "${YELLOW}⚠ Firewall is not active${NC}"
fi
echo

# Check logs for errors
echo "Checking logs for errors..."
if journalctl -u web5-dwn -n 10 | grep -i error &> /dev/null; then
    echo -e "${RED}✗ Errors found in Web5 DWN logs${NC}"
else
    echo -e "${GREEN}✓ No recent errors in Web5 DWN logs${NC}"
fi

if journalctl -u prometheus -n 10 | grep -i error &> /dev/null; then
    echo -e "${RED}✗ Errors found in Prometheus logs${NC}"
else
    echo -e "${GREEN}✓ No recent errors in Prometheus logs${NC}"
fi
echo

# Check BIP Support through RPC
echo "Checking BIP support on testnet..."
BIP_CHECK=$(curl -s --user mainnet_8731:c8f7af8a-c33c-4f49-954c-a997a50b9a22 \
    -H 'Content-Type: application/json' \
    -d '{"jsonrpc":"1.0","id":"anya-test","method":"getblockchaininfo","params":[]}' \
    https://testnet.getblock.io/3333/ | jq -r '.result.softforks // empty')

if [[ -n "$BIP_CHECK" && "$BIP_CHECK" == *"taproot"* ]]; then
    echo -e "${GREEN}✓ BIP 341/342 (Taproot) is supported on testnet${NC}"
else
    echo -e "${YELLOW}⚠ Could not verify Taproot support on testnet${NC}"
fi
echo

# Check Anya Core directory structure
echo "Checking Anya Core directory structure..."
if [ -d "/home/anya/projectanya" ]; then
    echo -e "${GREEN}✓ Anya Core project directory exists${NC}"
    
    # Check Hexagonal Architecture directories
    if [ -d "/home/anya/projectanya/src/core" ] && [ -d "/home/anya/projectanya/src/adapters" ] && [ -d "/home/anya/projectanya/src/ports" ]; then
        echo -e "${GREEN}✓ Hexagonal Architecture directory structure is set up${NC}"
    else
        echo -e "${YELLOW}⚠ Hexagonal Architecture directory structure is incomplete${NC}"
    fi
else
    echo -e "${RED}✗ Anya Core project directory not found${NC}"
fi
echo

# Final status report
echo "==== Verification Summary ===="
echo -e "${GREEN}Anya Core verification completed.${NC}"
echo "If all checks passed, the server is properly configured using Bitcoin testnet."
echo "If there were any issues, please refer to the troubleshooting guide in SERVER_SETUP.md"
echo
echo "Please address any warnings or errors before proceeding."
echo 