#!/bin/bash

# Verify DAO Contract Structure
# This script checks that all required contracts are present and correctly reference each other

echo "== Verifying DAO Contract Structure =="
echo ""

CONTRACTS_DIR="/workspaces/Anya-core/contracts/dao"
SRC_DIR="/workspaces/Anya-core/src"
ERROR_COUNT=0

# Check if required contracts exist
check_contract() {
    if [ -f "$CONTRACTS_DIR/$1" ]; then
        echo "✓ Found $1"
        return 0
    else
        echo "✗ Missing contract: $1"
        ERROR_COUNT=$((ERROR_COUNT + 1))
        return 1
    fi
}

# Check if source file exists
check_src_file() {
    if [ -f "$SRC_DIR/$1" ]; then
        echo "✓ Found $1"
        return 0
    else
        echo "✗ Missing file: $1"
        ERROR_COUNT=$((ERROR_COUNT + 1))
        return 1
    fi
}

# Check if a contract contains a specific reference
check_reference() {
    CONTRACT="$CONTRACTS_DIR/$1"
    REFERENCE="$2"
    if grep -q "$REFERENCE" "$CONTRACT"; then
        echo "  ✓ $1 contains reference to $REFERENCE"
        return 0
    else
        echo "  ✗ $1 is missing reference to $REFERENCE"
        ERROR_COUNT=$((ERROR_COUNT + 1))
        return 1
    fi
}

# Check if a source file contains a specific reference
check_src_reference() {
    SRC_FILE="$SRC_DIR/$1"
    REFERENCE="$2"
    if grep -q "$REFERENCE" "$SRC_FILE"; then
        echo "  ✓ $1 contains reference to $REFERENCE"
        return 0
    else
        echo "  ✗ $1 is missing reference to $REFERENCE"
        ERROR_COUNT=$((ERROR_COUNT + 1))
        return 1
    fi
}

# Check core contracts
echo "Checking core contracts..."
check_contract "shared/dao-constants.clar"
check_contract "governance-traits.clar"
check_contract "multi-sig-governance.clar"
check_contract "decentralized-treasury-management.clar"
check_contract "reporting-system.clar"
check_contract "reporting-system-decentralized.clar"

echo ""
echo "Checking dependencies between contracts..."

# Check that treasury uses governance
check_reference "decentralized-treasury-management.clar" "GOVERNANCE_CONTRACT"
check_reference "decentralized-treasury-management.clar" "multi-sig-governance"

# Check that reporting uses governance
check_reference "reporting-system.clar" "GOVERNANCE_CONTRACT"
check_reference "reporting-system.clar" "multi-sig-governance"

# Check modified functions in reporting
check_reference "reporting-system-decentralized.clar" "is-governance-contract"
check_reference "reporting-system-decentralized.clar" "add-report-generator"

# Check BOLT12 implementation
echo ""
echo "Checking BOLT12 implementation..."
check_src_file "lightning/bolt12.rs"

# Core BOLT12 components
check_src_reference "lightning/bolt12.rs" "invoice_request"
check_src_reference "lightning/bolt12.rs" "payment"
check_src_reference "lightning/bolt12.rs" "refund"

echo ""
if [ $ERROR_COUNT -eq 0 ]; then
    echo "✓ All contracts are correctly structured!"
    exit 0
else
    echo "✗ Found $ERROR_COUNT issues in contract structure"
    exit 1
fi
