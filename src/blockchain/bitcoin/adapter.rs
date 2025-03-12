//! Bitcoin blockchain adapter implementation
//!
//! This module provides a Bitcoin adapter implementation for the blockchain interface
//! using the hexagonal architecture pattern.
//! [AIR-1][AIS-1][AIM-1][AIP-1][RES-1]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::str::FromStr;

use async_trait::async_trait;
use bitcoin::{
    Address, Amount, Block, Network, Transaction, Txid, blockdata::constants::genesis_block,
    consensus::{encode, Decodable}, util::psbt::PartiallySignedTransaction,
};
use bitcoin::secp256k1::{Secp256k1, Message, PublicKey, SecretKey};
use log::{debug, info, warn, error, trace};
use serde::{Serialize, Deserialize};
use thiserror::Error;
use tokio::sync::RwLock;

use crate::blockchain::{
    BlockchainAdapter, BlockchainError, BlockchainMetrics, BlockchainState,
    NodePort, WalletPort, SmartContractPort, MetricsPort, SecurityPort,
    PeerInfo, MempoolStatus, BlockInfo, TransactionInfo, UtxoInfo,
    TransactionParams, TransactionAnalysis, AddressBalance, TxInput,
    ContractEvent, AlertComparison, ChainSplitInfo, UnusualTransaction,
};
use crate::bitcoin::rpc::{BitcoinRpcClient, RpcError};

/// Bitcoin adapter error type
#[derive(Error, Debug)]
pub enum BitcoinAdapterError {
    /// RPC error
    #[error("RPC error: {0}")]
    RpcError(#[from] RpcError),

    /// Bitcoin library error
    #[error("Bitcoin library error: {0}")]
    BitcoinError(String),

    /// Encoding error
    #[error("Encoding error: {0}")]
    EncodingError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Not supported feature
    #[error("Not supported in Bitcoin: {0}")]
    NotSupported(String),

    /// Metrics error
    #[error("Metrics error: {0}")]
    MetricsError(String),

    /// Security monitoring error
    #[error("Security monitoring error: {0}")]
    SecurityError(String),

    /// Invalid parameter
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

impl From<BitcoinAdapterError> for BlockchainError {
    fn from(err: BitcoinAdapterError) -> Self {
        match err {
            BitcoinAdapterError::RpcError(e) => BlockchainError::RpcError(e.to_string()),
            BitcoinAdapterError::BitcoinError(e) => BlockchainError::InternalError(e),
            BitcoinAdapterError::EncodingError(e) => BlockchainError::SerializationError(e),
            BitcoinAdapterError::ConfigError(e) => BlockchainError::ConfigError(e),
            BitcoinAdapterError::NotSupported(e) => BlockchainError::InternalError(e),
            BitcoinAdapterError::MetricsError(e) => BlockchainError::InternalError(e),
            BitcoinAdapterError::SecurityError(e) => BlockchainError::InternalError(e),
            BitcoinAdapterError::InvalidParameter(e) => BlockchainError::ValidationError(e),
        }
    }
}

/// Bitcoin adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinAdapterConfig {
    /// Bitcoin network (mainnet, testnet, regtest)
    pub network: Network,

    /// Bitcoin RPC URL
    pub rpc_url: String,

    /// Bitcoin RPC username
    pub rpc_user: String,

    /// Bitcoin RPC password
    pub rpc_password: String,

    /// Connection timeout in seconds
    pub timeout: u64,

    /// Metrics collection interval in seconds
    pub metrics_interval: u64,

    /// Security monitoring interval in seconds
    pub security_interval: u64,

    /// Mempool monitoring interval in seconds
    pub mempool_interval: u64,

    /// Fee estimation blocks target range
    pub fee_estimation_blocks: Vec<u16>,

    /// Enable security monitoring
    pub enable_security_monitoring: bool,

    /// Chain split detection threshold (in blocks)
    pub chain_split_threshold: u32,

    /// Fee spike threshold (multiplier)
    pub fee_spike_threshold: f64,

    /// Maximum UTXO cache size
    pub max_utxo_cache_size: usize,

    /// Maximum block cache size
    pub max_block_cache_size: usize,

    /// Maximum transaction cache size 
    pub max_tx_cache_size: usize,
}

impl Default for BitcoinAdapterConfig {
    fn default() -> Self {
        Self {
            network: Network::Testnet,
            rpc_url: "http://localhost:18332".to_string(),
            rpc_user: "bitcoin".to_string(),
            rpc_password: "password".to_string(),
            timeout: 30,
            metrics_interval: 60,
            security_interval: 300,
            mempool_interval: 30,
            fee_estimation_blocks: vec![1, 2, 6, 36, 144, 504, 1008],
            enable_security_monitoring: true,
            chain_split_threshold: 6,
            fee_spike_threshold: 3.0,
            max_utxo_cache_size: 10_000,
            max_block_cache_size: 1_000,
            max_tx_cache_size: 10_000,
        }
    }
}

/// Bitcoin adapter implementation
pub struct BitcoinAdapter {
    /// Configuration
    config: BitcoinAdapterConfig,

    /// RPC client
    rpc_client: Arc<BitcoinRpcClient>,

    /// Latest blockchain state
    state: RwLock<Option<BlockchainState>>,

    /// Latest metrics
    metrics: RwLock<Option<BlockchainMetrics>>,

    /// Latest mempool status
    mempool: RwLock<Option<MempoolStatus>>,

    /// Blocks cache (hash -> BlockInfo)
    blocks_cache: Mutex<HashMap<String, BlockInfo>>,

    /// Transaction cache (txid -> TransactionInfo)
    tx_cache: Mutex<HashMap<String, TransactionInfo>>,

    /// UTXO cache (txid:vout -> UtxoInfo)
    utxo_cache: Mutex<HashMap<String, UtxoInfo>>,

    /// Fee estimates cache
    fee_estimates: RwLock<HashMap<u16, u64>>,

    /// Unusual transactions detected
    unusual_txs: RwLock<Vec<UnusualTransaction>>,

    /// Security alerts
    security_alerts: RwLock<Vec<String>>,

    /// Is the metrics collection running
    metrics_running: RwLock<bool>,

    /// Is the security monitoring running
    security_running: RwLock<bool>,
}

impl BitcoinAdapter {
    /// Create a new Bitcoin adapter
    pub async fn new(config: BitcoinAdapterConfig) -> Result<Self, BitcoinAdapterError> {
        // Create RPC client
        let rpc_client = Arc::new(BitcoinRpcClient::new(
            &config.rpc_url,
            &config.rpc_user,
            &config.rpc_password,
            Duration::from_secs(config.timeout),
        )?);

        // Check connection
        let blockchain_info = rpc_client.get_blockchain_info().await?;
        
        // Verify network matches configuration
        let rpc_network = match blockchain_info.chain.as_str() {
            "main" => Network::Bitcoin,
            "test" => Network::Testnet,
            "regtest" => Network::Regtest,
            "signet" => Network::Signet,
            other => return Err(BitcoinAdapterError::ConfigError(
                format!("Unknown network: {}", other)
            )),
        };
        
        if rpc_network != config.network {
            return Err(BitcoinAdapterError::ConfigError(
                format!("Configured network ({:?}) doesn't match node network ({:?})",
                    config.network, rpc_network)
            ));
        }

        // Create adapter
        let adapter = Self {
            config,
            rpc_client,
            state: RwLock::new(None),
            metrics: RwLock::new(None),
            mempool: RwLock::new(None),
            blocks_cache: Mutex::new(HashMap::new()),
            tx_cache: Mutex::new(HashMap::new()),
            utxo_cache: Mutex::new(HashMap::new()),
            fee_estimates: RwLock::new(HashMap::new()),
            unusual_txs: RwLock::new(Vec::new()),
            security_alerts: RwLock::new(Vec::new()),
            metrics_running: RwLock::new(false),
            security_running: RwLock::new(false),
        };

        Ok(adapter)
    }

    /// Convert a Bitcoin block to a BlockInfo
    async fn convert_block_to_info(&self, hash: &str, height: u64) -> Result<BlockInfo, BitcoinAdapterError> {
        let block_data = self.rpc_client.get_block(hash).await?;
        let tx_count = block_data.tx.len() as u64;
        
        // Get the next block hash if available
        let next_hash = if height > 0 {
            match self.rpc_client.get_block_hash(height + 1).await {
                Ok(hash) => Some(hash),
                Err(_) => None,
            }
        } else {
            None
        };

        // Get chain tip info to calculate confirmations
        let chain_info = self.rpc_client.get_blockchain_info().await?;
        let confirmations = if chain_info.blocks >= height {
            chain_info.blocks - height + 1
        } else {
            0
        };

        let block_info = BlockInfo {
            hash: hash.to_string(),
            height,
            version: block_data.version as u32,
            time: block_data.time as u64,
            mediantime: block_data.mediantime as u64,
            nonce: block_data.nonce,
            difficulty: block_data.difficulty,
            previousblockhash: block_data.previousblockhash.unwrap_or_default(),
            nextblockhash: next_hash,
            chainwork: block_data.chainwork.unwrap_or_default(),
            tx_count,
            size: block_data.size as u64,
            weight: block_data.weight as u64,
            strippedsize: block_data.strippedsize as u64,
            merkleroot: block_data.merkleroot,
            bits: block_data.bits,
            valid: true, // Assume valid if returned by node
            confirmations,
        };

        // Cache the block info
        let mut blocks_cache = self.blocks_cache.lock().unwrap();
        blocks_cache.insert(hash.to_string(), block_info.clone());
        
        // Limit cache size
        if blocks_cache.len() > self.config.max_block_cache_size {
            // Remove random entries to get back to the limit
            let overflow = blocks_cache.len() - self.config.max_block_cache_size;
            let keys: Vec<String> = blocks_cache.keys().take(overflow).cloned().collect();
            for key in keys {
                blocks_cache.remove(&key);
            }
        }

        Ok(block_info)
    }

