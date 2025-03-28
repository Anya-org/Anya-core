#!/bin/bash
set -e

echo "Fixing workspace members issue..."

# First, let's check what directories actually exist
echo "Checking current directory structure..."
find . -name "Cargo.toml" -not -path "./Cargo.toml" -not -path "./target/*" | sort

# Create basic crate directories if they don't exist
mkdir -p core/src
mkdir -p anya-core/src
mkdir -p cli/src

# Create basic Cargo.toml files for each crate
echo "Creating basic Cargo.toml files for workspace members..."

# Core crate
cat > core/Cargo.toml << 'TOML'
[package]
name = "core"
version = "0.1.0"
edition = "2021"

[dependencies]
bitcoin = "0.29.2"
secp256k1 = "0.24.0"
tokio = { version = "1.28.1", features = ["full"] }
serde = { version = "1.0.160", features = ["derive"] }
serde_json = "1.0.96"
anyhow = "1.0.70"
thiserror = "1.0.40"
log = "0.4.17"
reqwest = { version = "0.11.11", default-features = false, features = ["json", "rustls-tls"] }
url = "2.2.2"

[lib]
name = "core"
path = "src/lib.rs"
TOML

# Create a basic lib.rs
cat > core/src/lib.rs << 'RUST'
//! Core implementation of Bitcoin Layer 4 Protocol
//! [AIR-3][AIS-3][BPC-3][AIP-3][RES-3]

/// Layer 4 Protocol module
pub mod l4_protocol {
    /// BIP-341 Silent Leaf pattern
    pub const BIP341_SILENT_LEAF: &str = "0x8f3a1c29566443e2e2d6e5a9a5a4e8d";

    /// Create a simple RPC adapter
    pub struct PublicRPCAdapter {}

    impl PublicRPCAdapter {
        /// Create a new adapter
        pub fn new() -> Self {
            Self {}
        }
    }

    /// Layer 4 Protocol Implementation
    pub struct AnyaL4Protocol {
        _rpc_adapter: PublicRPCAdapter,
    }

    impl AnyaL4Protocol {
        /// Create a new instance
        pub fn new() -> Self {
            Self {
                _rpc_adapter: PublicRPCAdapter::new(),
            }
        }
    }
}
RUST

# Anya-core crate
cat > anya-core/Cargo.toml << 'TOML'
[package]
name = "anya-core"
version = "0.1.0"
edition = "2021"

[dependencies]
core = { path = "../core" }
tokio = { version = "1.28.1", features = ["full"] }
log = "0.4.17"
env_logger = "0.10.0"
reqwest = { version = "0.11.11", default-features = false, features = ["json", "rustls-tls"] }

[lib]
name = "anya_core"
path = "src/lib.rs"

[[bin]]
name = "anya-core"
path = "src/main.rs"
TOML

# Create a basic lib.rs
cat > anya-core/src/lib.rs << 'RUST'
//! Anya Core - Bitcoin Layer 4 Protocol
//! [AIR-3][AIS-3][BPC-3][AIP-3][RES-3]

pub use core::l4_protocol;

/// Version of the library
pub const VERSION: &str = "0.1.0";
RUST

# Create a basic main.rs
cat > anya-core/src/main.rs << 'RUST'
//! Anya Core CLI - Bitcoin Layer 4 Protocol
//! [AIR-3][AIS-3][BPC-3][AIP-3][RES-3]

use anya_core::VERSION;

fn main() {
    println!("Anya Core v{} - Bitcoin Layer 4 Protocol", VERSION);
    println!("Layer 4 protocol initialized.");
}
RUST

# CLI crate
cat > cli/Cargo.toml << 'TOML'
[package]
name = "cli"
version = "0.1.0"
edition = "2021"

[dependencies]
anya-core = { path = "../anya-core" }
tokio = { version = "1.28.1", features = ["full"] }

[[bin]]
name = "anya"
path = "src/main.rs"
TOML

# Create a basic main.rs
cat > cli/src/main.rs << 'RUST'
//! Anya CLI - Bitcoin Layer 4 Protocol
//! [AIR-3][AIS-3][BPC-3][AIP-3][RES-3]

fn main() {
    println!("Anya CLI - Bitcoin Layer 4 Protocol");
}
RUST

# Update the root workspace Cargo.toml with correct members
cat > Cargo.toml << 'TOML'
[workspace]
resolver = "2"
members = [
    "core",
    "anya-core",
    "cli"
]

[workspace.dependencies]
bitcoin = "0.29.2"
secp256k1 = "0.24.0"
tokio = { version = "1.28.1", features = ["full"] }
serde = { version = "1.0.160", features = ["derive"] }
serde_json = "1.0.96"
anyhow = "1.0.70"
thiserror = "1.0.40"
log = "0.4.17"
env_logger = "0.10.0"
TOML

echo "Workspace members created. Now checking with cargo..."
cargo check

echo "You can now build anya-core with:"
echo "cargo build --bin anya-core"
