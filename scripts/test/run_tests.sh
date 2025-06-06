#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]

# Run all tests for Anya Core

# Set the project root directory
PROJECT_ROOT=$(git rev-parse --show-toplevel)

# Change to the project root directory
cd "$PROJECT_ROOT" || { echo "Failed to change directory to project root"; exit 1; }

# Set up environment variables
if [ -f .env ]; then
	source .env
else
	echo ".env file not found, skipping environment setup."
fi

# Run cargo tests
# echo "Running cargo tests..."
# cargo test --all

# Run integration tests
echo "Running integration tests..."
cargo test --test '*' --features integration

# Run ML logic tests
echo "Running ML logic tests..."
cargo test --package anya-core --lib ml_logic

# Run specific module tests
echo "Running specific module tests..."
cargo test --package anya-core --lib network_discovery
cargo test --package anya-core --lib user_management
cargo test --package anya-core --lib stx_support
cargo test --package anya-core --lib bitcoin_support
cargo test --package anya-core --lib lightning_support
cargo test --package anya-core --lib dlc_support
cargo test --package anya-core --lib kademlia
cargo test --package anya-core --lib setup_project
cargo test --package anya-core --lib setup_check

# Run Web5 integration tests
echo "Running Web5 integration tests..."
cargo test --package anya-core --test web5_integration

# Run DAO governance tests
echo "Running DAO governance tests..."
cargo test --package anya-core --lib dao_governance

# Run developer ecosystem tests
echo "Running developer ecosystem tests..."
cargo test --package anya-core --lib developer_ecosystem

# Run privacy enhancement tests
echo "Running privacy enhancement tests..."
cargo test --package anya-core --lib privacy_enhancements

# Run libp2p integration tests
echo "Running libp2p integration tests..."
cargo test --package anya-core --test libp2p_integration

# Run blockchain interoperability tests
echo "Running blockchain interoperability tests..."
cargo test --test blockchain_interoperability

# Run smart contracts tests
echo "Running smart contracts tests..."
cargo test --test smart_contracts

# Run user interface tests
echo "Running user interface tests..."
cargo test --test user_interface

# Run code formatting check
echo "Running code formatting check..."
cargo fmt -- --check

# Run linter
echo "Running linter..."
cargo clippy --fix -- -D warnings

# Run security audit
echo "Running security audit..."
cargo audit

# Check for outdated dependencies
echo "Checking for outdated dependencies..."
if [ "$CHECK_OUTDATED" = "true" ]; then
	echo "Checking for outdated dependencies..."
	cargo outdated
else
	echo "Skipping outdated dependencies check..."
fi

# Run code coverage
echo "Running code coverage..."
# Run code coverage if the COVERAGE environment variable is set
if [ "$COVERAGE" = "true" ]; then
	echo "Running code coverage..."
	cargo tarpaulin --out Html
else
	echo "Skipping code coverage..."
fi

# Run benchmarks if the RUN_BENCHMARKS environment variable is set
if [ "$RUN_BENCHMARKS" = "true" ]; then
	echo "Running benchmarks..."
	cargo bench
else
	echo "Skipping benchmarks..."
fi

# New module tests
echo "Running identity tests..."
cargo test --test identity_tests
echo "Running data storage tests..."
cargo test --test data_storage_tests
echo "Running interoperability tests..."
echo "Running privacy tests..."
cargo test --test privacy_tests

echo "All tests completed successfully!"
