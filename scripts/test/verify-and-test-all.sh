#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
set -e

echo "==============================================="
echo "Anya DAO - Complete Verification and Test Suite"
echo "==============================================="

# Step 1: Verify all contracts
./scripts/verify-contracts.sh

# Step 2: Run all tests
./scripts/run-all-tests.sh

# Step 3: Generate compliance report
npx ts-node scripts/generate-compliance-report.ts

echo -e "\n✅ Verification and testing complete!"
echo "Check compliance-report.json for detailed compliance information." 