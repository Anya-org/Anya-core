#!/bin/bash
set -e

echo "Running Layer 4 protocol tests..."

# Build the project
cargo build

# Run the unit tests
cargo test --lib

# Run the integration tests
cargo test --test l4_protocol_tests

echo "Running anya-core with different arguments..."

# Test default behavior
echo -e "\nRunning with default settings:"
cargo run --bin anya-core

# Test with network specification
echo -e "\nRunning on mainnet:"
cargo run --bin anya-core -- --network mainnet

# Test help command
echo -e "\nShowing help:"
cargo run --bin anya-core -- help

# Test DLC contract creation
echo -e "\nCreating DLC contract:"
cargo run --bin anya-core -- create-dlc 03a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2 outcome1,outcome2

echo "All tests completed successfully!"
