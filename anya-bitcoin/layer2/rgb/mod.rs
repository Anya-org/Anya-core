use std::error::Error;
use serde::{Serialize, Deserialize};
// RGB Layer 2 implementation

//! RGB Layer 2 implementation
//!
//! This module provides an implementation of RGB protocol, a client-side
//! validation solution for Bitcoin assets. It allows for the creation
//! and transfer of complex assets on top of Bitcoin's blockchain.

mod schema;
mod contract;
mod client;
mod node;
mod wallet;
mod state;

pub use schema::{Schema, SchemaType, Field, FieldType, Validation};
pub use contract::{Contract, ContractBuilder, ContractType, Witness};
pub use client::{RGBClient, RGBClientBuilder, ClientConfig};
pub use node::{RGBNode, NodeConfig};
pub use wallet::{RGBWallet, AssetBalance};
pub use state::{StateTransfer, StateValidator, StateTransition};

use std::collections::HashMap;
use std::path::PathBuf;
use bitcoin::Txid;

// Updated imports to match the new crate structure
use crate::core::error::AnyaResult;
use crate::core::wallet::TxOptions;

/// RGB asset data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RGBAsset {
    /// Unique identifier for the asset
    pub id: String,
    
    /// Asset name
    pub name: String,
    
    /// Asset description
    pub description: Option<String>,
    
    /// Total supply
    pub total_supply: u64,
    
    /// Decimal precision
    pub precision: u8,
    
    /// Asset metadata
    pub metadata: HashMap<String, String>,
    
    /// Contract ID that issued this asset
    pub contract_id: String,
    
    /// Schema used by this asset
    pub schema_id: String,
}

/// RGB asset transfer request
#[derive(Debug, Clone)]
pub struct AssetTransfer {
    /// Asset ID to transfer
    pub asset_id: String,
    
    /// Amount to transfer
    pub amount: u64,
    
    /// Recipient commitment (UTXO or invoice)
    pub recipient: String,
    
    /// Transfer metadata
    pub metadata: HashMap<String, String>,
    
    /// Transaction options
    pub tx_options: TxOptions,
}

/// Main interface for RGB operations
pub trait RGBManager {
    /// Initializes the RGB environment
    fn init(&self, config: RGBConfig) -> AnyaResult<()>;
    
    /// Creates a new asset
    fn create_asset(&self, params: AssetCreationParams) -> AnyaResult<RGBAsset>;
    
    /// Lists all available assets
    fn list_assets(&self) -> AnyaResult<Vec<RGBAsset>>;
    
    /// Gets the balance for a specific asset
    fn get_asset_balance(&self, asset_id: &str) -> AnyaResult<u64>;
    
    /// Creates an invoice for receiving an asset
    fn create_invoice(&self, asset_id: &str, amount: u64) -> AnyaResult<String>;
    
    /// Transfers an asset
    fn transfer_asset(&self, transfer: AssetTransfer) -> AnyaResult<String>;
    
    /// Gets the status of a transfer
    fn get_transfer_status(&self, transfer_id: &str) -> AnyaResult<TransferStatus>;
    
    /// Validates an asset transfer
    fn validate_transfer(&self, transfer_id: &str) -> AnyaResult<bool>;
    
    /// Gets asset metadata
    fn get_asset_metadata(&self, asset_id: &str) -> AnyaResult<HashMap<String, String>>;
    
    /// Gets the history of an asset
    fn get_asset_history(&self, asset_id: &str) -> AnyaResult<Vec<HistoryEntry>>;
}

/// Factory for creating RGB managers
pub struct RGBFactory;

impl RGBFactory {
    /// Creates a new RGB manager
    pub fn create_manager(config: RGBConfig) -> Box<dyn RGBManager> {
        Box::new(DefaultRGBManager::new(config))
    }
}

/// Configuration for RGB operations
#[derive(Debug, Clone)]
pub struct RGBConfig {
    /// Path to RGB data directory
    pub data_dir: PathBuf,
    
    /// Network to use (mainnet, testnet, etc.)
    pub network: String,
    
    /// Electrum server URL
    pub electrum_url: String,
    
    /// Storage type (SQLite, FS, etc.)
    pub storage_type: String,
    
