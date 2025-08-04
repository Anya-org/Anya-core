//! Layer2 protocols module for Bitcoin scaling solutions
//!
//! This module provides a comprehensive Layer2 Bitcoin scaling infrastructure
//! with async support and unified protocol management.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

// Module declarations
pub mod async_coordinator;
pub mod bob;
pub mod dlc;
pub mod lightning;
pub mod liquid;
pub mod manager;
pub mod mock; // Kept for backward compatibility and testing
pub mod production; // New production implementation
pub mod rgb;
pub mod rsk;
pub mod stacks;
pub mod state_channels;
pub mod taproot_assets;

// Re-exports for convenience
pub use async_coordinator::{AsyncLayer2Coordinator, AsyncProtocolWrapper, Layer2Event};
pub use bob::BobProtocol;
pub use dlc::DlcProtocol;
pub use lightning::LightningProtocol;
pub use liquid::LiquidProtocol;
pub use manager::Layer2Manager;
pub use production::{ProductionLayer2Protocol, RealLayer2Protocol}; // Use production implementation
pub use rgb::RgbProtocol;
pub use rsk::RskProtocol;
pub use stacks::StacksProtocol;
pub use state_channels::StateChannelsProtocol;
pub use taproot_assets::TaprootAssetsProtocol;

// Export mock for testing (but not as default)
#[cfg(test)]
pub use mock::MockLayer2Protocol;

/// Error types for Layer2 operations
#[derive(Debug, Clone, Error)]
pub enum Layer2Error {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Layer2 protocol type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Layer2ProtocolType {
    Lightning,
    RGB,
    DLC,
    StateChannels,
    Liquid,
    Stacks,
    BOB,
    RSK,
    TaprootAssets,
}

/// Transaction status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
    Rejected,
}

/// Asset parameters for issuance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetParams {
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    pub metadata: String,
}

/// Asset transfer parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetTransfer {
    pub asset_id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
}

/// Transaction result information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResult {
    pub tx_id: String,
    pub status: TransactionStatus,
    pub amount: Option<u64>,
    pub fee: Option<u64>,
    pub confirmations: u32,
    pub timestamp: u64,
    pub block_height: Option<u64>,
}

/// Transfer result information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResult {
    pub tx_id: String,
    pub status: TransactionStatus,
    pub fee: Option<u64>,
    pub timestamp: u64,
}

/// Proof data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub proof_type: String,
    pub data: Vec<u8>,
    pub block_height: Option<u64>,
    pub witness: Option<Vec<u8>>,
    pub merkle_root: String,
    pub merkle_proof: Vec<String>,
    pub block_header: String,
}

/// Verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub valid: bool,
    pub is_valid: bool,
    pub error: Option<String>,
    pub timestamp: u64,
    pub error_message: Option<String>,
    pub confidence_score: f64,
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub violations: Vec<String>,
    pub timestamp: u64,
}

/// Protocol state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolState {
    pub version: String,
    pub connections: u32,
    pub capacity: Option<u64>,
    pub operational: bool,
    pub height: u64,
    pub hash: String,
    pub timestamp: u64,
}

/// Protocol health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolHealth {
    pub healthy: bool,
    pub last_check: u64,
    pub error_count: u32,
    pub uptime_seconds: u64,
}

/// Protocol capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolCapabilities {
    pub supports_assets: bool,
    pub supports_smart_contracts: bool,
    pub supports_privacy: bool,
    pub max_transaction_size: u32,
    pub fee_estimation: bool,
}

/// Fee estimation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeEstimate {
    pub estimated_fee: u64,
    pub fee_rate: f64,
    pub confirmation_target: u32,
    pub slow_fee: u64,
    pub normal_fee: u64,
    pub fast_fee: u64,
    pub estimated_confirmation_time: u32,
}

/// Generic Layer2 protocol trait
#[async_trait]
pub trait Layer2Protocol: Send + Sync {
    /// Initialize the protocol
    async fn initialize(&self) -> Result<(), Layer2Error>;

    /// Connect to the protocol network
    async fn connect(&self) -> Result<(), Layer2Error>;

    /// Disconnect from the protocol network
    async fn disconnect(&self) -> Result<(), Layer2Error>;

    /// Check protocol health
    async fn health_check(&self) -> Result<ProtocolHealth, Layer2Error>;

    /// Get current protocol state
    async fn get_state(&self) -> Result<ProtocolState, Layer2Error>;

    /// Sync protocol state
    async fn sync_state(&mut self) -> Result<(), Layer2Error>;

    /// Validate protocol state
    async fn validate_state(&self, state: &ProtocolState) -> Result<ValidationResult, Layer2Error>;

