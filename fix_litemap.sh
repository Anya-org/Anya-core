#!/bin/bash
set -e

echo "Fixing litemap compatibility issue..."

# Find the latest version of litemap compatible with Rust 1.75.0
# According to the error message, we need to pin to an older version
echo "Updating litemap to a compatible version..."
cargo update litemap --precise 0.6.0

# If that fails, try a different approach to pin the version in Cargo.toml
if [ $? -ne 0 ]; then
    echo "Direct update failed. Adding patch to root Cargo.toml..."
    
    # Add a patch section to Cargo.toml
    cat >> Cargo.toml << 'PATCH'

# Patch for older Rust version compatibility
[patch.crates-io]
litemap = { version = "=0.6.0" }
PATCH
fi

echo "Running cargo check to verify the fix..."
cargo check --lib
