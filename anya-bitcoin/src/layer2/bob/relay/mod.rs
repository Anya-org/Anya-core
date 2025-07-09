// Bitcoin relay monitor module for BOB
// Implements Bitcoin relay monitoring for Bitcoin Optimistic Blockchain
// as per official Bitcoin Improvement Proposals (BIPs) requirements

use crate::layer2::bob::{BobConfig, BobError};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, warn};

/// Block confirmation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockConfirmationStatus {
    /// Block is pending confirmation
    Pending,
    /// Block is confirmed
    Confirmed,
    /// Block is invalidated (e.g. due to a reorg)
    Invalidated,
}

/// Block information for relay monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    /// Block hash
    pub hash: String,
    /// Block height
    pub height: u64,
    /// Block confirmation status
    pub status: BlockConfirmationStatus,
    /// Number of confirmations
    pub confirmations: u32,
    /// Block timestamp
    pub timestamp: u64,
}

/// Bitcoin transaction information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    /// Transaction ID
    pub txid: String,
    /// Block hash containing the transaction
    pub block_hash: Option<String>,
    /// Block height containing the transaction
    pub block_height: Option<u64>,
    /// Transaction confirmation status
    pub status: BlockConfirmationStatus,
    /// Number of confirmations
    pub confirmations: u32,
}

/// BitcoinRelayMonitor for BOB
pub struct BitcoinRelayMonitor {
    config: BobConfig,
    client: Arc<Client>,
    // Cache recent blocks to avoid redundant RPC calls
    blocks_cache: Arc<RwLock<HashMap<String, BlockInfo>>>,
    // Cache recent transactions to avoid redundant RPC calls
    tx_cache: Arc<RwLock<HashMap<String, TransactionInfo>>>,
    // Minimum confirmations required for finality
    min_confirmations: u32,
}

impl BitcoinRelayMonitor {
    /// Create a new Bitcoin relay monitor
    pub fn new(config: &BobConfig) -> Self {
        // Extract Bitcoin node RPC configuration
        let rpc_url = format!("{}:{}", config.bitcoin_rpc_host, config.bitcoin_rpc_port);
        
        // Create Bitcoin Core RPC client
        let auth = match (&config.bitcoin_rpc_user, &config.bitcoin_rpc_password) {
            (Some(user), Some(password)) => Auth::UserPass(user.clone(), password.clone()),
            _ => {
                warn!("No Bitcoin RPC credentials provided, using cookie auth");
                Auth::CookieFile(config.bitcoin_cookie_path.clone().unwrap_or_else(|| "/tmp/.cookie".to_string()))
            }
        };
        
        let client = Client::new(&rpc_url, auth)
            .expect("Failed to create Bitcoin RPC client");
            
        Self {
            config: config.clone(),
            client: Arc::new(client),
            blocks_cache: Arc::new(RwLock::new(HashMap::new())),
            tx_cache: Arc::new(RwLock::new(HashMap::new())),
            min_confirmations: config.min_confirmations.unwrap_or(6),
        }
    }

    /// Check connection to the Bitcoin network
    pub async fn check_connection(&self) -> Result<bool, BobError> {
        match self.client.get_blockchain_info() {
            Ok(_) => {
                debug!("Bitcoin RPC connection successful");
                Ok(true)
            }
            Err(e) => {
                warn!("Failed to connect to Bitcoin RPC: {}", e);
                Err(BobError::ConnectionError(format!("Failed to connect to Bitcoin RPC: {}", e)))
            }
        }
    }

    /// Get latest block number
    pub async fn get_latest_block(&self) -> Result<u64, BobError> {
        let block_count = self.client.get_block_count()
            .map_err(|e| BobError::RpcError(format!("Failed to get block count: {}", e)))?;
            
        Ok(block_count as u64)
    }
    