    /// Convert a Bitcoin transaction to a TransactionInfo
    async fn convert_tx_to_info(&self, txid: &str) -> Result<TransactionInfo, BitcoinAdapterError> {
        let tx_data = self.rpc_client.get_raw_transaction_verbose(txid).await?;
        
        // Calculate fee if we have input values
        let mut fee = None;
        let mut fee_per_vbyte = None;
        
        let mut input_sum = 0.0;
        let mut has_all_inputs = true;
        
        for input in &tx_data.vin {
            if input.is_coinbase() {
                has_all_inputs = false;
                break;
            }
            
            if let (Some(prev_txid), Some(prev_vout)) = (&input.txid, input.vout) {
                match self.rpc_client.get_raw_transaction_verbose(prev_txid).await {
                    Ok(prev_tx) => {
                        if let Some(prev_output) = prev_tx.vout.get(*prev_vout as usize) {
                            input_sum += prev_output.value;
                        } else {
                            has_all_inputs = false;
                            break;
                        }
                    },
                    Err(_) => {
                        has_all_inputs = false;
                        break;
                    }
                }
            } else {
                has_all_inputs = false;
                break;
            }
        }
        
        let output_sum: f64 = tx_data.vout.iter().map(|o| o.value).sum();
        
        if has_all_inputs {
            let fee_btc = input_sum - output_sum;
            if fee_btc >= 0.0 {
                let fee_sats = (fee_btc * 100_000_000.0).round() as u64;
                fee = Some(fee_sats);
                
                if tx_data.vsize > 0 {
                    fee_per_vbyte = Some(fee_sats as f64 / tx_data.vsize as f64);
                }
            }
        }
        
        // Determine if this is an RBF transaction
        let rbf = tx_data.vin.iter().any(|input| {
            input.sequence < 0xFFFFFFFE
        });
        
        let tx_info = TransactionInfo {
            txid: txid.to_string(),
            hash: tx_data.hash.unwrap_or_else(|| txid.to_string()),
            version: tx_data.version as u32,
            size: tx_data.size as u64,
            vsize: tx_data.vsize as u64,
            weight: tx_data.weight as u64,
            locktime: tx_data.locktime as u32,
            timestamp: tx_data.time.map(|t| t as u64),
            blockhash: tx_data.blockhash,
            blockheight: tx_data.blockheight.map(|h| h as u64),
            confirmations: tx_data.confirmations.map(|c| c as u64),
            fee,
            fee_per_vbyte,
            rbf,
        };
        
        // Cache the transaction info
        let mut tx_cache = self.tx_cache.lock().unwrap();
        tx_cache.insert(txid.to_string(), tx_info.clone());
        
        // Limit cache size
        if tx_cache.len() > self.config.max_tx_cache_size {
            // Remove random entries to get back to the limit
            let overflow = tx_cache.len() - self.config.max_tx_cache_size;
            let keys: Vec<String> = tx_cache.keys().take(overflow).cloned().collect();
            for key in keys {
                tx_cache.remove(&key);
            }
        }
        
        Ok(tx_info)
    }

