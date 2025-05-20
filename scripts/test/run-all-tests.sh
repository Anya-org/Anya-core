#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
set -e

echo "=== Running Anya DAO Test Suite ==="
echo "=================================="

# First verify all contracts
./scripts/verify-contracts.sh

# Run individual contract tests
echo -e "\nTesting Governance Token..."
clarinet test tests/governance-token.test.clar

echo -e "\nTesting DAO Core..."
clarinet test tests/dao-core.test.clar

echo -e "\nTesting Bitcoin Issuance..."
clarinet test tests/bitcoin-issuance.test.clar

echo -e "\nTesting DEX Adapter..."
clarinet test tests/dex-adapter.test.clar

echo -e "\nTesting Token Economics..."
clarinet test tests/token-economics.test.clar

echo -e "\nTesting Main DAO Contract..."
clarinet test tests/dao.test.clar

# Run integration tests
echo -e "\nRunning System Integration Tests..."
clarinet test tests/dao-system.test.ts

echo -e "\n✅ All tests completed successfully!" 