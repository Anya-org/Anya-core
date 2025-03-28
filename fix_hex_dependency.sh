#!/bin/bash
set -e

echo "Fixing hex dependency in core/Cargo.toml..."

# Add hex dependency to core/Cargo.toml
if ! grep -q "hex =" core/Cargo.toml; then
    sed -i '/\[dependencies\]/a hex = "0.4.3"' core/Cargo.toml
    echo "Added hex dependency to core/Cargo.toml"
fi

# Also clean up unused imports to fix warnings
if [ -f "core/src/l4_protocol/mod.rs" ]; then
    echo "Cleaning up unused imports in l4_protocol/mod.rs"
    
    # Fix unused imports
    sed -i 's/use bitcoin::secp256k1::{Secp256k1, KeyPair, PublicKey, XOnlyPublicKey};/use bitcoin::secp256k1::{PublicKey};/' core/src/l4_protocol/mod.rs
    sed -i '/use bitcoin::Transaction;/d' core/src/l4_protocol/mod.rs
    sed -i '/use std::sync::Arc;/d' core/src/l4_protocol/mod.rs
    sed -i '/use std::str::FromStr;/d' core/src/l4_protocol/mod.rs
    
    # Fix unused variable warning in rpc_adapter.rs
    if [ -f "core/src/l4_protocol/rpc_adapter.rs" ]; then
        sed -i 's/let payload/let _payload/' core/src/l4_protocol/rpc_adapter.rs
    fi
fi

# Build the project to make sure dependencies are resolved
echo "Building the project..."
cargo check

echo "Dependencies fixed. Now running all tests..."
