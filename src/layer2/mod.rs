// [AIR-3][AIS-3][AIM-3][BPC-3][RES-3]
//! Layer2 implementation for official Bitcoin Improvement Proposals (BIPs)
//!
//! This module implements Layer2 protocols for Bitcoin, following
//! the hexagonal architecture pattern required by BDF v2.5.

use serde::{Deserialize, Serialize};
use std::error::Error;

/// Layer2 protocol types supported by the implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Layer2ProtocolType {
    Lightning,
    StateChannels,
    RGB,
    DLC,
    BOB,
    Liquid,
    RSK,
    Stacks,
    TaprootAssets,
}

/// Transaction status in a Layer2 protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
    Rejected,
}

/// Protocol state for Layer2 implementations
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

/// Asset parameters for Layer2 protocols
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetParams {
    pub asset_id: String,
    pub name: String,
    pub symbol: String,
    pub precision: u8,
    pub decimals: u8,
    pub total_supply: u64,
    pub metadata: String,
}

/// Asset transfer parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetTransfer {
    pub asset_id: String,
    pub amount: u64,
    pub from: String,
    pub to: String,
    pub recipient: String,
    pub metadata: Option<String>,
}

/// Result of an asset transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResult {
    pub tx_id: String,
    pub status: TransactionStatus,
    pub fee: Option<u64>,
    pub timestamp: u64,
}

/// Proof for Layer2 operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub proof_type: String,
    pub data: Vec<u8>,
    pub block_height: Option<u32>,
    pub witness: Option<Vec<u8>>,
    pub merkle_root: String,
    pub merkle_proof: Vec<String>,
    pub block_header: String,
}

/// Result of a verification operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub valid: bool,
    pub is_valid: bool,
    pub error: Option<String>,
    pub timestamp: u64,
}

/// Result of a validation operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub violations: Vec<String>,
    pub timestamp: u64,
}

/// Layer2 protocol interface (async trait for modern implementation)
#[async_trait::async_trait]
pub trait Layer2Protocol {
    async fn initialize(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn connect(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn get_state(&self) -> Result<ProtocolState, Box<dyn std::error::Error + Send + Sync>>;
    async fn submit_transaction(&self, tx_data: &[u8]) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    async fn check_transaction_status(&self, tx_id: &str) -> Result<TransactionStatus, Box<dyn std::error::Error + Send + Sync>>;
    async fn sync_state(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn issue_asset(&self, params: AssetParams) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    async fn transfer_asset(&self, transfer: AssetTransfer) -> Result<TransferResult, Box<dyn std::error::Error + Send + Sync>>;
    async fn verify_proof(&self, proof: Proof) -> Result<VerificationResult, Box<dyn std::error::Error + Send + Sync>>;
    async fn validate_state(&self, state_data: &[u8]) -> Result<ValidationResult, Box<dyn std::error::Error + Send + Sync>>;
}

/// Legacy sync trait for backwards compatibility
pub trait Layer2ProtocolTrait {
    fn initialize(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    fn get_state(&self) -> Result<ProtocolState, Box<dyn std::error::Error + Send + Sync>>;
    fn submit_transaction(&self, tx_data: &[u8]) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    fn check_transaction_status(&self, tx_id: &str) -> Result<TransactionStatus, Box<dyn std::error::Error + Send + Sync>>;
    fn sync_state(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    fn issue_asset(&self, params: AssetParams) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    fn transfer_asset(&self, transfer: AssetTransfer) -> Result<TransferResult, Box<dyn std::error::Error + Send + Sync>>;
    fn verify_proof(&self, proof: Proof) -> Result<VerificationResult, Box<dyn std::error::Error + Send + Sync>>;
    fn validate_state(&self, state_data: &[u8]) -> Result<ValidationResult, Box<dyn std::error::Error + Send + Sync>>;
}

// Layer2 protocol implementations
pub mod lightning;
pub mod state_channels;
pub mod rgb;
pub mod dlc;
pub mod bob;
pub mod liquid;
pub mod rsk;
pub mod stacks;
pub mod taproot_assets;
pub mod mock;

// Example function using Layer2ProtocolType instead of Layer2Protocol
pub fn use_layer2_protocol(protocol: Layer2ProtocolType) {
    match protocol {
        Layer2ProtocolType::Lightning => println!("Using Lightning protocol"),
        Layer2ProtocolType::StateChannels => println!("Using StateChannels protocol"),
        Layer2ProtocolType::RGB => println!("Using RGB protocol"),
        Layer2ProtocolType::DLC => println!("Using DLC protocol"),
        Layer2ProtocolType::BOB => println!("Using BOB protocol"),
        Layer2ProtocolType::Liquid => println!("Using Liquid protocol"),
        Layer2ProtocolType::RSK => println!("Using RSK protocol"),
        Layer2ProtocolType::Stacks => println!("Using Stacks protocol"),
        Layer2ProtocolType::TaprootAssets => println!("Using TaprootAssets protocol"),
    }
}

#[cfg(test)]
pub mod comprehensive_tests;

// Re-export key components
pub use lightning::LightningNetwork;
pub use bob::BobClient;
pub use liquid::LiquidModule;
pub use rsk::RskClient;
pub use stacks::StacksClient;
pub use state_channels::StateChannel;
pub use taproot_assets::TaprootAssetsProtocol;

// Re-export protocol implementations for tests
pub use lightning::LightningProtocol;
pub use rgb::RgbProtocol;
pub use dlc::DlcProtocol;
pub use state_channels::StateChannelsProtocol;
pub use rsk::RskProtocol;
pub use stacks::StacksProtocol;
pub use liquid::LiquidProtocol;
pub use mock::MockLayer2Protocol;

// RGB Protocol trait implementation
pub struct RGBProtocol {
    pub version: String,
    pub network: String,
}

// DLC Protocol trait implementation
pub struct DiscreteLogContract {
    pub version: String,
    pub network: String,
}

/// Error types for Layer2 protocols
#[derive(Debug)]
pub enum Layer2Error {
    General(String),
    Connection(String),
    Protocol(String),
    Authentication(String),
    Transaction(String),
}

impl std::fmt::Display for Layer2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Layer2Error::General(msg) => write!(f, "Layer2 General Error: {}", msg),
            Layer2Error::Connection(msg) => write!(f, "Layer2 Connection Error: {}", msg),
            Layer2Error::Protocol(msg) => write!(f, "Layer2 Protocol Error: {}", msg),
            Layer2Error::Authentication(msg) => write!(f, "Layer2 Authentication Error: {}", msg),
            Layer2Error::Transaction(msg) => write!(f, "Layer2 Transaction Error: {}", msg),
        }
    }
}

impl Error for Layer2Error {}

/// Helper function to create a default ProtocolState with all required fields
pub fn create_protocol_state(
    version: &str,
    connections: u32,
    capacity: Option<u64>,
    operational: bool,
) -> ProtocolState {
    ProtocolState {
        version: version.to_string(),
        connections,
        capacity,
        operational,
        height: 0,
        hash: "default_hash".to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    }
}

/// Helper function to create a default VerificationResult
pub fn create_verification_result(is_valid: bool, error: Option<String>) -> VerificationResult {
    VerificationResult {
        valid: is_valid,
        is_valid,
        error,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    }
}

/// Helper function to create a default ValidationResult
pub fn create_validation_result(is_valid: bool, violations: Vec<String>) -> ValidationResult {
    ValidationResult {
        is_valid,
        violations,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    }
}
