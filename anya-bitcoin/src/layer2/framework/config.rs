// Layer 2 Protocol Configuration
// This file contains configuration structures for Layer 2 protocols

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Base configuration for all Layer 2 protocols
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer2Config {
    /// Protocol name
    pub protocol_name: String,

    /// Network type (mainnet, testnet, etc.)
    pub network: String,

    /// Data directory for protocol data
    pub data_dir: PathBuf,

    /// Enable debug mode
    pub debug: bool,

    /// Timeout in seconds for operations
    pub timeout_seconds: u64,
}

impl Default for Layer2Config {
    fn default() -> Self {
        Self {
            protocol_name: "unknown".to_string(),
            network: "testnet".to_string(),
            data_dir: PathBuf::from("./data"),
            debug: false,
            timeout_seconds: 30,
        }
    }
}

/// Configuration for RGB protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RGBConfig {
    /// Base configuration
    pub base: Layer2Config,

    /// Schema ID to use
    pub schema_id: Option<String>,
}

impl Default for RGBConfig {
    fn default() -> Self {
        Self {
            base: Layer2Config {
                protocol_name: "rgb".to_string(),
                network: "testnet".to_string(),
                data_dir: PathBuf::from("./data/rgb"),
                debug: false,
                timeout_seconds: 30,
            },
            schema_id: None,
        }
    }
}

/// Configuration for Lightning Network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningConfig {
    /// Base configuration
    pub base: Layer2Config,

    /// RPC address for lnd
    pub rpc_address: String,

    /// Certificate path
    pub cert_path: PathBuf,

    /// Macaroon path
    pub macaroon_path: PathBuf,
}

impl Default for LightningConfig {
    fn default() -> Self {
        Self {
            base: Layer2Config {
                protocol_name: "lightning".to_string(),
                network: "testnet".to_string(),
                data_dir: PathBuf::from("./data/lightning"),
                debug: false,
                timeout_seconds: 30,
            },
            rpc_address: "127.0.0.1:10009".to_string(),
            cert_path: PathBuf::from("./certs/tls.cert"),
            macaroon_path: PathBuf::from("./certs/admin.macaroon"),
        }
    }
}
