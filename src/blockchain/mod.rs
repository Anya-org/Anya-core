use std::error::Error;
//! Blockchain module
//!
//! This module provides a hexagonal architecture implementation for
//! blockchain interaction, following the requirements in the specifications.
//! [AIR-1][AIS-1][AIM-1][AIP-1][RES-1]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use log::{debug, info, warn, error};
use thiserror::Error;

/// Blockchain error type
#[derive(Error, Debug)]
pub enum BlockchainError {
    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    /// Synchronization error
    #[error("Synchronization error: {0}")]
    SyncError(String),
    
    /// Block processing error
    #[error("Block processing error: {0}")]
    BlockProcessingError(String),
    
    /// Transaction error
    #[error("Transaction error: {0}")]
    TransactionError(String),
    
    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    /// Storage error
    #[error("Storage error: {0}")]
    StorageError(String),
    
    /// RPC error
    #[error("RPC error: {0}")]
    RpcError(String),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    /// Not found error
    #[error("Not found: {0}")]
    NotFoundError(String),
    
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    /// Timeout error
    #[error("Timeout: {0}")]
    TimeoutError(String),
    
    /// Internal error
    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Blockchain metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainMetrics {
    /// Number of blocks in the chain
    pub block_count: u64,
    
    /// Number of transactions in the chain
    pub tx_count: u64,
    
    /// Size of the UTXO set
    pub utxo_set_size: u64,
    
    /// Difficulty
    pub difficulty: f64,
    
    /// Estimated hash rate
    pub hash_rate: f64,
    
    /// Network weight (for PoS chains)
    pub network_weight: Option<f64>,
    
    /// Block propagation time (ms)
    pub block_propagation_time: u64,
    
    /// Mempool size (bytes)
    pub mempool_size: u64,
    
    /// Mempool transaction count
    pub mempool_tx_count: u64,
    
    /// Fee estimates
    pub fee_estimates: HashMap<u16, u64>,
    
    /// Timestamp of this measurement
    pub timestamp: u64,
}

/// Blockchain state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainState {
    /// Chain ID
    pub chain_id: String,
    
    /// Network name
    pub network: String,
    
    /// Protocol version
    pub protocol_version: u32,
    
    /// Best block hash
    pub best_block_hash: String,
    
    /// Best block height
    pub best_block_height: u64,
    
    /// Median time past
    pub median_time_past: u64,
    
    /// Whether the chain is in initial block download
    pub initial_block_download: bool,
    
    /// Synchronization progress (0.0 to 1.0)
    pub sync_progress: f64,
    
    /// Chain work
    pub chain_work: String,
    
    /// Size on disk
    pub size_on_disk: u64,
    
    /// Number of connections
    pub connection_count: u32,
    
    /// Verification progress
    pub verification_progress: f64,
    
    /// Pruned status
    pub pruned: bool,
    
    /// Pruneheight (if pruned)
    pub prune_height: Option<u64>,
    
    /// Last checkpoint height
    pub last_checkpoint: Option<u64>,
    
    /// Warning message (if any)
    pub warnings: Option<String>,
}

/// Blockchain peer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    /// Peer ID
    pub id: u64,
    
    /// Peer address
    pub addr: String,
    
    /// Services provided by peer
    pub services: u64,
    
    /// Last send time
    pub last_send: u64,
    
    /// Last receive time
    pub last_recv: u64,
    
    /// Connection time
    pub conn_time: u64,
    
    /// Ping time (in milliseconds)
    pub ping_time: Option<f64>,
    
    /// Protocol version
    pub version: u32,
    
    /// Subversion string
    pub subver: String,
    
    /// Inbound connection
    pub inbound: bool,
    
    /// Starting height
    pub start_height: u64,
    
    /// Ban score
    pub ban_score: u32,
    
    /// Sync node
    pub sync_node: bool,
}

/// Mempool status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolStatus {
    /// Number of transactions
    pub tx_count: u64,
    
    /// Size in bytes
    pub size: u64,
    
    /// Memory usage in bytes
    pub memory_usage: u64,
    
    /// Minimum fee for transactions
    pub min_fee_per_kb: u64,
    
    /// Maximum fee for transactions
    pub max_fee_per_kb: u64,
    
    /// Average fee for transactions
    pub avg_fee_per_kb: u64,
    
    /// Maximum ancestors
    pub max_ancestors: u16,
    
    /// Tracking full RBF
    pub fullrbf: bool,
}

