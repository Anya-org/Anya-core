// Decentralized Web Node (DWN) Implementation
// Provides storage and messaging capabilities for Web5
// [AIR-012] Operational Reliability and [AIP-002] Modular Architecture

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use crate::web5::{Web5Error, Web5Result};
use crate::web5::identity::{Web5Result as IdentityWeb5Result, Web5Error as IdentityWeb5Error, DID};

/// DWN configuration
#[derive(Clone, Debug)]
pub struct DWNConfig {
    /// DWN endpoint URL
    pub endpoint: Option<String>,
    /// Whether to use local storage
    pub use_local_storage: bool,
    /// Maximum message size in bytes
    pub max_message_size: usize,
}

impl Default for DWNConfig {
    fn default() -> Self {
        Self {
            endpoint: None,
            use_local_storage: true,
            max_message_size: 1024 * 1024, // 1 MB
        }
    }
}

/// DWN Message
/// 
/// Represents a message in the Decentralized Web Node.
#[derive(Clone, Debug)]
pub struct DWNMessage {
    /// Message ID
    pub id: String,
    /// DID of the sender
    pub from: String,
    /// DID of the recipient
    pub to: String,
    /// Message protocol
    pub protocol: String,
    /// Message type
    pub message_type: String,
    /// Message data
    pub data: Vec<u8>,
    /// Timestamp
    pub timestamp: u64,
}

/// DWN Client
/// 
/// Client for interacting with a Decentralized Web Node.
pub struct DWNClient {
    /// Configuration
    config: DWNConfig,
    /// Local storage for messages
    local_storage: Arc<Mutex<HashMap<String, DWNMessage>>>,
    /// Identity DID
    identity: Option<String>,
}

impl DWNClient {
    /// Create a new DWN client with the specified configuration
    pub fn new(config: DWNConfig) -> Self {
        Self {
            config,
            local_storage: Arc::new(Mutex::new(HashMap::new())),
            identity: None,
        }
    }
    
    /// Set the identity DID for the client
    pub fn set_identity(&mut self, did: &str) {
        self.identity = Some(did.to_string());
    }
    
    /// Send a message to a DID through the DWN
    pub fn send_message(&self, to: &str, protocol: &str, message_type: &str, data: &[u8]) -> Web5Result<String> {
        // Check if identity is set
        let from = self.identity.as_ref().ok_or_else(|| {
            Web5Error::Identity("Identity not set for DWN client".to_string())
        })?;
        
        // Check message size
        if data.len() > self.config.max_message_size {
            return Err(Web5Error::Communication(format!(
                "Message size exceeds maximum allowed: {} > {}",
                data.len(),
                self.config.max_message_size
            )));
        }
        
        // Create message ID
        let id = format!("msg_{}", generate_id());
        
        // Create message
        let message = DWNMessage {
            id: id.clone(),
            from: from.clone(),
            to: to.to_string(),
            protocol: protocol.to_string(),
            message_type: message_type.to_string(),
            data: data.to_vec(),
            timestamp: current_time(),
        };
        
        // Store locally if configured
        if self.config.use_local_storage {
            let mut storage = self.local_storage.lock().unwrap();
            storage.insert(id.clone(), message);
        }
        
        // Here we would send to remote DWN if endpoint is configured
        if let Some(endpoint) = &self.config.endpoint {
            // In a real implementation, this would send the message to the DWN
            // For this example, we're just logging
            println!("Would send message to DWN at {}: {:?}", endpoint, message);
        }
        
        Ok(id)
    }
    
    /// Get messages for the identity DID
    pub fn get_messages(&self, protocol: Option<&str>) -> Web5Result<Vec<DWNMessage>> {
        // Check if identity is set
        let identity = self.identity.as_ref().ok_or_else(|| {
            Web5Error::Identity("Identity not set for DWN client".to_string())
        })?;
        
        let storage = self.local_storage.lock().unwrap();
        
        // Filter messages by recipient and optionally by protocol
        let messages: Vec<DWNMessage> = storage.values()
            .filter(|msg| msg.to == *identity && 
                   protocol.map_or(true, |p| msg.protocol == p))
            .cloned()
            .collect();
        
        Ok(messages)
    }
}

/// Generate a random ID
fn generate_id() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    format!("{:x}", now)
}

/// Get current time in seconds
fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// DWN Manager
/// 
/// Manages Decentralized Web Nodes (DWNs) for Web5.
#[derive(Debug)]
pub struct DWNManager {
    /// DWN endpoints to connect to
    endpoints: Vec<String>,
    /// Records stored in DWNs
    records: Arc<Mutex<HashMap<String, DWNRecord>>>,
}

/// DWN Record
/// 
/// Represents a record stored in a Decentralized Web Node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DWNRecord {
    /// Record ID
    pub id: String,
    /// Record owner DID
    pub owner: String,
    /// Record schema
    pub schema: String,
    /// Record data
    pub data: serde_json::Value,
    /// Record metadata
    pub metadata: HashMap<String, String>,
    /// Record attestations
    pub attestations: Vec<Attestation>,
}

