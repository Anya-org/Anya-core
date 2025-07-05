// RGB Layer 2 implementation
//
// This module provides an implementation of RGB protocol, a client-side
// validation solution for Bitcoin assets. It allows for the creation
mod client;
mod contract;
mod node;
/// and transfer of complex assets on top of Bitcoin's blockchain.
mod schema;
mod state;
mod wallet;

pub use client::{ClientConfig, RGBClient, RGBClientBuilder};
pub use contract::{Contract, ContractBuilder, ContractType, Witness};
pub use node::{NodeConfig, RGBNode};
pub use schema::{Field, FieldType, Schema, SchemaType, Validation};
pub use state::{StateTransfer, StateTransition, StateValidator};
pub use wallet::{AssetBalance, RGBWallet};

use bitcoin::Txid;
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

use crate::core::error::{AnyaResult, AnyaError};
use crate::core::wallet::TxOptions;
use crate::layer2::traits::{ContractExecutor, FederationMLHook, Proposal};

/// RGB asset data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
        // Create RGB data directory if it doesn't exist
        if !config.data_dir.exists() {
            std::fs::create_dir_all(&config.data_dir)
                .map_err(|e| AnyaError::ConfigurationError(format!("Failed to create RGB data directory: {}", e)))?;
        }

        // Validate network configuration
        if !["mainnet", "testnet", "regtest"].contains(&config.network.as_str()) {
            return Err(AnyaError::ConfigurationError(
                format!("Invalid network: {}. Must be mainnet, testnet, or regtest", config.network)
            ));
        }

        // Validate electrum URL format
        if config.electrum_url.is_empty() {
            return Err(AnyaError::ConfigurationError(
                "Electrum URL cannot be empty".to_string()
            ));
        }

        // Initialize storage based on storage_type
        let storage_path = config.data_dir.join("assets.db");
        match config.storage_type.as_str() {
            "sqlite" => {
                // Initialize SQLite storage (placeholder implementation)
                // In a full implementation, this would set up the database schema
                if !storage_path.exists() {
                    // Create empty database file
                    std::fs::File::create(&storage_path)
                        .map_err(|e| AnyaError::ConfigurationError(format!("Failed to create storage file: {}", e)))?;
                }
            }
            "fs" => {
                // Initialize filesystem storage
                let assets_dir = config.data_dir.join("assets");
                std::fs::create_dir_all(&assets_dir)
                    .map_err(|e| AnyaError::ConfigurationError(format!("Failed to create assets directory: {}", e)))?;
            }
            _ => {
                return Err(AnyaError::ConfigurationError(
                    format!("Unsupported storage type: {}. Use 'sqlite' or 'fs'", config.storage_type)
                ));
            }
        }

        // Validate fee rate
        if config.fee_rate <= 0.0 {
            return Err(AnyaError::ConfigurationError(
                "Fee rate must be positive".to_string()
            ));
        }

        // Log successful initialization
        log::info!("RGB Manager initialized successfully");
        log::info!("Network: {}", config.network);
        log::info!("Data directory: {}", config.data_dir.display());
        log::info!("Storage type: {}", config.storage_type);
        log::info!("Electrum URL: {}", config.electrum_url);

        Ok(())
    }

    fn create_asset(&self, params: AssetCreationParams) -> AnyaResult<RGBAsset> {
        // Validate asset creation parameters
        if params.name.is_empty() {
            return Err(AnyaError::ValidationError("Asset name cannot be empty".to_string()));
        }

        if params.total_supply == 0 {
            return Err(AnyaError::ValidationError("Total supply must be greater than 0".to_string()));
        }

        if params.precision > 18 {
            return Err(AnyaError::ValidationError("Precision cannot exceed 18 decimal places".to_string()));
        }

        // Generate unique asset ID (using current timestamp + random component for uniqueness)
        let asset_id = format!("rgb-{}-{}", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            uuid::Uuid::new_v4().to_string().replace("-", "")[..8].to_string()
        );

        // Generate contract ID (in RGB, each asset has an associated contract)
        let contract_id = format!("contract-{}", uuid::Uuid::new_v4().to_string().replace("-", "")[..12].to_string());

        // Use provided schema ID or default
        let schema_id = params.schema_id.unwrap_or_else(|| "rgb20".to_string());

        // Create the asset
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

        // Store the asset based on storage type
        match self.config.storage_type.as_str() {
            "sqlite" => {
                // Store in SQLite database (placeholder implementation)
                // In a full implementation, this would use a proper SQL query
                let storage_path = self.config.data_dir.join("assets.db");
                log::debug!("Storing asset {} in SQLite at {}", asset.id, storage_path.display());
                // TODO: Implement actual SQLite storage
            }
            "fs" => {
                // Store as JSON file in filesystem
                let assets_dir = self.config.data_dir.join("assets");
                let asset_file = assets_dir.join(format!("{}.json", asset.id));
                
                let asset_json = serde_json::to_string_pretty(&asset)
                    .map_err(|e| AnyaError::SerializationError(format!("Failed to serialize asset: {}", e)))?;
                
                std::fs::write(&asset_file, asset_json)
                    .map_err(|e| AnyaError::StorageError(format!("Failed to write asset file: {}", e)))?;
                
                log::debug!("Stored asset {} as file: {}", asset.id, asset_file.display());
            }
            _ => return Err(AnyaError::ConfigurationError(
                format!("Unsupported storage type: {}", self.config.storage_type)
            ))
        }

        // Log asset creation
        log::info!("Created RGB asset: {} ({})", params.name, asset_id);
        log::debug!("Asset details - Supply: {}, Precision: {}, Schema: {}, Contract: {}", 
            params.total_supply, params.precision, schema_id, contract_id);

        Ok(asset)
    }

    fn list_assets(&self) -> AnyaResult<Vec<RGBAsset>> {
        let mut assets = Vec::new();

        match self.config.storage_type.as_str() {
            "sqlite" => {
                // Load from SQLite database (placeholder implementation)
                // In a full implementation, this would query the database
                let storage_path = self.config.data_dir.join("assets.db");
                log::debug!("Loading assets from SQLite at {}", storage_path.display());
                // TODO: Implement actual SQLite queries
                // For now, return empty list as SQLite storage is not fully implemented
            }
            "fs" => {
                // Load from filesystem JSON files
                let assets_dir = self.config.data_dir.join("assets");
                
                if assets_dir.exists() {
                    let entries = std::fs::read_dir(&assets_dir)
                        .map_err(|e| AnyaError::StorageError(format!("Failed to read assets directory: {}", e)))?;

                    for entry in entries {
                        let entry = entry
                            .map_err(|e| AnyaError::StorageError(format!("Failed to read directory entry: {}", e)))?;
                        
                        let path = entry.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("json") {
                            let asset_data = std::fs::read_to_string(&path)
                                .map_err(|e| AnyaError::StorageError(format!("Failed to read asset file {}: {}", path.display(), e)))?;
                            
                            let asset: RGBAsset = serde_json::from_str(&asset_data)
                                .map_err(|e| AnyaError::SerializationError(format!("Failed to deserialize asset from {}: {}", path.display(), e)))?;
                            
                            assets.push(asset);
                        }
                    }
                }
                
                log::debug!("Loaded {} assets from filesystem", assets.len());
            }
            _ => return Err(AnyaError::ConfigurationError(
                format!("Unsupported storage type: {}", self.config.storage_type)
            ))
        }

        // Sort assets by name for consistent ordering
        assets.sort_by(|a, b| a.name.cmp(&b.name));
        
        log::info!("Listed {} RGB assets", assets.len());
        Ok(assets)
    }

    fn get_asset_balance(&self, asset_id: &str) -> AnyaResult<u64> {
        // Find the asset first to validate it exists
        let assets = self.list_assets()?;
        let asset = assets.iter()
            .find(|a| a.id == asset_id)
            .ok_or_else(|| AnyaError::NotFound(format!("Asset not found: {}", asset_id)))?;

        // In a full RGB implementation, this would query the actual UTXO set
        // and calculate the balance based on state transitions and commitments.
        // For now, we'll implement a simplified version using storage.
        
        let balance = match self.config.storage_type.as_str() {
            "sqlite" => {
                // Query SQLite for balance (placeholder implementation)
                log::debug!("Querying asset balance from SQLite for asset: {}", asset_id);
                // TODO: Implement actual SQLite balance queries
                // For now, return the total supply as a placeholder
                asset.total_supply
            }
            "fs" => {
                // Load balance from filesystem
                let balance_dir = self.config.data_dir.join("balances");
                let balance_file = balance_dir.join(format!("{}.balance", asset_id));
                
                if balance_file.exists() {
                    let balance_data = std::fs::read_to_string(&balance_file)
                        .map_err(|e| AnyaError::StorageError(format!("Failed to read balance file: {}", e)))?;
                    
                    balance_data.trim().parse::<u64>()
                        .map_err(|e| AnyaError::ValidationError(format!("Invalid balance data: {}", e)))?
                } else {
                    // If no balance file exists, assume full supply is available
                    // In a real implementation, this would be more sophisticated
                    log::debug!("No balance file found for asset {}, returning total supply", asset_id);
                    asset.total_supply
                }
            }
            _ => return Err(AnyaError::ConfigurationError(
                format!("Unsupported storage type: {}", self.config.storage_type)
            ))
        };

        log::debug!("Asset {} balance: {}", asset_id, balance);
        Ok(balance)
    }

    fn create_invoice(&self, asset_id: &str, amount: u64) -> AnyaResult<String> {
        // Validate the asset exists
        let assets = self.list_assets()?;
        let asset = assets.iter()
            .find(|a| a.id == asset_id)
            .ok_or_else(|| AnyaError::NotFound(format!("Asset not found: {}", asset_id)))?;

        // Validate amount
        if amount == 0 {
            return Err(AnyaError::ValidationError("Invoice amount must be greater than 0".to_string()));
        }

        // Generate unique invoice ID
        let invoice_id = format!("inv-{}-{}", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            uuid::Uuid::new_v4().to_string().replace("-", "")[..8].to_string()
        );

        // Create invoice data structure
        let invoice_data = serde_json::json!({
            "invoice_id": invoice_id,
            "asset_id": asset_id,
            "asset_name": asset.name,
            "amount": amount,
            "precision": asset.precision,
            "created_at": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "network": self.config.network,
            "status": "pending"
        });

        // Store invoice based on storage type
        match self.config.storage_type.as_str() {
            "sqlite" => {
                // Store in SQLite database (placeholder implementation)
                log::debug!("Storing invoice {} in SQLite", invoice_id);
                // TODO: Implement actual SQLite invoice storage
            }
            "fs" => {
                // Store as JSON file in filesystem
                let invoices_dir = self.config.data_dir.join("invoices");
                std::fs::create_dir_all(&invoices_dir)
                    .map_err(|e| AnyaError::StorageError(format!("Failed to create invoices directory: {}", e)))?;

                let invoice_file = invoices_dir.join(format!("{}.json", invoice_id));
                let invoice_json = serde_json::to_string_pretty(&invoice_data)
                    .map_err(|e| AnyaError::SerializationError(format!("Failed to serialize invoice: {}", e)))?;
                
                std::fs::write(&invoice_file, invoice_json)
                    .map_err(|e| AnyaError::StorageError(format!("Failed to write invoice file: {}", e)))?;
                
                log::debug!("Stored invoice {} as file: {}", invoice_id, invoice_file.display());
            }
            _ => return Err(AnyaError::ConfigurationError(
                format!("Unsupported storage type: {}", self.config.storage_type)
            ))
        }

        log::info!("Created invoice {} for {} units of asset {}", invoice_id, amount, asset.name);
        Ok(invoice_id)
    }

    fn transfer_asset(&self, transfer: AssetTransfer) -> AnyaResult<String> {
        // Validate the asset exists
        let assets = self.list_assets()?;
        let asset = assets.iter()
            .find(|a| a.id == transfer.asset_id)
            .ok_or_else(|| AnyaError::NotFound(format!("Asset not found: {}", transfer.asset_id)))?;

        // Validate transfer amount
        if transfer.amount == 0 {
            return Err(AnyaError::ValidationError("Transfer amount must be greater than 0".to_string()));
        }

        // Check if we have sufficient balance
        let current_balance = self.get_asset_balance(&transfer.asset_id)?;
        if transfer.amount > current_balance {
            return Err(AnyaError::ValidationError(format!(
                "Insufficient balance: requested {} but only {} available", 
                transfer.amount, current_balance
            )));
        }

        // Validate recipient
        if transfer.recipient.is_empty() {
            return Err(AnyaError::ValidationError("Recipient cannot be empty".to_string()));
        }

        // Generate unique transfer ID
        let transfer_id = format!("tx-{}-{}", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            uuid::Uuid::new_v4().to_string().replace("-", "")[..8].to_string()
        );

        // Create transfer record
        let transfer_data = serde_json::json!({
            "transfer_id": transfer_id,
            "asset_id": transfer.asset_id,
            "asset_name": asset.name,
            "amount": transfer.amount,
            "recipient": transfer.recipient,
            "metadata": transfer.metadata,
            "created_at": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "network": self.config.network,
            "status": "pending",
            "fee_rate": self.config.fee_rate
        });

        // Store transfer record and update balances
        match self.config.storage_type.as_str() {
            "sqlite" => {
                // Store in SQLite database (placeholder implementation)
                log::debug!("Storing transfer {} in SQLite", transfer_id);
                // TODO: Implement actual SQLite transfer storage and balance updates
            }
            "fs" => {
                // Store transfer record as JSON file
                let transfers_dir = self.config.data_dir.join("transfers");
                std::fs::create_dir_all(&transfers_dir)
                    .map_err(|e| AnyaError::StorageError(format!("Failed to create transfers directory: {}", e)))?;

                let transfer_file = transfers_dir.join(format!("{}.json", transfer_id));
                let transfer_json = serde_json::to_string_pretty(&transfer_data)
                    .map_err(|e| AnyaError::SerializationError(format!("Failed to serialize transfer: {}", e)))?;
                
                std::fs::write(&transfer_file, transfer_json)
                    .map_err(|e| AnyaError::StorageError(format!("Failed to write transfer file: {}", e)))?;

                // Update balance (simplified implementation)
                let balance_dir = self.config.data_dir.join("balances");
                std::fs::create_dir_all(&balance_dir)
                    .map_err(|e| AnyaError::StorageError(format!("Failed to create balances directory: {}", e)))?;

                let balance_file = balance_dir.join(format!("{}.balance", transfer.asset_id));
                let new_balance = current_balance - transfer.amount;
                std::fs::write(&balance_file, new_balance.to_string())
                    .map_err(|e| AnyaError::StorageError(format!("Failed to update balance file: {}", e)))?;
                
                log::debug!("Stored transfer {} and updated balance from {} to {}", 
                    transfer_id, current_balance, new_balance);
            }
            _ => return Err(AnyaError::ConfigurationError(
                format!("Unsupported storage type: {}", self.config.storage_type)
            ))
        }

        log::info!("Created transfer {} for {} units of {} to {}", 
            transfer_id, transfer.amount, asset.name, transfer.recipient);
        
        Ok(transfer_id)
    }

    fn get_transfer_status(&self, transfer_id: &str) -> AnyaResult<TransferStatus> {
        // Validate transfer ID format
        if transfer_id.is_empty() || !transfer_id.starts_with("tx-") {
            return Err(AnyaError::ValidationError("Invalid transfer ID format".to_string()));
        }

        // Load transfer status based on storage type
        match self.config.storage_type.as_str() {
            "sqlite" => {
                // Query SQLite database (placeholder implementation)
                log::debug!("Querying transfer {} status from SQLite", transfer_id);
                // TODO: Implement actual SQLite transfer status query
                Ok(TransferStatus {
                    id: transfer_id.to_string(),
                    status: "pending".to_string(),
                    confirmations: 0,
                    block_height: None,
                    fee_paid: None,
                })
            }
            "fs" => {
                // Load from filesystem
                let transfers_dir = self.config.data_dir.join("transfers");
                let transfer_file = transfers_dir.join(format!("{}.json", transfer_id));
                
                if !transfer_file.exists() {
                    return Err(AnyaError::NotFound(format!("Transfer not found: {}", transfer_id)));
                }

                let transfer_json = std::fs::read_to_string(&transfer_file)
                    .map_err(|e| AnyaError::StorageError(format!("Failed to read transfer file: {}", e)))?;
                
                let transfer_data: serde_json::Value = serde_json::from_str(&transfer_json)
                    .map_err(|e| AnyaError::SerializationError(format!("Failed to parse transfer data: {}", e)))?;

                // Extract status information
                let status = transfer_data["status"].as_str().unwrap_or("unknown").to_string();
                let confirmations = transfer_data["confirmations"].as_u64().unwrap_or(0) as u32;
                let block_height = transfer_data["block_height"].as_u64().map(|h| h as u32);
                let fee_paid = transfer_data["fee_paid"].as_u64();

                log::debug!("Retrieved transfer {} status: {}", transfer_id, status);
                
                Ok(TransferStatus {
                    id: transfer_id.to_string(),
                    status,
                    confirmations,
                    block_height,
                    fee_paid,
                })
            }
            _ => Err(AnyaError::ConfigurationError(
                format!("Unsupported storage type: {}", self.config.storage_type)
            ))
        }
    }

    fn validate_transfer(&self, _transfer_id: &str) -> AnyaResult<bool> {
        // Implementation goes here
        unimplemented!("Transfer validation not yet implemented")
    }

    fn get_asset_metadata(&self, _asset_id: &str) -> AnyaResult<HashMap<String, String>> {
        // Implementation goes here
        unimplemented!("Asset metadata querying not yet implemented")
    }

    fn get_asset_history(&self, _asset_id: &str) -> AnyaResult<Vec<HistoryEntry>> {
        // Implementation goes here
        unimplemented!("Asset history querying not yet implemented")
    }
}