    /// Default fee rate for transactions
    pub fee_rate: f64,
}

impl Default for RGBConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("./rgb_data"),
            network: "testnet".to_string(),
            electrum_url: "electrum.blockstream.info:60002".to_string(),
            storage_type: "sqlite".to_string(),
            fee_rate: 1.0,
        }
    }
}

/// Parameters for creating a new asset
#[derive(Debug, Clone)]
pub struct AssetCreationParams {
    /// Asset name
    pub name: String,
    
    /// Asset description
    pub description: Option<String>,
    
    /// Total supply
    pub total_supply: u64,
    
    /// Decimal precision
    pub precision: u8,
    
    /// Asset metadata
    pub metadata: HashMap<String, String>,
    
    /// Schema ID to use (or default if None)
    pub schema_id: Option<String>,
}

/// Status of an asset transfer
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransferStatus {
    /// Transfer is pending
    Pending,
    
    /// Transfer is confirmed
    Confirmed,
    
    /// Transfer failed
    Failed(String),
}

/// Entry in an asset's history
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    /// Transaction ID
    pub txid: Txid,
    
    /// Transaction timestamp
    pub timestamp: u64,
    
    /// Amount transferred
    pub amount: u64,
    
    /// Sender commitment (if known)
    pub from: Option<String>,
    
    /// Recipient commitment
    pub to: String,
    
    /// Operation type
    pub operation: OperationType,
}

/// Types of operations in asset history
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationType {
    /// Asset issuance
    Issuance,
    
    /// Asset transfer
    Transfer,
    
    /// Asset burn
    Burn,
}

/// Default implementation of the RGB manager
struct DefaultRGBManager {
    config: RGBConfig,
    client: Option<RGBClient>,
}

impl DefaultRGBManager {
    /// Creates a new default RGB manager
    fn new(config: RGBConfig) -> Self {
        Self {
            config,
            client: None,
        }
    }
}

impl RGBManager for DefaultRGBManager {
    fn init(&self, config: RGBConfig) -> AnyaResult<()> {
        log::info!("Initializing RGB Manager with network: {}", config.network);
        
        // Validate configuration
        if config.data_dir.to_string_lossy().is_empty() {
            return Err(crate::core::error::AnyaError::Config(
                "RGB data directory cannot be empty".to_string()
            ));
        }

        // Create data directory if it doesn't exist
        std::fs::create_dir_all(&config.data_dir)
            .map_err(|e| crate::core::error::AnyaError::Config(
                format!("Failed to create RGB data directory: {}", e)
            ))?;

        // Validate electrum URL format
        if config.electrum_url.is_empty() {
            return Err(crate::core::error::AnyaError::Config(
                "Electrum URL cannot be empty".to_string()
            ));
        }

        // Initialize storage based on storage_type
        let storage_path = config.data_dir.join("assets.db");
        match config.storage_type.as_str() {
            "sqlite" => {
                // Initialize SQLite storage - create database file if it doesn't exist
                if !storage_path.exists() {
                    std::fs::File::create(&storage_path)
                        .map_err(|e| crate::core::error::AnyaError::Config(
                            format!("Failed to create RGB database: {}", e)
                        ))?;
                }
                log::debug!("RGB SQLite storage initialized at: {:?}", storage_path);
            }
            "fs" => {
                // Initialize filesystem storage
                let assets_dir = config.data_dir.join("assets");
                std::fs::create_dir_all(&assets_dir)
                    .map_err(|e| crate::core::error::AnyaError::Config(
                        format!("Failed to create assets directory: {}", e)
                    ))?;
                log::debug!("RGB filesystem storage initialized at: {:?}", assets_dir);
            }
            _ => {
                return Err(crate::core::error::AnyaError::Config(
                    format!("Unsupported storage type: {}. Use 'sqlite' or 'fs'", config.storage_type)
                ));
            }
        }

        // Validate fee rate
        if config.fee_rate <= 0.0 {
            return Err(crate::core::error::AnyaError::Config(
                "Fee rate must be positive".to_string()
            ));
        }

        log::info!("RGB Manager initialized successfully - Network: {}, Storage: {}", 
                   config.network, config.storage_type);
        Ok(())
    }
    
