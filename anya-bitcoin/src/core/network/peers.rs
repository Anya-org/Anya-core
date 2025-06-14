//! Bitcoin peer management

use log::{debug, error, info, warn};
use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, SocketAddr};
///
/// This module provides peer management for the Bitcoin network,
/// focusing on Taproot-compatible peer connections and maintaining
/// network decentralization and privacy following Bitcoin Core principles.
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;

use super::PeerInfo;
use crate::core::error::AnyaError;
use crate::core::error::AnyaResult;

/// Default maximum number of connected peers
pub const DEFAULT_MAX_PEERS: usize = 125;

/// Default maximum outbound connections
pub const DEFAULT_MAX_OUTBOUND: usize = 8;

/// Default connection timeout in seconds
pub const DEFAULT_CONNECT_TIMEOUT_SECONDS: u64 = 30;

/// Minimum time between reconnection attempts (seconds)
pub const MIN_RECONNECT_DELAY: u64 = 60;

/// Error variants specific to peer management
#[derive(Debug, Error)]
pub enum PeerError {
    #[error("Connection to peer {0} failed: {1}")]
    ConnectionFailed(String, String),

    #[error("Peer {0} disconnected: {1}")]
    Disconnected(String, String),

    #[error("Handshake with peer {0} failed: {1}")]
    HandshakeFailed(String, String),

    #[error("Max peer limit reached")]
    MaxPeersReached,

    #[error("Peer {0} was banned")]
    PeerBanned(String),

    #[error("Peer {0} not found")]
    PeerNotFound(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("General error: {0}")]
    General(String),
}

impl From<PeerError> for AnyaError {
    fn from(err: PeerError) -> Self {
        AnyaError::Peer(err.to_string())
    }
}

/// Peer connection type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    /// Outbound connection initiated by us
    Outbound,
    /// Inbound connection initiated by the peer
    Inbound,
    /// Manual connection initiated by the user
    Manual,
    /// Feeler connection for network probing
    Feeler,
}

bitflags::bitflags! {
    /// Peer service flags
    /// Services offered by a peer
    #[derive(Copy, Clone)]
    pub struct ServiceFlags: u64 {
        /// NODE_NETWORK = (1 << 0)
        const NETWORK = 1;
        /// NODE_GETUTXO = (1 << 1)
        const GETUTXO = 2;
        /// NODE_BLOOM = (1 << 2)
        const BLOOM = 4;
        /// NODE_WITNESS = (1 << 3)
        const WITNESS = 8;
        /// NODE_XTHIN = (1 << 4)
        const XTHIN = 16;
        /// NODE_COMPACT_FILTERS = (1 << 6)
        const COMPACT_FILTERS = 64;
        /// NODE_NETWORK_LIMITED = (1 << 10)
        const NETWORK_LIMITED = 1024;
        /// NODE_TAPROOT = (1 << 24) - Custom for this implementation
        const TAPROOT = 16777216;
    }
}

/// Connection state of a peer
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeerState {
    /// Initial state before connection
    Initial,
    /// Connecting in progress
    Connecting,
    /// Connected but handshake not complete
    Connected,
    /// Handshake completed, peer ready
    Ready,
    /// Disconnecting in progress
    Disconnecting,
    /// Disconnected
    Disconnected,
    /// Connection failed
    Failed,
    /// Peer banned
    Banned,
}

/// Peer manager for Bitcoin network
pub struct PeerManager {
    /// Connected peers by ID
    peers: Arc<RwLock<HashMap<String, PeerInfo>>>,
    /// Banned peers by address
    banned: Arc<RwLock<HashSet<IpAddr>>>,
    /// Tried addresses and connection outcomes
    tried_addresses: Arc<RwLock<HashMap<SocketAddr, (SystemTime, bool)>>>,
    /// Maximum number of peers
    max_peers: usize,
    /// Maximum outbound connections
    max_outbound: usize,
    /// Connection timeout
    connect_timeout: Duration,
    /// Whether we're running on testnet
    is_testnet: bool,
    /// Our local services
    services: ServiceFlags,
    /// Taproot enabled (BIP341)
    taproot_enabled: bool,
}

