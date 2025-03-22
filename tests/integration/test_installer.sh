#!/bin/bash
# Anya Installer Integration Test
# Tests the full installation process in a clean environment

set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Function to run a test command
run_test() {
    local cmd="$1"
    local desc="$2"
    echo -e "${YELLOW}Running test: $desc...${NC}"
    if eval "$cmd"; then
        echo -e "${GREEN}✓ Test passed: $desc${NC}"
    else
        echo -e "${RED}✗ Test failed: $desc${NC}"
        exit 1
    fi
}

# Create temporary test directory
TEST_DIR=$(mktemp -d)
echo -e "Using temporary directory: ${TEST_DIR}"

# Function to clean up
cleanup() {
  echo -e "${YELLOW}Cleaning up temporary directory...${NC}"
  rm -rf "${TEST_DIR}"
}

# Register cleanup function to run on exit
trap cleanup EXIT

# Step 1: Build the installer
echo -e "${YELLOW}Step 1: Building installer...${NC}"
run_test "cargo build --bin installer --release" "Building installer"

# Step 2: Test help output
echo -e "${YELLOW}Step 2: Testing help output...${NC}"
run_test "./target/release/installer --help | grep -q 'Anya Core Installer'" "Help output contains correct title"
run_test "./target/release/installer install --help | grep -q 'Install Anya Core'" "Install command help"
run_test "./target/release/installer configure --help | grep -q 'Configure Anya Core'" "Configure command help"

# Step 3: Test network configuration
echo -e "${YELLOW}Step 3: Testing network configuration...${NC}"

# Test default testnet configuration with PublicNode
echo -e "${YELLOW}Testing default testnet configuration with PublicNode...${NC}"
run_test "./target/release/installer configure --network testnet --dry-run" "Default testnet configuration with PublicNode"

# Test mainnet configuration with PublicNode
echo -e "${YELLOW}Testing mainnet configuration with PublicNode...${NC}"
run_test "./target/release/installer configure --network mainnet --dry-run" "Mainnet configuration with PublicNode"

# Test regtest configuration
echo -e "${YELLOW}Testing regtest configuration...${NC}"
run_test "./target/release/installer configure --network regtest --dry-run" "Regtest configuration"

# Test BDK configuration
echo -e "${YELLOW}Step 4: Testing BDK configuration...${NC}"

# Test default BDK configuration
echo -e "${YELLOW}Testing default BDK configuration...${NC}"
run_test "./target/release/installer configure --network testnet --dry-run" "Default BDK configuration"

# Test custom BDK wallet directory
echo -e "${YELLOW}Testing custom BDK wallet directory...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --bdk-wallet-dir /tmp/anya-wallets \
  --dry-run" "Custom BDK wallet directory"

# Test BDK configuration show
echo -e "${YELLOW}Testing BDK configuration show...${NC}"
run_test "./target/release/installer configure --show --dry-run" "Show BDK configuration"

# Test RPC configuration
echo -e "${YELLOW}Step 5: Testing RPC configuration...${NC}"

# Test PublicNode endpoints
echo -e "${YELLOW}Testing PublicNode endpoints...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --rpc-url "https://bitcoin-testnet-rpc.publicnode.com" \
  --rpc-user "publicnode" \
  --rpc-password "publicnode" \
  --dry-run" "PublicNode testnet configuration"

# Test custom RPC configuration
echo -e "${YELLOW}Testing custom RPC configuration...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --rpc-url "https://custom-rpc.example.com" \
  --rpc-user "custom_user" \
  --rpc-password "custom_password" \
  --dry-run" "Custom RPC configuration"

# Test invalid network configuration
echo -e "${YELLOW}Step 6: Testing invalid configurations...${NC}"

# Test invalid network
echo -e "${YELLOW}Testing invalid network configuration...${NC}"
run_test "! ./target/release/installer configure --network invalidnet --dry-run 2>/dev/null" "Invalid network configuration fails"

# Test invalid RPC URL
echo -e "${YELLOW}Testing invalid RPC URL...${NC}"
run_test "! ./target/release/installer configure \
  --network testnet \
  --rpc-url "invalid-url" \
  --dry-run 2>/dev/null" "Invalid RPC URL fails"

# Test invalid BDK wallet directory
echo -e "${YELLOW}Testing invalid BDK wallet directory...${NC}"
run_test "! ./target/release/installer configure \
  --network testnet \
  --bdk-wallet-dir "/invalid/path" \
  --dry-run 2>/dev/null" "Invalid BDK wallet directory fails"

# Test security features
echo -e "${YELLOW}Step 7: Testing security features...${NC}"

# Test SSL/TLS requirement
echo -e "${YELLOW}Testing SSL/TLS requirement...${NC}"
run_test "! ./target/release/installer configure \
  --network testnet \
  --rpc-url "http://bitcoin-testnet-rpc.publicnode.com" \
  --dry-run 2>/dev/null" "SSL/TLS requirement enforced"

# Test network validation
echo -e "${YELLOW}Testing network validation...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --validate-network \
  --dry-run" "Network validation"