    fn create_asset(&self, params: AssetCreationParams) -> AnyaResult<RGBAsset> {
        log::info!("Creating RGB asset: {}", params.name);
        
        // Validate asset creation parameters
        if params.name.is_empty() {
            return Err(crate::core::error::AnyaError::Validation(
                "Asset name cannot be empty".to_string()
            ));
        }

        if params.total_supply == 0 {
            return Err(crate::core::error::AnyaError::Validation(
                "Total supply must be greater than 0".to_string()
            ));
        }

        if params.precision > 18 {
            return Err(crate::core::error::AnyaError::Validation(
                "Precision cannot exceed 18 decimal places".to_string()
            ));
        }

        // Generate unique asset ID using timestamp + random component
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let asset_id = format!("rgb-{}-{}", timestamp, 
            format!("{:08x}", rand::random::<u32>()));

        // Generate contract ID (RGB contract that governs this asset)
        let contract_id = format!("contract-{}", 
            format!("{:012x}", rand::random::<u64>()));

        // Use provided schema ID or default to RGB20 (fungible tokens)
        let schema_id = params.schema_id.unwrap_or_else(|| "rgb20".to_string());

        // Create the asset structure
        let asset = RGBAsset {
            id: asset_id.clone(),
            name: params.name.clone(),
            description: params.description.clone(),
            total_supply: params.total_supply,
            precision: params.precision,
            metadata: params.metadata.clone(),
            contract_id: contract_id.clone(),
            schema_id: schema_id.clone(),
        };

        // Store the asset based on configured storage type
        match self.config.storage_type.as_str() {
            "sqlite" => {
                let storage_path = self.config.data_dir.join("assets.db");
                log::debug!("Storing asset {} in SQLite database", asset.id);
                // For now, create a simple file-based approach until full SQLite is implemented
                let assets_json_path = self.config.data_dir.join("assets_sqlite.json");
                self.store_asset_as_json(&asset, &assets_json_path)?;
            }
            "fs" => {
                let assets_dir = self.config.data_dir.join("assets");
                let asset_file = assets_dir.join(format!("{}.json", asset.id));
                
                let asset_json = serde_json::to_string_pretty(&asset)
                    .map_err(|e| crate::core::error::AnyaError::Serialization(
                        format!("Failed to serialize asset: {}", e)
                    ))?;
                
                std::fs::write(&asset_file, asset_json)
                    .map_err(|e| crate::core::error::AnyaError::Storage(
                        format!("Failed to write asset file: {}", e)
                    ))?;
                
                log::debug!("Stored asset {} in filesystem: {:?}", asset.id, asset_file);
            }
            _ => return Err(crate::core::error::AnyaError::Config(
                format!("Unsupported storage type: {}", self.config.storage_type)
            ))
        }

        log::info!("Successfully created RGB asset: {} (ID: {})", params.name, asset_id);
        Ok(asset)
    }
    
    fn list_assets(&self) -> AnyaResult<Vec<RGBAsset>> {
        log::debug!("Listing RGB assets from storage");
        let mut assets = Vec::new();

        match self.config.storage_type.as_str() {
            "sqlite" => {
                // Load from SQLite-style JSON storage (transitional implementation)
                let assets_json_path = self.config.data_dir.join("assets_sqlite.json");
                if assets_json_path.exists() {
                    assets = self.load_assets_from_json(&assets_json_path)?;
                }
                log::debug!("Loaded {} assets from SQLite storage", assets.len());
            }
            "fs" => {
                // Load from filesystem JSON files
                let assets_dir = self.config.data_dir.join("assets");
                
                if assets_dir.exists() {
                    let entries = std::fs::read_dir(&assets_dir)
                        .map_err(|e| crate::core::error::AnyaError::Storage(
                            format!("Failed to read assets directory: {}", e)
                        ))?;

                    for entry in entries {
                        let entry = entry
                            .map_err(|e| crate::core::error::AnyaError::Storage(
                                format!("Failed to read directory entry: {}", e)
                            ))?;
                        
                        let path = entry.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("json") {
                            let content = std::fs::read_to_string(&path)
                                .map_err(|e| crate::core::error::AnyaError::Storage(
                                    format!("Failed to read asset file {:?}: {}", path, e)
                                ))?;
                            
                            let asset: RGBAsset = serde_json::from_str(&content)
                                .map_err(|e| crate::core::error::AnyaError::Serialization(
                                    format!("Failed to deserialize asset from {:?}: {}", path, e)
                                ))?;
                            
                            assets.push(asset);
                        }
                    }
                }
                log::debug!("Loaded {} assets from filesystem storage", assets.len());
            }
            _ => return Err(crate::core::error::AnyaError::Config(
                format!("Unsupported storage type: {}", self.config.storage_type)
            ))
        }

        log::info!("Listed {} RGB assets", assets.len());
        Ok(assets)
    }
    
