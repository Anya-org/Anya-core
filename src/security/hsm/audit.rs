use std::error::Error;
use crate::security::hsm::{HsmError, HsmAuditEvent};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, error, warn};
use chrono::{DateTime, Utc};
use std::path::PathBuf;
use std::fs::{self, OpenOptions, File};
use std::io::{Write, Read, Seek, SeekFrom};
use async_trait::async_trait;
use std::collections::HashMap;
use uuid::Uuid;
use std::path::Path;

use crate::security::hsm::error::{AuditEventType, AuditEventResult, AuditEventSeverity};

/// Audit logger configuration for HSM operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLoggerConfig {
    /// Whether audit logging is enabled
    pub enabled: bool,
    
    /// Log storage type
    pub storage_type: AuditStorageType,
    
    /// File path for file storage
    pub file_path: Option<String>,
    
    /// Database connection string for DB storage
    pub db_connection: Option<String>,
    
    /// Retention period in days
    pub retention_days: u32,
    
    /// Maximum number of events to keep
    pub max_events: u32,
    
    /// Whether to log sensitive details
    pub log_sensitive: bool,
    
    /// Additional audit metrics to collect
    #[serde(default)]
    pub additional_metrics: Vec<String>,
}

impl Default for AuditLoggerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            storage_type: AuditStorageType::File,
            file_path: Some("./logs/hsm_audit.log".to_string()),
            db_connection: None,
            retention_days: 90,
            max_events: 10000,
            log_sensitive: false,
            additional_metrics: vec![],
        }
    }
}

/// Type of storage for audit logs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditStorageType {
    /// In-memory storage (for testing)
    Memory,
    
    /// File-based storage
    File,
    
    /// Database storage
    Database,
}

/// Audit logger for HSM operations
/// [AIR-3][AIS-3][AIM-3][AIP-3][RES-3]
pub struct AuditLogger {
    /// Configuration for the audit logger
    config: AuditLoggerConfig,
    
    /// Storage for audit events
    storage: Arc<Mutex<Box<dyn AuditStorage + Send + Sync>>>,
    
    /// Operation tracker for tracking related operations
    operation_tracker: Arc<Mutex<HashMap<String, (DateTime<Utc>, String)>>>,
}

impl AuditLogger {
    /// Creates a new audit logger with the specified configuration
    pub async fn new(config: &AuditLoggerConfig) -> Result<Self, HsmError> {
        debug!("Creating HSM audit logger with storage type: {:?}", config.storage_type);
        
        // Create storage based on configuration
        let storage = create_storage(config).await?;
        
        let logger = Self {
            config: config.clone(),
            storage: Arc::new(Mutex::new(storage)),
            operation_tracker: Arc::new(Mutex::new(HashMap::new())),
        };
        
        // Initialize the storage
        logger.initialize().await?;
        
        Ok(logger)
    }
    
    /// Initializes the audit logger
    pub async fn initialize(&self) -> Result<(), HsmError> {
        debug!("Initializing HSM audit logger");
        
        // Initialize the storage
        let mut storage = self.storage.lock().await;
        storage.initialize().await?;
        
        // Log initialization event
        let event = AuditEvent::new(
            AuditEventType::HsmInitialize,
            AuditEventResult::Success,
            AuditEventSeverity::Info
        );
        
        storage.store_event(event).await?;
        
        // Perform cleanup if needed
        if let Err(e) = storage.cleanup(self.config.retention_days, Some(self.config.max_events)).await {
            warn!("Failed to cleanup audit logs: {}", e);
        }
        
        debug!("HSM audit logger initialized");
        Ok(())
    }
    
