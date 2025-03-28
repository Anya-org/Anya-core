#!/bin/bash
set -e

echo "Fixing Cargo.toml syntax errors..."

# 1. Fix anya-core/Cargo.toml - correct the reqwest dependency
if [ -f "anya-core/Cargo.toml" ]; then
    # First find and replace the incorrect reqwest line
    sed -i '/reqwest = { version/d' anya-core/Cargo.toml
    
    # Then add the correct line (after the dependencies section)
    sed -i '/\[dependencies\]/a reqwest = { version = "0.11.18", default-features = false, features = ["json", "rustls-tls"] }' anya-core/Cargo.toml
    
    echo "Fixed anya-core/Cargo.toml"
fi

# 2. Fix core/Cargo.toml if it exists and has the same issue
if [ -f "core/Cargo.toml" ] && grep -q "reqwest" "core/Cargo.toml"; then
    # First find and replace the incorrect reqwest line
    sed -i '/reqwest = { version/d' core/Cargo.toml
    
    # Then add the correct line (after the dependencies section)
    sed -i '/\[dependencies\]/a reqwest = { version = "0.11.18", default-features = false, features = ["json", "rustls-tls"] }' core/Cargo.toml
    
    echo "Fixed core/Cargo.toml"
fi

# 3. Add resolver = "2" to the workspace
sed -i '/\[workspace\]/a resolver = "2"' Cargo.toml

# 4. Add patch for older Rust 1.75.0 (installed from source)
cat >> Cargo.toml << 'PATCH'

# Patches for Rust 1.75.0 compatibility
[patch.crates-io]
native-tls = { version = "0.2.11" }
ring = { version = "0.16.20" }
h2 = { version = "0.3.19" }
PATCH

echo "Cargo.toml syntax errors fixed. Now attempting to build..."
cargo check
