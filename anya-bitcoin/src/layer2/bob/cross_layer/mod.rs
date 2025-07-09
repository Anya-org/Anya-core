// Cross-layer transaction management module for BOB
// Implements cross-layer transaction management for Bitcoin Optimistic Blockchain
// as per official Bitcoin Improvement Proposals (BIPs) requirements

use crate::layer2::bob::{BitVMProof, BobConfig, BobError, CrossLayerTransaction, EvmTransaction};
use crate::layer2::bob::relay::BlockConfirmationStatus;
use bitcoincore_rpc::{Auth, Client as BitcoinClient, RpcApi};
use ethers::prelude::{Address, TransactionRequest, U256};
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

/// Status of a cross-layer transaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrossLayerTransactionStatus {
    /// Transaction is pending
    Pending,
    /// Transaction is confirmed on Bitcoin but not yet on EVM
    BtcConfirmed,
    /// Transaction is confirmed on EVM but not yet on Bitcoin
    EvmConfirmed,
    /// Transaction is confirmed on both chains
    BothConfirmed,
    /// Transaction failed
    Failed(String),
}

/// Transaction record for tracking cross-layer transactions
#[derive(Debug, Clone)]
pub struct TransactionRecord {
    /// Transaction ID on Bitcoin
    pub btc_txid: Option<String>,
    /// Transaction ID on EVM
    pub evm_txid: Option<String>,
    /// Block hash on Bitcoin
    pub btc_block_hash: Option<String>,
    /// Block number on EVM
    pub evm_block_number: Option<u64>,
    /// Bitcoin confirmations
    pub btc_confirmations: u32,
    /// EVM confirmations
    pub evm_confirmations: u32,
    /// Current status
    pub status: CrossLayerTransactionStatus,
    /// Timestamp of last update
    pub last_update: u64,
}

/// Cross-layer transaction manager for BOB
pub struct CrossLayerTransactionManager {
    config: BobConfig,
    bitcoin_client: Arc<BitcoinClient>,
    evm_client: Arc<HttpClient>,
    // Track ongoing cross-layer transactions
    transaction_records: Arc<RwLock<std::collections::HashMap<String, TransactionRecord>>>,
}

