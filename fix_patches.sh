#!/bin/bash
set -e

echo "Fixing patch configuration in Cargo.toml..."

# Create a clean Cargo.toml without the problematic patches
cat > Cargo.toml << 'TOML'
[workspace]
resolver = "2"
members = [
    "anya-core",
    "core",
    "cli"
]

# Exclude problematic directories
exclude = [
    "src/extensions",
    "src/bitcoin",
    "src/enterprise",
    "workspace",
    "anya-mobile"
]

[workspace.dependencies]
# Bitcoin core dependencies
bitcoin = { version = "0.29.2", features = ["rand", "serde"] }
secp256k1 = { version = "0.24.0", features = ["rand", "serde"] }

# Common dependencies
tokio = { version = "1.28.1", features = ["full"] }
serde = { version = "1.0.160", features = ["derive"] }
serde_json = "1.0.96"
anyhow = "1.0.70"
thiserror = "1.0.40"
log = "0.4.17"
env_logger = "0.10.0"

# Crypto dependencies
sha2 = "0.10.6"
hex = "0.4.3"
TOML

echo "Fixed Cargo.toml. Now checking core/Cargo.toml..."

# Update core/Cargo.toml to use rustls-tls explicitly
if [ -f "core/Cargo.toml" ]; then
    sed -i 's/reqwest = { version = "0.11.18".*/reqwest = { version = "0.11.18", default-features = false, features = ["json", "rustls-tls"] }/' core/Cargo.toml
    echo "Updated core/Cargo.toml"
fi

# Update anya-core/Cargo.toml to use rustls-tls explicitly
if [ -f "anya-core/Cargo.toml" ]; then
    sed -i 's/reqwest = { version = "0.11.18".*/reqwest = { version = "0.11.18", default-features = false, features = ["json", "rustls-tls"] }/' anya-core/Cargo.toml
    echo "Updated anya-core/Cargo.toml"
fi

echo "Running cargo check to verify the fixes..."
cargo check

echo "Patch configuration fixed. You can now run anya-core with:"
echo "cargo run --bin anya-core"
