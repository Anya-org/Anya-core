#!/bin/bash
set -e

echo "Fixing dependency issues with a minimal approach..."

# 1. First, let's clean up any patches in the root Cargo.toml
# Create a temporary file without any patches
grep -v -A 100 '\[patch' Cargo.toml | grep -v -B 100 -m 1 '^\[patch' > Cargo.toml.tmp
mv Cargo.toml.tmp Cargo.toml

# 2. Manually edit the core/Cargo.toml to use an explicit reqwest with rustls
echo "Updating core/Cargo.toml..."
if [ -f "core/Cargo.toml" ]; then
    # Remove any existing reqwest dependency
    sed -i '/reqwest/d' core/Cargo.toml
    
    # Add a specific reqwest version that works with older Rust
    sed -i '/\[dependencies\]/a reqwest = { version = "0.11.11", default-features = false, features = ["json", "rustls-tls"] }' core/Cargo.toml
    
    # Remove url dependency that's pulling in newer dependencies
    sed -i '/url =/d' core/Cargo.toml
    
    # Add a specific version of url
    sed -i '/\[dependencies\]/a url = "2.2.2"' core/Cargo.toml
    
    echo "Updated core/Cargo.toml with compatible dependencies"
fi

# 3. Do the same for anya-core/Cargo.toml
echo "Updating anya-core/Cargo.toml..."
if [ -f "anya-core/Cargo.toml" ]; then
    # Remove any existing reqwest dependency
    sed -i '/reqwest/d' anya-core/Cargo.toml
    
    # Add a specific reqwest version that works with older Rust
    sed -i '/\[dependencies\]/a reqwest = { version = "0.11.11", default-features = false, features = ["json", "rustls-tls"] }' anya-core/Cargo.toml
    
    echo "Updated anya-core/Cargo.toml with compatible dependencies"
fi

# 4. Delete Cargo.lock to start fresh
echo "Deleting Cargo.lock to create a fresh dependency resolution..."
rm -f Cargo.lock

# 5. Run cargo check to generate a fresh Cargo.lock
echo "Generating a fresh Cargo.lock file..."
cargo check --lib

echo "Fix complete. The project should now build with Rust 1.75.0"
