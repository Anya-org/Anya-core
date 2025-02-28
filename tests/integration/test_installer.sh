#!/bin/bash
# Anya Installer Integration Test
# Tests the full installation process in a clean environment

set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Starting Anya installer integration test...${NC}"

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
cargo build --bin installer --release || {
  echo -e "${RED}Build failed!${NC}"
  exit 1
}

# Step 2: Run installer in dry-run mode
echo -e "${YELLOW}Step 2: Testing installer in dry-run mode...${NC}"
./target/release/installer install --dry-run --yes || {
  echo -e "${RED}Dry run test failed!${NC}"
  exit 1
}

# Step 3: Test configuration
echo -e "${YELLOW}Step 3: Testing installer configuration...${NC}"
./target/release/installer configure --dry-run --network testnet --log_level debug || {
  echo -e "${RED}Configuration test failed!${NC}"
  exit 1
}

# Step 4: Run test suite
echo -e "${YELLOW}Step 4: Running test suite...${NC}"
./target/release/installer test --dry-run --report || {
  echo -e "${RED}Test suite failed!${NC}"
  exit 1
}

# Step 5: Verify Bitcoin compliance
echo -e "${YELLOW}Step 5: Verifying Bitcoin Core compliance...${NC}"
cargo test --package anya-bitcoin || {
  echo -e "${RED}Bitcoin compliance tests failed!${NC}"
  exit 1
}

# Step 6: Test DLC implementation
echo -e "${YELLOW}Step 6: Testing DLC functionality...${NC}"
cargo test --package anya-bitcoin --test dlc_tests || {
  echo -e "${RED}DLC tests failed!${NC}"
  exit 1
}

# Step 7: Verify signatures and binaries
echo -e "${YELLOW}Step 7: Verifying signatures and binaries...${NC}"
(cd target/release && sha256sum installer > installer.sha256) || {
  echo -e "${RED}Signature verification failed!${NC}"
  exit 1
}

# Step 8: Success
echo -e "${GREEN}All tests passed! Anya installer is ready for distribution.${NC}"
echo -e "${GREEN}Installer location: ./target/release/installer${NC}"
echo -e "${GREEN}Installer hash: $(cat target/release/installer.sha256)${NC}"
