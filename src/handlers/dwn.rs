// DWN (Decentralized Web Node) Handler Implementation
// Author: Bo_theBig

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

/// DWN Message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DwnMessage {
    pub message_id: String,
    pub sender: String,
    pub recipient: String,
    pub data: serde_json::Value,
    pub timestamp: u64,
    pub signature: Option<String>,
}

/// DWN Record structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DwnRecord {
    pub record_id: String,
    pub did: String,
    pub schema: Option<String>,
    pub data: Vec<u8>,
    pub date_created: u64,
    pub date_modified: u64,
    pub tags: HashMap<String, String>,
}

/// DWN Query structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DwnQuery {
    pub filter: DwnFilter,
    pub date_sort: Option<String>,
    pub pagination: Option<DwnPagination>,
}

/// DWN Filter structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DwnFilter {
    pub recipient: Option<String>,
    pub schema: Option<String>,
    pub data_format: Option<String>,
    pub tags: Option<HashMap<String, String>>,
}

/// DWN Pagination structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DwnPagination {
    pub limit: u32,
    pub cursor: Option<String>,
}

/// DWN Handler for decentralized web node operations
#[derive(Debug)]
pub struct DwnHandler {
    messages: RwLock<HashMap<String, DwnMessage>>,
    records: RwLock<HashMap<String, DwnRecord>>,
}

impl Default for DwnHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DwnHandler {
    /// Create new DWN handler
    pub fn new() -> Self {
        Self {
            messages: RwLock::new(HashMap::new()),
            records: RwLock::new(HashMap::new()),
        }
    }

    /// Store a DWN message
    pub async fn store_message(&self, mut message: DwnMessage) -> Result<String, String> {
        if message.message_id.is_empty() {
            message.message_id = Uuid::new_v4().to_string();
        }

        let mut messages = self.messages.write().await;
        messages.insert(message.message_id.clone(), message.clone());
        Ok(message.message_id)
    }

    /// Retrieve a DWN message by ID
    pub async fn get_message(&self, message_id: &str) -> Result<Option<DwnMessage>, String> {
        let messages = self.messages.read().await;
        Ok(messages.get(message_id).cloned())
    }

    /// Store a DWN record
    pub async fn store_record(&self, mut record: DwnRecord) -> Result<String, String> {
        if record.record_id.is_empty() {
            record.record_id = Uuid::new_v4().to_string();
        }

        let mut records = self.records.write().await;
        records.insert(record.record_id.clone(), record.clone());
        Ok(record.record_id)
    }

    /// Retrieve a DWN record by ID
    pub async fn get_record(&self, record_id: &str) -> Result<Option<DwnRecord>, String> {
        let records = self.records.read().await;
        Ok(records.get(record_id).cloned())
    }

    /// Query DWN records
    pub async fn query_records(&self, query: DwnQuery) -> Result<Vec<DwnRecord>, String> {
        let records = self.records.read().await;
        let mut results: Vec<DwnRecord> = records
            .values()
            .filter(|record| {
                // Apply filters
                if let Some(ref recipient) = query.filter.recipient {
                    if record.did != *recipient {
                        return false;
                    }
                }
                if let Some(ref schema) = query.filter.schema {
                    if record.schema.as_ref() != Some(schema) {
                        return false;
                    }
                }
                if let Some(ref tags) = query.filter.tags {
                    for (key, value) in tags {
                        if record.tags.get(key) != Some(value) {
                            return false;
                        }
                    }
                }
                true
            })
            .cloned()
            .collect();

        // Sort by date if specified
        if let Some(ref sort) = query.date_sort {
            match sort.as_str() {
                "createdAscending" => results.sort_by_key(|r| r.date_created),
                "createdDescending" => results.sort_by_key(|r| std::cmp::Reverse(r.date_created)),
                _ => {} // No sorting
            }
        }

        // Apply pagination
        if let Some(ref pagination) = query.pagination {
            let limit = pagination.limit as usize;
            if results.len() > limit {
                results.truncate(limit);
            }
        }

        Ok(results)
    }

    /// Delete a DWN record
    pub async fn delete_record(&self, record_id: &str) -> Result<bool, String> {
        let mut records = self.records.write().await;
        Ok(records.remove(record_id).is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dwn_message_storage() {
        let handler = DwnHandler::new();
        let message = DwnMessage {
            message_id: String::new(),
            sender: "did:web:alice.example.com".to_string(),
            recipient: "did:web:bob.example.com".to_string(),
            data: serde_json::json!({"test": "data"}),
            timestamp: 1234567890,
            signature: None,
        };

        let message_id = handler.store_message(message).await.unwrap();
        let retrieved = handler.get_message(&message_id).await.unwrap();
        
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.sender, "did:web:alice.example.com");
        assert_eq!(retrieved.recipient, "did:web:bob.example.com");
    }

    #[tokio::test]
    async fn test_dwn_record_query() {
        let handler = DwnHandler::new();
        
        let record = DwnRecord {
            record_id: String::new(),
            did: "did:web:test.example.com".to_string(),
            schema: Some("https://schema.org/Person".to_string()),
            data: b"test data".to_vec(),
            date_created: 1234567890,
            date_modified: 1234567890,
            tags: HashMap::new(),
        };

        let record_id = handler.store_record(record).await.unwrap();
        
        let query = DwnQuery {
            filter: DwnFilter {
                recipient: Some("did:web:test.example.com".to_string()),
                schema: None,
                data_format: None,
                tags: None,
            },
            date_sort: None,
            pagination: None,
        };

        let results = handler.query_records(query).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].record_id, record_id);
    }
}