    /// Logs an HSM event
    pub async fn log_event<T: Serialize>(
        &self,
        event_type: AuditEventType,
        result: AuditEventResult,
        severity: AuditEventSeverity,
        details: T,
    ) -> Result<(), HsmError> {
        if !self.config.enabled {
            return Ok(());
        }
        
        let details_value = serde_json::to_value(details)
            .map_err(|e| HsmError::SerializationError(e.to_string()))?;
            
        let mut event = AuditEvent::new(event_type, result, severity);
        
        // Add details to the event
        let details_map: HashMap<String, String> = serde_json::from_value(details_value)
            .unwrap_or_else(|_| {
                // If we can't convert to a HashMap, create a single detail entry
                let mut map = HashMap::new();
                map.insert("data".to_string(), serde_json::to_string(&details_value).unwrap_or_default());
                map
            });
            
        for (key, value) in details_map {
            event = event.with_detail(key, value);
        }
        
        let mut storage = self.storage.lock().await;
        storage.store_event(event).await
    }
    
    /// Gets events from the audit log
    pub async fn get_events(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: Option<usize>,
    ) -> Result<Vec<AuditEvent>, HsmError> {
        if !self.config.enabled {
            return Ok(Vec::new());
        }
        
        let storage = self.storage.lock().await;
        storage.get_events(start_time, end_time, limit).await
    }
    
    /// Count audit events
    pub async fn count_events(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<usize, HsmError> {
        if !self.config.enabled {
            return Ok(0);
        }
        
        let storage = self.storage.lock().await;
        storage.count_events(start_time, end_time).await
    }
    
    /// Clean up old events
    pub async fn cleanup(&self) -> Result<usize, HsmError> {
        if !self.config.enabled {
            return Ok(0);
        }
        
        let mut storage = self.storage.lock().await;
        storage.cleanup(self.config.retention_days, Some(self.config.max_events)).await
    }
    
    /// Track an operation
    pub async fn track_operation(&self, operation_id: &str, details: &str) -> Result<(), HsmError> {
        if !self.config.enabled {
            return Ok(());
        }
        
        let mut tracker = self.operation_tracker.lock().await;
        tracker.insert(operation_id.to_string(), (Utc::now(), details.to_string()));
        Ok(())
    }
    
    /// Check if an operation is being tracked
    pub async fn is_operation_tracked(&self, operation_id: &str) -> bool {
        if !self.config.enabled {
            return false;
        }
        
        let tracker = self.operation_tracker.lock().await;
        tracker.contains_key(operation_id)
    }
    
    /// Get operation details
    pub async fn get_operation_details(&self, operation_id: &str) -> Option<(DateTime<Utc>, String)> {
        if !self.config.enabled {
            return None;
        }
        
        let tracker = self.operation_tracker.lock().await;
        tracker.get(operation_id).cloned()
    }
    
    /// Remove tracked operation
    pub async fn remove_operation(&self, operation_id: &str) -> Result<(), HsmError> {
        if !self.config.enabled {
            return Ok(());
        }
        
        let mut tracker = self.operation_tracker.lock().await;
        tracker.remove(operation_id);
        Ok(())
    }
}

/// Creates an appropriate storage backend based on configuration
async fn create_storage(config: &AuditLoggerConfig) -> Result<Box<dyn AuditStorage + Send + Sync>, HsmError> {
    match config.storage_type {
        AuditStorageType::Memory => {
            Ok(Box::new(MemoryAuditStorage::new()))
        },
        AuditStorageType::File => {
            let path = config.file_path.clone()
                .ok_or_else(|| HsmError::ConfigError("File path is required for file storage".to_string()))?;
                
            Ok(Box::new(FileAuditStorage::new(path)))
        },
        AuditStorageType::Database => {
            let conn_string = config.db_connection.clone()
                .ok_or_else(|| HsmError::ConfigError("Database connection string is required for DB storage".to_string()))?;
                
            Ok(Box::new(DbAuditStorage::new(conn_string)))
        },
    }
}

/// Storage trait for audit events
#[async_trait]
pub trait AuditStorage {
    /// Initialize the storage
    async fn initialize(&self) -> Result<(), HsmError>;
    