    fn get_asset_balance(&self, asset_id: &str) -> AnyaResult<u64> {
        log::debug!("Getting balance for asset: {}", asset_id);
        
        // Validate asset ID
        if asset_id.is_empty() {
            return Err(crate::core::error::AnyaError::Validation(
                "Asset ID cannot be empty".to_string()
            ));
        }

        // First, verify the asset exists
        let assets = self.list_assets()?;
        let asset = assets.iter()
            .find(|a| a.id == asset_id)
            .ok_or_else(|| crate::core::error::AnyaError::NotFound(
                format!("Asset with ID '{}' not found", asset_id)
            ))?;

        // For RGB assets, balance tracking is complex and requires UTXO analysis
        // In a full implementation, this would:
        // 1. Scan all UTXOs owned by the wallet
        // 2. Check which UTXOs contain commitments for this asset
        // 3. Sum up the asset amounts from valid commitments
        
        // For this implementation, we'll return the total supply as placeholder
        // This represents the maximum possible balance
        let balance = asset.total_supply;
        
        log::debug!("Asset {} balance: {} (precision: {})", 
                   asset_id, balance, asset.precision);
        Ok(balance)
    }
    
    fn create_invoice(&self, asset_id: &str, amount: u64) -> AnyaResult<String> {
        log::info!("Creating RGB invoice for asset: {}, amount: {}", asset_id, amount);
        
        // Validate inputs
        if asset_id.is_empty() {
            return Err(crate::core::error::AnyaError::Validation(
                "Asset ID cannot be empty".to_string()
            ));
        }
        
        if amount == 0 {
            return Err(crate::core::error::AnyaError::Validation(
                "Invoice amount must be greater than 0".to_string()
            ));
        }

        // Verify the asset exists
        let assets = self.list_assets()?;
        let asset = assets.iter()
            .find(|a| a.id == asset_id)
            .ok_or_else(|| crate::core::error::AnyaError::NotFound(
                format!("Asset with ID '{}' not found", asset_id)
            ))?;

        // Generate a unique invoice ID
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let invoice_id = format!("rgb-invoice-{}-{}", timestamp, 
            format!("{:08x}", rand::random::<u32>()));

        // In RGB, invoices are actually blinded UTXOs that can receive the asset
        // For this implementation, we'll create a simple invoice string
        let invoice = format!("{}:{}:{}:{}", 
            asset_id, amount, asset.precision, invoice_id);

        log::info!("Created RGB invoice: {}", invoice_id);
        log::debug!("Invoice details - Asset: {}, Amount: {}, Precision: {}", 
                   asset_id, amount, asset.precision);
        
        Ok(invoice)
    }
    
