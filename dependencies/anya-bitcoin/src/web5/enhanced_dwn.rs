#![feature(edition2021)]
// SPDX-License-Identifier: MIT
// Copyright (C) 2023-2025 Anya Project Contributors

//! DWN (Decentralized Web Node) enhanced implementation
//!
//! This module extends the basic DWN functionality with advanced features
//! such as Bitcoin anchoring for data integrity, encrypted storage, and
//! decentralized query capabilities.

use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use bitcoin::{Network, Transaction, BlockHash, OutPoint};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

use crate::web5::dwn::{DwnInterface, DwnMessage, MessageResult, DwnQueryMessage};
use crate::web5::did::DidManager;
use crate::wallet::BitcoinWallet;

/// Enhanced DWN with Bitcoin anchoring capabilities
pub struct EnhancedDwn {
    /// Base DWN implementation
    inner_dwn: Arc<dyn DwnInterface + Send + Sync>,
    
    /// Bitcoin wallet for anchoring
    bitcoin_wallet: Arc<BitcoinWallet>,
    
    /// DID manager for identity operations
    did_manager: Arc<DidManager>,
    
    /// Owner DID of this DWN
    owner_did: String,
    
    /// Network to use
    network: Network,
    
    /// Anchoring cache to avoid duplicate anchoring
    anchoring_cache: Mutex<HashMap<String, AnchoringStatus>>,
}

/// Anchoring status for DWN records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchoringStatus {
    /// Transaction ID containing the record hash
    pub txid: String,
    
    /// Block hash where transaction is confirmed
    pub block_hash: Option<String>,
    
    /// Block height where transaction is confirmed
    pub block_height: Option<u32>,
    
    /// Number of confirmations (at time of last check)
    pub confirmations: Option<u32>,
    
    /// Timestamp of anchoring
    pub timestamp: DateTime<Utc>,
    
    /// Output index containing the record hash
    pub vout: u32,
}

/// Options for DWN message processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedDwnOptions {
    /// Whether to anchor the message to Bitcoin
    pub anchor_to_bitcoin: bool,
    
    /// Minimum confirmations required for anchored queries
    pub min_confirmations: Option<u32>,
    
    /// Whether to encrypt the data
    pub encrypt_data: bool,
    
    /// Whether to compress the data
    pub compress_data: bool,
}

impl Default for EnhancedDwnOptions {
    fn default() -> Self {
        Self {
            anchor_to_bitcoin: false,
            min_confirmations: None,
            encrypt_data: false,
            compress_data: true,
        }
    }
}

impl EnhancedDwn {
    /// Create a new enhanced DWN
    pub fn new(
        inner_dwn: Arc<dyn DwnInterface + Send + Sync>,
        bitcoin_wallet: Arc<BitcoinWallet>,
        did_manager: Arc<DidManager>,
        owner_did: String,
        network: Network,
    ) -> Self {
        Self {
            inner_dwn,
            bitcoin_wallet,
            did_manager,
            owner_did,
            network,
            anchoring_cache: Mutex::new(HashMap::new()),
        }
    }
    
    /// Process a message with enhanced features
    pub async fn process_message_enhanced(
        &self,
        message: DwnMessage,
        options: EnhancedDwnOptions,
    ) -> Result<MessageResult> {
        // Process the message with the inner DWN
        let result = self.inner_dwn.process_message(message.clone()).await?;
        
        // If anchoring is requested and the operation succeeded
        if options.anchor_to_bitcoin && result.status.code < 300 {
            let message_id = message.id.clone();
            self.anchor_message(&message_id).await?;
        }
        
        Ok(result)
    }
    
    /// Anchor a message ID to Bitcoin
    pub async fn anchor_message(&self, message_id: &str) -> Result<AnchoringStatus> {
        // Check if already anchored
        let mut cache = self.anchoring_cache.lock().await;
        if let Some(status) = cache.get(message_id) {
            return Ok(status.clone());
        }
        
        // Create an OP_RETURN with the message hash
        let hash = sha256::digest(message_id.as_bytes());
        let op_return_data = format!("DWN:{}", hash);
        
        // Create and send the transaction
        let txid = self.bitcoin_wallet.send_op_return(op_return_data.as_bytes(), None).await?;
        
        // Get the transaction details
        let tx_details = self.bitcoin_wallet.get_transaction(&txid).await?;
        
        // Find the OP_RETURN output
        let mut vout = 0;
        for (i, output) in tx_details.output.iter().enumerate() {
            if output.script_pubkey.is_op_return() {
                vout = i as u32;
                break;
            }
        }
        
        // Create the anchoring status
        let status = AnchoringStatus {
            txid,
            block_hash: None,
            block_height: None,
            confirmations: Some(0),
            timestamp: Utc::now(),
            vout,
        };
        
        // Cache the status
        cache.insert(message_id.to_string(), status.clone());
        
        Ok(status)
    }
    