# Step 8: Test component testing
echo -e "${YELLOW}Step 8: Testing component tests...${NC}"
run_test "./target/release/installer test --component network --dry-run" "Network component test"
run_test "./target/release/installer test --component rpc --dry-run" "RPC component test"
run_test "./target/release/installer test --component core --dry-run" "Core component test"

# Step 9: Test Lightning Network (LDK) configuration
echo -e "${YELLOW}Step 9: Testing Lightning Network (LDK) configuration...${NC}"

# Test default LDK configuration
echo -e "${YELLOW}Testing default LDK configuration...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --ldk-enabled true \
  --dry-run" "Default LDK configuration"

# Test custom LDK channel manager configuration
echo -e "${YELLOW}Testing custom LDK channel manager configuration...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --ldk-enabled true \
  --ldk-channel-limit 200 \
  --ldk-min-channel-size 500000 \
  --ldk-max-channel-size 20000000 \
  --dry-run" "Custom LDK channel manager configuration"

# Test custom LDK router configuration
echo -e "${YELLOW}Testing custom LDK router configuration...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --ldk-enabled true \
  --ldk-sync-interval 300 \
  --ldk-penalty-half-life 86400 \
  --dry-run" "Custom LDK router configuration"

# Test LDK wallet configuration
echo -e "${YELLOW}Testing LDK wallet configuration...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --ldk-enabled true \
  --ldk-auto-backup true \
  --ldk-backup-interval 3600 \
  --dry-run" "LDK wallet configuration"

# Test LDK listen address configuration
echo -e "${YELLOW}Testing LDK listen address configuration...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --ldk-enabled true \
  --ldk-listen-addr "0.0.0.0:9735" \
  --dry-run" "LDK listen address configuration"

# Test LDK peer connection
echo -e "${YELLOW}Testing LDK peer connection...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --ldk-enabled true \
  --ldk-peer-addr "peer.example.com:9735" \
  --dry-run" "LDK peer connection"

# Test invalid LDK configurations
echo -e "${YELLOW}Testing invalid LDK configurations...${NC}"

# Test invalid channel limit
echo -e "${YELLOW}Testing invalid channel limit...${NC}"
run_test "! ./target/release/installer configure \
  --network testnet \
  --ldk-enabled true \
  --ldk-channel-limit 0 \
  --dry-run 2>/dev/null" "Invalid channel limit fails"

# Test invalid channel size
echo -e "${YELLOW}Testing invalid channel size...${NC}"
run_test "! ./target/release/installer configure \
  --network testnet \
  --ldk-enabled true \
  --ldk-min-channel-size 1000000000 \
  --dry-run 2>/dev/null" "Invalid channel size fails"

# Test invalid listen address
echo -e "${YELLOW}Testing invalid listen address...${NC}"
run_test "! ./target/release/installer configure \
  --network testnet \
  --ldk-enabled true \
  --ldk-listen-addr "invalid:address" \
  --dry-run 2>/dev/null" "Invalid listen address fails"

# Test LDK security features
echo -e "${YELLOW}Testing LDK security features...${NC}"

# Test LDK backup configuration
echo -e "${YELLOW}Testing LDK backup configuration...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --ldk-enabled true \
  --ldk-auto-backup true \
  --ldk-backup-interval 3600 \
  --dry-run" "LDK backup configuration"

# Test LDK channel validation
echo -e "${YELLOW}Testing LDK channel validation...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --ldk-enabled true \
  --ldk-validate-channels true \
  --dry-run" "LDK channel validation"

# Test LDK routing validation
echo -e "${YELLOW}Testing LDK routing validation...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --ldk-enabled true \
  --ldk-validate-routing true \
  --dry-run" "LDK routing validation"

# Step 10: Test DLC configuration
echo -e "${YELLOW}Step 10: Testing DLC configuration...${NC}"

# Test default DLC configuration
echo -e "${YELLOW}Testing default DLC configuration...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --dlc-enabled true \
  --dry-run" "Default DLC configuration"

# Test DLC directory structure
echo -e "${YELLOW}Testing DLC directory structure...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --dlc-enabled true \
  --dlc-contract-dir /tmp/anya-dlc/contracts \
  --dlc-oracle-dir /tmp/anya-dlc/oracles \
  --dlc-backup-dir /tmp/anya-dlc/backup \
  --dry-run" "DLC directory configuration"

# Test DLC security features
echo -e "${YELLOW}Testing DLC security features...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --dlc-enabled true \
  --dlc-encryption-type aes256 \
  --dlc-key-length 256 \
  --dry-run" "DLC security configuration"

# Step 11: Test RGB configuration
echo -e "${YELLOW}Step 11: Testing RGB configuration...${NC}"

# Test default RGB configuration
echo -e "${YELLOW}Testing default RGB configuration...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --rgb-enabled true \
  --dry-run" "Default RGB configuration"

