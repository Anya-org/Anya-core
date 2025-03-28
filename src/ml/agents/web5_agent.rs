#![feature(edition2021)]
use std::sync::{Arc, Mutex};
use log::{info, warn, error};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Metrics for tracking Read First Always principle compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadFirstMetrics {
    /// Number of read operations performed
    pub read_count: u64,
    /// Number of write operations performed
    pub write_count: u64,
    /// Number of write operations performed without a preceding read
    pub violation_count: u64,
    /// Timestamp of the last reset
    pub last_reset: DateTime<Utc>,
}

impl ReadFirstMetrics {
    /// Create a new ReadFirstMetrics instance
    pub fn new() -> Self {
        Self {
            read_count: 0,
            write_count: 0,
            violation_count: 0,
            last_reset: Utc::now(),
        }
    }

    /// Reset all metrics to zero
    pub fn reset(&mut self) {
        self.read_count = 0;
        self.write_count = 0;
        self.violation_count = 0;
        self.last_reset = Utc::now();
    }

    /// Calculate the compliance rate (percentage of writes preceded by reads)
    pub fn compliance_rate(&self) -> f64 {
        if self.write_count == 0 {
            return 100.0;
        }
        let compliant_writes = self.write_count.saturating_sub(self.violation_count);
        (compliant_writes as f64 / self.write_count as f64) * 100.0
    }

    /// Log the current metrics to the info log
    pub fn log_metrics(&self) {
        info!(
            "Read First Metrics: reads={}, writes={}, violations={}, compliance_rate={:.2}%",
            self.read_count,
            self.write_count,
            self.violation_count,
            self.compliance_rate()
        );
    }
}

/// ReadFirstDwnManager ensures the Read First Always principle is followed
/// in all DWN (Decentralized Web Node) operations.
#[derive(Debug)]
pub struct ReadFirstDwnManager {
    /// The Web5 client for performing DWN operations
    web5_client: Arc<dyn Web5Client>,
    /// Metrics for tracking Read First compliance
    metrics: Arc<Mutex<ReadFirstMetrics>>,
    /// Whether a read operation has been performed in the current operation context
    read_performed: Arc<Mutex<bool>>,
}

/// Trait for abstracting Web5 client operations
pub trait Web5Client: Send + Sync {
    fn create_record(&self, options: &CreateRecordOptions) -> Result<Record, Web5Error>;
    fn read_record(&self, record_id: &str) -> Result<Option<Record>, Web5Error>;
    fn update_record(&self, record_id: &str, options: &UpdateRecordOptions) -> Result<Record, Web5Error>;
    fn delete_record(&self, record_id: &str) -> Result<bool, Web5Error>;
    fn query_records(&self, query: &QueryOptions) -> Result<Vec<Record>, Web5Error>;
}

/// Options for creating a record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRecordOptions {
    pub data: String,
    pub schema: String,
    pub data_format: String,
}

/// Options for updating a record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRecordOptions {
    pub data: String,
    pub data_format: String,
}

/// Options for querying records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryOptions {
    pub schema: Option<String>,
    pub filter: Option<String>,
}

/// Record representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub data: String,
    pub schema: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Custom error type for Web5 operations
#[derive(Debug, thiserror::Error)]
pub enum Web5Error {
    #[error("Record not found: {0}")]
    RecordNotFound(String),
    
    #[error("Read First violation: attempted to {0} without reading first")]
    ReadFirstViolation(String),
    
    #[error("Web5 client error: {0}")]
    ClientError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

impl ReadFirstDwnManager {
    /// Create a new ReadFirstDwnManager
    pub fn new(web5_client: Arc<dyn Web5Client>) -> Self {
        Self {
            web5_client,
            metrics: Arc::new(Mutex::new(ReadFirstMetrics::new())),
            read_performed: Arc::new(Mutex::new(false)),
        }
    }
    
    /// Reset the read_performed flag for a new operation context
    fn reset_read_status(&self) {
        if let Ok(mut status) = self.read_performed.lock() {
            *status = false;
        }
    }
    
    /// Mark that a read has been performed in the current operation context
    fn mark_read_performed(&self) {
        if let Ok(mut status) = self.read_performed.lock() {
            *status = true;
        }
        
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.read_count += 1;
        }
    }
    