    /// Store an audit event
    async fn store_event(&self, event: AuditEvent) -> Result<(), HsmError>;
    
    /// Get audit events matching criteria
    async fn get_events(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: Option<usize>,
    ) -> Result<Vec<AuditEvent>, HsmError>;
    
    /// Count audit events matching criteria
    async fn count_events(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<usize, HsmError>;
    
    /// Clean up old events
    async fn cleanup(
        &self,
        retention_days: u32,
        max_events: Option<usize>,
    ) -> Result<usize, HsmError>;
}

/// Audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Unique event ID
    pub id: String,
    
    /// Timestamp of the event
    pub timestamp: DateTime<Utc>,
    
    /// Type of event
    pub event_type: String,
    
    /// Result of the event
    pub result: String,
    
    /// Severity of the event
    pub severity: String,
    
    /// User or service that performed the operation
    pub actor: Option<String>,
    
    /// Operation ID (for tracking related events)
    pub operation_id: Option<String>,
    
    /// Key ID (if applicable)
    pub key_id: Option<String>,
    
    /// Error message (if applicable)
    pub error: Option<String>,
    
    /// Additional details
    pub details: HashMap<String, String>,
    
    /// IP address of the client (if applicable)
    pub client_ip: Option<String>,
}

impl AuditEvent {
    /// Create a new audit event
    pub fn new(
        event_type: AuditEventType,
        result: AuditEventResult,
        severity: AuditEventSeverity,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type: event_type.to_string(),
            result: result.to_string(),
            severity: severity.to_string(),
            actor: None,
            operation_id: None,
            key_id: None,
            error: None,
            details: HashMap::new(),
            client_ip: None,
        }
    }
    
    /// Create a new success event
    pub fn success(event_type: AuditEventType) -> Self {
        Self::new(event_type, AuditEventResult::Success, AuditEventSeverity::Info)
    }
    
    /// Create a new failure event
    pub fn failure(event_type: AuditEventType, error: impl Into<String>) -> Self {
        let mut event = Self::new(event_type, AuditEventResult::Failure, AuditEventSeverity::Error);
        event.error = Some(error.into());
        event
    }
    
    /// Create a new in-progress event
    pub fn in_progress(event_type: AuditEventType) -> Self {
        Self::new(event_type, AuditEventResult::InProgress, AuditEventSeverity::Info)
    }
    
    /// Set actor
    pub fn with_actor(mut self, actor: impl Into<String>) -> Self {
        self.actor = Some(actor.into());
        self
    }
    
    /// Set operation ID
    pub fn with_operation_id(mut self, operation_id: impl Into<String>) -> Self {
        self.operation_id = Some(operation_id.into());
        self
    }
    
    /// Set key ID
    pub fn with_key_id(mut self, key_id: impl Into<String>) -> Self {
        self.key_id = Some(key_id.into());
        self
    }
    
    /// Add detail
    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.details.insert(key.into(), value.into());
        self
    }
    
    /// Set client IP
    pub fn with_client_ip(mut self, client_ip: impl Into<String>) -> Self {
        self.client_ip = Some(client_ip.into());
        self
    }
}

/// In-memory storage for audit events (for testing)
pub struct MemoryAuditStorage {
    events: Mutex<Vec<AuditEvent>>,
}

impl MemoryAuditStorage {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl AuditStorage for MemoryAuditStorage {
    async fn initialize(&self) -> Result<(), HsmError> {
        // No initialization needed for in-memory storage
        Ok(())
    }

    async fn store_event(&self, event: AuditEvent) -> Result<(), HsmError> {
        let mut events = self.events.lock().await;
        events.push(event);
        Ok(())
    }

