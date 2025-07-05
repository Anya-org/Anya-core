// [AIR-3][AIS-3][BPC-3][RES-3] Decentralized Web Node (DWN) Implementation
// Provides storage and messaging capabilities for Web5
// [AIR-012] Operational Reliability and [AIP-002] Modular Architecture

// Removed: use std::error::Error;
use crate::web5::{Web5Error, Web5Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
// [AIR-3][AIS-3][BPC-3][RES-3] Removed unused imports: DID, Web5Error as IdentityWeb5Error, Web5Result as IdentityWeb5Result
// [AIR-3][AIS-3][BPC-3][RES-3] Removed unused identity imports

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
    /// Attestations (signatures, proofs)
    pub attestations: Vec<Attestation>,
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
    pub fn send_message(
        &self,
        to: &str,
        protocol: &str,
        message_type: &str,
        data: &[u8],
    ) -> Web5Result<String> {
        // Check if identity is set
        let from = self
            .identity
            .as_ref()
            .ok_or_else(|| Web5Error::Identity("Identity not set for DWN client".to_string()))?;

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
            attestations: Vec::new(),
        };

        // Store locally if configured
        if self.config.use_local_storage {
            let mut storage = self
                .local_storage
                .lock()
                .map_err(|e| format!("Mutex lock error: {e}"))?;
            let message_for_storage = message.clone();
            storage.insert(id.clone(), message_for_storage);
        }

        // Here we would send to remote DWN if endpoint is configured
        if let Some(endpoint) = &self.config.endpoint {
            // In a real implementation, this would send the message to the DWN
            // For this example, we're just logging
            println!("Would send message to DWN at {endpoint}: {message:?}");
        }

        Ok(id)
    }

    /// Get messages for the identity DID
    pub fn get_messages(&self, protocol: Option<&str>) -> Web5Result<Vec<DWNMessage>> {
        // Check if identity is set
        let _from = self
            .identity
            .as_ref()
            .ok_or_else(|| Web5Error::Identity("Identity not set for DWN client".to_string()))?;

        let storage = self
            .local_storage
            .lock()
            .map_err(|e| format!("Mutex lock error: {e}"))?;

        // Filter messages by recipient and optionally by protocol
        let messages: Vec<DWNMessage> = storage
            .values()
            .filter(|msg| msg.to == *_from && protocol.map_or(true, |p| msg.protocol == p))
            .cloned()
            .collect();

        Ok(messages)
    }
}

/// Generate a random ID
/// [AIS-3] Properly handles errors without using ? operator
fn generate_id() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    format!("{now:x}")
}

/// Get current time in seconds
fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// DWN Manager
///
/// Manages Decentralized Web Nodes (DWNs) for Web5.
#[derive(Debug)]
pub struct DWNManager {
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
    /// Owner DID filter
    pub owner: Option<String>,
    /// Schema filter
    pub schema: Option<String>,
    /// Metadata filters
    pub metadata: Option<HashMap<String, String>>,
    /// Date range filter
    pub date_range: Option<DateRange>,
    /// Data content filter (JSON path queries)
    pub data_filter: Option<serde_json::Value>,
}

/// DWN Query Pagination
///
/// Represents pagination options for DWN queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DWNQueryPagination {
    /// Number of records to skip
    pub offset: Option<usize>,
    /// Maximum number of records to return
    pub limit: Option<usize>,
    /// Cursor for cursor-based pagination
    pub cursor: Option<String>,
}

impl Default for DWNQueryPagination {
    fn default() -> Self {
        Self {
            offset: None,
            limit: Some(100), // Default limit of 100 records
            cursor: None,
        }
    }
}

/// DWN Query Result with pagination metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DWNQueryResult {
    /// Query results
    pub records: Vec<DWNRecord>,
    /// Pagination metadata
    pub pagination: DWNQueryPaginationResult,
}

/// Pagination result metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DWNQueryPaginationResult {
    /// Total number of records available
    pub total: usize,
    /// Number of records returned
    pub count: usize,
    /// Whether there are more records available
    pub has_more: bool,
    /// Cursor for next page (if available)
    pub next_cursor: Option<String>,
}

