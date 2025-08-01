// DWN (Decentralized Web Node) Handler Implementation
// Author: Bo_theBig

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;
use axum::{
    extract::{Path, Query},
    response::Json,
    http::StatusCode,
};
use serde_json::Value;

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
    pub cursor: Option<String>,
    pub limit: Option<u32>,
}

/// DWN Handler for managing decentralized web node operations
pub struct DwnHandler {
    /// In-memory storage for messages (would be replaced with persistent storage)
    messages: RwLock<HashMap<String, DwnMessage>>,
    /// In-memory storage for records (would be replaced with persistent storage)
    records: RwLock<HashMap<String, DwnRecord>>,
}

impl DwnHandler {
    /// Create a new DWN handler instance
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
        let message_id = message.message_id.clone();
        messages.insert(message_id.clone(), message);

        Ok(message_id)
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
        let record_id = record.record_id.clone();
        records.insert(record_id.clone(), record);

        Ok(record_id)
    }

    /// Retrieve a DWN record by ID
    pub async fn get_record(&self, record_id: &str) -> Result<Option<DwnRecord>, String> {
        let records = self.records.read().await;
        Ok(records.get(record_id).cloned())
    }

    /// Query DWN records based on filter criteria
    pub async fn query_records(&self, query: DwnQuery) -> Result<Vec<DwnRecord>, String> {
        let records = self.records.read().await;
        let mut results: Vec<DwnRecord> = Vec::new();

        for record in records.values() {
            let mut matches = true;

            // Filter by recipient (DID)
            if let Some(ref recipient) = query.filter.recipient {
                if record.did != *recipient {
                    matches = false;
                }
            }

            // Filter by schema
            if let Some(ref schema) = query.filter.schema {
                match &record.schema {
                    Some(record_schema) => {
                        if record_schema != schema {
                            matches = false;
                        }
                    },
                    None => matches = false,
                }
            }

            // Filter by tags
            if let Some(ref filter_tags) = query.filter.tags {
                for (key, value) in filter_tags {
                    match record.tags.get(key) {
                        Some(record_value) => {
                            if record_value != value {
                                matches = false;
                                break;
                            }
                        },
                        None => {
                            matches = false;
                            break;
                        }
                    }
                }
            }

            if matches {
                results.push(record.clone());
            }
        }

        // Sort by date if specified
        if let Some(ref sort_order) = query.date_sort {
            match sort_order.as_str() {
                "createdAscending" => results.sort_by_key(|r| r.date_created),
                "createdDescending" => results.sort_by_key(|r| std::cmp::Reverse(r.date_created)),
                "publishedAscending" => results.sort_by_key(|r| r.date_modified),
                "publishedDescending" => results.sort_by_key(|r| std::cmp::Reverse(r.date_modified)),
                _ => {} // No sorting for unknown sort orders
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

impl Default for DwnHandler {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// HTTP API Handlers for Axum Routes  
// ============================================================================

/// List DWN protocols
pub async fn list_protocols() -> Result<Json<Value>, StatusCode> {
    let protocols = serde_json::json!({
        "protocols": [
            {
                "id": "https://dwn.example.com/protocol/messaging",
                "name": "DWN Messaging Protocol",
                "version": "1.0.0"
            },
            {
                "id": "https://dwn.example.com/protocol/records", 
                "name": "DWN Records Protocol",
                "version": "1.0.0"
            }
        ]
    });
    Ok(Json(protocols))
}

/// Create DWN protocol  
pub async fn create_protocol(Json(payload): Json<Value>) -> Result<Json<Value>, StatusCode> {
    let response = serde_json::json!({
        "success": true,
        "protocol": {
            "id": format!("protocol_{}", Uuid::new_v4()),
            "data": payload
        }
    });
    Ok(Json(response))
}

/// Query DWN records (route handler)
pub async fn query_records(Query(params): Query<HashMap<String, String>>) -> Result<Json<Value>, StatusCode> {
    let handler = DwnHandler::new();
    
    let filter = DwnFilter {
        recipient: params.get("recipient").cloned(),
        schema: params.get("schema").cloned(), 
        data_format: params.get("data_format").cloned(),
        tags: None,
    };
    
    let query = DwnQuery {
        filter,
        date_sort: params.get("date_sort").cloned(),
        pagination: None,
    };
    
    match handler.query_records(query).await {
        Ok(records) => Ok(Json(serde_json::json!({
            "success": true,
            "records": records
        }))),
        Err(e) => {
            tracing::error!("Failed to query records: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create DWN record (route handler)
pub async fn create_record(Json(payload): Json<DwnRecord>) -> Result<Json<Value>, StatusCode> {
    let handler = DwnHandler::new();
    
    match handler.store_record(payload).await {
        Ok(record_id) => Ok(Json(serde_json::json!({
            "success": true,
            "record_id": record_id
        }))),
        Err(e) => {
            tracing::error!("Failed to create record: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get DWN record by ID (route handler)
pub async fn get_record(Path(id): Path<String>) -> Result<Json<Value>, StatusCode> {
    let handler = DwnHandler::new();
    
    match handler.get_record(&id).await {
        Ok(Some(record)) => Ok(Json(serde_json::json!({
            "success": true,
            "record": record
        }))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get record: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Update DWN record (route handler)
pub async fn update_record(Path(id): Path<String>, Json(payload): Json<DwnRecord>) -> Result<Json<Value>, StatusCode> {
    let handler = DwnHandler::new();
    
    let mut updated_record = payload;
    updated_record.record_id = id.clone();
    updated_record.date_modified = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    match handler.store_record(updated_record).await {
        Ok(_) => Ok(Json(serde_json::json!({
            "success": true,
            "record_id": id
        }))),
        Err(e) => {
            tracing::error!("Failed to update record: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR) 
        }
    }
}

/// Delete DWN record (route handler)
pub async fn delete_record(Path(id): Path<String>) -> Result<Json<Value>, StatusCode> {
    let handler = DwnHandler::new();
    
    match handler.delete_record(&id).await {
        Ok(true) => Ok(Json(serde_json::json!({
            "success": true,
            "deleted": true
        }))),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to delete record: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
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
            data: serde_json::json!({"text": "Hello, Bob!"}),
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