    async fn get_events(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: Option<usize>,
    ) -> Result<Vec<AuditEvent>, HsmError> {
        let events = self.events.lock().await;
        
        let filtered_events: Vec<AuditEvent> = events
            .iter()
            .filter(|event| {
                if let Some(start) = start_time {
                    if event.timestamp < start {
                        return false;
                    }
                }
                
                if let Some(end) = end_time {
                    if event.timestamp > end {
                        return false;
                    }
                }
                
                true
            })
            .cloned()
            .collect();
            
        let result = if let Some(limit_val) = limit {
            filtered_events.into_iter().take(limit_val).collect()
        } else {
            filtered_events
        };
        
        Ok(result)
    }

    async fn count_events(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<usize, HsmError> {
        let events = self.events.lock().await;
        
        let count = events
            .iter()
            .filter(|event| {
                if let Some(start) = start_time {
                    if event.timestamp < start {
                        return false;
                    }
                }
                
                if let Some(end) = end_time {
                    if event.timestamp > end {
                        return false;
                    }
                }
                
                true
            })
            .count();
        
        Ok(count)
    }

    async fn cleanup(
        &self,
        retention_days: u32,
        max_events: Option<usize>,
    ) -> Result<usize, HsmError> {
        let mut events = self.events.lock().await;
        let initial_count = events.len();
        
        // Remove old events
        let cutoff = Utc::now() - chrono::Duration::days(retention_days as i64);
        events.retain(|e| e.timestamp >= cutoff);
        
        // Apply max events limit
        if let Some(max) = max_events {
            if events.len() > max {
                // Sort by timestamp (newest first)
                events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                events.truncate(max);
            }
        }
        
        let removed = initial_count - events.len();
        Ok(removed)
    }
}

/// File-based storage for audit events
pub struct FileAuditStorage {
    file_path: String,
}

impl FileAuditStorage {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path,
        }
    }
    
    /// Load events from file
    async fn load_events(&self) -> Result<Vec<AuditEvent>, HsmError> {
        let path = Path::new(&self.file_path);
        
        // If file doesn't exist, return empty vector
        if !path.exists() {
            return Ok(Vec::new());
        }
        
        // Read file contents
        let contents = fs::read_to_string(path)
            .map_err(|e| HsmError::AuditStorageError(format!("Failed to read audit log file: {}", e)))?;
            
        // Parse events (each line is a JSON object)
        let mut events = Vec::new();
        for line in contents.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<AuditEvent>(line) {
                Ok(event) => events.push(event),
                Err(e) => {
                    warn!("Failed to parse audit event: {}", e);
                }
            }
        }
        
        Ok(events)
    }
}

#[async_trait]
impl AuditStorage for FileAuditStorage {
    async fn initialize(&self) -> Result<(), HsmError> {
        let file_path = &self.file_path;
        let dir_path = Path::new(file_path).parent();
        
        if let Some(dir) = dir_path {
            if !dir.exists() {
                fs::create_dir_all(dir).map_err(|e| {
                    HsmError::AuditStorageError(format!("Failed to create directory: {}", e))
                })?;
            }
        }
        
        // Create the file if it doesn't exist
        if !Path::new(file_path).exists() {
            File::create(file_path).map_err(|e| {
                HsmError::AuditStorageError(format!("Failed to create audit log file: {}", e))
            })?;
        }
        
        Ok(())
    }
    
    async fn store_event(&self, event: AuditEvent) -> Result<(), HsmError> {
        // Serialize the event
        let json = serde_json::to_string(&event)
            .map_err(|e| HsmError::SerializationError(e.to_string()))?;
            
        // Open the file in append mode
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .map_err(|e| HsmError::AuditStorageError(format!("Failed to open audit log file: {}", e)))?;
            
        // Write the event with a newline
        writeln!(file, "{}", json)
            .map_err(|e| HsmError::AuditStorageError(format!("Failed to write to audit log file: {}", e)))?;
            
        Ok(())
    }
    