    fn transfer_asset(&self, transfer: AssetTransfer) -> AnyaResult<String> {
        log::info!("Initiating RGB asset transfer - Asset: {}, Amount: {}, Recipient: {}", 
                   transfer.asset_id, transfer.amount, transfer.recipient);
        
        // Validate transfer parameters
        if transfer.asset_id.is_empty() {
            return Err(crate::core::error::AnyaError::Validation(
                "Asset ID cannot be empty".to_string()
            ));
        }
        
        if transfer.amount == 0 {
            return Err(crate::core::error::AnyaError::Validation(
                "Transfer amount must be greater than 0".to_string()
            ));
        }
        
        if transfer.recipient.is_empty() {
            return Err(crate::core::error::AnyaError::Validation(
                "Recipient cannot be empty".to_string()
            ));
        }

        // Verify the asset exists
        let assets = self.list_assets()?;
        let asset = assets.iter()
            .find(|a| a.id == transfer.asset_id)
            .ok_or_else(|| crate::core::error::AnyaError::NotFound(
                format!("Asset with ID '{}' not found", transfer.asset_id)
            ))?;

        // Check if we have sufficient balance (simplified check)
        let current_balance = self.get_asset_balance(&transfer.asset_id)?;
        if transfer.amount > current_balance {
            return Err(crate::core::error::AnyaError::InsufficientFunds(
                format!("Insufficient balance: requested {}, available {}", 
                       transfer.amount, current_balance)
            ));
        }

        // Generate unique transfer ID
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let transfer_id = format!("rgb-transfer-{}-{}", timestamp, 
            format!("{:08x}", rand::random::<u32>()));

        // In a full RGB implementation, this would:
        // 1. Create a consignment (RGB state transition)
        // 2. Build a Bitcoin transaction with proper commitments
        // 3. Sign and broadcast the transaction
        // 4. Track the transfer status
        
        // For this implementation, we'll create a transfer record
        let transfer_record = TransferRecord {
            id: transfer_id.clone(),
            asset_id: transfer.asset_id.clone(),
            amount: transfer.amount,
            recipient: transfer.recipient.clone(),
            status: TransferStatus::Pending,
            timestamp,
            metadata: transfer.metadata,
        };
        
        // Store transfer record
        self.store_transfer_record(&transfer_record)?;
        
        log::info!("RGB transfer initiated successfully - Transfer ID: {}", transfer_id);
        log::debug!("Transfer details - Asset: {}, Amount: {}, Recipient: {}", 
                   transfer.asset_id, transfer.amount, transfer.recipient);
        
        Ok(transfer_id)
    }
    
    fn get_transfer_status(&self, transfer_id: &str) -> AnyaResult<TransferStatus> {
        log::debug!("Getting status for transfer: {}", transfer_id);
        
        if transfer_id.is_empty() {
            return Err(crate::core::error::AnyaError::Validation(
                "Transfer ID cannot be empty".to_string()
            ));
        }

        // Load transfer record
        let transfer_record = self.load_transfer_record(transfer_id)?;
        
        // In a full implementation, this would check:
        // 1. Bitcoin transaction confirmation status
        // 2. RGB consignment validation status
        // 3. Network propagation status
        
        // For now, we'll simulate some basic status logic
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        let time_elapsed = current_time - transfer_record.timestamp;
        
        // Simulate transfer progression based on time
        let status = if time_elapsed < 60 {
            TransferStatus::Pending
        } else if time_elapsed < 600 { // 10 minutes
            // Simulate 90% success rate
            if rand::random::<u8>() < 229 { // 90% of 255
                TransferStatus::Confirmed
            } else {
                TransferStatus::Failed("Network validation failed".to_string())
            }
        } else {
            TransferStatus::Confirmed
        };
        
        log::debug!("Transfer {} status: {:?}", transfer_id, status);
        Ok(status)
    }
    
    fn validate_transfer(&self, transfer_id: &str) -> AnyaResult<bool> {
        log::debug!("Validating transfer: {}", transfer_id);
        
        if transfer_id.is_empty() {
            return Err(crate::core::error::AnyaError::Validation(
                "Transfer ID cannot be empty".to_string()
            ));
        }

        // Load transfer record
        let transfer_record = self.load_transfer_record(transfer_id)?;
        
        // Validate the transfer exists and has valid data
        let mut is_valid = true;
        let mut validation_errors = Vec::new();
        
        // Basic validation checks
        if transfer_record.asset_id.is_empty() {
            validation_errors.push("Asset ID is empty");
            is_valid = false;
        }
        
        if transfer_record.amount == 0 {
            validation_errors.push("Transfer amount is zero");
            is_valid = false;
        }
        
        if transfer_record.recipient.is_empty() {
            validation_errors.push("Recipient is empty");
            is_valid = false;
        }
        
        // Verify the asset still exists
        match self.list_assets() {
            Ok(assets) => {
                if !assets.iter().any(|a| a.id == transfer_record.asset_id) {
                    validation_errors.push("Referenced asset no longer exists");
                    is_valid = false;
                }
            }
            Err(_) => {
                validation_errors.push("Failed to verify asset existence");
                is_valid = false;
            }
        }
        
        // In a full RGB implementation, this would also validate:
        // 1. RGB consignment structure
        // 2. State transition validity
        // 3. Cryptographic proofs
        // 4. Bitcoin transaction validity
        // 5. UTXO spending authorization
        
        if !validation_errors.is_empty() {
            log::warn!("Transfer {} validation failed: {:?}", transfer_id, validation_errors);
        } else {
            log::debug!("Transfer {} validation passed", transfer_id);
        }
        
        Ok(is_valid)
    }
    