impl CrossLayerTransactionManager {
    /// Create a new cross-layer transaction manager
    pub fn new(config: &BobConfig) -> Self {
        // Setup Bitcoin RPC client
        let bitcoin_rpc_url = format!("{}:{}", config.bitcoin_rpc_host, config.bitcoin_rpc_port);
        
        let bitcoin_auth = match (&config.bitcoin_rpc_user, &config.bitcoin_rpc_password) {
            (Some(user), Some(password)) => Auth::UserPass(user.clone(), password.clone()),
            _ => Auth::CookieFile(config.bitcoin_cookie_path.clone().unwrap_or_else(|| "/tmp/.cookie".to_string())),
        };
        
        let bitcoin_client = BitcoinClient::new(&bitcoin_rpc_url, bitcoin_auth)
            .expect("Failed to create Bitcoin RPC client");
            
        // Setup EVM RPC client
        let evm_timeout = Duration::from_millis(config.timeout_ms);
        let evm_client = HttpClientBuilder::default()
            .request_timeout(evm_timeout)
            .build(&config.rpc_url)
            .expect("Failed to build EVM client");
            
        Self {
            config: config.clone(),
            bitcoin_client: Arc::new(bitcoin_client),
            evm_client: Arc::new(evm_client),
            transaction_records: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Send a cross-layer transaction
    pub async fn send_transaction(
        &self,
        transaction: CrossLayerTransaction,
    ) -> Result<String, BobError> {
        // Generate a unique ID for this cross-layer transaction
        let transaction_id = format!(
            "cl-{}-{}", 
            transaction.source_chain, 
            hex::encode(&transaction.payload[..8])
        );
        
        info!("Processing cross-layer transaction {}", transaction_id);
        
        // Create transaction record
        let record = TransactionRecord {
            btc_txid: None,
            evm_txid: None,
            btc_block_hash: None,
            evm_block_number: None,
            btc_confirmations: 0,
            evm_confirmations: 0,
            status: CrossLayerTransactionStatus::Pending,
            last_update: chrono::Utc::now().timestamp() as u64,
        };
        
        // Store the record
        self.transaction_records.write().await.insert(transaction_id.clone(), record);
        
        // Process based on source chain
        match transaction.source_chain.as_str() {
            "bitcoin" => self.process_bitcoin_to_evm(transaction_id.clone(), transaction).await?,
            "evm" => self.process_evm_to_bitcoin(transaction_id.clone(), transaction).await?,
            _ => return Err(BobError::InvalidChainError(format!(
                "Invalid source chain: {}", transaction.source_chain
            ))),
        };
        
        // Start monitoring the transaction (this would typically be in a background task)
        self.spawn_transaction_monitor(transaction_id.clone());
        
        Ok(transaction_id)
    }
    
    /// Process a transaction from Bitcoin to EVM
    async fn process_bitcoin_to_evm(
        &self,
        transaction_id: String,
        transaction: CrossLayerTransaction,
    ) -> Result<(), BobError> {
        // In a real implementation, this would:
        // 1. Verify the Bitcoin transaction
        // 2. Create a corresponding EVM transaction
        // 3. Submit the EVM transaction
        
        // For now, we'll do a simplified version
        
        // Parse Bitcoin txid
        let btc_txid = transaction.transaction_id
            .ok_or_else(|| BobError::InvalidParamError("Missing Bitcoin txid".to_string()))?;
            
        // Update record with Bitcoin txid
        let mut records = self.transaction_records.write().await;
        if let Some(record) = records.get_mut(&transaction_id) {
            record.btc_txid = Some(btc_txid.clone());
        }
        
        // Verify Bitcoin transaction exists
        let btc_txid_obj = bitcoincore_rpc::bitcoin::Txid::from_str(&btc_txid)
            .map_err(|e| BobError::ParseError(format!("Invalid Bitcoin txid: {}", e)))?;
            
        match self.bitcoin_client.get_raw_transaction_info(&btc_txid_obj, None) {
            Ok(tx) => {
                // Transaction exists, now create EVM transaction
                let evm_tx = EvmTransaction {
                    from: self.config.bridge_address.clone(),
                    to: Some(self.config.contract_address.clone()),
                    value: 0,
                    gas_limit: 100000,
                    gas_price: 20000000000, // 20 gwei
                    nonce: None,
                    data: transaction.payload.clone(),
                    hash: format!("0x{}", hex::encode(&transaction.payload[..32])),
                };
                
                // Submit EVM transaction
                let evm_tx_request = self.create_evm_tx_request(&evm_tx)?
                    .data(evm_tx.data.clone());
                    
                let tx_serialized = serde_json::to_value(evm_tx_request)
                    .map_err(|e| BobError::SerializationError(format!("Failed to serialize transaction: {}", e)))?;
                    
                // Send the transaction
                let evm_txid: String = self.evm_client.request("eth_sendTransaction", &[tx_serialized])
                    .await
                    .map_err(|e| BobError::TransactionError(format!("Failed to send EVM transaction: {}", e)))?;
                    
                info!("Submitted EVM transaction: {}", evm_txid);
                
                // Update record with EVM txid
                if let Some(record) = records.get_mut(&transaction_id) {
                    record.evm_txid = Some(evm_txid.clone());
                    record.status = CrossLayerTransactionStatus::BtcConfirmed;
                    record.last_update = chrono::Utc::now().timestamp() as u64;
                }
                
                Ok(())
            },
            Err(e) => {
                // Transaction doesn't exist or other error
                error!("Failed to verify Bitcoin transaction: {}", e);
                
                // Update status to failed
                if let Some(record) = records.get_mut(&transaction_id) {
                    record.status = CrossLayerTransactionStatus::Failed(
                        format!("Failed to verify Bitcoin transaction: {}", e)
                    );
                    record.last_update = chrono::Utc::now().timestamp() as u64;
                }
                
                Err(BobError::BitcoinError(format!("Failed to verify Bitcoin transaction: {}", e)))
            }
        }
    }
    
    /// Process a transaction from EVM to Bitcoin
    async fn process_evm_to_bitcoin(
        &self,
        transaction_id: String,
        transaction: CrossLayerTransaction,
    ) -> Result<(), BobError> {
        // In a real implementation, this would:
        // 1. Verify the EVM transaction
        // 2. Create a corresponding Bitcoin transaction
        // 3. Submit the Bitcoin transaction
        
        // For now, we'll do a simplified version
        
        // Parse EVM txid
        let evm_txid = transaction.transaction_id
            .ok_or_else(|| BobError::InvalidParamError("Missing EVM txid".to_string()))?;
            
        // Update record with EVM txid
        let mut records = self.transaction_records.write().await;
        if let Some(record) = records.get_mut(&transaction_id) {
            record.evm_txid = Some(evm_txid.clone());
        }
        
        // Verify EVM transaction exists
        let receipt: Option<serde_json::Value> = self.evm_client
            .request("eth_getTransactionReceipt", &[serde_json::Value::String(evm_txid.clone())])
            .await
            .map_err(|e| BobError::RpcError(format!("Failed to get EVM transaction receipt: {}", e)))?;
            
        match receipt {
            Some(receipt) => {
                // Check transaction status
                let status = receipt.get("status")
                    .and_then(|s| s.as_str())
                    .ok_or_else(|| BobError::ParseError("Receipt missing status field".to_string()))?;
                    
                if status != "0x1" {
                    // Transaction failed
                    let error_msg = "EVM transaction failed".to_string();
                    
                    // Update status
                    if let Some(record) = records.get_mut(&transaction_id) {
                        record.status = CrossLayerTransactionStatus::Failed(error_msg.clone());
                        record.last_update = chrono::Utc::now().timestamp() as u64;
                    }
                    
                    return Err(BobError::EvmError(error_msg));
                }
                
                // Transaction succeeded, now create Bitcoin transaction
                // This would typically involve more complex logic to create a Bitcoin transaction
                // For now, we'll just simulate it
                
                // In a real implementation, we'd use something like:
                // let btc_tx = self.bitcoin_client.create_raw_transaction(...)
                // let signed_tx = self.bitcoin_client.sign_raw_transaction_with_wallet(...)
                // let btc_txid = self.bitcoin_client.send_raw_transaction(...)
                
                // For now, return a simulated txid
                let btc_txid = format!("{}btc", evm_txid.trim_start_matches("0x"));
                
                info!("Simulated Bitcoin transaction: {}", btc_txid);
                
                // Update record with Bitcoin txid
                if let Some(record) = records.get_mut(&transaction_id) {
                    record.btc_txid = Some(btc_txid.clone());
                    record.status = CrossLayerTransactionStatus::EvmConfirmed;
                    record.last_update = chrono::Utc::now().timestamp() as u64;
                }
                
                Ok(())
            },
            None => {
                // Transaction doesn't exist
                let error_msg = "EVM transaction not found".to_string();
                
                // Update status
                if let Some(record) = records.get_mut(&transaction_id) {
                    record.status = CrossLayerTransactionStatus::Failed(error_msg.clone());
                    record.last_update = chrono::Utc::now().timestamp() as u64;
                }
                
                Err(BobError::EvmError(error_msg))
            }
        }
    }
    
    /// Helper to create an EVM transaction request
    fn create_evm_tx_request(&self, tx: &EvmTransaction) -> Result<TransactionRequest, BobError> {
        let mut request = TransactionRequest::new();
        
        // Set from address
        let from = tx.from.parse()
            .map_err(|e| BobError::ParseError(format!("Invalid from address: {}", e)))?;
        request = request.from(from);
        
        // Set to address if present
        if let Some(to) = &tx.to {
            let to = to.parse()
                .map_err(|e| BobError::ParseError(format!("Invalid to address: {}", e)))?;
            request = request.to(to);
        }
        
        // Set value, gas limit and gas price
        request = request
            .value(U256::from(tx.value))
            .gas(U256::from(tx.gas_limit))
            .gas_price(U256::from(tx.gas_price));
            
        Ok(request)
    }
    
    /// Spawn a background task to monitor a transaction
    fn spawn_transaction_monitor(&self, transaction_id: String) {
        // In a real implementation, this would spawn a background task to monitor the transaction
        // For now, we'll just log
        debug!("Would monitor transaction {}", transaction_id);
        // Note: In a real implementation, you'd use tokio::spawn or similar to create a background task
    }
    
    /// Get the status of a cross-layer transaction
    pub async fn get_transaction_status(&self, transaction_id: &str) -> Result<CrossLayerTransactionStatus, BobError> {
        let records = self.transaction_records.read().await;
        
        match records.get(transaction_id) {
            Some(record) => Ok(record.status.clone()),
            None => Err(BobError::NotFoundError(format!("Transaction not found: {}", transaction_id))),
        }
    }
    
    /// Get detailed information about a cross-layer transaction
    pub async fn get_transaction_details(&self, transaction_id: &str) -> Result<TransactionRecord, BobError> {
        let records = self.transaction_records.read().await;
        
        match records.get(transaction_id) {
            Some(record) => Ok(record.clone()),
            None => Err(BobError::NotFoundError(format!("Transaction not found: {}", transaction_id))),
        }
    }
}

/// Bitcoin transaction type for cross-layer verification
#[derive(Debug, Clone)]
pub struct BtcTransaction {
    /// Transaction hash
    pub hash: String,
    /// Transaction data
    pub data: Vec<u8>,
}