/// Block information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    /// Block hash
    pub hash: String,
    
    /// Block height
    pub height: u64,
    
    /// Block version
    pub version: u32,
    
    /// Block time
    pub time: u64,
    
    /// Block median time past
    pub mediantime: u64,
    
    /// Block nonce
    pub nonce: u32,
    
    /// Block difficulty
    pub difficulty: f64,
    
    /// Previous block hash
    pub previousblockhash: String,
    
    /// Next block hash (if available)
    pub nextblockhash: Option<String>,
    
    /// Block chainwork
    pub chainwork: String,
    
    /// Number of transactions
    pub tx_count: u64,
    
    /// Size in bytes
    pub size: u64,
    
    /// Weight
    pub weight: u64,
    
    /// Stripped size
    pub strippedsize: u64,
    
    /// Block Merkle root
    pub merkleroot: String,
    
    /// Block bits
    pub bits: String,
    
    /// Whether this block is valid
    pub valid: bool,
    
    /// Confirmations
    pub confirmations: u64,
}

/// Transaction information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    /// Transaction ID
    pub txid: String,
    
    /// Transaction hash
    pub hash: String,
    
    /// Transaction version
    pub version: u32,
    
    /// Size in bytes
    pub size: u64,
    
    /// Virtual size
    pub vsize: u64,
    
    /// Weight
    pub weight: u64,
    
    /// Locktime
    pub locktime: u32,
    
    /// Timestamp (if mined)
    pub timestamp: Option<u64>,
    
    /// Block hash (if mined)
    pub blockhash: Option<String>,
    
    /// Block height (if mined)
    pub blockheight: Option<u64>,
    
    /// Confirmations (if mined)
    pub confirmations: Option<u64>,
    
    /// Fee
    pub fee: Option<u64>,
    
    /// Fee per vbyte
    pub fee_per_vbyte: Option<f64>,
    
    /// RBF status
    pub rbf: bool,
}

/// Blockchain port interface for node communication
#[async_trait]
pub trait NodePort {
    /// Get the current blockchain state
    async fn get_blockchain_state(&self) -> Result<BlockchainState, BlockchainError>;
    
    /// Get blockchain metrics
    async fn get_metrics(&self) -> Result<BlockchainMetrics, BlockchainError>;
    
    /// Get information about a block by hash
    async fn get_block_by_hash(&self, hash: &str) -> Result<BlockInfo, BlockchainError>;
    
    /// Get information about a block by height
    async fn get_block_by_height(&self, height: u64) -> Result<BlockInfo, BlockchainError>;
    
    /// Get raw block data
    async fn get_raw_block(&self, hash: &str) -> Result<Vec<u8>, BlockchainError>;
    
    /// Get information about a transaction
    async fn get_transaction(&self, txid: &str) -> Result<TransactionInfo, BlockchainError>;
    
    /// Get raw transaction data
    async fn get_raw_transaction(&self, txid: &str) -> Result<Vec<u8>, BlockchainError>;
    
    /// Broadcast a raw transaction
    async fn broadcast_transaction(&self, tx_data: &[u8]) -> Result<String, BlockchainError>;
    
    /// Get mempool status
    async fn get_mempool_status(&self) -> Result<MempoolStatus, BlockchainError>;
    
    /// Get transaction IDs in the mempool
    async fn get_mempool_transactions(&self) -> Result<Vec<String>, BlockchainError>;
    
    /// Get fee estimates for different confirmation targets
    async fn estimate_fee(&self, confirmation_target: u16) -> Result<u64, BlockchainError>;
    
    /// Get information about connected peers
    async fn get_peer_info(&self) -> Result<Vec<PeerInfo>, BlockchainError>;
    
    /// Get current network difficulty
    async fn get_difficulty(&self) -> Result<f64, BlockchainError>;
    
    /// Get network hash rate
    async fn get_network_hashrate(&self) -> Result<f64, BlockchainError>;
    