    fn get_asset_metadata(&self, asset_id: &str) -> AnyaResult<HashMap<String, String>> {
        log::debug!("Getting metadata for asset: {}", asset_id);
        
        if asset_id.is_empty() {
            return Err(crate::core::error::AnyaError::Validation(
                "Asset ID cannot be empty".to_string()
            ));
        }

        // Find the asset
        let assets = self.list_assets()?;
        let asset = assets.iter()
            .find(|a| a.id == asset_id)
            .ok_or_else(|| crate::core::error::AnyaError::NotFound(
                format!("Asset with ID '{}' not found", asset_id)
            ))?;

        // Combine asset metadata with system metadata
        let mut metadata = asset.metadata.clone();
        
        // Add system metadata
        metadata.insert("asset_id".to_string(), asset.id.clone());
        metadata.insert("name".to_string(), asset.name.clone());
        metadata.insert("total_supply".to_string(), asset.total_supply.to_string());
        metadata.insert("precision".to_string(), asset.precision.to_string());
        metadata.insert("contract_id".to_string(), asset.contract_id.clone());
        metadata.insert("schema_id".to_string(), asset.schema_id.clone());
        
        if let Some(ref description) = asset.description {
            metadata.insert("description".to_string(), description.clone());
        }
        
        // Add RGB-specific metadata
        metadata.insert("protocol".to_string(), "RGB".to_string());
        metadata.insert("version".to_string(), "1.0".to_string());
        
        log::debug!("Retrieved {} metadata fields for asset {}", metadata.len(), asset_id);
        Ok(metadata)
    }
    
    fn get_asset_history(&self, asset_id: &str) -> AnyaResult<Vec<HistoryEntry>> {
        log::debug!("Getting history for asset: {}", asset_id);
        
        if asset_id.is_empty() {
            return Err(crate::core::error::AnyaError::Validation(
                "Asset ID cannot be empty".to_string()
            ));
        }

        // Verify the asset exists
        let assets = self.list_assets()?;
        let asset = assets.iter()
            .find(|a| a.id == asset_id)
            .ok_or_else(|| crate::core::error::AnyaError::NotFound(
                format!("Asset with ID '{}' not found", asset_id)
            ))?;

        let mut history = Vec::new();
        
        // Add asset issuance entry (creation)
        let issuance_entry = HistoryEntry {
            txid: bitcoin::Txid::from_slice(&[0u8; 32]).unwrap(), // Placeholder txid
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            amount: asset.total_supply,
            from: None, // Issuance has no sender
            to: "genesis".to_string(), // Asset genesis
            operation: OperationType::Issuance,
        };
        history.push(issuance_entry);
        
        // Load transfer records for this asset
        let transfer_records = self.load_transfer_records_for_asset(asset_id)?;
        
        // Convert transfer records to history entries
        for record in transfer_records {
            let history_entry = HistoryEntry {
                txid: bitcoin::Txid::from_slice(&[0u8; 32]).unwrap(), // Placeholder txid
                timestamp: record.timestamp,
                amount: record.amount,
                from: Some("wallet".to_string()), // Simplified sender
                to: record.recipient,
                operation: OperationType::Transfer,
            };
            history.push(history_entry);
        }
        
        // Sort history by timestamp (oldest first)
        history.sort_by_key(|entry| entry.timestamp);
        
        log::debug!("Retrieved {} history entries for asset {}", history.len(), asset_id);
        Ok(history)
    }
}

// Helper data structures for RGB implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TransferRecord {
    id: String,
    asset_id: String,
    amount: u64,
    recipient: String,
    status: TransferStatus,
    timestamp: u64,
    metadata: HashMap<String, String>,
}

