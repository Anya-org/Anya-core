#!/bin/bash
set -e

echo "Fixing the anya-core crate structure..."

# Create the directory structure
mkdir -p anya-core/src

# Create a basic Cargo.toml file with proper target specification
cat > anya-core/Cargo.toml << 'TOML'
[package]
name = "anya-core"
version = "0.1.0"
edition = "2021"
authors = ["Anya Development Team"]
description = "Bitcoin Layer 4 Protocol Implementation"
license = "Apache-2.0"

[dependencies]
# Use workspace dependencies
bitcoin = { workspace = true }
secp256k1 = { workspace = true }
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
log = { workspace = true }

# Layer 4 specific dependencies
reqwest = { version = "0.11.18", features = ["json"] }
url = "2.3.1"
dashmap = "5.4.0"

# Add these specifically
core = { path = "../core" }

# Target specification - this was missing
[lib]
name = "anya_core"
path = "src/lib.rs"

[[bin]]
name = "anya-core"
path = "src/main.rs"

[features]
default = ["hsm", "l4_protocol"]
hsm = []
adapters = []
audit = []
l4_protocol = []
silent_leaf = []
privacy = []
enterprise = []
TOML

# Create a basic lib.rs file
cat > anya-core/src/lib.rs << 'RUST'
//! Anya Core - Bitcoin Layer 4 Protocol
//! [AIR-3][AIS-3][BPC-3][AIP-3][RES-3]

// Re-export from core
pub use core::l4_protocol;

// Define module structure
pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum AnyaError {
        #[error("Protocol error: {0}")]
        Protocol(#[from] core::l4_protocol::ProtocolError),
        
        #[error("IO error: {0}")]
        Io(#[from] std::io::Error),
        
        #[error("Unknown error: {0}")]
        Unknown(String),
    }
}

pub mod api {
    use core::l4_protocol::{AnyaL4Protocol, DlcContract};
    use bitcoin::Network;
    
    pub struct AnyaApi {
        l4_protocol: AnyaL4Protocol,
    }
    
    impl AnyaApi {
        pub fn new(network: Network) -> Self {
            Self {
                l4_protocol: AnyaL4Protocol::new(network),
            }
        }
        
        pub fn init_hsm(&mut self, hsm_type: &str) -> Result<(), core::l4_protocol::ProtocolError> {
            self.l4_protocol.init_hsm(hsm_type)
        }
    }
}

// Export main API
pub use api::AnyaApi;
RUST

# Create a basic main.rs file
cat > anya-core/src/main.rs << 'RUST'
//! Anya Core CLI - Bitcoin Layer 4 Protocol
//! [AIR-3][AIS-3][BPC-3][AIP-3][RES-3]

use anya_core::AnyaApi;
use bitcoin::Network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();
    
    // Create a new API instance
    let mut api = AnyaApi::new(Network::Testnet);
    
    // Initialize HSM
    api.init_hsm("software")?;
    
    println!("Anya Core initialized successfully");
    println!("Running Bitcoin Layer 4 Protocol on testnet");
    
    Ok(())
}
RUST

echo "Anya-core crate structure fixed. Now attempting to build..."
cargo check -p anya-core
