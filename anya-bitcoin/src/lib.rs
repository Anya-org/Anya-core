#![feature(edition2021)]
//! Bitcoin Core Implementation
//! Consensus-critical code only

use bitcoin::{
    Block, 
    BlockHeader,
    Transaction,
    Network,
    BlockHash,
    Error as BitcoinError,
    consensus::encode::deserialize,
    util::hash::Hash,
};
use std::{sync::Arc, path::PathBuf};
use log::{info, warn, error};

// Core Bitcoin modules only
pub mod consensus {
    pub mod validation;   // Block/tx validation
    pub mod rules;       // Consensus rules
    pub mod params;      // Network parameters
}

pub mod mempool {
    pub mod pool;        // Transaction mempool
    pub mod policy;      // Mempool policies
    pub mod fees;        // Fee estimation
}

pub mod net {
    pub mod p2p;        // P2P networking
    pub mod messages;   // Network messages
    pub mod peers;      // Peer management
}

pub mod script {
    pub mod interpreter; // Script verification
    pub mod standard;    // Standard scripts
}

/// AIM-004: Layer 2 Integration Modules
/// Layer 2 protocols for Bitcoin implemented with hexagonal architecture
pub mod layer2;

#[derive(Debug, Clone)]
pub struct Config {
    network: Network,
    datadir: PathBuf,
    max_peers: u32,      // Default: 125
    min_peers: u32,      // Default: 8
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: Network::Bitcoin,
            datadir: PathBuf::from("~/.bitcoin"),
            max_peers: 125,
            min_peers: 8,
        }
    }
}

pub struct BitcoinNode {
    config: Config,
    consensus: consensus::validation::Validator,
    mempool: mempool::pool::Mempool,
    network: net::p2p::P2P,
    /// Layer 2 protocol registry
    layer2_registry: Option<Arc<layer2::framework::Layer2Registry>>,
}

impl BitcoinNode {
    pub fn new(config: Config) -> Result<Self, BitcoinError> {
        Ok(Self {
            consensus: consensus::validation::Validator::new(&config)?,
            mempool: mempool::pool::Mempool::new(&config)?,
            network: net::p2p::P2P::new(&config)?,
            config,
            layer2_registry: None,
        })
    }

    pub fn start(&mut self) -> Result<(), BitcoinError> {
        info!("Starting Bitcoin node");
        self.consensus.start()?;
        self.mempool.start()?;
        self.network.start()?;
        
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
