#!/bin/bash
set -e

echo "Fixing missing dependencies in anya-core/Cargo.toml..."

# First, check if the dependencies already exist
if ! grep -q "bitcoin =" anya-core/Cargo.toml; then
    # Add bitcoin dependency to anya-core/Cargo.toml
    sed -i '/\[dependencies\]/a bitcoin = "0.29.2"' anya-core/Cargo.toml
    echo "Added bitcoin dependency to anya-core/Cargo.toml"
fi

if ! grep -q "serde_json =" anya-core/Cargo.toml; then
    # Add serde_json dependency to anya-core/Cargo.toml
    sed -i '/\[dependencies\]/a serde_json = "1.0.96"' anya-core/Cargo.toml
    echo "Added serde_json dependency to anya-core/Cargo.toml"
fi

# Fix any other potential imports
sed -i '/use std::str::FromStr;/d' anya-core/src/lib.rs
sed -i '5i use std::str::FromStr;' anya-core/src/lib.rs

echo "Dependencies fixed. Now building anya-core..."
cargo check -p anya-core

echo "Now running all tests..."
cargo test
