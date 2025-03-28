#![feature(edition2021)]
// SPDX-License-Identifier: MIT
// Copyright (C) 2023-2025 Anya Project Contributors

//! Decentralized Web Node (DWN) implementation for Web5
//!
//! DWNs are personal data stores that give users control over their data
//! and how it's accessed. This module provides functionality for creating,
//! managing, and interacting with DWNs.

use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bitcoin::Network;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

use super::did::DidManager;

/// Data format for messages stored in DWNs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DwnMessage {
    /// Message ID
    pub id: String,
    
    /// Descriptor for the message
    pub descriptor: MessageDescriptor,
    
    /// Message data (encrypted if needed)
    pub data: Option<Vec<u8>>,
    
    /// Attestation (signature) of the message
    pub attestation: Option<MessageAttestation>,
}

/// Descriptor for DWN messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDescriptor {
    /// Interface definition
    pub interface: String,
    
    /// Method being called
    pub method: String,
    
    /// Data format
    pub data_format: String,
    
    /// Schema for the data
    pub schema: Option<String>,
    
    /// URI for the data
    pub data_uri: Option<String>,
    
    /// Authorization details
    pub authorization: Option<MessageAuthorization>,
    
    /// Additional descriptor properties
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

/// Authorization for messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAuthorization {
    /// Authorization type
    pub type_: String,
    
    /// Authorization details
    #[serde(flatten)]
    pub details: HashMap<String, serde_json::Value>,
}

/// Message attestation (signature)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttestation {
    /// Attestation type
    pub type_: String,
    
    /// Did used for attestation
    pub did: String,
    
    /// Signature value
    pub signature: String,
}

/// Result of processing a DWN message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResult {
    /// Status of the operation
    pub status: MessageStatus,
    
    /// Any data returned (for queries)
    pub data: Option<Vec<u8>>,
    
    /// Entries returned (for collection queries)
    pub entries: Option<Vec<DwnMessage>>,
    
    /// Error information if the operation failed
    pub error: Option<MessageError>,
}

/// Status of a DWN message operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageStatus {
    /// Status code
    pub code: u16,
    
    /// Status message
    pub message: String,
}

/// Error information for DWN operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageError {
    /// Error code
    pub code: String,
    
    /// Error message
    pub message: String,
    
    /// Detailed error information
    pub details: Option<serde_json::Value>,
}

/// Interface for DWN operations
#[async_trait]
pub trait DwnInterface {
    /// Process a message
    async fn process_message(&self, message: DwnMessage) -> Result<MessageResult>;
    
    /// Query for messages
    async fn query(&self, query: DwnQueryMessage) -> Result<Vec<DwnMessage>>;
    
    /// Write a message
    async fn write(&self, message: DwnMessage) -> Result<()>;
    
    /// Delete a message
    async fn delete(&self, message_id: &str) -> Result<()>;
}

/// Query message for DWNs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DwnQueryMessage {
    /// ID of the query
    pub id: String,
    
    /// Descriptor of the query
    pub descriptor: QueryDescriptor,
    
    /// Attestation of the query
    pub attestation: Option<MessageAttestation>,
}

/// Descriptor for queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryDescriptor {
    /// Interface being queried
    pub interface: String,
    
    /// Method being used
    pub method: String,
    
    /// Data format being queried
    pub data_format: String,
    
    /// Schema being queried
    pub schema: Option<String>,
    
    /// Filter for the query
    pub filter: Option<QueryFilter>,
    
    /// Additional descriptor properties
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

/// Filter for queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilter {
    /// FROM filter
    pub from: Option<String>,
    
    /// WHERE conditions
    pub where_: Option<Vec<WhereCondition>>,
    
    /// ORDER BY conditions
    pub order_by: Option<Vec<OrderByCondition>>,
    
    /// LIMIT condition
    pub limit: Option<u32>,
}

/// WHERE condition for queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhereCondition {
    /// Path to the property
    pub path: Vec<String>,
    
    /// Operator for the condition
    pub op: String,
    
    /// Value for the condition
    pub value: serde_json::Value,
}

/// ORDER BY condition for queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderByCondition {
    /// Path to the property
    pub path: Vec<String>,
    
    /// Sort direction
    pub direction: String,
}

/// In-memory implementation of a DWN
pub struct MemoryDwn {
    /// Owner DID of the DWN
    owner_did: String,
    
    /// DID manager for identity operations
    did_manager: Arc<DidManager>,
    
    /// Messages stored in the DWN
    messages: Mutex<HashMap<String, DwnMessage>>,
}