/// Attestation
/// 
/// Represents an attestation for a DWN record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attestation {
    /// Attestation issuer DID
    pub issuer: String,
    /// Attestation timestamp
    pub timestamp: u64,
    /// Attestation signature
    pub signature: String,
}

/// DWN Message Type
/// 
/// Represents the type of a DWN message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DWNMessageType {
    /// Create a record
    #[serde(rename = "create")]
    Create,
    /// Read a record
    #[serde(rename = "read")]
    Read,
    /// Update a record
    #[serde(rename = "update")]
    Update,
    /// Delete a record
    #[serde(rename = "delete")]
    Delete,
    /// Query records
    #[serde(rename = "query")]
    Query,
}

/// DWN Message Descriptor
/// 
/// Represents the descriptor of a DWN message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DWNMessageDescriptor {
    /// Message ID
    pub id: String,
    /// Message author DID
    pub author: String,
    /// Message recipient DID
    pub recipient: Option<String>,
    /// Message protocol
    pub protocol: Option<String>,
    /// Message schema
    pub schema: String,
    /// Message data format
    pub data_format: String,
    /// Message timestamp
    pub timestamp: u64,
}

/// DWN Query
/// 
/// Represents a query for DWN records.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DWNQuery {
    /// Query filter
    pub filter: DWNQueryFilter,
    /// Query pagination
    pub pagination: Option<DWNQueryPagination>,
}

/// DWN Query Filter
/// 
/// Represents a filter for DWN queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DWNQueryFilter {
    /// Filter by record owner
    pub owner: Option<String>,
    /// Filter by record schema
    pub schema: Option<String>,
    /// Filter by record protocol
    pub protocol: Option<String>,
    /// Filter by record attestation issuer
    pub attestation_issuer: Option<String>,
    /// Filter by record data
    pub data: Option<HashMap<String, serde_json::Value>>,
}

/// DWN Query Pagination
/// 
/// Represents pagination for DWN queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DWNQueryPagination {
    /// Pagination limit
    pub limit: u32,
    /// Pagination cursor
    pub cursor: Option<String>,
}