    async fn get_events(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: Option<usize>,
    ) -> Result<Vec<AuditEvent>, HsmError> {
        // Load all events
        let mut events = self.load_events().await?;
        
        // Filter by time range
        if let Some(start) = start_time {
            events.retain(|e| e.timestamp >= start);
        }
        
        if let Some(end) = end_time {
            events.retain(|e| e.timestamp <= end);
        }
        
        // Sort by timestamp (newest first)
        events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        // Apply limit
        if let Some(lim) = limit {
            events.truncate(lim);
        }
        
        Ok(events)
    }
    
    async fn count_events(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<usize, HsmError> {
        // Load and filter events
        let events = self.load_events().await?;
        
        let count = events.iter()
            .filter(|e| {
                if let Some(start) = start_time {
                    if e.timestamp < start {
                        return false;
                    }
                }
                
                if let Some(end) = end_time {
                    if e.timestamp > end {
                        return false;
                    }
                }
                
                true
            })
            .count();
            
        Ok(count)
    }
    
    async fn cleanup(
        &self,
        retention_days: u32,
        max_events: Option<usize>,
    ) -> Result<usize, HsmError> {
        // Load all events
        let mut events = self.load_events().await?;
        let initial_count = events.len();
        
        // Filter out old events
        let cutoff = Utc::now() - chrono::Duration::days(retention_days as i64);
        events.retain(|e| e.timestamp >= cutoff);
        
        // Apply max events limit
        if let Some(max) = max_events {
            if events.len() > max {
                // Sort by timestamp (newest first)
                events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                events.truncate(max);
            }
        }
        
        // If we removed any events, rewrite the file
        if events.len() < initial_count {
            // Open file for writing (truncate)
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&self.file_path)
                .map_err(|e| HsmError::AuditStorageError(format!("Failed to open audit log file: {}", e)))?;
                
            // Write each event
            for event in &events {
                let json = serde_json::to_string(event)
                    .map_err(|e| HsmError::SerializationError(e.to_string()))?;
                    
                writeln!(file, "{}", json)
                    .map_err(|e| HsmError::AuditStorageError(format!("Failed to write to audit log file: {}", e)))?;
            }
        }
        
        Ok(initial_count - events.len())
    }
}

/// Database storage for audit events
pub struct DbAuditStorage {
    connection_string: String,
    memory_storage: MemoryAuditStorage, // Fallback storage
}

impl DbAuditStorage {
    pub fn new(connection_string: String) -> Self {
        Self {
            connection_string,
            memory_storage: MemoryAuditStorage::new(),
        }
    }
}

#[async_trait]
impl AuditStorage for DbAuditStorage {
    async fn initialize(&self) -> Result<(), HsmError> {
        // In a real implementation, this would initialize the database
        // For now, just use the memory storage as a fallback
        debug!("Initializing database audit storage (using memory fallback)");
        self.memory_storage.initialize().await
    }
    
    async fn store_event(&self, event: AuditEvent) -> Result<(), HsmError> {
        // In a real implementation, this would store in the database
        // For now, just use the memory storage as a fallback
        debug!("Storing event in database (using memory fallback): {}", event.id);
        self.memory_storage.store_event(event).await
    }
    
    async fn get_events(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: Option<usize>,
    ) -> Result<Vec<AuditEvent>, HsmError> {
        // In a real implementation, this would query the database
        // For now, just use the memory storage as a fallback
        debug!("Getting events from database (using memory fallback)");
        self.memory_storage.get_events(start_time, end_time, limit).await
    }
    
    async fn count_events(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<usize, HsmError> {
        // In a real implementation, this would count in the database
        // For now, just use the memory storage as a fallback
        debug!("Counting events in database (using memory fallback)");
        self.memory_storage.count_events(start_time, end_time).await
    }
    
    async fn cleanup(
        &self,
        retention_days: u32,
        max_events: Option<usize>,
    ) -> Result<usize, HsmError> {
        // In a real implementation, this would delete from the database
        // For now, just use the memory storage as a fallback
        debug!("Cleaning up database (using memory fallback)");
        self.memory_storage.cleanup(retention_days, max_events).await
    }
} 