    /// Check if a transaction is in the mempool
    async fn is_in_mempool(&self, txid: &str) -> Result<bool, BlockchainError>;
    
    /// Get UTXO information
    async fn get_utxo(&self, txid: &str, vout: u32) -> Result<Option<UtxoInfo>, BlockchainError>;
}

/// UTXO information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtxoInfo {
    /// Transaction ID
    pub txid: String,
    
    /// Output index
    pub vout: u32,
    
    /// Amount in satoshis
    pub amount: u64,
    
    /// Script pubkey (hex)
    pub script_pubkey: String,
    
    /// Script pubkey asm
    pub script_pubkey_asm: String,
    
    /// Script type
    pub script_type: String,
    
    /// Confirmations
    pub confirmations: u64,
    
    /// Coinbase transaction
    pub coinbase: bool,
}

/// Blockchain port interface for wallet operations
#[async_trait]
pub trait WalletPort {
    /// Create a transaction
    async fn create_transaction(&self, params: TransactionParams) -> Result<String, BlockchainError>;
    
    /// Sign a transaction
    async fn sign_transaction(&self, tx: &str, privkeys: Option<Vec<String>>) -> Result<String, BlockchainError>;
    
    /// Analyze a transaction
    async fn analyze_transaction(&self, tx: &str) -> Result<TransactionAnalysis, BlockchainError>;
    
    /// Get balance for an address
    async fn get_address_balance(&self, address: &str) -> Result<AddressBalance, BlockchainError>;
    
    /// Get transactions for an address
    async fn get_address_transactions(&self, address: &str, limit: Option<u32>) -> Result<Vec<String>, BlockchainError>;
    
    /// Get UTXO set for an address
    async fn get_address_utxos(&self, address: &str) -> Result<Vec<UtxoInfo>, BlockchainError>;
    
    /// Import a private key
    async fn import_private_key(&self, privkey: &str) -> Result<(), BlockchainError>;
    
    /// Export private keys
    async fn export_private_keys(&self) -> Result<Vec<String>, BlockchainError>;
    
    /// Create a raw transaction
    async fn create_raw_transaction(&self, inputs: Vec<TxInput>, outputs: HashMap<String, f64>) -> Result<String, BlockchainError>;
}

/// Transaction input for creating transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxInput {
    /// Transaction ID
    pub txid: String,
    
    /// Output index
    pub vout: u32,
    
    /// Sequence number
    pub sequence: Option<u32>,
}

/// Transaction parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionParams {
    /// Transaction inputs
    pub inputs: Option<Vec<TxInput>>,
    
    /// Transaction outputs as address -> amount
    pub outputs: HashMap<String, f64>,
    
    /// Fee rate in satoshis per vbyte
    pub fee_rate: Option<u64>,
    
    /// Lock time
    pub locktime: Option<u32>,
    
    /// Replace-by-fee
    pub rbf: Option<bool>,
    
    /// Change address
    pub change_address: Option<String>,
    
    /// Data to include in OP_RETURN
    pub op_return_data: Option<Vec<u8>>,
}

/// Transaction analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionAnalysis {
    /// Transaction ID
    pub txid: String,
    
    /// Transaction size
    pub size: u64,
    
    /// Transaction virtual size
    pub vsize: u64,
    
    /// Transaction weight
    pub weight: u64,
    
    /// Transaction fee
    pub fee: Option<u64>,
    
    /// Fee rate in satoshis per vbyte
    pub fee_rate: Option<f64>,
    
    /// Transaction inputs
    pub inputs: Vec<TxAnalysisInput>,
    
    /// Transaction outputs
    pub outputs: Vec<TxAnalysisOutput>,
    
    /// Total input amount
    pub input_amount: Option<u64>,
    
    /// Total output amount
    pub output_amount: u64,
    
    /// Is this a coinbase transaction
    pub is_coinbase: bool,
    
    /// Does this transaction signal RBF
    pub is_rbf: bool,
    
    /// Is this transaction completely signed
    pub is_fully_signed: bool,
    
    /// Transaction lock time
    pub locktime: u32,
    
    /// Number of signature operations
    pub sigops: u32,
}

