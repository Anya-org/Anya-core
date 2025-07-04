#!/bin/bash
# Script to run BIP341 compliance tests
# Following project convention for running Bitcoin tests

# Set environment variable to only compile what's needed
export RUSTFLAGS="--cfg bip341_tests_only"

echo "Running BIP341 compliance tests..."
echo "=================================="

# Run the simplified test first (should work)
echo "Running simplified BIP341 tests..."
cargo test bitcoin::protocol::bip341_compliance_simple:: -- --nocapture

# Run the full BIP341 implementation tests
echo ""
echo "Running full BIP341 compliance tests..."
cargo test bitcoin::protocol::bip341_compliance:: -- --nocapture

echo ""
echo "=================================="
echo "Tests completed"

# Exit with the status of the last command
exit ${PIPESTATUS[0]}