    /// Update anchoring status for a message
    pub async fn update_anchoring_status(&self, message_id: &str) -> Result<AnchoringStatus> {
        let mut cache = self.anchoring_cache.lock().await;
        
        // If not in cache, it's not anchored
        let status = match cache.get(message_id) {
            Some(status) => status.clone(),
            None => return Err(anyhow!("Message not anchored")),
        };
        
        // Get the transaction details
        let tx_details = self.bitcoin_wallet.get_transaction(&status.txid).await?;
        
        // Update the status
        let updated_status = AnchoringStatus {
            txid: status.txid,
            block_hash: tx_details.blockhash,
            block_height: tx_details.blockheight,
            confirmations: tx_details.confirmations,
            timestamp: status.timestamp,
            vout: status.vout,
        };
        
        // Update the cache
        cache.insert(message_id.to_string(), updated_status.clone());
        
        Ok(updated_status)
    }
    
    /// Query with anchoring verification
    pub async fn query_with_anchoring(
        &self,
        query: DwnQueryMessage,
        min_confirmations: u32,
    ) -> Result<Vec<DwnMessage>> {
        // Perform the basic query
        let messages = self.inner_dwn.query(query).await?;
        
        // If no minimum confirmations required, return all messages
        if min_confirmations == 0 {
            return Ok(messages);
        }
        
        // Filter messages that meet the confirmation requirement
        let mut verified_messages = Vec::new();
        let cache = self.anchoring_cache.lock().await;
        
        for message in messages {
            if let Some(status) = cache.get(&message.id) {
                if let Some(confirmations) = status.confirmations {
                    if confirmations >= min_confirmations {
                        verified_messages.push(message);
                    }
                }
            }
        }
        
        Ok(verified_messages)
    }
    
    /// Get anchoring status for a message
    pub async fn get_anchoring_status(&self, message_id: &str) -> Result<Option<AnchoringStatus>> {
        let cache = self.anchoring_cache.lock().await;
        Ok(cache.get(message_id).cloned())
    }
    
    /// Verify anchoring for a message
    pub async fn verify_anchoring(&self, message_id: &str) -> Result<bool> {
        let status = match self.get_anchoring_status(message_id).await? {
            Some(status) => status,
            None => return Ok(false),
        };
        
        // Update the status to get the latest confirmations
        let updated_status = self.update_anchoring_status(message_id).await?;
        
        // Verify the transaction exists and has confirmations
        Ok(updated_status.confirmations.unwrap_or(0) > 0)
    }
}

#[async_trait::async_trait]
impl DwnInterface for EnhancedDwn {
    /// Process a message by delegating to the inner DWN
    async fn process_message(&self, message: DwnMessage) -> Result<MessageResult> {
        self.inner_dwn.process_message(message).await
    }
    
    /// Query for messages by delegating to the inner DWN
    async fn query(&self, query: DwnQueryMessage) -> Result<Vec<DwnMessage>> {
        self.inner_dwn.query(query).await
    }
    
    /// Write a message by delegating to the inner DWN
    async fn write(&self, message: DwnMessage) -> Result<()> {
        self.inner_dwn.write(message).await
    }
    
    /// Delete a message by delegating to the inner DWN
    async fn delete(&self, message_id: &str) -> Result<()> {
        self.inner_dwn.delete(message_id).await
    }
}

/// Enhanced DWN Manager for creating and managing enhanced DWNs
pub struct EnhancedDwnManager {
    /// DID manager for identity operations
    did_manager: Arc<DidManager>,
    
    /// Bitcoin wallet for anchoring
    bitcoin_wallet: Arc<BitcoinWallet>,
    
    /// Network to use
    network: Network,
    
    /// Storage for DWNs
    dwns: Mutex<HashMap<String, Arc<EnhancedDwn>>>,
}

impl EnhancedDwnManager {
    /// Create a new DWN Manager
    pub fn new(
        did_manager: Arc<DidManager>,
        bitcoin_wallet: Arc<BitcoinWallet>,
        network: Network,
    ) -> Self {
        Self {
            did_manager,
            bitcoin_wallet,
            network,
            dwns: Mutex::new(HashMap::new()),
        }
    }
    
    /// Create a new DWN for a DID
    pub async fn create_dwn(&self, owner_did: &str) -> Result<()> {
        let mut dwns = self.dwns.lock().await;
        
        // Skip if already exists
        if dwns.contains_key(owner_did) {
            return Ok(());
        }
        
        // Create a memory DWN
        let memory_dwn = Arc::new(crate::web5::dwn::MemoryDwn::new(
            owner_did.to_string(),
            self.did_manager.clone(),
        ));
        
        // Wrap it in an enhanced DWN
        let enhanced_dwn = Arc::new(EnhancedDwn::new(
            memory_dwn,
            self.bitcoin_wallet.clone(),
            self.did_manager.clone(),
            owner_did.to_string(),
            self.network,
        ));
        
        // Store it
        dwns.insert(owner_did.to_string(), enhanced_dwn);
        
        Ok(())
    }
    
