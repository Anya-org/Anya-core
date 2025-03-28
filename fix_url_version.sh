#!/bin/bash
set -e

echo "Fixing URL crate version to avoid litemap compatibility issue..."

# Add a patch for url in Cargo.toml
cat >> Cargo.toml << 'PATCH'

# Pin url to an older version that doesn't require newer Rust
[patch.crates-io]
url = { version = "=2.2.2" }
PATCH

# Make sure core is using explicit URL dependency
if [ -f "core/Cargo.toml" ]; then
    # Replace existing url dependency with explicit version
    sed -i '/url = /d' core/Cargo.toml
    sed -i '/\[dependencies\]/a url = { version = "2.2.2" }' core/Cargo.toml
    echo "Updated URL version in core/Cargo.toml"
fi

# Same for anya-core if needed
if [ -f "anya-core/Cargo.toml" ] && grep -q "url" "anya-core/Cargo.toml"; then
    sed -i '/url = /d' anya-core/Cargo.toml
    sed -i '/\[dependencies\]/a url = { version = "2.2.2" }' anya-core/Cargo.toml
    echo "Updated URL version in anya-core/Cargo.toml"
fi

echo "Running cargo update to apply the patch..."
cargo update -p url --precise 2.2.2

echo "Running cargo check to verify the fix..."
cargo check --lib