    /// Get block information by block hash
    pub async fn get_block_info(&self, block_hash: &str) -> Result<BlockInfo, BobError> {
        // First check cache
        if let Some(block_info) = self.blocks_cache.read()
            .map_err(|e| BobError::LockError(format!("Failed to acquire read lock: {}", e)))?
            .get(block_hash)
        {
            return Ok(block_info.clone());
        }
        
        // Not in cache, fetch from RPC
        let block_hash_obj = bitcoincore_rpc::bitcoin::BlockHash::from_str(block_hash)
            .map_err(|e| BobError::ParseError(format!("Invalid block hash: {}", e)))?;
            
        let block = self.client.get_block_info(&block_hash_obj)
            .map_err(|e| BobError::RpcError(format!("Failed to get block info: {}", e)))?;
            
        let confirmations = block.confirmations;
        let status = if confirmations >= self.min_confirmations as i32 {
            BlockConfirmationStatus::Confirmed
        } else if confirmations < 0 {
            BlockConfirmationStatus::Invalidated
        } else {
            BlockConfirmationStatus::Pending
        };
        
        let block_info = BlockInfo {
            hash: block_hash.to_string(),
            height: block.height as u64,
            status,
            confirmations: if confirmations < 0 { 0 } else { confirmations as u32 },
            timestamp: block.time as u64,
        };
        
        // Update cache
        self.blocks_cache.write()
            .map_err(|e| BobError::LockError(format!("Failed to acquire write lock: {}", e)))?
            .insert(block_hash.to_string(), block_info.clone());
            
        Ok(block_info)
    }
    
    /// Get transaction information by txid
    pub async fn get_transaction_info(&self, txid: &str) -> Result<TransactionInfo, BobError> {
        // First check cache
        if let Some(tx_info) = self.tx_cache.read()
            .map_err(|e| BobError::LockError(format!("Failed to acquire read lock: {}", e)))?
            .get(txid)
        {
            return Ok(tx_info.clone());
        }
        
        // Not in cache, fetch from RPC
        let txid_obj = bitcoincore_rpc::bitcoin::Txid::from_str(txid)
            .map_err(|e| BobError::ParseError(format!("Invalid txid: {}", e)))?;
            
        let tx = self.client.get_raw_transaction_info(&txid_obj, None)
            .map_err(|e| BobError::RpcError(format!("Failed to get transaction info: {}", e)))?;
            
        // Extract transaction information
        let (block_hash, block_height) = match tx.blockhash {
            Some(bh) => {
                let block_info = self.client.get_block_info(&bh)
                    .map_err(|e| BobError::RpcError(format!("Failed to get block info: {}", e)))?;
                    
                (Some(bh.to_string()), Some(block_info.height as u64))
            },
            None => (None, None),
        };
        
        let confirmations = tx.confirmations.unwrap_or(0);
        let status = if confirmations >= self.min_confirmations as u32 {
            BlockConfirmationStatus::Confirmed
        } else if confirmations == 0 && block_hash.is_none() {
            BlockConfirmationStatus::Pending
        } else {
            BlockConfirmationStatus::Pending
        };
        
        let tx_info = TransactionInfo {
            txid: txid.to_string(),
            block_hash,
            block_height,
            status,
            confirmations,
        };
        
        // Update cache
        self.tx_cache.write()
            .map_err(|e| BobError::LockError(format!("Failed to acquire write lock: {}", e)))?
            .insert(txid.to_string(), tx_info.clone());
            
        Ok(tx_info)
    }
    
    /// Listen for new blocks and invoke callback when a block is received
    pub async fn subscribe_to_blocks<F>(&self, callback: F) -> Result<(), BobError> 
    where
        F: FnMut(BlockInfo) + Send + 'static,
    {
        // This would typically use ZMQ subscriptions to Bitcoin Core
        // For now, we'll return a placeholder error since proper async subscription
        // requires more elaborate setup with ZMQ or other notification mechanisms
        Err(BobError::NotImplementedError("Block subscription requires ZMQ setup".to_string()))
    }
    
    /// Listen for transactions to a specific address and invoke callback when a transaction is received
    pub async fn subscribe_to_address<F>(&self, address: &str, callback: F) -> Result<(), BobError>
    where
        F: FnMut(TransactionInfo) + Send + 'static,
    {
        // This would typically use a combination of importaddress and ZMQ notifications
        // For now, we'll return a placeholder error
        Err(BobError::NotImplementedError("Address subscription requires ZMQ setup".to_string()))
    }
    
    /// Clear cached data that may be stale (e.g., after a reorg)
    pub async fn clear_cache(&self) -> Result<(), BobError> {
        let mut blocks_cache = self.blocks_cache.write()
            .map_err(|e| BobError::LockError(format!("Failed to acquire write lock: {}", e)))?;
            
        let mut tx_cache = self.tx_cache.write()
            .map_err(|e| BobError::LockError(format!("Failed to acquire write lock: {}", e)))?;
            
        blocks_cache.clear();
        tx_cache.clear();
        
        info!("Relay monitor caches cleared");
        Ok(())
    }
}