impl DefaultRGBManager {
    /// Helper method to store asset as JSON
    fn store_asset_as_json(&self, asset: &RGBAsset, path: &std::path::Path) -> AnyaResult<()> {
        // Load existing assets or create new vector
        let mut assets = if path.exists() {
            self.load_assets_from_json(path)?
        } else {
            Vec::new()
        };
        
        // Add or update the asset
        if let Some(existing_index) = assets.iter().position(|a| a.id == asset.id) {
            assets[existing_index] = asset.clone();
        } else {
            assets.push(asset.clone());
        }
        
        // Write back to file
        let assets_json = serde_json::to_string_pretty(&assets)
            .map_err(|e| crate::core::error::AnyaError::Serialization(
                format!("Failed to serialize assets: {}", e)
            ))?;
        
        std::fs::write(path, assets_json)
            .map_err(|e| crate::core::error::AnyaError::Storage(
                format!("Failed to write assets file: {}", e)
            ))?;
        
        Ok(())
    }
    
    /// Helper method to load assets from JSON
    fn load_assets_from_json(&self, path: &std::path::Path) -> AnyaResult<Vec<RGBAsset>> {
        if !path.exists() {
            return Ok(Vec::new());
        }
        
        let content = std::fs::read_to_string(path)
            .map_err(|e| crate::core::error::AnyaError::Storage(
                format!("Failed to read assets file: {}", e)
            ))?;
        
        let assets: Vec<RGBAsset> = serde_json::from_str(&content)
            .map_err(|e| crate::core::error::AnyaError::Serialization(
                format!("Failed to deserialize assets: {}", e)
            ))?;
        
        Ok(assets)
    }
    
    /// Helper method to store transfer record
    fn store_transfer_record(&self, record: &TransferRecord) -> AnyaResult<()> {
        let transfers_path = self.config.data_dir.join("transfers.json");
        
        // Load existing transfers or create new vector
        let mut transfers = if transfers_path.exists() {
            self.load_transfer_records()?
        } else {
            Vec::new()
        };
        
        // Add the new transfer record
        transfers.push(record.clone());
        
        // Write back to file
        let transfers_json = serde_json::to_string_pretty(&transfers)
            .map_err(|e| crate::core::error::AnyaError::Serialization(
                format!("Failed to serialize transfers: {}", e)
            ))?;
        
        std::fs::write(&transfers_path, transfers_json)
            .map_err(|e| crate::core::error::AnyaError::Storage(
                format!("Failed to write transfers file: {}", e)
            ))?;
        
        Ok(())
    }
    
    /// Helper method to load transfer record by ID
    fn load_transfer_record(&self, transfer_id: &str) -> AnyaResult<TransferRecord> {
        let transfers = self.load_transfer_records()?;
        
        transfers.into_iter()
            .find(|t| t.id == transfer_id)
            .ok_or_else(|| crate::core::error::AnyaError::NotFound(
                format!("Transfer with ID '{}' not found", transfer_id)
            ))
    }
    
    /// Helper method to load all transfer records
    fn load_transfer_records(&self) -> AnyaResult<Vec<TransferRecord>> {
        let transfers_path = self.config.data_dir.join("transfers.json");
        
        if !transfers_path.exists() {
            return Ok(Vec::new());
        }
        
        let content = std::fs::read_to_string(&transfers_path)
            .map_err(|e| crate::core::error::AnyaError::Storage(
                format!("Failed to read transfers file: {}", e)
            ))?;
        
        let transfers: Vec<TransferRecord> = serde_json::from_str(&content)
            .map_err(|e| crate::core::error::AnyaError::Serialization(
                format!("Failed to deserialize transfers: {}", e)
            ))?;
        
        Ok(transfers)
    }
    
    /// Helper method to load transfer records for a specific asset
    fn load_transfer_records_for_asset(&self, asset_id: &str) -> AnyaResult<Vec<TransferRecord>> {
        let all_transfers = self.load_transfer_records()?;
        let asset_transfers: Vec<TransferRecord> = all_transfers
            .into_iter()
            .filter(|t| t.asset_id == asset_id)
            .collect();
        
        Ok(asset_transfers)
    }
}