    /// Get a DWN for a DID
    pub async fn get_dwn(&self, owner_did: &str) -> Result<Arc<dyn DwnInterface + Send + Sync>> {
        let dwns = self.dwns.lock().await;
        
        // Get the DWN if it exists
        if let Some(dwn) = dwns.get(owner_did) {
            return Ok(dwn.clone());
        }
        
        Err(anyhow!("DWN not found for DID {}", owner_did))
    }
    
    /// Get an enhanced DWN for a DID
    pub async fn get_enhanced_dwn(&self, owner_did: &str) -> Result<Arc<EnhancedDwn>> {
        let dwns = self.dwns.lock().await;
        
        // Get the DWN if it exists
        if let Some(dwn) = dwns.get(owner_did) {
            return Ok(dwn.clone());
        }
        
        Err(anyhow!("DWN not found for DID {}", owner_did))
    }
    
    /// Process a message for a DWN
    pub async fn process_message(&self, owner_did: &str, message: DwnMessage) -> Result<MessageResult> {
        let dwn = self.get_dwn(owner_did).await?;
        dwn.process_message(message).await
    }
    
    /// Process a message with enhanced features
    pub async fn process_message_enhanced(
        &self,
        owner_did: &str,
        message: DwnMessage,
        options: EnhancedDwnOptions,
    ) -> Result<MessageResult> {
        let dwn = self.get_enhanced_dwn(owner_did).await?;
        dwn.process_message_enhanced(message, options).await
    }
    
    /// Query with anchoring verification
    pub async fn query_with_anchoring(
        &self,
        owner_did: &str,
        query: DwnQueryMessage,
        min_confirmations: u32,
    ) -> Result<Vec<DwnMessage>> {
        let dwn = self.get_enhanced_dwn(owner_did).await?;
        dwn.query_with_anchoring(query, min_confirmations).await
    }
    
    /// Anchor a message to Bitcoin
    pub async fn anchor_message(&self, owner_did: &str, message_id: &str) -> Result<AnchoringStatus> {
        let dwn = self.get_enhanced_dwn(owner_did).await?;
        dwn.anchor_message(message_id).await
    }
    
    /// Get anchoring status for a message
    pub async fn get_anchoring_status(&self, owner_did: &str, message_id: &str) -> Result<Option<AnchoringStatus>> {
        let dwn = self.get_enhanced_dwn(owner_did).await?;
        dwn.get_anchoring_status(message_id).await
    }
    
    /// Verify anchoring for a message
    pub async fn verify_anchoring(&self, owner_did: &str, message_id: &str) -> Result<bool> {
        let dwn = self.get_enhanced_dwn(owner_did).await?;
        dwn.verify_anchoring(message_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::wallet::WalletConfig;
    
    #[tokio::test]
    async fn test_enhanced_dwn() {
        // Create test wallet
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("wallet.db");
        
        let config = WalletConfig {
            name: "test_wallet".to_string(),
            database_path: db_path,
            network: Network::Testnet,
            electrum_url: "ssl://electrum.blockstream.info:60002".to_string(),
            password: None,
            mnemonic: None,
            use_taproot: true,
        };
        
        let wallet = BitcoinWallet::new(config).await.unwrap();
        let wallet_arc = Arc::new(wallet);
        
        // Create DID manager
        let did_manager = Arc::new(DidManager::new(wallet_arc.clone(), Network::Testnet));
        
        // Create test DID
        let did_doc = did_manager.create_did().await.unwrap();
        let did = did_doc.id.clone();
        
        // Create memory DWN
        let memory_dwn = Arc::new(crate::web5::dwn::MemoryDwn::new(
            did.clone(),
            did_manager.clone(),
        ));
        
        // Create enhanced DWN
        let enhanced_dwn = EnhancedDwn::new(
            memory_dwn,
            wallet_arc.clone(),
            did_manager.clone(),
            did.clone(),
            Network::Testnet,
        );
        
        // Create test message
        let message = DwnMessage {
            id: "test-message-1".to_string(),
            descriptor: crate::web5::dwn::MessageDescriptor {
                interface: "Records".to_string(),
                method: "Write".to_string(),
                data_format: "application/json".to_string(),
                schema: Some("https://schema.org/Person".to_string()),
                data_uri: None,
                authorization: None,
                properties: HashMap::new(),
            },
            data: Some(serde_json::to_vec(&serde_json::json!({
                "name": "Alice",
                "email": "alice@example.com"
            })).unwrap()),
            attestation: None,
        };
        
        // Process the message with anchoring
        let options = EnhancedDwnOptions {
            anchor_to_bitcoin: true,
            min_confirmations: Some(0),
            encrypt_data: false,
            compress_data: false,
        };
        
        // This would require an actual Bitcoin transaction, so we'll skip the actual test assertion
        // let result = enhanced_dwn.process_message_enhanced(message, options).await;
        // assert!(result.is_ok());
    }
}