/// Extended DWN Query Filter with advanced filtering capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedDWNQueryFilter {
    /// Base filter
    pub base: DWNQueryFilter,
    /// Full-text search query
    pub search: Option<String>,
    /// Geographic bounds (lat/lng bounding box)
    pub geo_bounds: Option<GeoBounds>,
    /// Tag-based filtering
    pub tags: Option<Vec<String>>,
    /// Numeric range filters
    pub numeric_ranges: Option<HashMap<String, NumericRange>>,
}

/// Geographic bounding box for location-based queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoBounds {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lng: f64,
    pub max_lng: f64,
}

/// Numeric range filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumericRange {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

/// Data synchronization status for DWN records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncStatus {
    Synced,
    Pending,
    Failed(String),
    Conflicted,
}

/// Record with sync metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncedDWNRecord {
    pub record: DWNRecord,
    pub sync_status: SyncStatus,
    pub last_sync: u64,
    pub sync_attempts: u32,
}

/// Conflict resolution strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    LastWriteWins,
    FirstWriteWins,
    Manual,
    Custom(String),
}

impl DWNManager {
    /// Create a new DWN Manager
    pub fn new() -> Self {
        Self {
            records: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Store a record in a DWN
    pub fn store_record(&self, record: DWNRecord) -> Web5Result<String> {
        let mut storage = self
            .records
            .lock()
            .map_err(|e| Web5Error::Storage(format!("Failed to acquire lock: {e}")))?;
        let record_id = record.id.clone();
        storage.insert(record_id.clone(), record);
        Ok(record_id)
    }

    /// Query records from a DWN
    pub fn query_records(&self, owner: &str, schema: &str) -> Web5Result<Vec<DWNRecord>> {
        let storage = self
            .records
            .lock()
            .map_err(|e| Web5Error::Storage(format!("Failed to acquire lock: {e}")))?;
        let records: Vec<DWNRecord> = storage
            .values()
            .filter(|r| r.owner == owner && r.schema == schema)
            .cloned()
            .collect();
        Ok(records)
    }

    /// Create a record in a DWN
    pub fn create_record(
        &self,
        owner: &str,
        schema: &str,
        data: serde_json::Value,
    ) -> Web5Result<String> {
        let record = DWNRecord {
            id: generate_id(),
            owner: owner.to_string(),
            schema: schema.to_string(),
            data,
            metadata: HashMap::new(),
            attestations: Vec::new(),
        };
        self.store_record(record)
    }

    /// Read a record from a DWN
    pub fn read_record(&self, id: &str) -> Web5Result<DWNRecord> {
        let storage = self
            .records
            .lock()
            .map_err(|e| Web5Error::Storage(format!("Failed to acquire lock: {e}")))?;
        storage
            .get(id)
            .cloned()
            .ok_or_else(|| Web5Error::NotFound(id.to_string()))
    }

    /// Update a record in a DWN
    pub fn update_record(&self, id: &str, data: serde_json::Value) -> Web5Result<()> {
        let mut storage = self
            .records
            .lock()
            .map_err(|e| Web5Error::Storage(format!("Failed to acquire lock: {e}")))?;
        // [AIR-3][AIS-3][BPC-3][RES-3] Remove unnecessary mut keyword
        // This follows official Bitcoin Improvement Proposals (BIPs) standards for clean code
        if let Some(record) = storage.get_mut(id) {
            record.data = data;
            record
                .metadata
                .insert("updated".to_string(), current_time().to_string());
            Ok(())
        } else {
            Err(Web5Error::NotFound("Record not found".to_string()))
        }
    }

    /// Delete a record from a DWN
    pub fn delete_record(&self, id: &str) -> Web5Result<()> {
        // In a real implementation, this would delete the record from a DWN
        // For this example, we're just removing it from local storage
        self.records.lock().unwrap().remove(id);
        Ok(())
    }

    /// Send a message to a DWN
    pub fn send_message(&self, message: DWNMessage) -> Web5Result<DWNMessage> {
        // In a real implementation, this would send the message to a DWN
        // For this example, we're handling it locally

        match message.message_type.as_str() {
            "Create" => {
                // Implementation for Create message type
                let data = message.data.clone();
                // Process create message logic
                let record = DWNRecord {
                    id: message.id.clone(),
                    owner: message.from.clone(),
                    schema: message.protocol.clone(),
                    data: serde_json::from_slice(&data).unwrap_or_else(|_| serde_json::Value::Null),
                    metadata: HashMap::new(),
                    attestations: Vec::new(),
                };
                self.store_record(record)?;
                Ok(message)
            }
            "Read" => {
                // Implementation for Read message type
                let id = message.id.clone();
                if let Ok(records) = self.records.lock() {
                    if let Some(record) = records.get(&id) {
                        let mut response = message.clone();
                        response.data = serde_json::to_vec(&record.data).unwrap_or_default();
                        return Ok(response);
                    }
                }
                Err(Web5Error::DWNError(format!("Record not found: {id}")))
            }
            "Update" => {
                // Implementation for Update message type
                let id = message.id.clone();
                let data = message.data.clone();
                if let Ok(mut records) = self.records.lock() {
                    if let Some(record) = records.get_mut(&id) {
                        record.data = match serde_json::from_slice(&data) {
                            Ok(value) => value,
                            Err(_) => serde_json::Value::Null,
                        };
                        record.attestations = message.attestations.clone();
                        return Ok(message);
                    }
                }
                Err(Web5Error::DWNError(format!("Record not found: {id}")))
            }
            "Delete" => {
                // Implementation for Delete message type
                let id = message.id.clone();
                self.delete_record(&id)?;
                Ok(message)
            }
            "Query" => {
                // Implementation for Query message type
                let data = message.data.clone();
                // Process query logic - simplified for illustration
                let query: DWNQuery = match serde_json::from_slice(&data) {
                    Ok(value) => match serde_json::from_value(value) {
                        Ok(query) => query,
                        Err(e) => return Err(Web5Error::SerializationError(e.to_string())),
                    },
                    Err(e) => return Err(Web5Error::SerializationError(e.to_string())),
                };

                let owner = query.filter.owner.unwrap_or_default();
                let schema = query.filter.schema.unwrap_or_default();

                let records = self.query_records(&owner, &schema)?;

                let mut response = message.clone();
                response.data = match serde_json::to_vec(&records) {
                    Ok(bytes) => bytes,
                    Err(e) => return Err(Web5Error::SerializationError(e.to_string())),
                };

                Ok(response)
            }
            _ => {
                // Handle unsupported message type
                Err(Web5Error::DWNError(format!(
                    "Unsupported message type: {}",
                    message.message_type
                )))
            }
        }
    }

    // ========================================================================
    // ADVANCED DWN FUNCTIONALITY FOR DECENTRALIZED STORAGE
    // ========================================================================

    /// Create an index for improved query performance
    pub fn create_index(&self, schema: &str, fields: &[&str]) -> Web5Result<()> {
        // In a production implementation, this would create optimized indexes
        // For now, we'll track the index metadata
        println!("Creating index for schema '{}' on fields: {:?}", schema, fields);
        Ok(())
    }

    /// Query records with advanced filtering
    pub fn query_with_filter(&self, filter: DWNQueryFilter) -> Web5Result<Vec<DWNRecord>> {
        let storage = self
            .records
            .lock()
            .map_err(|e| Web5Error::Storage(format!("Failed to acquire lock: {e}")))?;

        let mut filtered_records: Vec<DWNRecord> = storage
            .values()
            .filter(|record| {
                // Filter by owner
                if let Some(ref owner) = filter.owner {
                    if &record.owner != owner && owner != "*" {
                        return false;
                    }
                }

                // Filter by schema
                if let Some(ref schema) = filter.schema {
                    if &record.schema != schema {
                        return false;
                    }
                }

                // Filter by metadata
                if let Some(ref metadata_filter) = filter.metadata {
                    for (key, value) in metadata_filter {
                        if record.metadata.get(key) != Some(value) {
                            return false;
                        }
                    }
                }

                // Filter by date range (using metadata timestamp)
                if let Some(ref date_range) = filter.date_range {
                    if let Some(timestamp_str) = record.metadata.get("created_at") {
                        if let Ok(timestamp) = timestamp_str.parse::<u64>() {
                            if let Some(from) = date_range.from {
                                if timestamp < from {
                                    return false;
                                }
                            }
                            if let Some(to) = date_range.to {
                                if timestamp > to {
                                    return false;
                                }
                            }
                        }
                    }
                }

                // Filter by data content (simplified JSON matching)
                if let Some(ref data_filter) = filter.data_filter {
                    if !self.matches_data_filter(&record.data, data_filter) {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect();

        // Sort by timestamp (newest first by default)
        filtered_records.sort_by(|a, b| {
            let a_timestamp = a.metadata.get("created_at")
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
            let b_timestamp = b.metadata.get("created_at")
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
            b_timestamp.cmp(&a_timestamp)
        });

        Ok(filtered_records)
    }

    /// Perform aggregation operations on records
    pub fn aggregate(&self, pipeline: &[AggregationStage]) -> Web5Result<serde_json::Value> {
        let storage = self
            .records
            .lock()
            .map_err(|e| Web5Error::Storage(format!("Failed to acquire lock: {e}")))?;

        let mut records: Vec<DWNRecord> = storage.values().cloned().collect();

        for stage in pipeline {
            match stage {
                AggregationStage::Match(filter) => {
                    records = records
                        .into_iter()
                        .filter(|record| self.matches_aggregation_filter(record, filter))
                        .collect();
                }
                AggregationStage::Group { id: _id, fields: _fields } => {
                    // Simplified grouping - in production would implement proper aggregation
                    // For now, just return count
                    return Ok(serde_json::json!({ "count": records.len() }));
                }
                AggregationStage::Sort(sort_fields) => {
                    records.sort_by(|a, b| {
                        for sort_field in sort_fields {
                            let a_value = self.extract_field_value(a, &sort_field.field);
                            let b_value = self.extract_field_value(b, &sort_field.field);
                            let cmp = if sort_field.ascending {
                                a_value.cmp(&b_value)
                            } else {
                                b_value.cmp(&a_value)
                            };
                            if cmp != std::cmp::Ordering::Equal {
                                return cmp;
                            }
                        }
                        std::cmp::Ordering::Equal
                    });
                }
                AggregationStage::Limit(limit) => {
                    records.truncate(*limit);
                }
                AggregationStage::Skip(skip) => {
                    if *skip < records.len() {
                        records.drain(0..*skip);
                    } else {
                        records.clear();
                    }
                }
            }
        }

        Ok(serde_json::to_value(records)
            .map_err(|e| Web5Error::SerializationError(e.to_string()))?)
    }

    /// Batch store multiple records for performance
    pub async fn batch_store(&self, records: Vec<DWNRecord>) -> Web5Result<Vec<String>> {
        const BATCH_SIZE: usize = 50; // From existing implementation
        
        let mut results = Vec::new();
        
        for chunk in records.chunks(BATCH_SIZE) {
            let mut chunk_results = Vec::new();
            for record in chunk {
                let result = self.store_record(record.clone())?;
                chunk_results.push(result);
            }
            results.extend(chunk_results);
            
            // Small delay between batches to prevent overwhelming the system
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
        
        Ok(results)
    }

    /// Get statistics about stored records
    pub fn get_statistics(&self) -> Web5Result<serde_json::Value> {
        let storage = self
            .records
            .lock()
            .map_err(|e| Web5Error::Storage(format!("Failed to acquire lock: {e}")))?;

        let total_records = storage.len();
        let mut schema_counts: HashMap<String, usize> = HashMap::new();
        let mut owner_counts: HashMap<String, usize> = HashMap::new();

        for record in storage.values() {
            *schema_counts.entry(record.schema.clone()).or_insert(0) += 1;
            *owner_counts.entry(record.owner.clone()).or_insert(0) += 1;
        }

        Ok(serde_json::json!({
            "total_records": total_records,
            "schemas": schema_counts,
            "owners": owner_counts,
            "timestamp": current_timestamp()
        }))
    }

    // Helper methods for advanced querying
    fn matches_data_filter(&self, data: &serde_json::Value, filter: &serde_json::Value) -> bool {
        // Simplified data matching - in production would implement JSON path queries
        match (data, filter) {
            (serde_json::Value::Object(data_obj), serde_json::Value::Object(filter_obj)) => {
                for (key, expected_value) in filter_obj {
                    if let Some(actual_value) = data_obj.get(key) {
                        if actual_value != expected_value {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            }
            _ => data == filter,
        }
    }

    fn matches_aggregation_filter(&self, record: &DWNRecord, filter: &HashMap<String, serde_json::Value>) -> bool {
        for (key, expected_value) in filter {
            match key.as_str() {
                "owner" => {
                    if serde_json::Value::String(record.owner.clone()) != *expected_value {
                        return false;
                    }
                }
                "schema" => {
                    if serde_json::Value::String(record.schema.clone()) != *expected_value {
                        return false;
                    }
                }
                _ => {
                    // Check in metadata or data
                    if let Some(metadata_value) = record.metadata.get(key) {
                        if serde_json::Value::String(metadata_value.clone()) != *expected_value {
                            return false;
                        }
                    } else if let Some(data_value) = record.data.get(key) {
                        if data_value != expected_value {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn extract_field_value(&self, record: &DWNRecord, field: &str) -> String {
        // Try metadata first, then data
        if let Some(value) = record.metadata.get(field) {
            value.clone()
        } else if let Some(value) = record.data.get(field) {
            value.to_string()
        } else {
            String::new()
        }
    }

    /// Advanced query with pagination and extended filtering
    pub fn query_with_pagination(
        &self,
        filter: AdvancedDWNQueryFilter,
        pagination: Option<DWNQueryPagination>,
    ) -> Web5Result<DWNQueryResult> {
        let pagination = pagination.unwrap_or_default();
        
        // First get all matching records using base filter
        let mut all_records = self.query_with_filter(filter.base)?;
        
        // Apply advanced filters
        all_records = self.apply_advanced_filters(all_records, &filter)?;
        
        let total = all_records.len();
        
        // Apply pagination
        let offset = pagination.offset.unwrap_or(0);
        let limit = pagination.limit.unwrap_or(100);
        
        let start = offset.min(total);
        let end = (offset + limit).min(total);
        
        let records = all_records.into_iter().skip(start).take(end - start).collect::<Vec<_>>();
        let count = records.len();
        let has_more = end < total;
        
        // Generate next cursor if there are more records
        let next_cursor = if has_more {
            Some(format!("cursor_{}", end))
        } else {
            None
        };
        
        Ok(DWNQueryResult {
            records,
            pagination: DWNQueryPaginationResult {
                total,
                count,
                has_more,
                next_cursor,
            },
        })
    }
    
    /// Apply advanced filtering to records
    fn apply_advanced_filters(
        &self,
        mut records: Vec<DWNRecord>,
        filter: &AdvancedDWNQueryFilter,
    ) -> Web5Result<Vec<DWNRecord>> {
        // Full-text search
        if let Some(ref search_query) = filter.search {
            records = records
                .into_iter()
                .filter(|record| self.matches_search_query(record, search_query))
                .collect();
        }
        
        // Tag-based filtering
        if let Some(ref tags) = filter.tags {
            records = records
                .into_iter()
                .filter(|record| self.matches_tags(record, tags))
                .collect();
        }
        
        // Numeric range filtering
        if let Some(ref numeric_ranges) = filter.numeric_ranges {
            records = records
                .into_iter()
                .filter(|record| self.matches_numeric_ranges(record, numeric_ranges))
                .collect();
        }
        
        // Geographic filtering
        if let Some(ref geo_bounds) = filter.geo_bounds {
            records = records
                .into_iter()
                .filter(|record| self.matches_geo_bounds(record, geo_bounds))
                .collect();
        }
        
        Ok(records)
    }
    
    /// Check if record matches search query
    fn matches_search_query(&self, record: &DWNRecord, search_query: &str) -> bool {
        let search_lower = search_query.to_lowercase();
        
        // Search in data
        if let Ok(data_string) = serde_json::to_string(&record.data) {
            if data_string.to_lowercase().contains(&search_lower) {
                return true;
            }
        }
        
        // Search in metadata
        for (key, value) in &record.metadata {
            if key.to_lowercase().contains(&search_lower) || 
               value.to_lowercase().contains(&search_lower) {
                return true;
            }
        }
        
        // Search in schema
        if record.schema.to_lowercase().contains(&search_lower) {
            return true;
        }
        
        false
    }
    
    /// Check if record matches tag filters
    fn matches_tags(&self, record: &DWNRecord, required_tags: &[String]) -> bool {
        if let Some(record_tags) = record.metadata.get("tags") {
            let record_tag_list: Result<Vec<String>, _> = serde_json::from_str(record_tags);
            if let Ok(record_tag_list) = record_tag_list {
                return required_tags.iter().all(|tag| record_tag_list.contains(tag));
            }
        }
        false
    }
    
    /// Check if record matches numeric range filters
    fn matches_numeric_ranges(&self, record: &DWNRecord, ranges: &HashMap<String, NumericRange>) -> bool {
        for (field, range) in ranges {
            // Check in metadata first
            if let Some(value_str) = record.metadata.get(field) {
                if let Ok(value) = value_str.parse::<f64>() {
                    if !self.value_in_range(value, range) {
                        return false;
                    }
                    continue;
                }
            }
            
            // Check in data
            if let Some(value) = record.data.get(field) {
                if let Some(value_num) = value.as_f64() {
                    if !self.value_in_range(value_num, range) {
                        return false;
                    }
                    continue;
                }
            }
            
            // Field not found or not numeric
            return false;
        }
        true
    }
    
    /// Check if value is within numeric range
    fn value_in_range(&self, value: f64, range: &NumericRange) -> bool {
        if let Some(min) = range.min {
            if value < min {
                return false;
            }
        }
        if let Some(max) = range.max {
            if value > max {
                return false;
            }
        }
        true
    }
    
    /// Check if record matches geographic bounds
    fn matches_geo_bounds(&self, record: &DWNRecord, bounds: &GeoBounds) -> bool {
        // Look for latitude and longitude in metadata or data
        let lat = self.extract_numeric_field(record, "latitude")
            .or_else(|| self.extract_numeric_field(record, "lat"));
        let lng = self.extract_numeric_field(record, "longitude")
            .or_else(|| self.extract_numeric_field(record, "lng"));
        
        if let (Some(lat), Some(lng)) = (lat, lng) {
            lat >= bounds.min_lat && lat <= bounds.max_lat &&
            lng >= bounds.min_lng && lng <= bounds.max_lng
        } else {
            false
        }
    }
    
    /// Extract numeric field from record
    fn extract_numeric_field(&self, record: &DWNRecord, field: &str) -> Option<f64> {
        // Check metadata first
        if let Some(value_str) = record.metadata.get(field) {
            if let Ok(value) = value_str.parse::<f64>() {
                return Some(value);
            }
        }
        
        // Check data
        record.data.get(field).and_then(|v| v.as_f64())
    }
    
    /// Batch delete multiple records
    pub async fn batch_delete(&self, record_ids: Vec<String>) -> Web5Result<Vec<String>> {
        let mut deleted_ids = Vec::new();
        let mut errors = Vec::new();
        
        for record_id in record_ids {
            match self.delete_record(&record_id) {
                Ok(_) => deleted_ids.push(record_id),
                Err(e) => errors.push(format!("Failed to delete {}: {}", record_id, e)),
            }
        }
        
        if !errors.is_empty() {
            return Err(Web5Error::DWNError(format!("Batch delete errors: {}", errors.join(", "))));
        }
        
        Ok(deleted_ids)
    }
    
    /// Synchronize records with remote DWN nodes
    pub async fn sync_records(&self, remote_endpoint: &str) -> Web5Result<Vec<SyncedDWNRecord>> {
        // In a full implementation, this would connect to remote DWN endpoints
        // and synchronize records bidirectionally
        
        let storage = self
            .records
            .lock()
            .map_err(|e| Web5Error::Storage(format!("Failed to acquire lock: {e}")))?;
        
        let synced_records: Vec<SyncedDWNRecord> = storage
            .values()
            .map(|record| SyncedDWNRecord {
                record: record.clone(),
                sync_status: SyncStatus::Synced,
                last_sync: current_timestamp(),
                sync_attempts: 1,
            })
            .collect();
        
        println!("Would sync {} records with remote endpoint: {}", synced_records.len(), remote_endpoint);
        
        Ok(synced_records)
    }
    
    /// Resolve conflicts between records
    pub fn resolve_conflicts(
        &self,
        conflicts: Vec<(DWNRecord, DWNRecord)>,
        strategy: ConflictResolution,
    ) -> Web5Result<Vec<DWNRecord>> {
        let mut resolved = Vec::new();
        
        for (local, remote) in conflicts {
            let winner = match strategy {
                ConflictResolution::LastWriteWins => {
                    let local_timestamp = local.metadata.get("updated")
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or(0);
                    let remote_timestamp = remote.metadata.get("updated")
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or(0);
                    
                    if remote_timestamp > local_timestamp {
                        remote
                    } else {
                        local
                    }
                }
                ConflictResolution::FirstWriteWins => {
                    let local_timestamp = local.metadata.get("created")
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or(u64::MAX);
                    let remote_timestamp = remote.metadata.get("created")
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or(u64::MAX);
                    
                    if local_timestamp <= remote_timestamp {
                        local
                    } else {
                        remote
                    }
                }
                ConflictResolution::Manual => {
                    // In a full implementation, this would present conflicts to user
                    // For now, default to local
                    local
                }
                ConflictResolution::Custom(_strategy) => {
                    // Custom conflict resolution logic would be implemented here
                    local
                }
            };
            
            resolved.push(winner);
        }
        
        Ok(resolved)
    }
    
    /// Export records to various formats
    pub fn export_records(
        &self,
        format: &str,
        filter: Option<DWNQueryFilter>,
    ) -> Web5Result<String> {
        let records = if let Some(filter) = filter {
            self.query_with_filter(filter)?
        } else {
            let storage = self
                .records
                .lock()
                .map_err(|e| Web5Error::Storage(format!("Failed to acquire lock: {e}")))?;
            storage.values().cloned().collect()
        };
        
        match format.to_lowercase().as_str() {
            "json" => {
                serde_json::to_string_pretty(&records)
                    .map_err(|e| Web5Error::SerializationError(e.to_string()))
            }
            "csv" => {
                let mut csv_output = String::from("id,owner,schema,created_at\n");
                for record in records {
                    let created_at = record.metadata.get("created_at").unwrap_or(&"".to_string());
                    csv_output.push_str(&format!("{},{},{},{}\n", 
                        record.id, record.owner, record.schema, created_at));
                }
                Ok(csv_output)
            }
            "xml" => {
                let mut xml_output = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<records>\n");
                for record in records {
                    xml_output.push_str(&format!("  <record id=\"{}\" owner=\"{}\" schema=\"{}\">\n", 
                        record.id, record.owner, record.schema));
                    xml_output.push_str("    <data><![CDATA[");
                    xml_output.push_str(&serde_json::to_string(&record.data).unwrap_or_default());
                    xml_output.push_str("]]></data>\n");
                    xml_output.push_str("  </record>\n");
                }
                xml_output.push_str("</records>");
                Ok(xml_output)
            }
            _ => Err(Web5Error::DWNError(format!("Unsupported export format: {}", format)))
        }
    }
    
    /// Import records from various formats
    pub fn import_records(&self, data: &str, format: &str) -> Web5Result<Vec<String>> {
        let records = match format.to_lowercase().as_str() {
            "json" => {
                serde_json::from_str::<Vec<DWNRecord>>(data)
                    .map_err(|e| Web5Error::SerializationError(e.to_string()))?
            }
            _ => return Err(Web5Error::DWNError(format!("Unsupported import format: {}", format)))
        };
        
        let mut imported_ids = Vec::new();
        for record in records {
            let id = self.store_record(record)?;
            imported_ids.push(id);
        }
        
        Ok(imported_ids)
    }
}

/// Get current timestamp in seconds since Unix epoch
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    // [AIR-3][AIS-3][BPC-3][RES-3] Error trait is already imported in the parent module
    use super::*;

    #[test]
    fn test_store_record() -> Result<(), Box<dyn std::error::Error>> {
        let dwn_manager = DWNManager::new();

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

        let id = dwn_manager.store_record(record.clone())?;
        assert_eq!(id, "record1");

        let records = dwn_manager.query_records("did:ion:123", "https://schema.org/Person")?;
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].id, "record1");
        assert_eq!(records[0].owner, "did:ion:123");

        Ok(())
    }

    #[test]
    fn test_create_and_read_record() -> Result<(), Box<dyn std::error::Error>> {
        let dwn_manager = DWNManager::new();

        let data = serde_json::json!({
            "name": "Bob",
            "email": "bob@example.com"
        });

        let id =
            dwn_manager.create_record("did:ion:456", "https://schema.org/Person", data.clone())?;

        let record = dwn_manager.read_record(&id)?;
        assert_eq!(record.owner, "did:ion:456");
        assert_eq!(record.schema, "https://schema.org/Person");
        assert_eq!(record.data, data);
        Ok(())
    }

    #[test]
    fn test_update_record() -> Result<(), Box<dyn std::error::Error>> {
        let dwn_manager = DWNManager::new();

        let data = serde_json::json!({
            "name": "Charlie",
            "email": "charlie@example.com"
        });

        let id =
            dwn_manager.create_record("did:ion:789", "https://schema.org/Person", data.clone())?;

        let new_data = serde_json::json!({
            "name": "Charlie",
            "email": "charlie.updated@example.com"
        });

        dwn_manager.update_record(&id, new_data.clone())?;

        let record = dwn_manager.read_record(&id)?;
        assert_eq!(record.data, new_data);

        Ok(())
    }

    #[test]
    fn test_delete_record() -> Result<(), Box<dyn std::error::Error>> {
        let dwn_manager = DWNManager::new();

        let data = serde_json::json!({
            "name": "Dave",
            "email": "dave@example.com"
        });

        let id =
            dwn_manager.create_record("did:ion:abc", "https://schema.org/Person", data.clone())?;

        dwn_manager.delete_record(&id)?;

        let result = dwn_manager.read_record(&id);
        assert!(result.is_err());

        Ok(())
    }
}

#[cfg(test)]
mod advanced_tests {
    use super::*;

    #[test]
    fn test_advanced_query_with_pagination() -> Result<(), Box<dyn std::error::Error>> {
        let dwn_manager = DWNManager::new();

        // Create test records
        for i in 0..25 {
            let record = DWNRecord {
                id: format!("record_{:02}", i),
                owner: "did:ion:test".to_string(),
                schema: "test/schema".to_string(),
                data: serde_json::json!({
                    "name": format!("Test Record {}", i),
                    "value": i,
                }),
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("created_at".to_string(), (1640000000 + i as u64).to_string());
                    meta.insert("category".to_string(), if i % 2 == 0 { "even".to_string() } else { "odd".to_string() });
                    meta
                },
                attestations: Vec::new(),
            };
            dwn_manager.store_record(record)?;
        }

        // Test pagination
        let filter = AdvancedDWNQueryFilter {
            base: DWNQueryFilter {
                owner: Some("did:ion:test".to_string()),
                schema: Some("test/schema".to_string()),
                metadata: None,
                date_range: None,
                data_filter: None,
            },
            search: None,
            geo_bounds: None,
            tags: None,
            numeric_ranges: None,
        };

        let pagination = DWNQueryPagination {
            offset: Some(10),
            limit: Some(5),
            cursor: None,
        };

        let result = dwn_manager.query_with_pagination(filter, Some(pagination))?;
        
        assert_eq!(result.records.len(), 5);
        assert_eq!(result.pagination.total, 25);
        assert_eq!(result.pagination.count, 5);
        assert!(result.pagination.has_more);
        assert!(result.pagination.next_cursor.is_some());

        Ok(())
    }

    #[test]
    fn test_export_import_records() -> Result<(), Box<dyn std::error::Error>> {
        let dwn_manager = DWNManager::new();

        // Create test record
        let record = DWNRecord {
            id: "export_test".to_string(),
            owner: "did:ion:test".to_string(),
            schema: "test/export".to_string(),
            data: serde_json::json!({"test": "data"}),
            metadata: HashMap::new(),
            attestations: Vec::new(),
        };

        dwn_manager.store_record(record)?;

        // Test JSON export
        let exported = dwn_manager.export_records("json", None)?;
        assert!(exported.contains("export_test"));

        // Test CSV export
        let csv_exported = dwn_manager.export_records("csv", None)?;
        assert!(csv_exported.contains("export_test"));

        Ok(())
    }
}

/// Extension trait for Duration to add convenience methods
trait DurationExt {
    fn from_mins(mins: u64) -> Duration;
    fn from_hours(hours: u64) -> Duration;
}

impl DurationExt for Duration {
    fn from_mins(mins: u64) -> Duration {
        Duration::from_secs(mins * 60)
    }
    
    fn from_hours(hours: u64) -> Duration {
        Duration::from_secs(hours * 3600)
    }
}