/// Transaction input details in analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxAnalysisInput {
    /// Transaction ID
    pub txid: String,
    
    /// Output index
    pub vout: u32,
    
    /// Input amount (if available)
    pub amount: Option<u64>,
    
    /// Address (if available)
    pub address: Option<String>,
    
    /// Script signature (hex)
    pub script_sig: String,
    
    /// Witness data (if any)
    pub witness: Option<Vec<String>>,
    
    /// Sequence number
    pub sequence: u32,
}

/// Transaction output details in analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxAnalysisOutput {
    /// Output index
    pub n: u32,
    
    /// Output amount
    pub amount: u64,
    
    /// Address (if available)
    pub address: Option<String>,
    
    /// Script pubkey (hex)
    pub script_pubkey: String,
    
    /// Script pubkey type
    pub script_type: String,
    
    /// Is this an OP_RETURN output
    pub is_op_return: bool,
}

/// Address balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressBalance {
    /// Confirmed balance
    pub confirmed: u64,
    
    /// Unconfirmed balance
    pub unconfirmed: u64,
    
    /// Total balance
    pub total: u64,
    
    /// Number of transactions
    pub tx_count: u64,
    
    /// Number of unspent outputs
    pub utxo_count: u64,
}

/// Blockchain port interface for smart contract execution
#[async_trait]
pub trait SmartContractPort {
    /// Deploy a contract
    async fn deploy_contract(&self, bytecode: &str, abi: &str, params: &[String]) -> Result<String, BlockchainError>;
    
    /// Call a contract method (read-only)
    async fn call_contract(&self, address: &str, abi: &str, method: &str, params: &[String]) -> Result<String, BlockchainError>;
    
    /// Send a transaction to a contract
    async fn send_to_contract(&self, address: &str, abi: &str, method: &str, params: &[String], value: Option<u64>) -> Result<String, BlockchainError>;
    
    /// Get contract events
    async fn get_contract_events(&self, address: &str, abi: &str, event: &str, from_block: Option<u64>, to_block: Option<u64>) -> Result<Vec<ContractEvent>, BlockchainError>;
    
    /// Get contract bytecode
    async fn get_contract_bytecode(&self, address: &str) -> Result<String, BlockchainError>;
    
    /// Get contract balance
    async fn get_contract_balance(&self, address: &str) -> Result<u64, BlockchainError>;
}

/// Contract event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractEvent {
    /// Contract address
    pub address: String,
    
    /// Block hash
    pub block_hash: String,
    
    /// Block number
    pub block_number: u64,
    
    /// Transaction hash
    pub transaction_hash: String,
    
    /// Transaction index
    pub transaction_index: u32,
    
    /// Log index
    pub log_index: u32,
    
    /// Event name
    pub event: String,
    
    /// Event parameters
    pub parameters: HashMap<String, String>,
}

/// Blockchain port interface for metrics collection
#[async_trait]
pub trait MetricsPort {
    /// Start collecting metrics
    async fn start_metrics(&self) -> Result<(), BlockchainError>;
    
    /// Stop collecting metrics
    async fn stop_metrics(&self) -> Result<(), BlockchainError>;
    
    /// Get the latest metrics
    async fn get_latest_metrics(&self) -> Result<BlockchainMetrics, BlockchainError>;
    
    /// Get historical metrics
    async fn get_historical_metrics(&self, start_time: u64, end_time: u64, interval: u64) -> Result<Vec<BlockchainMetrics>, BlockchainError>;
    
    /// Get specific metric
    async fn get_metric(&self, name: &str, start_time: u64, end_time: u64, interval: u64) -> Result<Vec<(u64, f64)>, BlockchainError>;
    
    /// Set metric alert
    async fn set_metric_alert(&self, name: &str, threshold: f64, comparison: AlertComparison) -> Result<(), BlockchainError>;
    
    /// Remove metric alert
    async fn remove_metric_alert(&self, name: &str) -> Result<(), BlockchainError>;
}

/// Alert comparison type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertComparison {
    /// Greater than
    GreaterThan,
    
    /// Greater than or equal
    GreaterThanOrEqual,
    
    /// Less than
    LessThan,
    
    /// Less than or equal
    LessThanOrEqual,
    
    /// Equal to
    Equal,
    
    /// Not equal to
    NotEqual,
}