    /// Check if a read has been performed and update metrics for a write operation
    fn check_read_before_write(&self, operation: &str) -> Result<(), Web5Error> {
        let read_performed = if let Ok(status) = self.read_performed.lock() {
            *status
        } else {
            false
        };
        
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.write_count += 1;
            
            if !read_performed {
                metrics.violation_count += 1;
                warn!("Read First violation: {} operation performed without a preceding read", operation);
                return Err(Web5Error::ReadFirstViolation(operation.to_string()));
            }
        }
        
        Ok(())
    }
    
    /// Get a copy of the current metrics
    pub fn get_metrics(&self) -> ReadFirstMetrics {
        if let Ok(metrics) = self.metrics.lock() {
            metrics.clone()
        } else {
            ReadFirstMetrics::new()
        }
    }
    
    /// Log the current metrics
    pub fn log_metrics(&self) {
        if let Ok(metrics) = self.metrics.lock() {
            metrics.log_metrics();
        }
    }
    
    /// Reset all metrics
    pub fn reset_metrics(&self) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.reset();
        }
    }
    
    /// Create a record with Read First enforcement
    pub fn create_record(&self, options: &CreateRecordOptions) -> Result<Record, Web5Error> {
        // Reset read status for new operation
        self.reset_read_status();
        
        // READ FIRST: Query for similar records based on schema
        let query_options = QueryOptions {
            schema: Some(options.schema.clone()),
            filter: None,
        };
        
        // Perform the query (read operation)
        let _ = self.query_records(&query_options)?;
        
        // Check if read was performed before write
        self.check_read_before_write("create")?;
        
        // Perform the actual create operation
        self.web5_client.create_record(options)
    }
    
    /// Read a record and track metrics
    pub fn read_record(&self, record_id: &str) -> Result<Option<Record>, Web5Error> {
        // Mark that a read has been performed
        self.mark_read_performed();
        
        // Perform the actual read operation
        self.web5_client.read_record(record_id)
    }
    
    /// Query records and track metrics
    pub fn query_records(&self, query: &QueryOptions) -> Result<Vec<Record>, Web5Error> {
        // Mark that a read has been performed
        self.mark_read_performed();
        
        // Perform the actual query operation
        self.web5_client.query_records(query)
    }
    
    /// Update a record with Read First enforcement
    pub fn update_record(&self, record_id: &str, options: &UpdateRecordOptions) -> Result<Record, Web5Error> {
        // Reset read status for new operation
        self.reset_read_status();
        
        // READ FIRST: Read the record before updating
        let record = self.read_record(record_id)?;
        
        // Ensure the record exists
        let record = record.ok_or_else(|| Web5Error::RecordNotFound(record_id.to_string()))?;
        
        // Check if read was performed before write
        self.check_read_before_write("update")?;
        
        // Perform the actual update operation
        self.web5_client.update_record(record_id, options)
    }
    
    /// Delete a record with Read First enforcement
    pub fn delete_record(&self, record_id: &str) -> Result<bool, Web5Error> {
        // Reset read status for new operation
        self.reset_read_status();
        
        // READ FIRST: Read the record before deleting
        let record = self.read_record(record_id)?;
        
        // Ensure the record exists
        let _ = record.ok_or_else(|| Web5Error::RecordNotFound(record_id.to_string()))?;
        
        // Check if read was performed before write
        self.check_read_before_write("delete")?;
        
        // Perform the actual delete operation
        self.web5_client.delete_record(record_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;
    
    mock! {
        TestWeb5Client {}
        impl Web5Client for TestWeb5Client {
            fn create_record(&self, options: &CreateRecordOptions) -> Result<Record, Web5Error>;
            fn read_record(&self, record_id: &str) -> Result<Option<Record>, Web5Error>;
            fn update_record(&self, record_id: &str, options: &UpdateRecordOptions) -> Result<Record, Web5Error>;
            fn delete_record(&self, record_id: &str) -> Result<bool, Web5Error>;
            fn query_records(&self, query: &QueryOptions) -> Result<Vec<Record>, Web5Error>;
        }
    }
    
    #[test]
    fn test_create_record_enforces_read_first() {
        let mut mock = MockTestWeb5Client::new();
        
        // Setup expectations
        mock.expect_query_records()
            .times(1)
            .returning(|_| Ok(vec![]));
            
        mock.expect_create_record()
            .times(1)
            .returning(|_| {
                Ok(Record {
                    id: "test-id".to_string(),
                    data: "test-data".to_string(),
                    schema: Some("test-schema".to_string()),
                    created_at: Utc::now(),
                    updated_at: None,
                })
            });
            
        // Create the manager with the mock
        let manager = ReadFirstDwnManager::new(Arc::new(mock));
        
        // Test creating a record
        let result = manager.create_record(&CreateRecordOptions {
            data: "test-data".to_string(),
            schema: "test-schema".to_string(),
            data_format: "application/json".to_string(),
        });
        
        // Verify success
        assert!(result.is_ok());
        
        // Verify metrics
        let metrics = manager.get_metrics();
        assert_eq!(metrics.read_count, 1);
        assert_eq!(metrics.write_count, 1);
        assert_eq!(metrics.violation_count, 0);
        assert_eq!(metrics.compliance_rate(), 100.0);
    }
    
    #[test]
    fn test_update_record_enforces_read_first() {
        let mut mock = MockTestWeb5Client::new();
        
        // Setup expectations
        mock.expect_read_record()
            .times(1)
            .returning(|_| {
                Ok(Some(Record {
                    id: "test-id".to_string(),
                    data: "original-data".to_string(),
                    schema: Some("test-schema".to_string()),
                    created_at: Utc::now(),
                    updated_at: None,
                }))
            });
            
        mock.expect_update_record()
            .times(1)
            .returning(|_, _| {
                Ok(Record {
                    id: "test-id".to_string(),
                    data: "updated-data".to_string(),
                    schema: Some("test-schema".to_string()),
                    created_at: Utc::now(),
                    updated_at: Some(Utc::now()),
                })
            });
            
        // Create the manager with the mock
        let manager = ReadFirstDwnManager::new(Arc::new(mock));
        
        // Test updating a record
        let result = manager.update_record("test-id", &UpdateRecordOptions {
            data: "updated-data".to_string(),
            data_format: "application/json".to_string(),
        });
        
        // Verify success
        assert!(result.is_ok());
        
        // Verify metrics
        let metrics = manager.get_metrics();
        assert_eq!(metrics.read_count, 1);
        assert_eq!(metrics.write_count, 1);
        assert_eq!(metrics.violation_count, 0);
        assert_eq!(metrics.compliance_rate(), 100.0);
    }
    
    #[test]
    fn test_update_nonexistent_record_fails() {
        let mut mock = MockTestWeb5Client::new();
        
        // Setup expectations
        mock.expect_read_record()
            .times(1)
            .returning(|_| Ok(None));
            
        // Create the manager with the mock
        let manager = ReadFirstDwnManager::new(Arc::new(mock));
        
        // Test updating a record
        let result = manager.update_record("nonexistent-id", &UpdateRecordOptions {
            data: "updated-data".to_string(),
            data_format: "application/json".to_string(),
        });
        
        // Verify error
        assert!(result.is_err());
        match result {
            Err(Web5Error::RecordNotFound(_)) => (),
            _ => panic!("Expected RecordNotFound error"),
        }
    }
    
    #[test]
    fn test_delete_record_enforces_read_first() {
        let mut mock = MockTestWeb5Client::new();
        
        // Setup expectations
        mock.expect_read_record()
            .times(1)
            .returning(|_| {
                Ok(Some(Record {
                    id: "test-id".to_string(),
                    data: "test-data".to_string(),
                    schema: Some("test-schema".to_string()),
                    created_at: Utc::now(),
                    updated_at: None,
                }))
            });
            
        mock.expect_delete_record()
            .times(1)
            .returning(|_| Ok(true));
            
        // Create the manager with the mock
        let manager = ReadFirstDwnManager::new(Arc::new(mock));
        
        // Test deleting a record
        let result = manager.delete_record("test-id");
        
        // Verify success
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        // Verify metrics
        let metrics = manager.get_metrics();
        assert_eq!(metrics.read_count, 1);
        assert_eq!(metrics.write_count, 1);
        assert_eq!(metrics.violation_count, 0);
        assert_eq!(metrics.compliance_rate(), 100.0);
    }
}
