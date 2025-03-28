#!/bin/bash
set -e

echo "Updating Cargo.toml with missing dependencies..."

# Check if Cargo.toml exists
if [ ! -f Cargo.toml ]; then
    echo "Error: Cargo.toml not found in current directory"
    exit 1
fi

# Add missing dependencies if not already present
if ! grep -q "bitcoin =" Cargo.toml; then
    sed -i '/\[dependencies\]/a bitcoin = { version = "0.29.2", features = ["rand", "serde"] }' Cargo.toml
    echo "Added bitcoin dependency"
fi

if ! grep -q "secp256k1 =" Cargo.toml; then
    sed -i '/\[dependencies\]/a secp256k1 = { version = "0.24.0", features = ["rand", "serde"] }' Cargo.toml
    echo "Added secp256k1 dependency"
fi

if ! grep -q "web5 =" Cargo.toml; then
    sed -i '/\[dependencies\]/a web5 = { version = "0.5.1", features = ["did", "dwn"] }' Cargo.toml
    echo "Added web5 dependency"
fi

if ! grep -q "argon2 =" Cargo.toml; then
    sed -i '/\[dependencies\]/a argon2 = "0.5.0"' Cargo.toml
    echo "Added argon2 dependency"
fi

if ! grep -q "blake3 =" Cargo.toml; then
    sed -i '/\[dependencies\]/a blake3 = "1.3.3"' Cargo.toml
    echo "Added blake3 dependency"
fi

echo "Dependencies updated successfully!"
