//! Bitcoin Core Implementation
//! Following hexagonal architecture principles and Bitcoin Development Framework v2.5

use std::{sync::Arc, path::PathBuf};

// Re-export core modules
pub mod core;
pub mod layer2;
pub mod ports;
pub mod adapters;
pub mod protocol;
pub mod riscv;
pub mod security;

// Configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub network: bitcoin::Network,
    pub datadir: PathBuf,
    pub max_peers: u32,      // Default: 125
    pub min_peers: u32,      // Default: 8
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: bitcoin::Network::Bitcoin,
            datadir: PathBuf::from("~/.bitcoin"),
            max_peers: 125,
            min_peers: 8,
        }
    }
}

// Main node implementation
pub struct BitcoinNode {
    config: Config,
    consensus: core::consensus::Validator,
    mempool: core::mempool::Mempool,
    network: core::network::P2P,
    /// Layer 2 protocol registry
    layer2_registry: Option<Arc<layer2::framework::Layer2Registry>>,
}

impl BitcoinNode {
    pub fn new(config: Config) -> Result<Self, bitcoin::Error> {
        Ok(Self {
            consensus: core::consensus::Validator::new(&config)?,
            mempool: core::mempool::Mempool::new(&config)?,
            network: core::network::P2P::new(&config)?,
            config,
            layer2_registry: None,
        })
    }

    pub fn start(&mut self) -> Result<(), bitcoin::Error> {
        // Initialize Layer 2 factory and registry
        let factory = Arc::new(layer2::framework::Layer2Factory::new());
        let registry = Arc::new(layer2::framework::Layer2Registry::new(factory));
        self.layer2_registry = Some(registry);
        
        Ok(())
    }
    
    /// Get Layer 2 protocol registry
    pub fn layer2_registry(&self) -> Option<Arc<layer2::framework::Layer2Registry>> {
        self.layer2_registry.clone()
    }
}
