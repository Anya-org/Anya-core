pub mod p2p;
pub mod messages;
pub mod peers;

// Re-export commonly used items
pub use p2p::P2PNetwork;
pub use messages::MessageHandler;
pub use peers::PeerManager;

// Bitcoin network implementation
//
use async_trait::async_trait;
use bitcoin::{Block, Transaction};
use crate::core::error::AnyaResult;

/// Peer information
#[derive(Debug, Clone)]
pub struct PeerInfo {
    /// Peer ID
    pub id: String,
    /// Peer address
    pub address: String,
    /// User agent
    pub user_agent: String,
    /// Protocol version
    pub version: u32,
    /// Services
    pub services: u64,
    /// Connected since (UNIX timestamp)
    pub connected_since: u64,
    /// Last seen (UNIX timestamp)
    pub last_seen: u64,
    /// Number of bytes sent
    pub bytes_sent: u64,
    /// Number of bytes received
    pub bytes_recv: u64,
}

impl PeerInfo {
    /// Returns true if the peer is an outbound connection
    pub fn is_outbound(&self) -> bool {
        // Assuming outbound peers have a specific service flag or user agent marker
        // Adjust logic as needed for your implementation
        self.user_agent.contains("outbound") || (self.services & 1) != 0
    }
}

/// Network statistics
#[derive(Debug, Clone)]
pub struct NetworkStats {
    /// Number of connected peers
    pub peer_count: usize,
    /// Number of inbound connections
    pub inbound_count: usize,
    /// Number of outbound connections
    pub outbound_count: usize,
    /// Total bytes sent
    pub total_bytes_sent: u64,
    /// Total bytes received
    pub total_bytes_recv: u64,
    /// Network uptime in seconds
    pub uptime: u64,
}

/// P2P network interface
#[async_trait]
pub trait P2P: Send + Sync {
    /// Start the P2P network
    async fn start(&self) -> AnyaResult<()>;
    
    /// Stop the P2P network
    async fn stop(&self) -> AnyaResult<()>;
    
    /// Broadcast a transaction to the network
    async fn broadcast_transaction(&self, tx: &Transaction) -> AnyaResult<()>;
    
    /// Broadcast a block to the network
    async fn broadcast_block(&self, block: &Block) -> AnyaResult<()>;
    
    /// Get information about connected peers
    async fn get_peers(&self) -> AnyaResult<Vec<PeerInfo>>;
    
    /// Get network statistics
    async fn get_network_stats(&self) -> AnyaResult<NetworkStats>;
    
    /// Add a peer to the network
    async fn add_peer(&self, address: &str) -> AnyaResult<bool>;
    
    /// Remove a peer from the network
    async fn remove_peer(&self, id: &str) -> AnyaResult<bool>;
}

/// No-op P2P implementation for testing
pub struct NoopP2P;

#[async_trait]
impl P2P for NoopP2P {
    async fn start(&self) -> AnyaResult<()> {
        Ok(())
    }
    
    async fn stop(&self) -> AnyaResult<()> {
        Ok(())
    }
    
    async fn broadcast_transaction(&self, _tx: &Transaction) -> AnyaResult<()> {
        Ok(())
    }
    
    async fn broadcast_block(&self, _block: &Block) -> AnyaResult<()> {
        Ok(())
    }
    
    async fn get_peers(&self) -> AnyaResult<Vec<PeerInfo>> {
        Ok(Vec::new())
    }
    
    async fn get_network_stats(&self) -> AnyaResult<NetworkStats> {
        Ok(NetworkStats {
            peer_count: 0,
            inbound_count: 0,
            outbound_count: 0,
            total_bytes_sent: 0,
            total_bytes_recv: 0,
            uptime: 0,
        })
    }
    
    async fn add_peer(&self, _address: &str) -> AnyaResult<bool> {
        Ok(true)
    }
    
    async fn remove_peer(&self, _id: &str) -> AnyaResult<bool> {
        Ok(true)
    }
}