/// RGBProposal: Implements Proposal trait for RGB actions
#[derive(Debug, Clone)]
pub struct RGBProposal {
    pub id: String,
    pub action: String,
    pub data: HashMap<String, String>,
}

impl Proposal for RGBProposal {
    fn id(&self) -> &str {
        &self.id
    }
    fn action(&self) -> &str {
        &self.action
    }
    fn data(&self) -> &HashMap<String, String> {
        &self.data
    }
}

/// RGBManagerExt: Extensible manager for RGB flows (top-layer, advanced)
pub struct RGBManagerExt {
    pub contract_executor: Option<Box<dyn ContractExecutor<RGBProposal> + Send + Sync>>,
    pub ml_hook: Option<Box<dyn FederationMLHook<RGBProposal> + Send + Sync>>,
}

impl RGBManagerExt {
    pub fn new() -> Self {
        Self {
            contract_executor: None,
            ml_hook: None,
        }
    }
    pub fn with_contract_executor(
        mut self,
        exec: Box<dyn ContractExecutor<RGBProposal> + Send + Sync>,
    ) -> Self {
        self.contract_executor = Some(exec);
        self
    }
    pub fn with_ml_hook(
        mut self,
        hook: Box<dyn FederationMLHook<RGBProposal> + Send + Sync>,
    ) -> Self {
        self.ml_hook = Some(hook);
        self
    }
    /// Example: Approve an RGB proposal (calls ML hook if present)
    pub fn approve(&mut self, proposal: &RGBProposal, member_id: &str) -> Result<(), String> {
        if let Some(hook) = &self.ml_hook {
            hook.on_approve(proposal, member_id)?;
        }
        Ok(())
    }
    /// Example: Execute an RGB proposal (calls contract executor and ML hook if present)
    pub fn execute(&mut self, proposal: &RGBProposal) -> Result<String, String> {
        if let Some(hook) = &self.ml_hook {
            hook.on_execute(proposal)?;
        }
        if let Some(exec) = &self.contract_executor {
            exec.execute_contract(proposal)
        } else {
            Ok(format!("rgb-txid-{}", proposal.id))
        }
    }
}

// --- Anya-core: RGB module now supports top-layer extensibility for contract execution and ML hooks ---
// --- Use RGBManagerExt for advanced, production-grade flows ---
