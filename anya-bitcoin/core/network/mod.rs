pub mod p2p;
pub mod messages;
pub mod peers;

// Re-export commonly used items
pub use p2p::P2PNetwork;
pub use messages::MessageHandler;
pub use peers::PeerManager; 