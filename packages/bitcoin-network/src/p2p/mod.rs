// Bitcoin P2P Network Manager
//
// Provides peer-to-peer network connectivity for the Bitcoin network

use bitcoin::{
    Network,
    Transaction,
    Block,
    BlockHeader,
    BlockHash,
    p2p::{self, ServiceFlags, message::NetworkMessage},
};
use std::sync::Arc;
use std::time::Duration;
use std::collections::HashMap;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use tokio::sync::{RwLock, mpsc};
use log::{info, warn, error, debug};
use super::{BitcoinNetworkConfig, BitcoinNetworkError};

pub mod connection;
pub mod peer;
pub mod messages;

/// Maximum number of outbound connections
const MAX_OUTBOUND_CONNECTIONS: usize = 8;

/// Maximum number of inbound connections
const MAX_INBOUND_CONNECTIONS: usize = 117;

/// P2P Manager
pub struct P2PManager {
    /// Network configuration
    config: BitcoinNetworkConfig,
    /// Connected peers
    peers: HashMap<SocketAddr, Arc<RwLock<peer::Peer>>>,
    /// Connection manager
    connection_manager: Arc<RwLock<connection::ConnectionManager>>,
    /// Message queue sender
    message_tx: mpsc::Sender<(SocketAddr, NetworkMessage)>,
    /// Message queue receiver
    message_rx: mpsc::Receiver<(SocketAddr, NetworkMessage)>,
    /// Running flag
    running: bool,
}

impl P2PManager {
    /// Create a new P2P manager
    pub async fn new(config: BitcoinNetworkConfig) -> Result<Self, BitcoinNetworkError> {
        let (message_tx, message_rx) = mpsc::channel(1000);
        
        let connection_manager = connection::ConnectionManager::new(
            config.clone(),
            message_tx.clone(),
        ).await?;
        
        Ok(Self {
            config,
            peers: HashMap::new(),
            connection_manager: Arc::new(RwLock::new(connection_manager)),
            message_tx,
            message_rx,
            running: false,
        })
    }
    
    /// Start the P2P manager
    pub async fn start(&mut self) -> Result<(), BitcoinNetworkError> {
        if self.running {
            return Ok(());
        }
        
        info!("Starting Bitcoin P2P manager");
        
        // Start the connection manager
        {
            let mut connection_manager = self.connection_manager.write().await;
            connection_manager.start().await?;
        }
        
        // Start the message processor
        let message_rx = self.message_rx.clone();
        let message_tx = self.message_tx.clone();
        let connection_manager = self.connection_manager.clone();
        let peers_ref = Arc::new(RwLock::new(self.peers.clone()));
        
        tokio::spawn(async move {
            Self::message_processor(message_rx, message_tx, connection_manager, peers_ref).await;
        });
        
        self.running = true;
        
        info!("Bitcoin P2P manager started");
        Ok(())
    }
    
    /// Stop the P2P manager
    pub async fn stop(&mut self) -> Result<(), BitcoinNetworkError> {
        if !self.running {
            return Ok(());
        }
        
        info!("Stopping Bitcoin P2P manager");
        
        // Stop the connection manager
        {
            let mut connection_manager = self.connection_manager.write().await;
            connection_manager.stop().await?;
        }
        
        // Disconnect all peers
        for (addr, peer) in &self.peers {
            let mut peer_lock = peer.write().await;
            if let Err(e) = peer_lock.disconnect().await {
                warn!("Error disconnecting peer {}: {}", addr, e);
            }
        }
        
        self.peers.clear();
        self.running = false;
        
        info!("Bitcoin P2P manager stopped");
        Ok(())
    }
    
    /// Broadcast a transaction to all connected peers
    pub async fn broadcast_transaction(&mut self, tx: Transaction) -> Result<(), BitcoinNetworkError> {
        debug!("Broadcasting transaction: {}", tx.txid());
        
        let message = NetworkMessage::Tx(tx);
        
        // Broadcast to all connected peers
        for (addr, _) in &self.peers {
            if let Err(e) = self.message_tx.send((*addr, message.clone())).await {
                warn!("Error sending transaction to peer {}: {}", addr, e);
            }
        }
        
        Ok(())
    }
    
    /// Message processor loop
    async fn message_processor(
        mut message_rx: mpsc::Receiver<(SocketAddr, NetworkMessage)>,
        message_tx: mpsc::Sender<(SocketAddr, NetworkMessage)>,
        connection_manager: Arc<RwLock<connection::ConnectionManager>>,
        peers: Arc<RwLock<HashMap<SocketAddr, Arc<RwLock<peer::Peer>>>>>,
    ) {
        info!("Starting Bitcoin P2P message processor");
        
        while let Some((addr, message)) = message_rx.recv().await {
            debug!("Received message from {}: {:?}", addr, message);
            
            // Process the message
            match message {
                NetworkMessage::Version(_) => {
                    // Handle version message
                    // In a real implementation, we would validate the version
                    // For now, respond with verack
                    if let Err(e) = message_tx.send((addr, NetworkMessage::Verack)).await {
                        warn!("Error sending verack to {}: {}", addr, e);
                    }
                },
                NetworkMessage::Ping(nonce) => {
                    // Respond with pong
                    if let Err(e) = message_tx.send((addr, NetworkMessage::Pong(nonce))).await {
                        warn!("Error sending pong to {}: {}", addr, e);
                    }
                },
                NetworkMessage::Inv(inventory) => {
                    // Process inventory
                    debug!("Received inventory of {} items from {}", inventory.len(), addr);
                },
                NetworkMessage::Tx(tx) => {
                    // Process transaction
                    debug!("Received transaction from {}: {}", addr, tx.txid());
                },
                NetworkMessage::Block(block) => {
                    // Process block
                    debug!("Received block from {}: {}", addr, block.block_hash());
                },
                _ => {
                    // Handle other messages
                    debug!("Ignoring message type: {:?}", message);
                }
            }
        }
        
        info!("Bitcoin P2P message processor stopped");
    }
    
    /// Connect to a peer
    pub async fn connect_to_peer(&mut self, addr: SocketAddr) -> Result<(), BitcoinNetworkError> {
        if self.peers.contains_key(&addr) {
            debug!("Already connected to peer {}", addr);
            return Ok(());
        }
        
        info!("Connecting to peer {}", addr);
        
        // Use the connection manager to establish the connection
        let connection_manager = self.connection_manager.read().await;
        connection_manager.connect(addr).await?;
        
        Ok(())
    }
    
    /// Add a peer
    pub async fn add_peer(&mut self, addr: SocketAddr, peer: peer::Peer) {
        info!("Adding peer {}", addr);
        self.peers.insert(addr, Arc::new(RwLock::new(peer)));
    }
    
    /// Remove a peer
    pub async fn remove_peer(&mut self, addr: SocketAddr) {
        if let Some(peer) = self.peers.remove(&addr) {
            info!("Removing peer {}", addr);
            
            // Disconnect the peer
            let mut peer_lock = peer.write().await;
            if let Err(e) = peer_lock.disconnect().await {
                warn!("Error disconnecting peer {}: {}", addr, e);
            }
        }
    }
    
    /// Get the number of connected peers
    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }
}