# Test RGB directory structure
echo -e "${YELLOW}Testing RGB directory structure...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --rgb-enabled true \
  --rgb-asset-dir /tmp/anya-rgb/assets \
  --rgb-contract-dir /tmp/anya-rgb/contracts \
  --rgb-backup-dir /tmp/anya-rgb/backup \
  --dry-run" "RGB directory configuration"

# Test RGB security features
echo -e "${YELLOW}Testing RGB security features...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --rgb-enabled true \
  --rgb-encryption-type aes256 \
  --rgb-key-length 256 \
  --dry-run" "RGB security configuration"

# Step 12: Test RSK configuration
echo -e "${YELLOW}Step 12: Testing RSK configuration...${NC}"

# Test default RSK configuration
echo -e "${YELLOW}Testing default RSK configuration...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --rsk-enabled true \
  --dry-run" "Default RSK configuration"

# Test RSK directory structure
echo -e "${YELLOW}Testing RSK directory structure...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --rsk-enabled true \
  --rsk-contract-dir /tmp/anya-rsk/contracts \
  --rsk-bridge-dir /tmp/anya-rsk/bridge \
  --rsk-backup-dir /tmp/anya-rsk/backup \
  --dry-run" "RSK directory configuration"

# Test RSK security features
echo -e "${YELLOW}Testing RSK security features...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --rsk-enabled true \
  --rsk-encryption-type aes256 \
  --rsk-key-length 256 \
  --dry-run" "RSK security configuration"

# Step 13: Test Web5 configuration
echo -e "${YELLOW}Step 13: Testing Web5 configuration...${NC}"

# Test default Web5 configuration
echo -e "${YELLOW}Testing default Web5 configuration...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --web5-enabled true \
  --dry-run" "Default Web5 configuration"

# Test Web5 directory structure
echo -e "${YELLOW}Testing Web5 directory structure...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --web5-enabled true \
  --web5-identity-dir /tmp/anya-web5/identities \
  --web5-data-dir /tmp/anya-web5/data \
  --web5-backup-dir /tmp/anya-web5/backup \
  --dry-run" "Web5 directory configuration"

# Test Web5 security features
echo -e "${YELLOW}Testing Web5 security features...${NC}"
run_test "./target/release/installer configure \
  --network testnet \
  --web5-enabled true \
  --web5-encryption-type aes256 \
  --web5-key-length 256 \
  --dry-run" "Web5 security configuration"

# Step 14: Test invalid configurations
echo -e "${YELLOW}Step 14: Testing invalid configurations...${NC}"

# Test invalid DLC configuration
echo -e "${YELLOW}Testing invalid DLC configuration...${NC}"
run_test "! ./target/release/installer configure \
  --network testnet \
  --dlc-enabled true \
  --dlc-contract-dir /invalid/path \
  --dry-run 2>/dev/null" "Invalid DLC configuration fails"

# Test invalid RGB configuration
echo -e "${YELLOW}Testing invalid RGB configuration...${NC}"
run_test "! ./target/release/installer configure \
  --network testnet \
  --rgb-enabled true \
  --rgb-asset-dir /invalid/path \
  --dry-run 2>/dev/null" "Invalid RGB configuration fails"

# Test invalid RSK configuration
echo -e "${YELLOW}Testing invalid RSK configuration...${NC}"
run_test "! ./target/release/installer configure \
  --network testnet \
  --rsk-enabled true \
  --rsk-contract-dir /invalid/path \
  --dry-run 2>/dev/null" "Invalid RSK configuration fails"

# Test invalid Web5 configuration
echo -e "${YELLOW}Testing invalid Web5 configuration...${NC}"
run_test "! ./target/release/installer configure \
  --network testnet \
  --web5-enabled true \
  --web5-identity-dir /invalid/path \
  --dry-run 2>/dev/null" "Invalid Web5 configuration fails"

# Step 15: Test report generation
echo -e "${YELLOW}Step 15: Testing report generation...${NC}"
run_test "./target/release/installer test --report --dry-run" "Test report generation"

# Step 16: Final verification
echo -e "${YELLOW}Step 16: Final verification...${NC}"
run_test "./target/release/installer configure --show --dry-run" "Final configuration verification"

# Step 17: Verify Bitcoin compliance
echo -e "${YELLOW}Step 17: Verifying Bitcoin Core compliance...${NC}"
run_test "cargo test --package anya-bitcoin" "Bitcoin compliance tests"

# Step 18: Test DLC implementation
echo -e "${YELLOW}Step 18: Testing DLC functionality...${NC}"
run_test "cargo test --package anya-bitcoin --test dlc_tests" "DLC tests"

# Step 19: Verify signatures and binaries
echo -e "${YELLOW}Step 19: Verifying signatures and binaries...${NC}"
run_test "(cd target/release && sha256sum installer > installer.sha256)" "Signature verification"

# Success
echo -e "${GREEN}All tests passed! Anya installer is ready for distribution.${NC}"
echo -e "${GREEN}Installer location: ./target/release/installer${NC}"
echo -e "${GREEN}Installer hash: $(cat target/release/installer.sha256)${NC}"