impl DWNManager {
    /// Create a new DWN Manager
    pub fn new(endpoints: Vec<String>) -> Self {
        Self {
            endpoints,
            records: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Store a record in a DWN
    pub fn store_record(&self, record: DWNRecord) -> Web5Result<String> {
        // In a real implementation, this would store the record in a DWN
        // For this example, we're storing it locally
        
        if let Ok(mut records) = self.records.lock() {
            records.insert(record.id.clone(), record.clone());
        }
        
        Ok(record.id)
    }
    
    /// Query records from a DWN
    pub fn query_records(&self, owner: &str, schema: &str) -> Web5Result<Vec<DWNRecord>> {
        // In a real implementation, this would query records from a DWN
        // For this example, we're querying locally
        
        let mut result = Vec::new();
        
        if let Ok(records) = self.records.lock() {
            for record in records.values() {
                if record.owner == owner && record.schema == schema {
                    result.push(record.clone());
                }
            }
        }
        
        Ok(result)
    }
    
    /// Delete a record from a DWN
    pub fn delete_record(&self, id: &str) -> Web5Result<()> {
        // In a real implementation, this would delete the record from a DWN
        // For this example, we're deleting it locally
        
        if let Ok(mut records) = self.records.lock() {
            records.remove(id);
        }
        
        Ok(())
    }
    
    /// Send a message to a DWN
    pub fn send_message(&self, message: DWNMessage) -> Web5Result<DWNMessage> {
        // In a real implementation, this would send the message to a DWN
        // For this example, we're handling it locally
        
        match message.message_type {
            DWNMessageType::Create => {
                // Create a record
                let data = message.data.ok_or_else(|| Web5Error::DWNError("No data provided".to_string()))?;
                
                let record = DWNRecord {
                    id: message.descriptor.id.clone(),
                    owner: message.descriptor.author.clone(),
                    schema: message.descriptor.schema.clone(),
                    data,
                    metadata: HashMap::new(),
                    attestations: message.attestations.clone(),
                };
                
                self.store_record(record)?;
                
                Ok(message)
            },
            DWNMessageType::Read => {
                // Read a record
                let id = message.descriptor.id.clone();
                
                if let Ok(records) = self.records.lock() {
                    if let Some(record) = records.get(&id) {
                        let mut response = message.clone();
                        response.data = Some(record.data.clone());
                        return Ok(response);
                    }
                }
                
                Err(Web5Error::DWNError(format!("Record not found: {}", id)))
            },
            DWNMessageType::Update => {
                // Update a record
                let id = message.descriptor.id.clone();
                let data = message.data.ok_or_else(|| Web5Error::DWNError("No data provided".to_string()))?;
                
                if let Ok(mut records) = self.records.lock() {
                    if let Some(record) = records.get_mut(&id) {
                        record.data = data;
                        record.attestations = message.attestations.clone();
                        return Ok(message);
                    }
                }
                
                Err(Web5Error::DWNError(format!("Record not found: {}", id)))
            },
            DWNMessageType::Delete => {
                // Delete a record
                let id = message.descriptor.id.clone();
                
                self.delete_record(&id)?;
                
                Ok(message)
            },
            DWNMessageType::Query => {
                // Query records
                let data = message.data.ok_or_else(|| Web5Error::DWNError("No query provided".to_string()))?;
                
                let query: DWNQuery = serde_json::from_value(data)
                    .map_err(|e| Web5Error::SerializationError(e.to_string()))?;
                
                let owner = query.filter.owner.unwrap_or_default();
                let schema = query.filter.schema.unwrap_or_default();
                
                let records = self.query_records(&owner, &schema)?;
                
                let mut response = message.clone();
                response.data = Some(serde_json::to_value(records)
                    .map_err(|e| Web5Error::SerializationError(e.to_string()))?);
                
                Ok(response)
            },
        }
    }
    
    /// Create a record in a DWN
    pub fn create_record(&self, owner: &str, schema: &str, data: serde_json::Value) -> Web5Result<String> {
        let id = generate_random_id();
        
        let record = DWNRecord {
            id: id.clone(),
            owner: owner.to_string(),
            schema: schema.to_string(),
            data,
            metadata: HashMap::new(),
            attestations: Vec::new(),
        };
        
        self.store_record(record)?;
        
        Ok(id)
    }
    
    /// Read a record from a DWN
    pub fn read_record(&self, id: &str) -> Web5Result<DWNRecord> {
        if let Ok(records) = self.records.lock() {
            if let Some(record) = records.get(id) {
                return Ok(record.clone());
            }
        }
        
        Err(Web5Error::DWNError(format!("Record not found: {}", id)))
    }
    
    /// Update a record in a DWN
    pub fn update_record(&self, id: &str, data: serde_json::Value) -> Web5Result<()> {
        if let Ok(mut records) = self.records.lock() {
            if let Some(record) = records.get_mut(id) {
                record.data = data;
                return Ok(());
            }
        }
        
        Err(Web5Error::DWNError(format!("Record not found: {}", id)))
    }
}

/// Generate a random ID
fn generate_random_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let id: u64 = rng.gen();
    format!("{:x}", id)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_store_record() {
        let dwn_manager = DWNManager::new(vec!["https://dwn.tbddev.org".to_string()]);
        
        let record = DWNRecord {
            id: "record1".to_string(),
            owner: "did:ion:123".to_string(),
            schema: "https://schema.org/Person".to_string(),
            data: serde_json::json!({
                "name": "Alice",
                "email": "alice@example.com"
            }),
            metadata: HashMap::new(),
            attestations: Vec::new(),
        };
        
        let id = dwn_manager.store_record(record.clone()).unwrap();
        assert_eq!(id, "record1");
        
        let records = dwn_manager.query_records("did:ion:123", "https://schema.org/Person").unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].id, "record1");
        assert_eq!(records[0].owner, "did:ion:123");
    }
    
    #[test]
    fn test_create_and_read_record() {
        let dwn_manager = DWNManager::new(vec!["https://dwn.tbddev.org".to_string()]);
        
        let data = serde_json::json!({
            "name": "Bob",
            "email": "bob@example.com"
        });
        
        let id = dwn_manager.create_record("did:ion:456", "https://schema.org/Person", data.clone()).unwrap();
        
        let record = dwn_manager.read_record(&id).unwrap();
        assert_eq!(record.owner, "did:ion:456");
        assert_eq!(record.schema, "https://schema.org/Person");
        assert_eq!(record.data, data);
    }
    
    #[test]
    fn test_update_record() {
        let dwn_manager = DWNManager::new(vec!["https://dwn.tbddev.org".to_string()]);
        
        let data = serde_json::json!({
            "name": "Charlie",
            "email": "charlie@example.com"
        });
        
        let id = dwn_manager.create_record("did:ion:789", "https://schema.org/Person", data.clone()).unwrap();
        
        let new_data = serde_json::json!({
            "name": "Charlie",
            "email": "charlie.updated@example.com"
        });
        
        dwn_manager.update_record(&id, new_data.clone()).unwrap();
        
        let record = dwn_manager.read_record(&id).unwrap();
        assert_eq!(record.data, new_data);
    }
    
    #[test]
    fn test_delete_record() {
        let dwn_manager = DWNManager::new(vec!["https://dwn.tbddev.org".to_string()]);
        
        let data = serde_json::json!({
            "name": "Dave",
            "email": "dave@example.com"
        });
        
        let id = dwn_manager.create_record("did:ion:abc", "https://schema.org/Person", data.clone()).unwrap();
        
        dwn_manager.delete_record(&id).unwrap();
        
        let result = dwn_manager.read_record(&id);
        assert!(result.is_err());
    }
} 