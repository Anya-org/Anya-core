#![forbid(unsafe_code)]
#![warn(missing_docs)]
//! Bitcoin Network implementation for Anya Core
//! 
//! This package provides Bitcoin network functionality, including P2P connectivity,
//! block and transaction propagation, and mempool management.

use bitcoin::{
    Network,
    Transaction,
    Block,
    BlockHeader,
    BlockHash,
    Error as BitcoinError,
};
use std::sync::Arc;
use std::path::PathBuf;
use thiserror::Error;
use tokio::sync::RwLock;
use log::{info, warn, error, debug};

// Submodules
pub mod p2p;
pub mod mempool;
pub mod blockchain;

/// Bitcoin Network error type
#[derive(Debug, Error)]
pub enum BitcoinNetworkError {
    /// Bitcoin protocol error
    #[error("Bitcoin protocol error: {0}")]
    BitcoinError(#[from] BitcoinError),
    
    /// Connection error
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    /// Block validation error
    #[error("Block validation error: {0}")]
    BlockValidationError(String),
    
    /// Transaction validation error
    #[error("Transaction validation error: {0}")]
    TransactionValidationError(String),
    
    /// General error
    #[error("Bitcoin network error: {0}")]
    General(String),
}

/// Bitcoin Network configuration
#[derive(Debug, Clone)]
pub struct BitcoinNetworkConfig {
    /// Bitcoin network
    pub network: Network,
    /// Data directory
    pub datadir: PathBuf,
    /// User agent string
    pub user_agent: String,
    /// Maximum number of peers
    pub max_peers: u32,
    /// Minimum number of peers
    pub min_peers: u32,
    /// DNS seeds to use
    pub dns_seeds: Vec<String>,
    /// Connect to these peers
    pub connect: Vec<String>,
}

impl Default for BitcoinNetworkConfig {
    fn default() -> Self {
        Self {
            network: Network::Testnet,
            datadir: PathBuf::from(".anya/bitcoin"),
            user_agent: format!("Anya-Core/{}", env!("CARGO_PKG_VERSION")),
            max_peers: 125,
            min_peers: 8,
            dns_seeds: vec![],
            connect: vec![],
        }
    }
}

/// Bitcoin Network service
pub struct BitcoinNetwork {
    /// Network configuration
    config: BitcoinNetworkConfig,
    /// P2P manager
    p2p_manager: Arc<RwLock<p2p::P2PManager>>,
    /// Mempool
    mempool: Arc<RwLock<mempool::Mempool>>,
    /// Blockchain manager
    blockchain: Arc<RwLock<blockchain::Blockchain>>,
}

impl BitcoinNetwork {
    /// Create a new Bitcoin Network service
    pub async fn new(config: BitcoinNetworkConfig) -> Result<Self, BitcoinNetworkError> {
        info!("Creating Bitcoin Network service for {:?}", config.network);
        
        // Create the P2P manager
        let p2p_manager = p2p::P2PManager::new(config.clone()).await?;
        
        // Create the mempool
        let mempool = mempool::Mempool::new(config.clone()).await?;
        
        // Create the blockchain manager
        let blockchain = blockchain::Blockchain::new(config.clone()).await?;
        
        Ok(Self {
            config,
            p2p_manager: Arc::new(RwLock::new(p2p_manager)),
            mempool: Arc::new(RwLock::new(mempool)),
            blockchain: Arc::new(RwLock::new(blockchain)),
        })
    }
    
    /// Start the Bitcoin Network service
    pub async fn start(&self) -> Result<(), BitcoinNetworkError> {
        info!("Starting Bitcoin Network service");
        
        // Start the P2P manager
        {
            let mut p2p_manager = self.p2p_manager.write().await;
            p2p_manager.start().await?;
        }
        
        // Start the mempool
        {
            let mut mempool = self.mempool.write().await;
            mempool.start().await?;
        }
        
        // Start the blockchain manager
        {
            let mut blockchain = self.blockchain.write().await;
            blockchain.start().await?;
        }
        
        info!("Bitcoin Network service started");
        Ok(())
    }
    
    /// Stop the Bitcoin Network service
    pub async fn stop(&self) -> Result<(), BitcoinNetworkError> {
        info!("Stopping Bitcoin Network service");
        
        // Stop in reverse order
        
        // Stop the blockchain manager
        {
            let mut blockchain = self.blockchain.write().await;
            blockchain.stop().await?;
        }
        
        // Stop the mempool
        {
            let mut mempool = self.mempool.write().await;
            mempool.stop().await?;
        }
        
        // Stop the P2P manager
        {
            let mut p2p_manager = self.p2p_manager.write().await;
            p2p_manager.stop().await?;
        }
        
        info!("Bitcoin Network service stopped");
        Ok(())
    }
    
    /// Broadcast a transaction to the network
    pub async fn broadcast_transaction(&self, tx: Transaction) -> Result<(), BitcoinNetworkError> {
        debug!("Broadcasting transaction: {}", tx.txid());
        
        // Add to mempool
        {
            let mut mempool = self.mempool.write().await;
            mempool.add_transaction(tx.clone()).await?;
        }
        
        // Broadcast via P2P
        {
            let mut p2p_manager = self.p2p_manager.write().await;
            p2p_manager.broadcast_transaction(tx).await?;
        }
        
        Ok(())
    }
    
    /// Get the mempool
    pub async fn get_mempool(&self) -> Arc<RwLock<mempool::Mempool>> {
        self.mempool.clone()
    }
    
    /// Get the blockchain manager
    pub async fn get_blockchain(&self) -> Arc<RwLock<blockchain::Blockchain>> {
        self.blockchain.clone()
    }
    
    /// Get the P2P manager
    pub async fn get_p2p_manager(&self) -> Arc<RwLock<p2p::P2PManager>> {
        self.p2p_manager.clone()
    }
}
