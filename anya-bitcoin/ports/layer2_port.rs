//! Layer2 port for Bitcoin components
//! 
//! This port defines the Layer2 interface for Bitcoin components in the hexagonal architecture.
//! It provides methods for interacting with Layer 2 protocols like Lightning, RGB, and DLCs.

use bitcoin::{Transaction, TxOut, BlockHash, Script, ScriptBuf, Address, Amount};
use std::collections::HashMap;

/// Error types for Layer2 operations
#[derive(Debug, thiserror::Error)]
pub enum Layer2Error {
    #[error("Lightning error: {0}")]
    Lightning(String),
    
    #[error("RGB error: {0}")]
    RGB(String),
    
    #[error("DLC error: {0}")]
    DLC(String),
    
    #[error("RSK error: {0}")]
    RSK(String),
    
    #[error("Taproot Assets error: {0}")]
    TaprootAssets(String),
    
    #[error("Layer2 network error: {0}")]
    Network(String),
    
    #[error("Layer2 validation error: {0}")]
    Validation(String),
}

/// Layer2 result type
pub type Layer2Result<T> = Result<T, Layer2Error>;

/// Lightning Network specific payment data
#[derive(Debug, Clone)]
pub struct LightningPayment {
    /// Payment hash
    pub payment_hash: [u8; 32],
    
    /// Payment preimage (if known)
    pub payment_preimage: Option<[u8; 32]>,
    
    /// Payment amount in millisatoshis
    pub amount_msat: u64,
    
    /// Payment expiry time in seconds
    pub expiry: u32,
    
    /// Payment description
    pub description: Option<String>,
    
    /// Route hints
    pub route_hints: Vec<Vec<u8>>,
}

/// RGB asset data
#[derive(Debug, Clone)]
pub struct RGBAsset {
    /// Asset ID
    pub asset_id: String,
    
    /// Asset name
    pub name: String,
    
    /// Asset precision
    pub precision: u8,
    
    /// Total supply
    pub total_supply: u64,
    
    /// Asset metadata
    pub metadata: HashMap<String, String>,
}

/// DLC contract data
#[derive(Debug, Clone)]
pub struct DLCContract {
    /// Contract ID
    pub contract_id: [u8; 32],
    
    /// Oracle public keys
    pub oracle_pubkeys: Vec<[u8; 33]>,
    
    /// Outcomes
    pub outcomes: Vec<String>,
    
    /// Collateral amounts
    pub collateral: Vec<Amount>,
    
    /// Payout schedule
    pub payouts: HashMap<String, Vec<Amount>>,
}

/// Layer2 port interface
pub trait Layer2Port {
    /// Create a Lightning invoice
    fn create_lightning_invoice(&self, 
                               amount_msat: u64, 
                               description: &str, 
                               expiry_seconds: u32) -> Layer2Result<String>;
    
    /// Pay a Lightning invoice
    fn pay_lightning_invoice(&self, invoice: &str) -> Layer2Result<LightningPayment>;
    
    /// Create an RGB asset
    fn create_rgb_asset(&self,
                       name: &str,
                       supply: u64,
                       precision: u8,
                       metadata: HashMap<String, String>) -> Layer2Result<RGBAsset>;
    
    /// Transfer RGB assets
    fn transfer_rgb_asset(&self,
                         asset_id: &str,
                         amount: u64,
                         recipient: &Address) -> Layer2Result<Transaction>;
    
    /// Create a DLC contract
    fn create_dlc_contract(&self,
                          outcomes: Vec<String>,
                          oracle_pubkeys: Vec<[u8; 33]>,
                          collateral: Vec<Amount>,
                          payouts: HashMap<String, Vec<Amount>>) -> Layer2Result<DLCContract>;
    
    /// Execute a DLC contract with outcome
    fn execute_dlc_contract(&self,
                           contract_id: [u8; 32],
                           outcome: &str,
                           oracle_signatures: Vec<Vec<u8>>) -> Layer2Result<Transaction>;
}

/// Lightning Network specific port
pub trait LightningPort: Layer2Port {
    /// Open a Lightning channel
    fn open_channel(&self, node_pubkey: [u8; 33], capacity: Amount) -> Layer2Result<Transaction>;
    
    /// Close a Lightning channel
    fn close_channel(&self, channel_id: [u8; 32], force: bool) -> Layer2Result<Transaction>;
    
    /// Get Lightning channel information
    fn get_channel_info(&self, channel_id: [u8; 32]) -> Layer2Result<HashMap<String, String>>;
    
    /// Route a Lightning payment
    fn route_payment(&self, 
                    payment_hash: [u8; 32], 
                    destination: [u8; 33],
                    amount_msat: u64) -> Layer2Result<LightningPayment>;
}

/// RGB specific port
pub trait RGBPort: Layer2Port {
    /// Issue an RGB asset
    fn issue_asset(&self, metadata: HashMap<String, String>, supply: u64) -> Layer2Result<RGBAsset>;
    
    /// List RGB assets
    fn list_assets(&self) -> Layer2Result<Vec<RGBAsset>>;
    
    /// Get RGB asset balance
    fn get_asset_balance(&self, asset_id: &str) -> Layer2Result<u64>;
    
    /// Create an RGB schema
    fn create_schema(&self, schema_json: &str) -> Layer2Result<String>;
}

/// DLC specific port
pub trait DLCPort: Layer2Port {
    /// Create an oracle announcement
    fn create_oracle_announcement(&self, 
                                event_id: &str, 
                                event_description: &str,
                                outcomes: Vec<String>) -> Layer2Result<String>;
    
    /// Sign as oracle
    fn sign_as_oracle(&self, 
                     event_id: &str, 
                     outcome: &str,
                     private_key: &[u8]) -> Layer2Result<Vec<u8>>;
    
    /// Offer a DLC contract
    fn offer_dlc(&self, 
                contract: &DLCContract, 
                counterparty_pubkey: [u8; 33]) -> Layer2Result<String>;
    
    /// Accept a DLC offer
    fn accept_dlc(&self, offer_id: &str) -> Layer2Result<DLCContract>;
}