impl PeerManager {
    /// Create a new peer manager with default settings
    pub fn new() -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            banned: Arc::new(RwLock::new(HashSet::new())),
            tried_addresses: Arc::new(RwLock::new(HashMap::new())),
            max_peers: DEFAULT_MAX_PEERS,
            max_outbound: DEFAULT_MAX_OUTBOUND,
            connect_timeout: Duration::from_secs(DEFAULT_CONNECT_TIMEOUT_SECONDS),
            is_testnet: false,
            services: ServiceFlags::NETWORK | ServiceFlags::WITNESS | ServiceFlags::TAPROOT,
            taproot_enabled: true,
        }
    }

    /// Create a peer manager with custom configuration
    pub fn with_config(
        max_peers: usize,
        max_outbound: usize,
        connect_timeout: Duration,
        is_testnet: bool,
        taproot_enabled: bool,
    ) -> Self {
        let mut services = ServiceFlags::NETWORK | ServiceFlags::WITNESS;
        if taproot_enabled {
            services |= ServiceFlags::TAPROOT;
        }

        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            banned: Arc::new(RwLock::new(HashSet::new())),
            tried_addresses: Arc::new(RwLock::new(HashMap::new())),
            max_peers,
            max_outbound,
            connect_timeout,
            is_testnet,
            services,
            taproot_enabled,
        }
    }

    /// Connect to a peer by address
    pub async fn connect(
        &self,
        addr: SocketAddr,
        connection_type: ConnectionType,
    ) -> AnyaResult<String> {
        // Check if we've reached the peer limit
        {
            let peers = self.peers.read().unwrap();

            if peers.len() >= self.max_peers && connection_type != ConnectionType::Manual {
                return Err(PeerError::MaxPeersReached.into());
            }

            // Count outbound connections
            if connection_type == ConnectionType::Outbound {
                let outbound_count = peers.values().filter(|info| info.is_outbound()).count();

                if outbound_count >= self.max_outbound {
                    return Err(
                        PeerError::General("Max outbound connections reached".to_string()).into(),
                    );
                }
            }
        }

        // Check if the address is banned
        if self.is_banned(&addr.ip()) {
            return Err(PeerError::PeerBanned(addr.to_string()).into());
        }

        // In a real implementation, this would establish a connection to the peer
        // For now, we'll just simulate it

        // Create a peer ID based on the address
        let peer_id = format!("peer-{}", addr);

        // Update the tried addresses
        {
            let mut tried = self.tried_addresses.write().unwrap();
            tried.insert(addr, (SystemTime::now(), true));
        }

        // Create peer info
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let peer_info = PeerInfo {
            id: peer_id.clone(),
            address: addr.to_string(),
            user_agent: "".to_string(), // Will be filled during handshake
            version: 0,                 // Will be filled during handshake
            services: 0,                // Will be filled during handshake
            connected_since: now,
            last_seen: now,
            bytes_sent: 0,
            bytes_recv: 0,
        };

        // Add to connected peers
        {
            let mut peers = self.peers.write().unwrap();
            peers.insert(peer_id.clone(), peer_info);
        }

        info!("Connected to peer {} ({})", peer_id, addr);

        Ok(peer_id)
    }

    /// Disconnect from a peer
    pub async fn disconnect(&self, peer_id: &str) -> AnyaResult<bool> {
        let peer_found = {
            let mut peers = self.peers.write().unwrap();
            peers.remove(peer_id).is_some()
        };

        if peer_found {
            info!("Disconnected from peer {}", peer_id);
        } else {
            debug!("Attempted to disconnect from unknown peer {}", peer_id);
        }

        Ok(peer_found)
    }

    /// Ban a peer by IP address
    pub async fn ban_peer(&self, ip: IpAddr, _duration: Option<Duration>) -> AnyaResult<()> {
        {
            let mut banned = self.banned.write().unwrap();
            banned.insert(ip);
        }

        // Disconnect any peers with matching IP
        let peers_to_disconnect = {
            let peers = self.peers.read().unwrap();
            peers
                .iter()
                .filter(|(_, info)| {
                    // Extract IP from address (e.g., "1.2.3.4:8333" -> "1.2.3.4")
                    if let Some(addr_ip) = info.address.split(':').next() {
                        // Try to parse as IP address
                        if let Ok(peer_ip) = addr_ip.parse::<IpAddr>() {
                            return peer_ip == ip;
                        }
                    }
                    false
                })
                .map(|(id, _)| id.clone())
                .collect::<Vec<_>>()
        };

        for id in peers_to_disconnect {
            // Don't use disconnect directly to avoid double locking peers
            let mut peers = self.peers.write().unwrap();
            peers.remove(&id);
            info!("Disconnected banned peer {}", id);
        }

        info!("Banned peer with IP {}", ip);

        Ok(())
    }

    /// Unban a peer by IP address
    pub async fn unban_peer(&self, ip: IpAddr) -> AnyaResult<bool> {
        let removed = {
            let mut banned = self.banned.write().unwrap();
            banned.remove(&ip)
        };

        if removed {
            info!("Unbanned peer with IP {}", ip);
        }

        Ok(removed)
    }

    /// Check if an IP is banned
    pub fn is_banned(&self, ip: &IpAddr) -> bool {
        let banned = self.banned.read().unwrap();
        banned.contains(ip)
    }

    /// Get information about a specific peer
    pub async fn get_peer(&self, peer_id: &str) -> AnyaResult<Option<PeerInfo>> {
        let peers = self.peers.read().unwrap();
        Ok(peers.get(peer_id).cloned())
    }

    /// Get information about all connected peers
    pub async fn get_all_peers(&self) -> AnyaResult<Vec<PeerInfo>> {
        let peers = self.peers.read().unwrap();
        Ok(peers.values().cloned().collect())
    }

    /// Get the number of connected peers
    pub async fn get_peer_count(&self) -> AnyaResult<usize> {
        let peers = self.peers.read().unwrap();
        Ok(peers.len())
    }

    /// Broadcast a message to all connected peers
    pub async fn broadcast(&self, _data: &[u8]) -> AnyaResult<usize> {
        let peer_count = {
            let peers = self.peers.read().unwrap();
            peers.len()
        };

        // In a real implementation, this would actually send the message to peers
        info!("Broadcasting message to {} peers", peer_count);

        Ok(peer_count)
    }

    /// Update peer information
    pub async fn update_peer_info(&self, peer_id: &str, info: PeerInfo) -> AnyaResult<bool> {
        let mut peers = self.peers.write().unwrap();

        if let Some(existing) = peers.get_mut(peer_id) {
            *existing = info;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Register a successful handshake with a peer
    pub async fn register_handshake(
        &self,
        peer_id: &str,
        version: u32,
        user_agent: &str,
        services: u64,
    ) -> AnyaResult<bool> {
        let mut peers = self.peers.write().unwrap();

        if let Some(info) = peers.get_mut(peer_id) {
            info.version = version;
            info.user_agent = user_agent.to_string();
            info.services = services;

            info!(
                "Handshake completed with peer {} (version: {}, user agent: '{}')",
                peer_id, version, user_agent
            );

            Ok(true)
        } else {
            warn!(
                "Attempted to register handshake for unknown peer {}",
                peer_id
            );
            Ok(false)
        }
    }

    /// Register activity from a peer (updates last_seen timestamp)
    pub fn register_activity(&self, peer_id: &str) -> AnyaResult<bool> {
        let mut peers = self.peers.write().unwrap();

        if let Some(info) = peers.get_mut(peer_id) {
            info.last_seen = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Register sent bytes to a peer
    pub fn register_sent(&self, peer_id: &str, bytes: u64) -> AnyaResult<bool> {
        let mut peers = self.peers.write().unwrap();

        if let Some(info) = peers.get_mut(peer_id) {
            info.bytes_sent += bytes;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Register received bytes from a peer
    pub fn register_received(&self, peer_id: &str, bytes: u64) -> AnyaResult<bool> {
        let mut peers = self.peers.write().unwrap();

        if let Some(info) = peers.get_mut(peer_id) {
            info.bytes_recv += bytes;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Check if a peer supports Taproot
    pub fn supports_taproot(&self, peer_id: &str) -> AnyaResult<bool> {
        let peers = self.peers.read().unwrap();

        if let Some(info) = peers.get(peer_id) {
            // Check if the peer advertises Taproot support in its services
            Ok((info.services & ServiceFlags::TAPROOT.bits()) == ServiceFlags::TAPROOT.bits())
        } else {
            Err(PeerError::PeerNotFound(peer_id.to_string()).into())
        }
    }

    /// Get the default port for Bitcoin based on network
    pub fn get_default_port(&self) -> u16 {
        if self.is_testnet {
            18333 // Testnet port
        } else {
            8333 // Mainnet port
        }
    }

    /// Get our local service flags
    pub fn get_services(&self) -> ServiceFlags {
        self.services
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_peer_connection() {
        // This would test connecting to a peer
    }

    #[tokio::test]
    async fn test_peer_ban() {
        // This would test banning and unbanning peers
    }
}