    /// Submit a transaction
    async fn submit_transaction(&self, tx_data: &[u8]) -> Result<String, Layer2Error>;

    /// Check transaction status
    async fn check_transaction_status(&self, tx_id: &str)
        -> Result<TransactionStatus, Layer2Error>;

    /// Get transaction history
    async fn get_transaction_history(
        &self,
        limit: Option<u32>,
    ) -> Result<Vec<TransactionResult>, Layer2Error>;

    /// Issue an asset
    async fn issue_asset(&self, params: AssetParams) -> Result<String, Layer2Error>;

    /// Transfer an asset
    async fn transfer_asset(&self, transfer: AssetTransfer) -> Result<TransferResult, Layer2Error>;

    /// Verify a proof
    async fn verify_proof(&self, proof: Proof) -> Result<VerificationResult, Layer2Error>;

    /// Generate a proof for a transaction
    async fn generate_proof(&self, transaction_id: &str) -> Result<Proof, Layer2Error>;

    /// Get protocol capabilities
    async fn get_capabilities(&self) -> Result<ProtocolCapabilities, Layer2Error>;

    /// Estimate fees for an operation
    async fn estimate_fees(
        &self,
        operation: &str,
        params: &[u8],
    ) -> Result<FeeEstimate, Layer2Error>;
}

/// Utility functions for creating common structures
pub fn create_protocol_state(
    version: &str,
    connections: u32,
    capacity: Option<u64>,
    operational: bool,
) -> ProtocolState {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    ProtocolState {
        version: version.to_string(),
        connections,
        capacity,
        operational,
        height: 800000, // Mock block height
        hash: "0".repeat(64),
        timestamp,
    }
}

pub fn create_validation_result(is_valid: bool, violations: Vec<String>) -> ValidationResult {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    ValidationResult {
        is_valid,
        violations,
        timestamp,
    }
}

pub fn create_verification_result(is_valid: bool, error: Option<String>) -> VerificationResult {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    VerificationResult {
        valid: is_valid,
        is_valid,
        error: error.clone(),
        timestamp,
        error_message: error,
        confidence_score: if is_valid { 1.0 } else { 0.0 },
    }
}

/// Lightning Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningConfig {
    /// Network type: mainnet, testnet, regtest
    pub network: String,
    /// Node URL
    pub node_url: String,
    /// Macaroon for authentication (hex encoded)
    pub macaroon: String,
    /// TLS certificate (base64 encoded)
    pub cert: String,
    /// Node alias
    pub alias: String,
    /// Auto-pilot enabled
    pub autopilot: bool,
    /// Channel capacity limits
    pub min_channel_size: u64,
    pub max_channel_size: u64,
}

impl Default for LightningConfig {
    fn default() -> Self {
        Self {
            network: "regtest".to_string(),
            node_url: "127.0.0.1:10009".to_string(),
            macaroon: "0201036c6e64022f030a10b493a60e861b6c8a0e0a854355b4320612071f9e0f708e354d9234d6171d7cd0111d1313c7cd088f8ac2cd900101201301".to_string(),
            cert: String::new(),
            alias: "anya-core-ln-node".to_string(),
            autopilot: false,
            min_channel_size: 100_000, // 100k sats
            max_channel_size: 100_000_000, // 100M sats (1 BTC)
        }
    }
}

/// Lightning Network channel information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub channel_id: String,
    pub capacity: u64,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub active: bool,
    pub peer_pubkey: String,
    pub initiator: bool,
    pub private: bool,
}

/// Lightning Network invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningInvoice {
    pub payment_request: String,
    pub r_hash: String,
    pub r_preimage: Option<String>,
    pub value: u64,
    pub settled: bool,
    pub creation_date: u64,
    pub expiry: u64,
    pub description: String,
}

/// Lightning Network payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningPayment {
    pub payment_hash: String,
    pub payment_preimage: Option<String>,
    pub value: u64,
    pub creation_date: u64,
    pub fee: u64,
    pub payment_request: String,
    pub status: PaymentStatus,
    pub failure_reason: Option<String>,
}

/// Payment status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentStatus {
    InFlight,
    Succeeded,
    Failed,
    Unknown,
}

/// Lightning Network node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub identity_pubkey: String,
    pub alias: String,
    pub color: String,
    pub num_pending_channels: u32,
    pub num_active_channels: u32,
    pub num_inactive_channels: u32,
    pub num_peers: u32,
    pub block_height: u32,
    pub block_hash: String,
    pub best_header_timestamp: u64,
    pub synced_to_chain: bool,
    pub synced_to_graph: bool,
    pub testnet: bool,
    pub chains: Vec<String>,
    pub version: String,
}

// Lightning Network implementation is in the lightning module

// Lightning Network implementation methods are in the lightning module
