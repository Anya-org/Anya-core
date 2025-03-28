#!/bin/bash
set -e

echo "Fixing root Cargo.toml - removing invalid [lib] section..."

# Create a properly formatted root Cargo.toml without lib section
cat > Cargo.toml << 'TOML'
[workspace]
members = [
    "anya-core",
    "anya-mobile",
    "anyacore",
    "core",
    "cli",
    "installer",
    "mobile",
    "bitcoin-adapter",
    "dependencies/anya-bitcoin",
    "dependencies/anya-extensions",
    "dependencies/anya-enterprise",
    "scripts"
]

# Exclude these problematic paths
exclude = [
    "src/extensions/anya-extensions",
    "src/bitcoin/anya-bitcoin",
    "src/enterprise/anya-enterprise",
    "workspace"
]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Anya Development Team"]
description = "Bitcoin Layer 4 Protocol Implementation"
license = "Apache-2.0"

[workspace.dependencies]
# Bitcoin core dependencies
bitcoin = { version = "0.29.2", features = ["rand", "serde"] }
secp256k1 = { version = "0.24.0", features = ["rand", "serde"] }
bitcoincore-rpc = "0.16.0"

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

echo "Root Cargo.toml fixed. Now attempting to build..."
cargo check