/// Blockchain port interface for security monitoring
#[async_trait]
pub trait SecurityPort {
    /// Check if a block is valid
    async fn is_block_valid(&self, hash: &str) -> Result<bool, BlockchainError>;
    
    /// Get blockchain difficulty history
    async fn get_difficulty_history(&self, blocks: u32) -> Result<Vec<(u64, f64)>, BlockchainError>;
    
    /// Detect potential chain split
    async fn detect_chain_split(&self) -> Result<Option<ChainSplitInfo>, BlockchainError>;
    
    /// Detect unusual transaction patterns
    async fn detect_unusual_transactions(&self) -> Result<Vec<UnusualTransaction>, BlockchainError>;
    
    /// Monitor mempool for fee spikes
    async fn monitor_fee_spikes(&self, threshold: f64) -> Result<bool, BlockchainError>;
    
    /// Monitor network hashrate changes
    async fn monitor_hashrate_change(&self, window: u32, threshold: f64) -> Result<Option<f64>, BlockchainError>;
    
    /// Check if an address is known to be malicious
    async fn is_address_malicious(&self, address: &str) -> Result<bool, BlockchainError>;
    
    /// Report security incident
    async fn report_security_incident(&self, incident_type: &str, details: &str) -> Result<(), BlockchainError>;
}

/// Chain split information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainSplitInfo {
    /// Height of the split
    pub split_height: u64,
    
    /// Main chain block hash
    pub main_chain_hash: String,
    
    /// Split chain block hash
    pub split_chain_hash: String,
    
    /// Length of the main chain from split
    pub main_chain_length: u32,
    
    /// Length of the split chain from split
    pub split_chain_length: u32,
    
    /// Work difference (as ratio)
    pub work_difference: f64,
}

/// Unusual transaction information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnusualTransaction {
    /// Transaction ID
    pub txid: String,
    
    /// Reason for flagging
    pub reason: String,
    
    /// Severity (0-100)
    pub severity: u8,
    
    /// Additional details
    pub details: HashMap<String, String>,
}

/// Core application logic
pub struct BlockchainCore {
    /// Available adapters
    adapters: HashMap<String, Box<dyn BlockchainAdapter>>,
    
    /// Active adapter
    active_adapter: String,
    
    /// Latest metrics
    metrics: Arc<Mutex<Option<BlockchainMetrics>>>,
    
    /// Latest blockchain state
    state: Arc<Mutex<Option<BlockchainState>>>,
    
    /// Metrics collection interval
    metrics_interval: Duration,
    
    /// Sync interval
    sync_interval: Duration,
}

