//! Bitcoin Core Implementation
//! Following hexagonal architecture principles and official Bitcoin Improvement Proposals (BIPs)

use std::{path::PathBuf, sync::Arc};

// Re-export core modules
pub mod adapters;
pub mod core;
pub mod error;
pub mod layer2;
pub mod ports;
pub mod prelude;
pub mod protocol;
pub mod riscv;
pub mod security;
pub mod testing;

/// Configuration for Bitcoin node
#[derive(Debug, Clone)]
pub struct Config {
    /// Bitcoin network (mainnet, testnet, etc.)
    pub network: bitcoin::Network,
    /// Data directory for Bitcoin data
    pub datadir: PathBuf,
    /// Maximum number of peers
    pub max_peers: u32, // Default: 125
    /// Minimum number of peers
    pub min_peers: u32, // Default: 8
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

/// Main Bitcoin node implementation
pub struct BitcoinNode {
    #[allow(dead_code)] // Preserved for compatibility with upstream Bitcoin libraries and future protocol upgrades (see docs/INDEX_CORRECTED.md)
    config: Config,
    #[allow(dead_code)] // Preserved for extensibility and consensus validation (see docs/INDEX_CORRECTED.md)
    consensus: Arc<dyn core::consensus::Validator>,
    #[allow(dead_code)] // Preserved for mempool management and future features (see docs/INDEX_CORRECTED.md)
    mempool: Arc<dyn core::mempool::Mempool>,
    #[allow(dead_code)] // Preserved for P2P network extensibility (see docs/INDEX_CORRECTED.md)
    network: Arc<dyn core::network::P2P>,
    /// Layer 2 protocol registry
    layer2_registry: Option<Arc<layer2::framework::Layer2Registry>>,
}

impl BitcoinNode {
    /// Create a new Bitcoin node with the given configuration
    pub fn new(config: Config) -> Result<Self, error::AnyaError> {
        // These would normally be instantiated with concrete implementations
        let consensus = Arc::new(core::consensus::NoopValidator {});
        let mempool = Arc::new(core::mempool::NoopMempool {});
        let network = Arc::new(core::network::NoopP2P {});

        Ok(Self {
            config,
            consensus,
            mempool,
            network,
            layer2_registry: None,
        })
    }

    /// Start the Bitcoin node
    pub fn start(&mut self) -> Result<(), error::AnyaError> {
        // Initialize Layer 2 factory and registry
        let factory = Arc::new(layer2::framework::factory::Layer2Factory::new());
        let registry = Arc::new(layer2::framework::Layer2Registry::new(factory));
        self.layer2_registry = Some(registry);

        Ok(())
    }

    /// Get the Layer 2 protocol registry
    pub fn layer2_registry(&self) -> Option<Arc<layer2::framework::Layer2Registry>> {
        self.layer2_registry.clone()
    }
}