impl MemoryDwn {
    /// Create a new in-memory DWN
    pub fn new(owner_did: String, did_manager: Arc<DidManager>) -> Self {
        Self {
            owner_did,
            did_manager,
            messages: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl DwnInterface for MemoryDwn {
    async fn process_message(&self, message: DwnMessage) -> Result<MessageResult> {
        // Process the message based on the interface and method
        match (message.descriptor.interface.as_str(), message.descriptor.method.as_str()) {
            ("Records", "Write") => {
                self.write(message).await?;
                Ok(MessageResult {
                    status: MessageStatus {
                        code: 200,
                        message: "OK".to_string(),
                    },
                    data: None,
                    entries: None,
                    error: None,
                })
            },
            ("Records", "Query") => {
                // Convert the message to a query message
                let query = DwnQueryMessage {
                    id: message.id,
                    descriptor: QueryDescriptor {
                        interface: message.descriptor.interface,
                        method: message.descriptor.method,
                        data_format: message.descriptor.data_format,
                        schema: message.descriptor.schema,
                        filter: None, // Extract filter from properties
                        properties: message.descriptor.properties,
                    },
                    attestation: message.attestation,
                };
                
                let entries = self.query(query).await?;
                
                Ok(MessageResult {
                    status: MessageStatus {
                        code: 200,
                        message: "OK".to_string(),
                    },
                    data: None,
                    entries: Some(entries),
                    error: None,
                })
            },
            ("Records", "Delete") => {
                // Extract the message ID from the descriptor
                let message_id = message.descriptor.properties.get("recordId")
                    .and_then(|id| id.as_str())
                    .ok_or_else(|| anyhow!("Missing recordId for Delete operation"))?;
                
                self.delete(message_id).await?;
                
                Ok(MessageResult {
                    status: MessageStatus {
                        code: 200,
                        message: "OK".to_string(),
                    },
                    data: None,
                    entries: None,
                    error: None,
                })
            },
            _ => {
                Err(anyhow!("Unsupported interface or method: {}/{}", 
                    message.descriptor.interface, message.descriptor.method))
            }
        }
    }
    
    async fn query(&self, query: DwnQueryMessage) -> Result<Vec<DwnMessage>> {
        let messages = self.messages.lock().await;
        
        // For now, just return all messages
        // TODO: Implement proper filtering
        Ok(messages.values().cloned().collect())
    }
    
    async fn write(&self, message: DwnMessage) -> Result<()> {
        // Verify the attestation
        if let Some(attestation) = &message.attestation {
            // TODO: Implement proper verification
        }
        
        // Store the message
        let mut messages = self.messages.lock().await;
        messages.insert(message.id.clone(), message);
        
        Ok(())
    }
    
    async fn delete(&self, message_id: &str) -> Result<()> {
        let mut messages = self.messages.lock().await;
        
        if messages.remove(message_id).is_none() {
            return Err(anyhow!("Message not found: {}", message_id));
        }
        
        Ok(())
    }
}

/// DWN Manager for creating and managing DWNs
pub struct DwnManager {
    /// DID Manager for identity operations
    did_manager: Arc<DidManager>,
    
    /// DWNs managed by this manager
    dwns: Mutex<HashMap<String, Arc<dyn DwnInterface + Send + Sync>>>,
}

impl DwnManager {
    /// Create a new DWN Manager
    pub fn new(did_manager: Arc<DidManager>) -> Self {
        Self {
            did_manager,
            dwns: Mutex::new(HashMap::new()),
        }
    }
    
    /// Create a new DWN for a DID
    pub async fn create_dwn(&self, owner_did: &str) -> Result<()> {
        // Verify that the DID exists
        let owner_did_doc = self.did_manager.resolve_did(owner_did).await?;
        
        // Create a new DWN
        let dwn = Arc::new(MemoryDwn::new(owner_did.to_string(), self.did_manager.clone()));
        
        // Store the DWN
        let mut dwns = self.dwns.lock().await;
        dwns.insert(owner_did.to_string(), dwn);
        
        Ok(())
    }
    
    /// Get a DWN for a DID
    pub async fn get_dwn(&self, owner_did: &str) -> Result<Arc<dyn DwnInterface + Send + Sync>> {
        let dwns = self.dwns.lock().await;
        
        dwns.get(owner_did)
            .cloned()
            .ok_or_else(|| anyhow!("DWN not found for DID: {}", owner_did))
    }
    
    /// Process a message for a DWN
    pub async fn process_message(&self, owner_did: &str, message: DwnMessage) -> Result<MessageResult> {
        let dwn = self.get_dwn(owner_did).await?;
        dwn.process_message(message).await
    }
}

/// Tests for DWN functionality
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use bitcoin::Network;
    use crate::wallet::{BitcoinWallet, WalletConfig};
    
    #[tokio::test]
    async fn test_dwn_operations() {
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
        let did_manager = Arc::new(DidManager::new(wallet_arc, Network::Testnet));
        
        // Create a DID
        let did_document = did_manager.create_did().await.unwrap();
        
        // Create DWN manager
        let dwn_manager = DwnManager::new(did_manager.clone());
        
        // Create a DWN for the DID
        dwn_manager.create_dwn(&did_document.id).await.unwrap();
        
        // Get the DWN
        let dwn = dwn_manager.get_dwn(&did_document.id).await.unwrap();
        
        // Create a message
        let message = DwnMessage {
            id: "msg1".to_string(),
            descriptor: MessageDescriptor {
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
                "email": "alice@example.com",
            })).unwrap()),
            attestation: None,
        };
        
        // Write the message
        dwn.write(message.clone()).await.unwrap();
        
        // Create a query
        let query = DwnQueryMessage {
            id: "query1".to_string(),
            descriptor: QueryDescriptor {
                interface: "Records".to_string(),
                method: "Query".to_string(),
                data_format: "application/json".to_string(),
                schema: Some("https://schema.org/Person".to_string()),
                filter: None,
                properties: HashMap::new(),
            },
            attestation: None,
        };
        
        // Execute the query
        let results = dwn.query(query).await.unwrap();
        
        // Verify the results
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "msg1");
        
        // Delete the message
        dwn.delete("msg1").await.unwrap();
        
        // Query again to verify deletion
        let query = DwnQueryMessage {
            id: "query2".to_string(),
            descriptor: QueryDescriptor {
                interface: "Records".to_string(),
                method: "Query".to_string(),
                data_format: "application/json".to_string(),
                schema: Some("https://schema.org/Person".to_string()),
                filter: None,
                properties: HashMap::new(),
            },
            attestation: None,
        };
        
        let results = dwn.query(query).await.unwrap();
        assert_eq!(results.len(), 0);
    }
}