    /// Get UTXO information by txid and vout
    async fn get_utxo_info(&self, txid: &str, vout: u32) -> Result<Option<UtxoInfo>, BitcoinAdapterError> {
        let cache_key = format!("{}:{}", txid, vout);
        
        // Check cache first
        {
            let utxo_cache = self.utxo_cache.lock().unwrap();
            if let Some(utxo) = utxo_cache.get(&cache_key) {
                return Ok(Some(utxo.clone()));
            }
        }
        
        // Try to get from the node
        match self.rpc_client.get_tx_out(txid, vout, true).await {
            Ok(Some(tx_out)) => {
                let script_type = if tx_out.script_pub_key.type_.is_some() {
                    tx_out.script_pub_key.type_.unwrap_or_else(|| "unknown".to_string())
                } else {
                    "unknown".to_string()
                };
                
                let utxo = UtxoInfo {
                    txid: txid.to_string(),
                    vout,
                    amount: (tx_out.value * 100_000_000.0).round() as u64,
                    script_pubkey: tx_out.script_pub_key.hex,
                    script_pubkey_asm: tx_out.script_pub_key.asm,
                    script_type,
                    confirmations: tx_out.confirmations as u64,
                    coinbase: tx_out.coinbase,
                };
                
                // Cache the UTXO
                let mut utxo_cache = self.utxo_cache.lock().unwrap();
                utxo_cache.insert(cache_key, utxo.clone());
                
                // Limit cache size
                if utxo_cache.len() > self.config.max_utxo_cache_size {
                    // Remove random entries to get back to the limit
                    let overflow = utxo_cache.len() - self.config.max_utxo_cache_size;
                    let keys: Vec<String> = utxo_cache.keys().take(overflow).cloned().collect();
                    for key in keys {
                        utxo_cache.remove(&key);
                    }
                }
                
                Ok(Some(utxo))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(BitcoinAdapterError::RpcError(e)),
        }
    }

    /// Update the blockchain state
    async fn update_blockchain_state(&self) -> Result<BlockchainState, BitcoinAdapterError> {
        let blockchain_info = self.rpc_client.get_blockchain_info().await?;
        let network_info = self.rpc_client.get_network_info().await?;
        
        // Get the best block hash
        let best_block_hash = blockchain_info.best_block_hash.clone();
        
        // Get the best block header
        let best_block_header = self.rpc_client.get_block_header(&best_block_hash).await?;
        
        let state = BlockchainState {
            chain_id: format!("bitcoin-{}", blockchain_info.chain),
            network: blockchain_info.chain,
            protocol_version: network_info.protocol_version as u32,
            best_block_hash,
            best_block_height: blockchain_info.blocks as u64,
            median_time_past: best_block_header.mediantime as u64,
            initial_block_download: blockchain_info.initial_block_download,
            sync_progress: blockchain_info.verification_progress,
            chain_work: blockchain_info.chain_work,
            size_on_disk: blockchain_info.size_on_disk as u64,
            connection_count: network_info.connections as u32,
            verification_progress: blockchain_info.verification_progress,
            pruned: blockchain_info.pruned,
            prune_height: if blockchain_info.pruned { blockchain_info.prune_height.map(|h| h as u64) } else { None },
            last_checkpoint: blockchain_info.softforks.values()
                .map(|f| f.height)
                .max()
                .map(|h| h as u64),
            warnings: if !blockchain_info.warnings.is_empty() {
                Some(blockchain_info.warnings)
            } else {
                None
            },
        };
        
        // Update the state in the adapter
        *self.state.write().await = Some(state.clone());
        
        Ok(state)
    }

    /// Update blockchain metrics
    async fn update_blockchain_metrics(&self) -> Result<BlockchainMetrics, BitcoinAdapterError> {
        let blockchain_info = self.rpc_client.get_blockchain_info().await?;
        let mempool_info = self.rpc_client.get_mempool_info().await?;
        let tx_stats = self.rpc_client.get_chain_tx_stats(None).await?;
        let hashps = self.rpc_client.get_network_hashps(120, -1).await?;
        
        // Get fee estimates for different confirmation targets
        let mut fee_estimates = HashMap::new();
        for blocks in &self.config.fee_estimation_blocks {
            match self.rpc_client.estimate_smart_fee(*blocks).await {
                Ok(estimate) => {
                    if let Some(fee_rate) = estimate.fee_rate {
                        // Convert to satoshis per byte
                        let fee_rate_sat_per_byte = (fee_rate * 100_000_000.0 / 1000.0).round() as u64;
                        fee_estimates.insert(*blocks, fee_rate_sat_per_byte);
                    }
                },
                Err(e) => {
                    warn!("Failed to get fee estimate for {} blocks: {}", blocks, e);
                }
            }
        }
        
        let metrics = BlockchainMetrics {
            block_count: blockchain_info.blocks as u64,
            tx_count: tx_stats.tx_count as u64,
            utxo_set_size: match self.rpc_client.get_blockchain_stats().await {
                Ok(stats) => stats.utxo_set_size as u64,
                Err(_) => 0,
            },
            difficulty: blockchain_info.difficulty,
            hash_rate: hashps,
            network_weight: None, // Not applicable for Bitcoin
            block_propagation_time: 0, // Not directly available from Bitcoin RPC
            mempool_size: mempool_info.bytes as u64,
            mempool_tx_count: mempool_info.size as u64,
            fee_estimates,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        
        // Update the metrics in the adapter
        *self.metrics.write().await = Some(metrics.clone());
        
        // Update fee estimates cache
        let mut fee_estimates_lock = self.fee_estimates.write().await;
        *fee_estimates_lock = fee_estimates;
        
        Ok(metrics)
    }

    /// Update mempool status
    async fn update_mempool_status(&self) -> Result<MempoolStatus, BitcoinAdapterError> {
        let mempool_info = self.rpc_client.get_mempool_info().await?;
        
        // Get fee stats from the mempool
        let mempool_entries = self.rpc_client.get_raw_mempool_verbose().await?;
        
        let mut fees: Vec<u64> = Vec::new();
        for (_, entry) in mempool_entries {
            // Convert BTC/kB to sat/kB
            let fee_per_kb = (entry.fee / entry.size as f64 * 1000.0 * 100_000_000.0).round() as u64;
            fees.push(fee_per_kb);
        }
        
        // Calculate fee statistics
        let min_fee = fees.iter().min().copied().unwrap_or(0);
        let max_fee = fees.iter().max().copied().unwrap_or(0);
        let avg_fee = if !fees.is_empty() {
            fees.iter().sum::<u64>() / fees.len() as u64
        } else {
            0
        };
        
        let status = MempoolStatus {
            tx_count: mempool_info.size as u64,
            size: mempool_info.bytes as u64,
            memory_usage: mempool_info.usage as u64,
            min_fee_per_kb: min_fee,
            max_fee_per_kb: max_fee,
            avg_fee_per_kb: avg_fee,
            max_ancestors: mempool_info.max_mempool_ancestors as u16,
            fullrbf: mempool_info.full_rbf.unwrap_or(false),
        };
        
        // Update the mempool status in the adapter
        *self.mempool.write().await = Some(status.clone());
        
        Ok(status)
    }
}

#[async_trait]
impl NodePort for BitcoinAdapter {
    async fn get_blockchain_state(&self) -> Result<BlockchainState, BlockchainError> {
        // Check if we have a cached state
        if let Some(state) = self.state.read().await.clone() {
            return Ok(state);
        }
        
        // Otherwise update and return the state
        self.update_blockchain_state().await
            .map_err(|e| e.into())
    }
    
    async fn get_metrics(&self) -> Result<BlockchainMetrics, BlockchainError> {
        // Check if we have cached metrics
        if let Some(metrics) = self.metrics.read().await.clone() {
            return Ok(metrics);
        }
        
        // Otherwise update and return the metrics
        self.update_blockchain_metrics().await
            .map_err(|e| e.into())
    }
    
    async fn get_block_by_hash(&self, hash: &str) -> Result<BlockInfo, BlockchainError> {
        // Check if the block is in the cache
        {
            let blocks_cache = self.blocks_cache.lock().unwrap();
            if let Some(block) = blocks_cache.get(hash) {
                return Ok(block.clone());
            }
        }
        
        // Get the block height
        let height = match self.rpc_client.get_block_header(hash).await {
            Ok(header) => header.height as u64,
            Err(e) => return Err(BlockchainError::NotFoundError(format!("Block not found: {}", e))),
        };
        
        // Convert the block to BlockInfo
        self.convert_block_to_info(hash, height).await
            .map_err(|e| e.into())
    }
    
    async fn get_block_by_height(&self, height: u64) -> Result<BlockInfo, BlockchainError> {
        // Get the block hash
        let hash = match self.rpc_client.get_block_hash(height).await {
            Ok(hash) => hash,
            Err(e) => return Err(BlockchainError::NotFoundError(format!("Block not found: {}", e))),
        };
        
        // Check if the block is in the cache
        {
            let blocks_cache = self.blocks_cache.lock().unwrap();
            if let Some(block) = blocks_cache.get(&hash) {
                return Ok(block.clone());
            }
        }
        
        // Convert the block to BlockInfo
        self.convert_block_to_info(&hash, height).await
            .map_err(|e| e.into())
    }
    
    async fn get_raw_block(&self, hash: &str) -> Result<Vec<u8>, BlockchainError> {
        // Get the raw block
        let block_hex = self.rpc_client.get_block_hex(hash).await
            .map_err(|e| BlockchainError::NotFoundError(format!("Block not found: {}", e)))?;
        
        // Decode the hex string
        hex::decode(block_hex)
            .map_err(|e| BlockchainError::SerializationError(format!("Failed to decode block hex: {}", e)))
    }
    
    async fn get_transaction(&self, txid: &str) -> Result<TransactionInfo, BlockchainError> {
        // Check if the transaction is in the cache
        {
            let tx_cache = self.tx_cache.lock().unwrap();
            if let Some(tx) = tx_cache.get(txid) {
                return Ok(tx.clone());
            }
        }
        
        // Convert the transaction to TransactionInfo
        self.convert_tx_to_info(txid).await
            .map_err(|e| e.into())
    }
    
    async fn get_raw_transaction(&self, txid: &str) -> Result<Vec<u8>, BlockchainError> {
        // Get the raw transaction
        let tx_hex = self.rpc_client.get_raw_transaction(txid).await
            .map_err(|e| BlockchainError::NotFoundError(format!("Transaction not found: {}", e)))?;
        
        // Decode the hex string
        hex::decode(tx_hex)
            .map_err(|e| BlockchainError::SerializationError(format!("Failed to decode transaction hex: {}", e)))
    }
    
    async fn broadcast_transaction(&self, tx_data: &[u8]) -> Result<String, BlockchainError> {
        // Encode the transaction as hex
        let tx_hex = hex::encode(tx_data);
        
        // Broadcast the transaction
        self.rpc_client.send_raw_transaction(&tx_hex).await
            .map_err(|e| BlockchainError::TransactionError(format!("Failed to broadcast transaction: {}", e)))
    }
    
    async fn get_mempool_status(&self) -> Result<MempoolStatus, BlockchainError> {
        // Check if we have cached mempool status
        if let Some(status) = self.mempool.read().await.clone() {
            return Ok(status);
        }
        
        // Otherwise update and return the mempool status
        self.update_mempool_status().await
            .map_err(|e| e.into())
    }
    
    async fn get_mempool_transactions(&self) -> Result<Vec<String>, BlockchainError> {
        // Get the mempool transaction IDs
        self.rpc_client.get_raw_mempool().await
            .map_err(|e| BlockchainError::InternalError(format!("Failed to get mempool transactions: {}", e)))
    }
    
    async fn estimate_fee(&self, confirmation_target: u16) -> Result<u64, BlockchainError> {
        // Check if we have a cached fee estimate
        {
            let fee_estimates = self.fee_estimates.read().await;
            if let Some(fee) = fee_estimates.get(&confirmation_target) {
                return Ok(*fee);
            }
        }
        
        // Try to find the closest target
        {
            let fee_estimates = self.fee_estimates.read().await;
            if !fee_estimates.is_empty() {
                // Find the closest target
                let closest_target = fee_estimates.keys()
                    .min_by_key(|k| ((**k as i32 - confirmation_target as i32).abs()))
                    .unwrap();
                
                // Return the fee for the closest target
                return Ok(*fee_estimates.get(closest_target).unwrap());
            }
        }
        
        // Otherwise get a fresh estimate
        let estimate = self.rpc_client.estimate_smart_fee(confirmation_target).await
            .map_err(|e| BlockchainError::InternalError(format!("Failed to estimate fee: {}", e)))?;
        
        if let Some(fee_rate) = estimate.fee_rate {
            // Convert to satoshis per byte
            let fee_rate_sat_per_byte = (fee_rate * 100_000_000.0 / 1000.0).round() as u64;
            Ok(fee_rate_sat_per_byte)
        } else if let Some(errors) = estimate.errors {
            Err(BlockchainError::InternalError(format!("Fee estimation error: {}", errors.join(", "))))
        } else {
            Err(BlockchainError::InternalError("Fee estimation failed with unknown error".to_string()))
        }
    }
    
    async fn get_peer_info(&self) -> Result<Vec<PeerInfo>, BlockchainError> {
        let peers = self.rpc_client.get_peer_info().await
            .map_err(|e| BlockchainError::NetworkError(format!("Failed to get peer info: {}", e)))?;
        
        let mut result = Vec::with_capacity(peers.len());
        for peer in peers {
            result.push(PeerInfo {
                id: peer.id as u64,
                addr: peer.addr,
                services: peer.services_parsed.map(|s| s as u64).unwrap_or(0),
                last_send: peer.lastsend as u64,
                last_recv: peer.lastrecv as u64,
                conn_time: peer.conntime as u64,
                ping_time: peer.pingtime,
                version: peer.version as u32,
                subver: peer.subver,
                inbound: peer.inbound,
                start_height: peer.startingheight as u64,
                ban_score: peer.banscore as u32,
                sync_node: peer.synced_headers.is_some() && peer.synced_blocks.is_some(),
            });
        }
        
        Ok(result)
    }
    
    async fn get_difficulty(&self) -> Result<f64, BlockchainError> {
        let blockchain_info = self.rpc_client.get_blockchain_info().await
            .map_err(|e| BlockchainError::InternalError(format!("Failed to get blockchain info: {}", e)))?;
        
        Ok(blockchain_info.difficulty)
    }
    
    async fn get_network_hashrate(&self) -> Result<f64, BlockchainError> {
        let hashps = self.rpc_client.get_network_hashps(120, -1).await
            .map_err(|e| BlockchainError::InternalError(format!("Failed to get network hashrate: {}", e)))?;
        
        Ok(hashps)
    }
    
    async fn is_in_mempool(&self, txid: &str) -> Result<bool, BlockchainError> {
        let mempool = self.rpc_client.get_raw_mempool().await
            .map_err(|e| BlockchainError::InternalError(format!("Failed to get mempool: {}", e)))?;
        
        Ok(mempool.contains(&txid.to_string()))
    }
    
    async fn get_utxo(&self, txid: &str, vout: u32) -> Result<Option<UtxoInfo>, BlockchainError> {
        self.get_utxo_info(txid, vout).await
            .map_err(|e| e.into())
    }
}

#[async_trait]
impl WalletPort for BitcoinAdapter {
    async fn create_transaction(&self, params: TransactionParams) -> Result<String, BlockchainError> {
        // Validate the parameters
        if params.outputs.is_empty() {
            return Err(BlockchainError::ValidationError("No outputs provided".to_string()));
        }
        
        // Convert outputs to the format expected by the RPC client
        let mut outputs = HashMap::new();
        for (address, amount) in &params.outputs {
            outputs.insert(address.clone(), *amount);
        }
        
        // Create the raw transaction
        let mut raw_tx = if let Some(inputs) = &params.inputs {
            // Convert inputs to the format expected by the RPC client
            let mut rpc_inputs = Vec::new();
            for input in inputs {
                rpc_inputs.push(serde_json::json!({
                    "txid": input.txid,
                    "vout": input.vout
                }));
            }
            
            // Create with specific inputs
            self.rpc_client.create_raw_transaction(&rpc_inputs, &outputs, params.locktime).await
                .map_err(|e| BlockchainError::TransactionError(format!("Failed to create transaction: {}", e)))?
        } else {
            // Create with automatic input selection (fund raw transaction)
            let raw_tx = self.rpc_client.create_raw_transaction(&[], &outputs, params.locktime).await
                .map_err(|e| BlockchainError::TransactionError(format!("Failed to create transaction: {}", e)))?;
            
            // Create options for funding
            let mut options = serde_json::Map::new();
            if let Some(fee_rate) = params.fee_rate {
                options.insert("fee_rate".to_string(), serde_json::json!(fee_rate));
            }
            if let Some(change_address) = &params.change_address {
                options.insert("changeAddress".to_string(), serde_json::json!(change_address));
            }
            if let Some(rbf) = params.rbf {
                options.insert("replaceable".to_string(), serde_json::json!(rbf));
            }
            
            // Fund the transaction
            let fund_result = self.rpc_client.fund_raw_transaction(&raw_tx, Some(options)).await
                .map_err(|e| BlockchainError::TransactionError(format!("Failed to fund transaction: {}", e)))?;
            
            fund_result.hex
        };
        
        // Add OP_RETURN if requested
        if let Some(data) = &params.op_return_data {
            if data.len() > 80 {
                return Err(BlockchainError::ValidationError("OP_RETURN data too large (max 80 bytes)".to_string()));
            }
            
            // Parse the transaction
            let tx_bytes = hex::decode(&raw_tx)
                .map_err(|e| BlockchainError::SerializationError(format!("Failed to decode transaction: {}", e)))?;
            
            let mut tx: Transaction = encode::deserialize(&tx_bytes)
                .map_err(|e| BlockchainError::SerializationError(format!("Failed to deserialize transaction: {}", e)))?;
            
            // Create OP_RETURN output
            let op_return_script = Script::new_op_return(data);
            let op_return_output = bitcoin::TxOut {
                value: 0,
                script_pubkey: op_return_script,
            };
            
            // Add the output to the transaction
            tx.output.push(op_return_output);
            
            // Serialize the transaction
            raw_tx = hex::encode(encode::serialize(&tx));
        }
        
        Ok(raw_tx)
    }
    
    async fn sign_transaction(&self, tx: &str, privkeys: Option<Vec<String>>) -> Result<String, BlockchainError> {
        let sign_result = self.rpc_client.sign_raw_transaction_with_wallet(tx).await
            .map_err(|e| BlockchainError::TransactionError(format!("Failed to sign transaction: {}", e)))?;
        
        if !sign_result.complete {
            return Err(BlockchainError::TransactionError("Transaction signing incomplete".to_string()));
        }
        
        Ok(sign_result.hex)
    }
    
    async fn analyze_transaction(&self, tx: &str) -> Result<TransactionAnalysis, BlockchainError> {
        // Decode the transaction
        let decode_result = self.rpc_client.decode_raw_transaction(tx).await
            .map_err(|e| BlockchainError::SerializationError(format!("Failed to decode transaction: {}", e)))?;
        
        // Get information about the inputs
        let mut inputs = Vec::new();
        let mut input_amount: Option<u64> = Some(0);
        let mut is_coinbase = false;
        
        for (idx, input) in decode_result.vin.iter().enumerate() {
            if input.coinbase.is_some() {
                is_coinbase = true;
                input_amount = None;
                
                inputs.push(TxAnalysisInput {
                    txid: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
                    vout: 0,
                    amount: None,
                    address: None,
                    script_sig: input.script_sig.as_ref().map_or("".to_string(), |s| s.hex.clone()),
                    witness: input.txinwitness.clone(),
                    sequence: input.sequence,
                });
                
                continue;
            }
            
            if let (Some(txid), Some(vout)) = (&input.txid, input.vout) {
                // Try to get the input transaction
                let prev_tx = match self.rpc_client.get_raw_transaction_verbose(txid).await {
                    Ok(tx) => Some(tx),
                    Err(_) => None,
                };
                
                let (amount, address) = if let Some(prev_tx) = prev_tx {
                    if let Some(prev_output) = prev_tx.vout.get(*vout as usize) {
                        // Convert BTC to satoshis
                        let amount_sats = (prev_output.value * 100_000_000.0).round() as u64;
                        
                        // Add to the input amount
                        if let Some(input_amount_value) = input_amount.as_mut() {
                            *input_amount_value += amount_sats;
                        }
                        
                        (Some(amount_sats), prev_output.script_pub_key.address.clone())
                    } else {
                        (None, None)
                    }
                } else {
                    (None, None)
                };
                
                inputs.push(TxAnalysisInput {
                    txid: txid.clone(),
                    vout: *vout,
                    amount,
                    address,
                    script_sig: input.script_sig.as_ref().map_or("".to_string(), |s| s.hex.clone()),
                    witness: input.txinwitness.clone(),
                    sequence: input.sequence,
                });
            }
        }
        
        // Get information about the outputs
        let mut outputs = Vec::new();
        let mut output_amount: u64 = 0;
        
        for (idx, output) in decode_result.vout.iter().enumerate() {
            // Convert BTC to satoshis
            let amount_sats = (output.value * 100_000_000.0).round() as u64;
            output_amount += amount_sats;
            
            let is_op_return = output.script_pub_key.type_ == Some("nulldata".to_string());
            
            outputs.push(TxAnalysisOutput {
                n: idx as u32,
                amount: amount_sats,
                address: output.script_pub_key.address.clone(),
                script_pubkey: output.script_pub_key.hex.clone(),
                script_type: output.script_pub_key.type_.clone().unwrap_or_else(|| "unknown".to_string()),
                is_op_return,
            });
        }
        
        // Calculate fee if we have input amounts
        let (fee, fee_rate) = if let Some(input_amount_value) = input_amount {
            if input_amount_value > output_amount {
                let fee = input_amount_value - output_amount;
                let fee_rate = if decode_result.vsize > 0 {
                    Some(fee as f64 / decode_result.vsize as f64)
                } else {
                    None
                };
                (Some(fee), fee_rate)
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };
        
        // Check if this is an RBF transaction
        let is_rbf = decode_result.vin.iter().any(|input| input.sequence < 0xFFFFFFFE);
        
        // Calculate signature operations
        let sigops = decode_result.vin.len() * 2; // Very rough estimate, in reality more complex
        
        let analysis = TransactionAnalysis {
            txid: decode_result.txid,
            size: decode_result.size as u64,
            vsize: decode_result.vsize as u64,
            weight: decode_result.weight as u64,
            fee,
            fee_rate,
            inputs,
            outputs,
            input_amount,
            output_amount,
            is_coinbase,
            is_rbf,
            is_fully_signed: true, // Assume true, not easy to determine without more info
            locktime: decode_result.locktime as u32,
            sigops: sigops as u32,
        };
        
        Ok(analysis)
    }
    
    async fn get_address_balance(&self, address: &str) -> Result<AddressBalance, BlockchainError> {
        // Validate the address
        let _ = Address::from_str(address)
            .map_err(|e| BlockchainError::ValidationError(format!("Invalid address: {}", e)))?;
        
        // Get UTXOs for the address
        let utxos = self.rpc_client.list_unspent(0, 9999999, Some(vec![address.to_string()])).await
            .map_err(|e| BlockchainError::InternalError(format!("Failed to get UTXOs: {}", e)))?;
        
        let mut confirmed = 0u64;
        let mut unconfirmed = 0u64;
        
        for utxo in &utxos {
            // Convert BTC to satoshis
            let amount_sats = (utxo.amount * 100_000_000.0).round() as u64;
            
            if utxo.confirmations >= 1 {
                confirmed += amount_sats;
            } else {
                unconfirmed += amount_sats;
            }
        }
        
        let total = confirmed + unconfirmed;
        
        // Get transaction count
        let tx_count = self.rpc_client.search_raw_transactions(address, 1, 0, None, true, None).await
            .map(|txs| txs.len() as u64)
            .unwrap_or(0);
        
        let balance = AddressBalance {
            confirmed,
            unconfirmed,
            total,
            tx_count,
            utxo_count: utxos.len() as u64,
        };
        
        Ok(balance)
    }
    
    async fn get_address_transactions(&self, address: &str, limit: Option<u32>) -> Result<Vec<String>, BlockchainError> {
        // Validate the address
        let _ = Address::from_str(address)
            .map_err(|e| BlockchainError::ValidationError(format!("Invalid address: {}", e)))?;
        
        // Get transactions for the address
        let txs = self.rpc_client.search_raw_transactions(address, 1, 0, limit, false, None).await
            .map_err(|e| BlockchainError::InternalError(format!("Failed to get transactions: {}", e)))?;
        
        Ok(txs)
    }
    
    async fn get_address_utxos(&self, address: &str) -> Result<Vec<UtxoInfo>, BlockchainError> {
        // Validate the address
        let _ = Address::from_str(address)
            .map_err(|e| BlockchainError::ValidationError(format!("Invalid address: {}", e)))?;
        
        // Get UTXOs for the address
        let utxos = self.rpc_client.list_unspent(0, 9999999, Some(vec![address.to_string()])).await
            .map_err(|e| BlockchainError::InternalError(format!("Failed to get UTXOs: {}", e)))?;
        
        let mut result = Vec::with_capacity(utxos.len());
        
        for utxo in utxos {
            // Convert BTC to satoshis
            let amount_sats = (utxo.amount * 100_000_000.0).round() as u64;
            
            // Get the script information
            let script_info = self.rpc_client.decode_script(&utxo.script_pub_key).await
                .map_err(|e| BlockchainError::InternalError(format!("Failed to decode script: {}", e)))?;
            
            result.push(UtxoInfo {
                txid: utxo.txid,
                vout: utxo.vout,
                amount: amount_sats,
                script_pubkey: utxo.script_pub_key,
                script_pubkey_asm: script_info.asm,
                script_type: utxo.script_type.unwrap_or_else(|| "unknown".to_string()),
                confirmations: utxo.confirmations as u64,
                coinbase: false, // Not directly available from listunspent
            });
        }
        
        Ok(result)
    }
    
    async fn import_private_key(&self, privkey: &str) -> Result<(), BlockchainError> {
        // Validate the private key format (basic check)
        if !privkey.starts_with("K") && !privkey.starts_with("L") && !privkey.starts_with("5") &&
           !privkey.starts_with("c") && !privkey.starts_with("9") && !privkey.starts_with("k") {
            return Err(BlockchainError::ValidationError("Invalid private key format".to_string()));
        }
        
        // Import the private key
        self.rpc_client.import_priv_key(privkey, None, Some(false)).await
            .map_err(|e| BlockchainError::InternalError(format!("Failed to import private key: {}", e)))?;
        
        Ok(())
    }
    
    async fn export_private_keys(&self) -> Result<Vec<String>, BlockchainError> {
        // This is a very sensitive operation and should be carefully implemented
        // For security reasons, we'll return an error by default
        Err(BlockchainError::SecurityError(
            "Export private keys is disabled by default for security reasons".to_string()
        ))
    }
    
    async fn create_raw_transaction(&self, inputs: Vec<TxInput>, outputs: HashMap<String, f64>) -> Result<String, BlockchainError> {
        // Convert inputs to the format expected by the RPC client
        let mut rpc_inputs = Vec::new();
        for input in &inputs {
            let mut input_obj = serde_json::Map::new();
            input_obj.insert("txid".to_string(), serde_json::json!(input.txid));
            input_obj.insert("vout".to_string(), serde_json::json!(input.vout));
            
            if let Some(sequence) = input.sequence {
                input_obj.insert("sequence".to_string(), serde_json::json!(sequence));
            }
            
            rpc_inputs.push(serde_json::Value::Object(input_obj));
        }
        
        // Create the raw transaction
        self.rpc_client.create_raw_transaction(&rpc_inputs, &outputs, None).await
            .map_err(|e| BlockchainError::TransactionError(format!("Failed to create raw transaction: {}", e)))
    }
}

#[async_trait]
impl SecurityPort for BitcoinAdapter {
    async fn check_chain_split(&self) -> Result<Option<ChainSplitInfo>, BlockchainError> {
        // Get the current blockchain state
        let state = self.get_blockchain_state().await?;
        
        // Get peer information
        let peers = self.get_peer_info().await?;
        
        // Filter out peers with no height info or significantly behind
        let active_peers: Vec<_> = peers.iter()
            .filter(|p| p.height > 0 && p.height > state.best_block_height.saturating_sub(10))
            .collect();
        
        if active_peers.is_empty() {
            return Ok(None);
        }
        
        // Count peers at each height
        let mut height_counts: HashMap<u64, usize> = HashMap::new();
        for peer in &active_peers {
            *height_counts.entry(peer.height).or_insert(0) += 1;
        }
        
        // Check if there's a significant disagreement about the chain height
        let total_peers = active_peers.len();
        let our_height = state.best_block_height;
        
        // Identify heights that have a significant minority of peers (more than 25%)
        let mut potential_splits = Vec::new();
        for (&height, &count) in &height_counts {
            let percentage = (count as f64 / total_peers as f64) * 100.0;
            
            // Height is different from ours and has more than 25% of peers
            if height != our_height && percentage > 25.0 {
                potential_splits.push((height, percentage));
            }
        }
        
        // If we found potential chain splits
        if !potential_splits.is_empty() {
            // Sort by percentage (highest first)
            potential_splits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            
            // Get the most likely chain split
            let (split_height, split_percentage) = potential_splits[0];
            
            // Get details about our chain's tip
            let our_hash = self.rpc_client.get_block_hash(our_height).await
                .map_err(|e| BlockchainError::QueryError(format!("Failed to get block hash: {}", e)))?;
            
            // Find a peer with the alternative height
            let alt_peers: Vec<_> = active_peers.iter()
                .filter(|p| p.height == split_height)
                .collect();
            
            // Create chain split info
            return Ok(Some(ChainSplitInfo {
                our_height,
                our_hash,
                alternative_height: split_height,
                alternative_hash: "unknown".to_string(), // We don't have direct access to the hash
                peers_on_alternative: alt_peers.len(),
                peers_on_our_chain: height_counts.get(&our_height).copied().unwrap_or(0),
                total_peers,
                split_percentage,
            }));
        }
        
        Ok(None)
    }
    
    async fn detect_double_spend(&self, txid: &str, confirmations: u32) -> Result<Option<DoubleSpendInfo>, BlockchainError> {
        // Get the transaction
        let tx = match self.rpc_client.get_raw_transaction_verbose(txid).await {
            Ok(tx) => tx,
            Err(e) => {
                return Err(BlockchainError::NotFoundError(
                    format!("Transaction not found: {}, error: {}", txid, e)
                ));
            }
        };
        
        // Check if the transaction is confirmed
        if tx.confirmations.unwrap_or(0) >= confirmations as i32 {
            // Transaction is confirmed with enough confirmations, no double-spend risk
            return Ok(None);
        }
        
        // For each input, check if there's another transaction spending the same output
        let mut double_spends = Vec::new();
        
        for input in &tx.vin {
            if let (Some(prev_txid), Some(prev_vout)) = (&input.txid, input.vout) {
                // Check the mempool for transactions spending the same output
                let mempool = self.rpc_client.get_raw_mempool().await
                    .map_err(|e| BlockchainError::QueryError(format!("Failed to get mempool: {}", e)))?;
                
                for &other_txid in &mempool {
                    // Skip the transaction itself
                    if &other_txid == txid {
                        continue;
                    }
                    
                    // Get the other transaction
                    let other_tx = match self.rpc_client.get_raw_transaction_verbose(&other_txid).await {
                        Ok(tx) => tx,
                        Err(_) => continue, // Skip if transaction can't be retrieved
                    };
                    
                    // Check if any input spends the same output
                    for other_input in &other_tx.vin {
                        if let (Some(other_prev_txid), Some(other_prev_vout)) = (&other_input.txid, other_input.vout) {
                            if other_prev_txid == prev_txid && other_prev_vout == *prev_vout {
                                // Found a double-spend attempt
                                double_spends.push(DoubleSpendAttempt {
                                    original_txid: txid.to_string(),
                                    double_spend_txid: other_txid.clone(),
                                    input_index: input.vout.unwrap_or(0) as usize,
                                    previous_output: format!("{}:{}", prev_txid, prev_vout),
                                    original_fee: tx.fee.unwrap_or(0.0) * 100_000_000.0, // Convert BTC to satoshis
                                    double_spend_fee: other_tx.fee.unwrap_or(0.0) * 100_000_000.0, // Convert BTC to satoshis
                                    detected_at: chrono::Utc::now().timestamp(),
                                });
                            }
                        }
                    }
                }
            }
        }
        
        if double_spends.is_empty() {
            Ok(None)
        } else {
            Ok(Some(DoubleSpendInfo {
                txid: txid.to_string(),
                confirmations: tx.confirmations.unwrap_or(0) as u32,
                attempts: double_spends,
            }))
        }
    }
    
    async fn detect_anomalous_fees(&self) -> Result<Vec<AnomalousFeeInfo>, BlockchainError> {
        // Get mempool entries
        let mempool_entries = self.rpc_client.get_raw_mempool_verbose().await
            .map_err(|e| BlockchainError::QueryError(format!("Failed to get mempool entries: {}", e)))?;
        
        if mempool_entries.is_empty() {
            return Ok(Vec::new());
        }
        
        // Calculate fee statistics
        let mut fee_rates = Vec::new();
        for (txid, entry) in &mempool_entries {
            let fee_rate = entry.fee / entry.size as f64 * 100_000_000.0; // satoshis per byte
            fee_rates.push((txid, fee_rate, entry.size));
        }
        
        // Sort by fee rate
        fee_rates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Calculate median and get fee estimates
        let median_index = fee_rates.len() / 2;
        let median_fee_rate = if !fee_rates.is_empty() { fee_rates[median_index].1 } else { 0.0 };
        
        // Get current fee estimates for comparison
        let fee_estimates = self.get_fee_estimates().await?;
        let high_priority_estimate = *fee_estimates.get(&1).unwrap_or(&0.0); // 1-block confirmation
        
        // Identify anomalous fees (either extremely high or extremely low compared to median)
        let mut anomalies = Vec::new();
        
        for (txid, fee_rate, size) in fee_rates {
            // Check for very high fees (10x median or more)
            if fee_rate > median_fee_rate * 10.0 && fee_rate > high_priority_estimate * 5.0 {
                anomalies.push(AnomalousFeeInfo {
                    txid: txid.clone(),
                    fee_rate,
                    size: *size,
                    median_fee_rate,
                    anomaly_type: "extremely_high".to_string(),
                    ratio_to_median: fee_rate / median_fee_rate,
                    detected_at: chrono::Utc::now().timestamp(),
                });
            }
            // Check for very low fees (less than 1% of median) on larger transactions
            else if fee_rate < median_fee_rate * 0.01 && *size > 1000 {
                anomalies.push(AnomalousFeeInfo {
                    txid: txid.clone(),
                    fee_rate,
                    size: *size,
                    median_fee_rate,
                    anomaly_type: "extremely_low".to_string(),
                    ratio_to_median: fee_rate / median_fee_rate,
                    detected_at: chrono::Utc::now().timestamp(),
                });
            }
        }
        
        Ok(anomalies)
    }
    
    async fn check_transaction_malleability(&self, txid: &str) -> Result<Option<MalleabilityInfo>, BlockchainError> {
        // Get the transaction
        let tx = match self.rpc_client.get_raw_transaction_verbose(txid).await {
            Ok(tx) => tx,
            Err(e) => {
                return Err(BlockchainError::NotFoundError(
                    format!("Transaction not found: {}, error: {}", txid, e)
                ));
            }
        };
        
        // Get the raw transaction (hex)
        let raw_tx = self.rpc_client.get_raw_transaction(txid).await
            .map_err(|e| BlockchainError::QueryError(format!("Failed to get raw transaction: {}", e)))?;
        
        // Check if transaction is SegWit
        let is_segwit = raw_tx.contains("0001"); // Very basic check for witness data
        
        // Check if transaction has RBF signaled
        let has_rbf = tx.vin.iter().any(|input| {
            input.sequence < 0xfffffffe
        });
        
        // For each input, check if it's using a signature type that's malleable
        let mut malleable_inputs = Vec::new();
        
        for (i, input) in tx.vin.iter().enumerate() {
            // Skip coinbase inputs
            if input.coinbase.is_some() {
                continue;
            }
            
            // For non-SegWit transactions, check signature types
            if !is_segwit {
                // We would need to parse the scriptSig to determine the signature type
                // This is simplified - a real implementation would decode the script
                if let Some(script_sig) = &input.script_sig {
                    // Check for presence of SIGHASH_ALL (0x01) - this is the only non-malleable type
                    // Note: This is a simplified check; a proper implementation would decode the scriptSig
                    let is_malleable = !script_sig.hex.ends_with("01");
                    
                    if is_malleable {
                        malleable_inputs.push(MalleableInput {
                            input_index: i,
                            reason: "non_segwit_malleable_sighash".to_string(),
                            risk_level: "high".to_string(),
                        });
                    }
                }
            }
        }
        
        // If no malleable inputs are found and it's SegWit, return None
        if malleable_inputs.is_empty() && is_segwit {
            return Ok(None);
        }
        
        // If it's not SegWit but there are no malleable inputs found, mark all inputs as potentially malleable
        if !is_segwit && malleable_inputs.is_empty() {
            for i in 0..tx.vin.len() {
                malleable_inputs.push(MalleableInput {
                    input_index: i,
                    reason: "non_segwit_transaction".to_string(),
                    risk_level: "medium".to_string(),
                });
            }
        }
        
        Ok(Some(MalleabilityInfo {
            txid: txid.to_string(),
            is_segwit,
            has_rbf,
            malleable_inputs,
            overall_risk: if is_segwit { "low" } else { "medium" }.to_string(),
            advice: if is_segwit {
                "Transaction uses SegWit, which protects against malleability".to_string()
            } else if has_rbf {
                "Non-SegWit transaction with RBF enabled - consider using SegWit for malleability protection".to_string()
            } else {
                "Non-SegWit transaction without RBF - vulnerable to transaction malleability".to_string()
            },
        }))
    }
    
    async fn monitor_large_transactions(&self, threshold_btc: f64) -> Result<Vec<LargeTransactionInfo>, BlockchainError> {
        // Convert threshold from BTC to satoshis
        let threshold_sats = threshold_btc * 100_000_000.0;
        
        // Get recent large transactions from mempool
        let mempool_entries = self.rpc_client.get_raw_mempool_verbose().await
            .map_err(|e| BlockchainError::QueryError(format!("Failed to get mempool entries: {}", e)))?;
        
        let mut large_transactions = Vec::new();
        
        for (txid, entry) in &mempool_entries {
            // Get detailed transaction info
            let tx = match self.rpc_client.get_raw_transaction_verbose(txid).await {
                Ok(tx) => tx,
                Err(_) => continue, // Skip if transaction can't be retrieved
            };
            
            // Calculate total output value
            let total_value: f64 = tx.vout.iter().map(|output| output.value).sum();
            let total_value_sats = total_value * 100_000_000.0;
            
            // Check if this is a large transaction
            if total_value_sats >= threshold_sats {
                // Get more details about the transaction
                let inputs_count = tx.vin.len();
                let outputs_count = tx.vout.len();
                let fee = entry.fee * 100_000_000.0; // convert to satoshis
                
                // Extract recipient addresses
                let recipient_addresses: Vec<String> = tx.vout.iter()
                    .filter_map(|output| output.script_pub_key.address.clone())
                    .collect();
                
                large_transactions.push(LargeTransactionInfo {
                    txid: txid.clone(),
                    value_sats: total_value_sats as u64,
                    fee_sats: fee as u64,
                    inputs_count,
                    outputs_count,
                    recipient_addresses,
                    in_mempool: true,
                    detected_at: chrono::Utc::now().timestamp(),
                });
            }
        }
        
        // Also check recent blocks for large transactions
        let state = self.get_blockchain_state().await?;
        let current_height = state.best_block_height;
        
        // Check the last 6 blocks (approximately 1 hour)
        for height in (current_height.saturating_sub(6))..=current_height {
            // Get block hash
            let hash = match self.rpc_client.get_block_hash(height).await {
                Ok(hash) => hash,
                Err(_) => continue, // Skip if block doesn't exist
            };
            
            // Get block
            let block = match self.rpc_client.get_block(&hash).await {
                Ok(block) => block,
                Err(_) => continue, // Skip if block can't be retrieved
            };
            
            // For each transaction in the block
            for txid in &block.tx {
                // Skip coinbase transaction (first transaction in the block)
                if txid == &block.tx[0] {
                    continue;
                }
                
                // Get transaction details
                let tx = match self.rpc_client.get_raw_transaction_verbose(txid).await {
                    Ok(tx) => tx,
                    Err(_) => continue, // Skip if transaction can't be retrieved
                };
                
                // Calculate total output value
                let total_value: f64 = tx.vout.iter().map(|output| output.value).sum();
                let total_value_sats = total_value * 100_000_000.0;
                
                // Check if this is a large transaction
                if total_value_sats >= threshold_sats {
                    // Calculate approximate fee
                    let inputs_value = match self.calculate_inputs_value(&tx).await {
                        Ok(value) => value,
                        Err(_) => continue, // Skip if we can't calculate inputs value
                    };
                    
                    let fee = (inputs_value - total_value) * 100_000_000.0; // convert to satoshis
                    
                    // Extract recipient addresses
                    let recipient_addresses: Vec<String> = tx.vout.iter()
                        .filter_map(|output| output.script_pub_key.address.clone())
                        .collect();
                    
                    large_transactions.push(LargeTransactionInfo {
                        txid: txid.clone(),
                        value_sats: total_value_sats as u64,
                        fee_sats: fee as u64,
                        inputs_count: tx.vin.len(),
                        outputs_count: tx.vout.len(),
                        recipient_addresses,
                        in_mempool: false,
                        detected_at: chrono::Utc::now().timestamp(),
                    });
                }
            }
        }
        
        Ok(large_transactions)
    }
    
    async fn check_reorg_depth(&self, height: u64) -> Result<Option<u32>, BlockchainError> {
        // Get block hash at the specified height
        let current_hash = match self.rpc_client.get_block_hash(height).await {
            Ok(hash) => hash,
            Err(_) => {
                return Err(BlockchainError::NotFoundError(
                    format!("Block at height {} not found", height)
                ));
            }
        };
        
        // Check if this hash exists in our cache
        if let Some(cached_hash) = self.block_cache.get(&height) {
            // If the hashes are different, a reorg has occurred
            if cached_hash != &current_hash {
                // Try to find the common ancestor
                let mut reorg_depth = 1;
                let mut check_height = height - 1;
                
                loop {
                    // Get the current hash at this height
                    let check_hash = match self.rpc_client.get_block_hash(check_height).await {
                        Ok(hash) => hash,
                        Err(_) => break, // Can't go further
                    };
                    
                    // Check if this hash exists in our cache
                    if let Some(cached_check_hash) = self.block_cache.get(&check_height) {
                        // If the hashes match, we've found the common ancestor
                        if cached_check_hash == &check_hash {
                            return Ok(Some(reorg_depth));
                        }
                    } else {
                        // We don't have a cached hash for this height
                        break;
                    }
                    
                    reorg_depth += 1;
                    if check_height == 0 || reorg_depth > 100 {
                        break; // Prevent infinite loops or excessively deep searches
                    }
                    check_height -= 1;
                }
                
                return Ok(Some(reorg_depth));
            }
        }
        
        // No reorg detected
        Ok(None)
    }
    
    // Helper method to calculate the total input value of a transaction
    async fn calculate_inputs_value(&self, tx: &TransactionInfo) -> Result<f64, BlockchainError> {
        let mut total_input_value = 0.0;
        
        for input in &tx.vin {
            // Skip coinbase inputs
            if input.coinbase.is_some() {
                continue;
            }
            
            if let (Some(prev_txid), Some(prev_vout)) = (&input.txid, input.vout) {
                // Get previous transaction
                let prev_tx = match self.rpc_client.get_raw_transaction_verbose(prev_txid).await {
                    Ok(tx) => tx,
                    Err(e) => {
                        return Err(BlockchainError::QueryError(
                            format!("Failed to get previous transaction {}: {}", prev_txid, e)
                        ));
                    }
                };
                
                // Get the output being spent
                if let Some(prev_output) = prev_tx.vout.get(*prev_vout as usize) {
                    total_input_value += prev_output.value;
                }
            }
        }
        
        Ok(total_input_value)
    }
}

// The rest of the adapter implementation will follow in subsequent chunks... 

// Implementation of the BlockchainAdapter trait which combines all other traits
#[async_trait]
impl BlockchainAdapter for BitcoinAdapter {
    fn get_chain_id(&self) -> String {
        format!("bitcoin-{}", self.config.network.to_string())
    }
    
    async fn initialize(&self) -> Result<(), BlockchainError> {
        // Update blockchain state
        let _ = self.update_blockchain_state().await
            .map_err(|e| BlockchainError::InternalError(format!("Failed to update blockchain state: {}", e)))?;
            
        // Update metrics
        let _ = self.update_blockchain_metrics().await
            .map_err(|e| BlockchainError::InternalError(format!("Failed to update metrics: {}", e)))?;
            
        // Update mempool status
        let _ = self.update_mempool_status().await
            .map_err(|e| BlockchainError::InternalError(format!("Failed to update mempool status: {}", e)))?;
        
        // Initialize caches
        self.initialize_caches().await
            .map_err(|e| BlockchainError::InternalError(format!("Failed to initialize caches: {}", e)))?;
        
        Ok(())
    }
    
    async fn start_monitoring(&self) -> Result<(), BlockchainError> {
        // Start metrics collection if not already running
        {
            let mut metrics_running = self.metrics_running.write().await;
            if !*metrics_running {
                *metrics_running = true;
                
                // Clone the adapter for the metrics task
                let adapter_clone = self.clone();
                let metrics_interval = self.config.metrics_interval;
                
                // Spawn a task to collect metrics periodically
                tokio::spawn(async move {
                    loop {
                        // Sleep for the interval
                        tokio::time::sleep(Duration::from_secs(metrics_interval)).await;
                        
                        // Check if we should still be running
                        if !*adapter_clone.metrics_running.read().await {
                            break;
                        }
                        
                        // Update metrics
                        match adapter_clone.update_blockchain_metrics().await {
                            Ok(_) => {
                                trace!("Updated blockchain metrics");
                            },
                            Err(e) => {
                                warn!("Failed to update blockchain metrics: {}", e);
                            }
                        }
                        
                        // Update mempool status
                        match adapter_clone.update_mempool_status().await {
                            Ok(_) => {
                                trace!("Updated mempool status");
                            },
                            Err(e) => {
                                warn!("Failed to update mempool status: {}", e);
                            }
                        }
                    }
                });
            }
        }
        
        // Start security monitoring if enabled and not already running
        if self.config.enable_security_monitoring {
            let mut security_running = self.security_running.write().await;
            if !*security_running {
                *security_running = true;
                
                // Clone the adapter for the security task
                let adapter_clone = self.clone();
                let security_interval = self.config.security_interval;
                
                // Spawn a task to monitor security periodically
                tokio::spawn(async move {
                    loop {
                        // Sleep for the interval
                        tokio::time::sleep(Duration::from_secs(security_interval)).await;
                        
                        // Check if we should still be running
                        if !*adapter_clone.security_running.read().await {
                            break;
                        }
                        
                        // Check for chain splits
                        match adapter_clone.check_chain_split().await {
                            Ok(Some(split_info)) => {
                                warn!("Detected potential chain split: our height: {}, alternative height: {}, percentage: {}%",
                                      split_info.our_height, split_info.alternative_height, split_info.split_percentage);
                                
                                // Add to security alerts
                                let mut alerts = adapter_clone.security_alerts.write().await;
                                alerts.push(format!("Chain split detected: our height: {}, alternative height: {}, percentage: {}%",
                                                  split_info.our_height, split_info.alternative_height, split_info.split_percentage));
                                
                                // Limit alerts size
                                if alerts.len() > 100 {
                                    alerts.remove(0);
                                }
                            },
                            Ok(None) => {
                                trace!("No chain split detected");
                            },
                            Err(e) => {
                                warn!("Failed to check for chain splits: {}", e);
                            }
                        }
                        
                        // Check for anomalous fees
                        match adapter_clone.detect_anomalous_fees().await {
                            Ok(anomalies) => {
                                if !anomalies.is_empty() {
                                    for anomaly in &anomalies {
                                        warn!("Anomalous fee detected: txid: {}, type: {}, rate: {}, ratio to median: {}",
                                              anomaly.txid, anomaly.anomaly_type, anomaly.fee_rate, anomaly.ratio_to_median);
                                    }
                                    
                                    // Add to unusual transactions
                                    let mut unusual_txs = adapter_clone.unusual_txs.write().await;
                                    for anomaly in anomalies {
                                        unusual_txs.push(UnusualTransaction {
                                            txid: anomaly.txid,
                                            reason: format!("Anomalous fee: {}, ratio to median: {:.2}", 
                                                          anomaly.anomaly_type, anomaly.ratio_to_median),
                                            detected_at: SystemTime::now()
                                                .duration_since(UNIX_EPOCH)
                                                .unwrap_or_default()
                                                .as_secs(),
                                        });
                                    }
                                    
                                    // Limit unusual transactions size
                                    if unusual_txs.len() > 100 {
                                        unusual_txs.drain(0..unusual_txs.len() - 100);
                                    }
                                }
                            },
                            Err(e) => {
                                warn!("Failed to detect anomalous fees: {}", e);
                            }
                        }
                        
                        // Monitor large transactions
                        match adapter_clone.monitor_large_transactions(10.0).await { // 10 BTC threshold
                            Ok(large_txs) => {
                                if !large_txs.is_empty() {
                                    for tx in &large_txs {
                                        info!("Large transaction detected: txid: {}, value: {} BTC, inputs: {}, outputs: {}",
                                             tx.txid, tx.value_sats as f64 / 100_000_000.0, tx.inputs_count, tx.outputs_count);
                                    }
                                }
                            },
                            Err(e) => {
                                warn!("Failed to monitor large transactions: {}", e);
                            }
                        }
                    }
                });
            }
        }
        
        Ok(())
    }
    
    async fn stop_monitoring(&self) -> Result<(), BlockchainError> {
        // Stop metrics collection
        {
            let mut metrics_running = self.metrics_running.write().await;
            *metrics_running = false;
        }
        
        // Stop security monitoring
        {
            let mut security_running = self.security_running.write().await;
            *security_running = false;
        }
        
        Ok(())
    }
    
    async fn get_unusual_transactions(&self) -> Result<Vec<UnusualTransaction>, BlockchainError> {
        let unusual_txs = self.unusual_txs.read().await;
        Ok(unusual_txs.clone())
    }
    
    async fn get_security_alerts(&self) -> Result<Vec<String>, BlockchainError> {
        let alerts = self.security_alerts.read().await;
        Ok(alerts.clone())
    }
    
    async fn compare_with_alert(&self, alert: &AlertComparison) -> Result<bool, BlockchainError> {
        match alert.field.as_str() {
            "block_height" => {
                let state = self.get_blockchain_state().await?;
                let current_value = state.best_block_height;
                
                match alert.comparison.as_str() {
                    "eq" => Ok(current_value == alert.value.parse::<u64>().unwrap_or(0)),
                    "neq" => Ok(current_value != alert.value.parse::<u64>().unwrap_or(0)),
                    "gt" => Ok(current_value > alert.value.parse::<u64>().unwrap_or(0)),
                    "lt" => Ok(current_value < alert.value.parse::<u64>().unwrap_or(0)),
                    "gte" => Ok(current_value >= alert.value.parse::<u64>().unwrap_or(0)),
                    "lte" => Ok(current_value <= alert.value.parse::<u64>().unwrap_or(0)),
                    _ => Err(BlockchainError::ValidationError(format!("Invalid comparison operator: {}", alert.comparison))),
                }
            },
            "mempool_size" => {
                let mempool = self.get_mempool_status().await?;
                let current_value = mempool.size;
                
                match alert.comparison.as_str() {
                    "eq" => Ok(current_value == alert.value.parse::<u64>().unwrap_or(0)),
                    "neq" => Ok(current_value != alert.value.parse::<u64>().unwrap_or(0)),
                    "gt" => Ok(current_value > alert.value.parse::<u64>().unwrap_or(0)),
                    "lt" => Ok(current_value < alert.value.parse::<u64>().unwrap_or(0)),
                    "gte" => Ok(current_value >= alert.value.parse::<u64>().unwrap_or(0)),
                    "lte" => Ok(current_value <= alert.value.parse::<u64>().unwrap_or(0)),
                    _ => Err(BlockchainError::ValidationError(format!("Invalid comparison operator: {}", alert.comparison))),
                }
            },
            "peer_count" => {
                let peers = self.get_peer_info().await?;
                let current_value = peers.len() as u64;
                
                match alert.comparison.as_str() {
                    "eq" => Ok(current_value == alert.value.parse::<u64>().unwrap_or(0)),
                    "neq" => Ok(current_value != alert.value.parse::<u64>().unwrap_or(0)),
                    "gt" => Ok(current_value > alert.value.parse::<u64>().unwrap_or(0)),
                    "lt" => Ok(current_value < alert.value.parse::<u64>().unwrap_or(0)),
                    "gte" => Ok(current_value >= alert.value.parse::<u64>().unwrap_or(0)),
                    "lte" => Ok(current_value <= alert.value.parse::<u64>().unwrap_or(0)),
                    _ => Err(BlockchainError::ValidationError(format!("Invalid comparison operator: {}", alert.comparison))),
                }
            },
            "fee_rate" => {
                let fee_estimates = self.fee_estimates.read().await;
                let blocks = alert.extra.as_ref()
                    .and_then(|e| e.get("blocks"))
                    .and_then(|b| b.parse::<u16>().ok())
                    .unwrap_or(1);
                
                let current_value = match fee_estimates.get(&blocks) {
                    Some(fee) => *fee as f64,
                    None => self.estimate_fee(blocks).await? as f64,
                };
                
                let comparison_value = alert.value.parse::<f64>().unwrap_or(0.0);
                
                match alert.comparison.as_str() {
                    "eq" => Ok((current_value - comparison_value).abs() < 0.01),
                    "neq" => Ok((current_value - comparison_value).abs() >= 0.01),
                    "gt" => Ok(current_value > comparison_value),
                    "lt" => Ok(current_value < comparison_value),
                    "gte" => Ok(current_value >= comparison_value),
                    "lte" => Ok(current_value <= comparison_value),
                    _ => Err(BlockchainError::ValidationError(format!("Invalid comparison operator: {}", alert.comparison))),
                }
            },
            "sync_progress" => {
                let state = self.get_blockchain_state().await?;
                let current_value = state.sync_progress;
                
                let comparison_value = alert.value.parse::<f64>().unwrap_or(0.0);
                
                match alert.comparison.as_str() {
                    "eq" => Ok((current_value - comparison_value).abs() < 0.01),
                    "neq" => Ok((current_value - comparison_value).abs() >= 0.01),
                    "gt" => Ok(current_value > comparison_value),
                    "lt" => Ok(current_value < comparison_value),
                    "gte" => Ok(current_value >= comparison_value),
                    "lte" => Ok(current_value <= comparison_value),
                    _ => Err(BlockchainError::ValidationError(format!("Invalid comparison operator: {}", alert.comparison))),
                }
            },
            _ => Err(BlockchainError::ValidationError(format!("Invalid alert field: {}", alert.field))),
        }
    }
}

// Add Clone implementation to support cloning for async tasks
impl Clone for BitcoinAdapter {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            rpc_client: self.rpc_client.clone(),
            state: RwLock::new(self.state.try_read().unwrap_or_else(|_| None).clone()),
            metrics: RwLock::new(self.metrics.try_read().unwrap_or_else(|_| None).clone()),
            mempool: RwLock::new(self.mempool.try_read().unwrap_or_else(|_| None).clone()),
            blocks_cache: Mutex::new(self.blocks_cache.lock().unwrap().clone()),
            tx_cache: Mutex::new(self.tx_cache.lock().unwrap().clone()),
            utxo_cache: Mutex::new(self.utxo_cache.lock().unwrap().clone()),
            fee_estimates: RwLock::new(self.fee_estimates.try_read().unwrap_or_else(|_| HashMap::new()).clone()),
            unusual_txs: RwLock::new(self.unusual_txs.try_read().unwrap_or_else(|_| Vec::new()).clone()),
            security_alerts: RwLock::new(self.security_alerts.try_read().unwrap_or_else(|_| Vec::new()).clone()),
            metrics_running: RwLock::new(*self.metrics_running.try_read().unwrap_or_else(|_| &false)),
            security_running: RwLock::new(*self.security_running.try_read().unwrap_or_else(|_| &false)),
        }
    }
}

impl BitcoinAdapter {
    // Initialize caches with recent blocks and transactions
    async fn initialize_caches(&self) -> Result<(), BitcoinAdapterError> {
        // Get the current blockchain state
        let state = self.get_blockchain_state().await
            .map_err(|e| BitcoinAdapterError::ConfigError(format!("Failed to get blockchain state: {}", e)))?;
        
        let best_height = state.best_block_height;
        
        // Cache the last 10 blocks
        for height in (best_height.saturating_sub(10))..=best_height {
            match self.rpc_client.get_block_hash(height).await {
                Ok(hash) => {
                    // Convert the block to BlockInfo and cache it
                    let _ = self.convert_block_to_info(&hash, height).await;
                },
                Err(e) => {
                    warn!("Failed to get block hash for height {}: {}", height, e);
                }
            }
        }
        
        // Cache some transactions from the last block
        if let Ok(hash) = self.rpc_client.get_block_hash(best_height).await {
            if let Ok(block) = self.rpc_client.get_block(&hash).await {
                // Cache up to 20 transactions from the last block
                for (i, txid) in block.tx.iter().enumerate() {
                    if i >= 20 {
                        break;
                    }
                    
                    let _ = self.convert_tx_to_info(txid).await;
                }
            }
        }
        
        // Cache current fee estimates
        for &block_target in &self.config.fee_estimation_blocks {
            if let Ok(estimate) = self.rpc_client.estimate_smart_fee(block_target).await {
                if let Some(fee_rate) = estimate.fee_rate {
                    // Convert BTC/kB to sat/B
                    let fee_sats = (fee_rate * 100_000_000.0 / 1000.0).round() as u64;
                    
                    let mut fee_estimates = self.fee_estimates.write().await;
                    fee_estimates.insert(block_target, fee_sats);
                }
            }
        }
        
        Ok(())
    }
}

// Helper structs for Bitcoin RPC types that aren't directly used in the public interface

// Double spend detection types
#[derive(Debug, Clone)]
struct DoubleSpendAttempt {
    original_txid: String,
    double_spend_txid: String,
    input_index: usize,
    previous_output: String,
    original_fee: f64,
    double_spend_fee: f64,
    detected_at: i64,
}

#[derive(Debug, Clone)]
struct DoubleSpendInfo {
    txid: String,
    confirmations: u32,
    attempts: Vec<DoubleSpendAttempt>,
}

// Fee anomaly detection types
#[derive(Debug, Clone)]
struct AnomalousFeeInfo {
    txid: String,
    fee_rate: f64,
    size: u32,
    median_fee_rate: f64,
    anomaly_type: String,
    ratio_to_median: f64,
    detected_at: i64,
}

// Transaction malleability check types
#[derive(Debug, Clone)]
struct MalleableInput {
    input_index: usize,
    reason: String,
    risk_level: String,
}

#[derive(Debug, Clone)]
struct MalleabilityInfo {
    txid: String,
    is_segwit: bool,
    has_rbf: bool,
    malleable_inputs: Vec<MalleableInput>,
    overall_risk: String,
    advice: String,
}

// Large transaction monitoring types
#[derive(Debug, Clone)]
struct LargeTransactionInfo {
    txid: String,
    value_sats: u64,
    fee_sats: u64,
    inputs_count: usize,
    outputs_count: usize,
    recipient_addresses: Vec<String>,
    in_mempool: bool,
    detected_at: i64,
}

// Transaction analysis types for internal use
#[derive(Debug, Clone)]
struct TxAnalysisInput {
    txid: String,
    vout: u32,
    amount: Option<u64>,
    address: Option<String>,
    script_sig: String,
    witness: Option<Vec<String>>,
    sequence: u32,
}

#[derive(Debug, Clone)]
struct TxAnalysisOutput {
    n: u32,
    amount: u64,
    address: Option<String>,
    script_pubkey: String,
    script_type: String,
    is_op_return: bool,
} 