impl BlockchainCore {
    /// Create a new blockchain core
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
            active_adapter: String::new(),
            metrics: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(None)),
            metrics_interval: Duration::from_secs(60),
            sync_interval: Duration::from_secs(10),
        }
    }
    
    /// Register a blockchain adapter
    pub fn register_adapter(&mut self, name: &str, adapter: Box<dyn BlockchainAdapter>) {
        self.adapters.insert(name.to_string(), adapter);
        
        // If this is the first adapter, make it active
        if self.active_adapter.is_empty() {
            self.active_adapter = name.to_string();
        }
    }
    
    /// Set the active adapter
    pub fn set_active_adapter(&mut self, name: &str) -> Result<(), BlockchainError> {
        if !self.adapters.contains_key(name) {
            return Err(BlockchainError::ConfigError(format!("Adapter '{}' not found", name)));
        }
        
        self.active_adapter = name.to_string();
        Ok(())
    }
    
    /// Get the active adapter
    pub fn get_active_adapter(&self) -> Result<&Box<dyn BlockchainAdapter>, BlockchainError> {
        self.adapters.get(&self.active_adapter)
            .ok_or_else(|| BlockchainError::ConfigError("No active adapter set".to_string()))
    }
    
    /// Initialize the blockchain core
    pub async fn init(&self) -> Result<(), BlockchainError> {
        let adapter = self.get_active_adapter()?;
        adapter.init().await?;
        
        // Get initial state
        let state = adapter.get_blockchain_state().await?;
        
        // Set initial state
        let mut state_lock = self.state.lock()
            .map_err(|_| BlockchainError::InternalError("Failed to lock state mutex".to_string()))?;
        *state_lock = Some(state);
        
        // Get initial metrics
        let metrics = adapter.get_metrics().await?;
        
        // Set initial metrics
        let mut metrics_lock = self.metrics.lock()
            .map_err(|_| BlockchainError::InternalError("Failed to lock metrics mutex".to_string()))?;
        *metrics_lock = Some(metrics);
        
        Ok(())
    }
    
    /// Start background synchronization
    pub async fn start_sync(&self) -> Result<(), BlockchainError> {
        let adapter = self.get_active_adapter()?;
        
        // Clone Arc references for the closure
        let state_arc = self.state.clone();
        let metrics_arc = self.metrics.clone();
        let sync_interval = self.sync_interval;
        let metrics_interval = self.metrics_interval;
        
        // Get adapter as Arc for the background task
        let adapter_arc = Arc::new(adapter);
        
        // Spawn background task for sync
        tokio::spawn(async move {
            let mut last_metrics_update = SystemTime::now();
            
            loop {
                // Update blockchain state
                match adapter_arc.get_blockchain_state().await {
                    Ok(new_state) => {
                        let mut state_lock = match state_arc.lock() {
                            Ok(lock) => lock,
                            Err(e) => {
                                error!("Failed to lock state mutex: {}", e);
                                continue;
                            }
                        };
                        *state_lock = Some(new_state);
                    },
                    Err(e) => {
                        error!("Failed to update blockchain state: {}", e);
                    }
                }
                
                // Update metrics less frequently
                if last_metrics_update.elapsed().unwrap_or(Duration::from_secs(0)) >= metrics_interval {
                    match adapter_arc.get_metrics().await {
                        Ok(new_metrics) => {
                            let mut metrics_lock = match metrics_arc.lock() {
                                Ok(lock) => lock,
                                Err(e) => {
                                    error!("Failed to lock metrics mutex: {}", e);
                                    continue;
                                }
                            };
                            *metrics_lock = Some(new_metrics);
                            last_metrics_update = SystemTime::now();
                        },
                        Err(e) => {
                            error!("Failed to update blockchain metrics: {}", e);
                        }
                    }
                }
                
                // Sleep before next update
                tokio::time::sleep(sync_interval).await;
            }
        });
        
        Ok(())
    }
    
    /// Get the latest blockchain state
    pub fn get_latest_state(&self) -> Result<BlockchainState, BlockchainError> {
        let state_lock = self.state.lock()
            .map_err(|_| BlockchainError::InternalError("Failed to lock state mutex".to_string()))?;
        
        state_lock.clone()
            .ok_or_else(|| BlockchainError::InternalError("Blockchain state not initialized".to_string()))
    }
    
    /// Get the latest blockchain metrics
    pub fn get_latest_metrics(&self) -> Result<BlockchainMetrics, BlockchainError> {
        let metrics_lock = self.metrics.lock()
            .map_err(|_| BlockchainError::InternalError("Failed to lock metrics mutex".to_string()))?;
        
        metrics_lock.clone()
            .ok_or_else(|| BlockchainError::InternalError("Blockchain metrics not initialized".to_string()))
    }
}

/// Blockchain adapter interface
#[async_trait]
pub trait BlockchainAdapter: NodePort + WalletPort + SmartContractPort + MetricsPort + SecurityPort + Send + Sync {
    /// Initialize the adapter
    async fn init(&self) -> Result<(), BlockchainError>;
    
    /// Get adapter name
    fn get_name(&self) -> &str;
    
    /// Get supported blockchain
    fn get_blockchain(&self) -> &str;
    
    /// Get adapter version
    fn get_version(&self) -> &str;
    
    /// Get supported features
    fn get_features(&self) -> Vec<String>;
    
    /// Check if a feature is supported
    fn supports_feature(&self, feature: &str) -> bool {
        self.get_features().iter().any(|f| f == feature)
    }
}

/// Bitcoin adapter implementation
pub mod bitcoin;

/// Ethereum adapter implementation
pub mod ethereum;

/// Polkadot adapter implementation
pub mod polkadot;

/// General utilities
pub mod utils; 
