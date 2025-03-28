#!/bin/bash
set -e

echo "Creating a proper root Cargo.toml file..."

# Create a valid workspace Cargo.toml for the root
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
    "dependencies/anya-enterprise"
]

# Exclude irrelevant directories or duplicate packages
exclude = [
    "src/extensions/anya-extensions",
    "src/bitcoin/anya-bitcoin",
    "src/enterprise/anya-enterprise",
    "scripts"
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

# Web5 dependency
web5 = { git = "https://github.com/TBD54566975/web5-rs", branch = "main" }

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
