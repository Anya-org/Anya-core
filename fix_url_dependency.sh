#!/bin/bash
set -e

echo "Fixing URL dependency to avoid litemap compatibility issue..."

# Backup current Cargo.toml
cp core/Cargo.toml core/Cargo.toml.bak

# First, delete the existing url dependency from core/Cargo.toml
sed -i '/url =/d' core/Cargo.toml

# Add a very specific url version that doesn't trigger the problematic dependency chain
sed -i '/\[dependencies\]/a url = "=2.2.2"' core/Cargo.toml

echo "URL pinned to version 2.2.2 in core/Cargo.toml"

# Remove Cargo.lock to force dependency resolution
echo "Removing Cargo.lock to force resolution..."
rm -f Cargo.lock

echo "Building core only first..."
cargo build -p core

echo "Now building full project..."
cargo build
