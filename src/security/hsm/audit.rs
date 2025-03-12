use crate::security::hsm::{HsmError, HsmAuditEvent};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, error, warn};
use chrono::{DateTime, Utc};
use std::path::PathBuf;
use std::fs::{OpenOptions, File};
use std::io::{Write, Read, Seek, SeekFrom};
use async_trait::async_trait;
use std::collections::HashMap;
use uuid::Uuid;

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
    storage: Arc<Mutex<Box<dyn AuditStorage>>>,
}

impl AuditLogger {
    /// Creates a new audit logger with the specified configuration
    pub async fn new(config: &AuditLoggerConfig) -> Result<Self, HsmError> {
        debug!("Creating HSM audit logger with storage type: {:?}", config.storage_type);
        
        // Create storage based on configuration
        let storage = create_storage(config).await?;
        
        Ok(Self {
            config: config.clone(),
            storage: Arc::new(Mutex::new(storage)),
        })
    }
    
    /// Initializes the audit logger
    pub async fn initialize(&self) -> Result<(), HsmError> {
        debug!("Initializing HSM audit logger");
        
        // Initialize the storage
        let mut storage = self.storage.lock().await;
        storage.initialize().await?;
        
        // Log initialization event
        let event = AuditEvent {
            timestamp: Utc::now(),
            event_type: "audit.initialize".to_string(),
            user: "system".to_string(),
            source_ip: "127.0.0.1".to_string(),
            details: serde_json::to_value(AuditInitializeEvent {
                storage_type: format!("{:?}", self.config.storage_type),
                retention_days: self.config.retention_days,
                enabled: self.config.enabled,
            }).map_err(|e| HsmError::SerializationError(e.to_string()))?,
        };
        
        storage.store_event(&event).await?;
        
        // Perform cleanup if needed
        if let Err(e) = storage.cleanup(self.config.retention_days, self.config.max_events).await {
            warn!("Failed to cleanup audit logs: {}", e);
        }
        
        debug!("HSM audit logger initialized");
        Ok(())
    }
    
    /// Logs an HSM event
    pub async fn log_event<T: Serialize>(
        &self,
        event_type: &str,
        details: &T,
    ) -> Result<(), HsmError> {
        if !self.config.enabled {
            return Ok(());
        }
        
        // Serialize the details
        let details_value = match serde_json::to_value(details) {
            Ok(value) => value,
            Err(e) => {
                error!("Failed to serialize audit event details: {}", e);
                return Err(HsmError::SerializationError(e.to_string()));
            }
        };
        
        // Create the audit event
        let event = AuditEvent {
            timestamp: Utc::now(),
            event_type: event_type.to_string(),
            user: get_current_user(),
            source_ip: get_source_ip(),
            details: details_value,
        };
        
        // Store the event
        let mut storage = self.storage.lock().await;
        storage.store_event(&event).await?;
        
        debug!("Logged HSM audit event: {}", event_type);
        Ok(())
    }
    
    /// Gets events from the audit log
    pub async fn get_events(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: Option<usize>,
    ) -> Result<Vec<HsmAuditEvent>, HsmError> {
        let storage = self.storage.lock().await;
        
        // Get the raw events
        let events = storage.get_events(start_time, end_time, limit).await?;
        
        // Convert to HsmAuditEvent format
        let hsm_events = events.into_iter()
            .filter_map(|event| {
                // Only include HSM-related events
                if !event.event_type.starts_with("hsm.") {
                    return None;
                }
                
                // Extract provider and status
                let provider = match event.details.get("provider") {
                    Some(val) => val.as_str().unwrap_or("unknown").to_string(),
                    None => "unknown".to_string(),
                };
                
                let status = match event.details.get("status") {
                    Some(val) => val.as_str().unwrap_or("unknown").to_string(),
                    None => "unknown".to_string(),
                };
                
                // Extract operation_id if present
                let operation_id = event.details.get("operation_id")
                    .and_then(|val| val.as_str())
                    .map(|s| s.to_string());
                
                // Redact sensitive details if configured
                let details = if self.config.log_sensitive {
                    event.details.get("details")
                        .and_then(|val| val.as_str())
                        .map(|s| s.to_string())
                } else {
                    // Redact sensitive info
                    event.details.get("details")
                        .and_then(|val| val.as_str())
                        .map(|s| {
                            if s.contains("key") || s.contains("signature") || s.contains("secret") {
                                "REDACTED".to_string()
                            } else {
                                s.to_string()
                            }
                        })
                };
                
                Some(HsmAuditEvent {
                    event_type: event.event_type,
                    provider,
                    status,
                    details,
                    operation_id,
                })
            })
            .collect();
            
        Ok(hsm_events)
    }
}

/// Creates an appropriate storage backend based on configuration
async fn create_storage(config: &AuditLoggerConfig) -> Result<Box<dyn AuditStorage>, HsmError> {
    match config.storage_type {
        AuditStorageType::Memory => {
            Ok(Box::new(MemoryAuditStorage::new()))
        },
        AuditStorageType::File => {
            let path = config.file_path.clone()
                .ok_or_else(|| HsmError::ConfigError("File path is required for file storage".to_string()))?;
                
            Ok(Box::new(FileAuditStorage::new(path)?))
        },
        AuditStorageType::Database => {
            let conn_string = config.db_connection.clone()
                .ok_or_else(|| HsmError::ConfigError("Database connection string is required for DB storage".to_string()))?;
                
            Ok(Box::new(DbAuditStorage::new(conn_string).await?))
        },
    }
}

/// Gets the current user (or system if not available)
fn get_current_user() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "system".to_string())
}

/// Gets the source IP (or localhost if not available)
fn get_source_ip() -> String {
    // In a real implementation, this would get the client IP
    // For now, just return localhost
    "127.0.0.1".to_string()
}

/// Storage trait for audit events
#[async_trait::async_trait]
trait AuditStorage: Send + Sync {
    /// Initializes the storage
    async fn initialize(&mut self) -> Result<(), HsmError>;
    
    /// Stores an audit event
    async fn store_event(&mut self, event: &AuditEvent) -> Result<(), HsmError>;
    
    /// Gets events from the storage
    async fn get_events(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: Option<usize>,
    ) -> Result<Vec<AuditEvent>, HsmError>;
    
    /// Cleans up old events
    async fn cleanup(
        &mut self,
        retention_days: u32,
        max_events: Option<usize>,
    ) -> Result<(), HsmError>;
}

/// In-memory storage for audit events (for testing)
struct MemoryAuditStorage {
    events: Vec<AuditEvent>,
}

impl MemoryAuditStorage {
    fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }
}

#[async_trait::async_trait]
impl AuditStorage for MemoryAuditStorage {
    async fn initialize(&mut self) -> Result<(), HsmError> {
        // No initialization needed for memory storage
        Ok(())
    }
    
    async fn store_event(&mut self, event: &AuditEvent) -> Result<(), HsmError> {
        self.events.push(event.clone());
        Ok(())
    }
    
    async fn get_events(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: Option<usize>,
    ) -> Result<Vec<AuditEvent>, HsmError> {
        let mut events = self.events.clone();
        
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
        if let Some(limit) = limit {
            events.truncate(limit);
        }
        
        Ok(events)
    }
    
    async fn cleanup(
        &mut self,
        retention_days: u32,
        max_events: Option<usize>,
    ) -> Result<(), HsmError> {
        // Remove events older than retention period
        let cutoff = Utc::now() - chrono::Duration::days(retention_days as i64);
        self.events.retain(|e| e.timestamp >= cutoff);
        
        // Apply max events limit
        if let Some(max) = max_events {
            if self.events.len() > max {
                // Sort by timestamp (newest first)
                self.events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                self.events.truncate(max);
            }
        }
        
        Ok(())
    }
}

/// File-based storage for audit events
struct FileAuditStorage {
    path: PathBuf,
    file: Option<File>,
}

impl FileAuditStorage {
    fn new(path: String) -> Result<Self, HsmError> {
        let path = PathBuf::from(path);
        
        // Ensure directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| 
                HsmError::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to create directory: {}", e)
                ))
            )?;
        }
        
        Ok(Self {
            path,
            file: None,
        })
    }
}

#[async_trait::async_trait]
impl AuditStorage for FileAuditStorage {
    async fn initialize(&mut self) -> Result<(), HsmError> {
        // Open the file for appending
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| HsmError::IoError(e))?;
            
        self.file = Some(file);
        Ok(())
    }
    
    async fn store_event(&mut self, event: &AuditEvent) -> Result<(), HsmError> {
        let file = self.file.as_mut().ok_or_else(|| 
            HsmError::AuditError("File not initialized".to_string())
        )?;
        
        // Serialize the event to JSON
        let json = serde_json::to_string(event)
            .map_err(|e| HsmError::SerializationError(e.to_string()))?;
            
        // Write the event to the file with a newline
        writeln!(file, "{}", json)
            .map_err(|e| HsmError::IoError(e))?;
            
        // Ensure it's written to disk
        file.flush().map_err(|e| HsmError::IoError(e))?;
        
        Ok(())
    }
    
    async fn get_events(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: Option<usize>,
    ) -> Result<Vec<AuditEvent>, HsmError> {
        // Open the file for reading
        let mut file = OpenOptions::new()
            .read(true)
            .open(&self.path)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    // Return empty vec if file doesn't exist yet
                    return HsmError::AuditError("Audit log file not found".to_string());
                }
                HsmError::IoError(e)
            })?;
            
        // Read the entire file
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| HsmError::IoError(e))?;
            
        // Parse each line as an event
        let mut events: Vec<AuditEvent> = Vec::new();
        for line in contents.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<AuditEvent>(line) {
                Ok(event) => {
                    // Filter by time range
                    if let Some(start) = start_time {
                        if event.timestamp < start {
                            continue;
                        }
                    }
                    
                    if let Some(end) = end_time {
                        if event.timestamp > end {
                            continue;
                        }
                    }
                    
                    events.push(event);
                },
                Err(e) => {
                    warn!("Failed to parse audit event: {}", e);
                }
            }
        }
        
        // Sort by timestamp (newest first)
        events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        // Apply limit
        if let Some(limit) = limit {
            events.truncate(limit);
        }
        
        Ok(events)
    }
    
    async fn cleanup(
        &mut self,
        retention_days: u32,
        max_events: Option<usize>,
    ) -> Result<(), HsmError> {
        // Get all events
        let mut events = match self.get_events(None, None, None).await {
            Ok(events) => events,
            Err(HsmError::AuditError(_)) => {
                // File doesn't exist yet, nothing to clean up
                return Ok(());
            },
            Err(e) => return Err(e),
        };
        
        // Remove events older than retention period
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
        
        // Rewrite the file with the filtered events
        let mut temp_path = self.path.clone();
        temp_path.set_extension("tmp");
        
        // Create a temporary file
        let mut temp_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&temp_path)
            .map_err(|e| HsmError::IoError(e))?;
            
        // Write events to the temporary file
        for event in events {
            let json = serde_json::to_string(&event)
                .map_err(|e| HsmError::SerializationError(e.to_string()))?;
                
            writeln!(temp_file, "{}", json)
                .map_err(|e| HsmError::IoError(e))?;
        }
        
        // Ensure it's written to disk
        temp_file.flush().map_err(|e| HsmError::IoError(e))?;
        
        // Replace the original file with the temporary file
        std::fs::rename(&temp_path, &self.path)
            .map_err(|e| HsmError::IoError(e))?;
            
        // Reopen the file for appending
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| HsmError::IoError(e))?;
            
        self.file = Some(file);
        
        Ok(())
    }
}

/// Database storage for audit events
struct DbAuditStorage {
    connection_string: String,
    // In a real implementation, this would hold a database connection
    // For now, we'll just use memory storage as a placeholder
    memory_storage: MemoryAuditStorage,
}

impl DbAuditStorage {
    async fn new(connection_string: String) -> Result<Self, HsmError> {
        Ok(Self {
            connection_string,
            memory_storage: MemoryAuditStorage::new(),
        })
    }
}

#[async_trait::async_trait]
impl AuditStorage for DbAuditStorage {
    async fn initialize(&mut self) -> Result<(), HsmError> {
        // In a real implementation, this would connect to the database
        // For now, just initialize the memory storage
        self.memory_storage.initialize().await
    }
    
    async fn store_event(&mut self, event: &AuditEvent) -> Result<(), HsmError> {
        // In a real implementation, this would store in the database
        // For now, just use the memory storage
        self.memory_storage.store_event(event).await
    }
    
    async fn get_events(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: Option<usize>,
    ) -> Result<Vec<AuditEvent>, HsmError> {
        // In a real implementation, this would query the database
        // For now, just use the memory storage
        self.memory_storage.get_events(start_time, end_time, limit).await
    }
    
    async fn cleanup(
        &mut self,
        retention_days: u32,
        max_events: Option<usize>,
    ) -> Result<(), HsmError> {
        // In a real implementation, this would delete old records from the database
        // For now, just use the memory storage
        self.memory_storage.cleanup(retention_days, max_events).await
    }
}

/// Audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AuditEvent {
    /// Timestamp of the event
    pub timestamp: DateTime<Utc>,
    
    /// Type of event
    pub event_type: String,
    
    /// User who triggered the event
    pub user: String,
    
    /// Source IP of the request
    pub source_ip: String,
    
    /// Event details
    pub details: serde_json::Value,
}

/// Initialization event details
#[derive(Debug, Serialize, Deserialize)]
struct AuditInitializeEvent {
    /// Storage type
    pub storage_type: String,
    
    /// Retention period in days
    pub retention_days: u32,
    
    /// Whether audit logging is enabled
    pub enabled: bool,
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

/// Audit storage trait
#[async_trait]
pub trait AuditStorage: Send + Sync {
    /// Initialize the storage
    async fn initialize(&self) -> Result<(), HsmError>;
    
    /// Store an audit event
    async fn store_event(&self, event: AuditEvent) -> Result<(), HsmError>;
    
    /// Get audit events matching a filter
    async fn get_events(&self, filter: AuditFilter) -> Result<Vec<AuditEvent>, HsmError>;
    
    /// Count audit events matching a filter
    async fn count_events(&self, filter: AuditFilter) -> Result<usize, HsmError>;
    
    /// Clean up old events
    async fn cleanup(&self, retention_days: u32, max_events: u32) -> Result<usize, HsmError>;
}

/// Audit filter
#[derive(Debug, Clone)]
pub struct AuditFilter {
    /// Filter by event type
    pub event_type: Option<AuditEventType>,
    
    /// Filter by result
    pub result: Option<AuditEventResult>,
    
    /// Filter by severity
    pub severity: Option<AuditEventSeverity>,
    
    /// Filter by actor
    pub actor: Option<String>,
    
    /// Filter by operation ID
    pub operation_id: Option<String>,
    
    /// Filter by key ID
    pub key_id: Option<String>,
    
    /// Filter by time range (start)
    pub start_time: Option<DateTime<Utc>>,
    
    /// Filter by time range (end)
    pub end_time: Option<DateTime<Utc>>,
    
    /// Maximum number of events to return
    pub limit: Option<usize>,
    
    /// Number of events to skip
    pub offset: Option<usize>,
}

impl Default for AuditFilter {
    fn default() -> Self {
        Self {
            event_type: None,
            result: None,
            severity: None,
            actor: None,
            operation_id: None,
            key_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            offset: None,
        }
    }
}

/// In-memory audit storage
pub struct MemoryAuditStorage {
    events: Mutex<Vec<AuditEvent>>,
}

impl MemoryAuditStorage {
    /// Create a new in-memory audit storage
    pub fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl AuditStorage for MemoryAuditStorage {
    async fn initialize(&self) -> Result<(), HsmError> {
        Ok(())
    }
    
    async fn store_event(&self, event: AuditEvent) -> Result<(), HsmError> {
        let mut events = self.events.lock().await;
        events.push(event);
        Ok(())
    }
    
    async fn get_events(&self, filter: AuditFilter) -> Result<Vec<AuditEvent>, HsmError> {
        let events = self.events.lock().await;
        let mut result = Vec::new();
        
        for event in events.iter() {
            if matches_filter(event, &filter) {
                result.push(event.clone());
            }
        }
        
        // Apply limit and offset
        if let Some(offset) = filter.offset {
            if offset < result.len() {
                result = result.into_iter().skip(offset).collect();
            } else {
                result.clear();
            }
        }
        
        if let Some(limit) = filter.limit {
            result.truncate(limit);
        }
        
        Ok(result)
    }
    
    async fn count_events(&self, filter: AuditFilter) -> Result<usize, HsmError> {
        let events = self.events.lock().await;
        let count = events.iter().filter(|event| matches_filter(event, &filter)).count();
        Ok(count)
    }
    
    async fn cleanup(&self, retention_days: u32, max_events: u32) -> Result<usize, HsmError> {
        let mut events = self.events.lock().await;
        let initial_count = events.len();
        
        // Remove old events
        if retention_days > 0 {
            let retention_threshold = Utc::now() - chrono::Duration::days(retention_days as i64);
            events.retain(|event| event.timestamp >= retention_threshold);
        }
        
        // Limit total number of events
        if max_events > 0 && events.len() > max_events as usize {
            let excess = events.len() - max_events as usize;
            events.drain(0..excess);
        }
        
        Ok(initial_count - events.len())
    }
}

/// File-based audit storage
pub struct FileAuditStorage {
    file_path: String,
}

impl FileAuditStorage {
    /// Create a new file-based audit storage
    pub fn new(file_path: impl Into<String>) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }
    
    /// Ensure the log file exists
    fn ensure_file(&self) -> Result<File, HsmError> {
        let path = Path::new(&self.file_path);
        
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                HsmError::AuditError(format!("Failed to create directory: {}", e))
            })?;
        }
        
        // Open or create the file
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| HsmError::AuditError(format!("Failed to open log file: {}", e)))
    }
}

#[async_trait]
impl AuditStorage for FileAuditStorage {
    async fn initialize(&self) -> Result<(), HsmError> {
        // Ensure the file exists
        self.ensure_file()?;
        Ok(())
    }
    
    async fn store_event(&self, event: AuditEvent) -> Result<(), HsmError> {
        let mut file = self.ensure_file()?;
        
        // Serialize to JSON
        let json = serde_json::to_string(&event)
            .map_err(|e| HsmError::SerializationError(e.to_string()))?;
        
        // Write to file
        writeln!(file, "{}", json)
            .map_err(|e| HsmError::AuditError(format!("Failed to write to log file: {}", e)))?;
        
        Ok(())
    }
    
    async fn get_events(&self, filter: AuditFilter) -> Result<Vec<AuditEvent>, HsmError> {
        let path = Path::new(&self.file_path);
        if !path.exists() {
            return Ok(Vec::new());
        }
        
        let file = File::open(path)
            .map_err(|e| HsmError::AuditError(format!("Failed to open log file: {}", e)))?;
        
        let reader = BufReader::new(file);
        let mut events = Vec::new();
        
        for line in reader.lines() {
            let line = line.map_err(|e| {
                HsmError::AuditError(format!("Failed to read from log file: {}", e))
            })?;
            
            if line.trim().is_empty() {
                continue;
            }
            
            let event: AuditEvent = serde_json::from_str(&line)
                .map_err(|e| HsmError::SerializationError(e.to_string()))?;
            
            if matches_filter(&event, &filter) {
                events.push(event);
            }
        }
        
        // Apply limit and offset
        if let Some(offset) = filter.offset {
            if offset < events.len() {
                events = events.into_iter().skip(offset).collect();
            } else {
                events.clear();
            }
        }
        
        if let Some(limit) = filter.limit {
            events.truncate(limit);
        }
        
        Ok(events)
    }
    
    async fn count_events(&self, filter: AuditFilter) -> Result<usize, HsmError> {
        let path = Path::new(&self.file_path);
        if !path.exists() {
            return Ok(0);
        }
        
        let file = File::open(path)
            .map_err(|e| HsmError::AuditError(format!("Failed to open log file: {}", e)))?;
        
        let reader = BufReader::new(file);
        let mut count = 0;
        
        for line in reader.lines() {
            let line = line.map_err(|e| {
                HsmError::AuditError(format!("Failed to read from log file: {}", e))
            })?;
            
            if line.trim().is_empty() {
                continue;
            }
            
            let event: AuditEvent = serde_json::from_str(&line)
                .map_err(|e| HsmError::SerializationError(e.to_string()))?;
            
            if matches_filter(&event, &filter) {
                count += 1;
            }
        }
        
        Ok(count)
    }
    
    async fn cleanup(&self, retention_days: u32, max_events: u32) -> Result<usize, HsmError> {
        let path = Path::new(&self.file_path);
        if !path.exists() {
            return Ok(0);
        }
        
        // This is a naive implementation: read all events, filter them, and write back.
        // A more efficient approach would be to use a proper database.
        
        let file = File::open(path)
            .map_err(|e| HsmError::AuditError(format!("Failed to open log file: {}", e)))?;
        
        let reader = BufReader::new(file);
        let mut events = Vec::new();
        
        for line in reader.lines() {
            let line = line.map_err(|e| {
                HsmError::AuditError(format!("Failed to read from log file: {}", e))
            })?;
            
            if line.trim().is_empty() {
                continue;
            }
            
            let event: AuditEvent = serde_json::from_str(&line)
                .map_err(|e| HsmError::SerializationError(e.to_string()))?;
            
            events.push(event);
        }
        
        let initial_count = events.len();
        
        // Remove old events
        if retention_days > 0 {
            let retention_threshold = Utc::now() - chrono::Duration::days(retention_days as i64);
            events.retain(|event| event.timestamp >= retention_threshold);
        }
        
        // Limit total number of events
        if max_events > 0 && events.len() > max_events as usize {
            let excess = events.len() - max_events as usize;
            events.drain(0..excess);
        }
        
        // Write back to file
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(|e| HsmError::AuditError(format!("Failed to open log file for writing: {}", e)))?;
        
        for event in &events {
            let json = serde_json::to_string(event)
                .map_err(|e| HsmError::SerializationError(e.to_string()))?;
            
            writeln!(file, "{}", json)
                .map_err(|e| HsmError::AuditError(format!("Failed to write to log file: {}", e)))?;
        }
        
        Ok(initial_count - events.len())
    }
}

/// Database audit storage
pub struct DbAuditStorage {
    db_connection: String,
    // In a real implementation, this would have a database connection pool
}

impl DbAuditStorage {
    /// Create a new database audit storage
    pub fn new(db_connection: impl Into<String>) -> Self {
        Self {
            db_connection: db_connection.into(),
        }
    }
}

#[async_trait]
impl AuditStorage for DbAuditStorage {
    async fn initialize(&self) -> Result<(), HsmError> {
        // In a real implementation, this would initialize the database connection
        // and create the necessary tables if they don't exist
        Err(HsmError::NotImplemented)
    }
    
    async fn store_event(&self, event: AuditEvent) -> Result<(), HsmError> {
        // In a real implementation, this would store the event in the database
        Err(HsmError::NotImplemented)
    }
    
    async fn get_events(&self, filter: AuditFilter) -> Result<Vec<AuditEvent>, HsmError> {
        // In a real implementation, this would query the database
        Err(HsmError::NotImplemented)
    }
    
    async fn count_events(&self, filter: AuditFilter) -> Result<usize, HsmError> {
        // In a real implementation, this would count the matching events in the database
        Err(HsmError::NotImplemented)
    }
    
    async fn cleanup(&self, retention_days: u32, max_events: u32) -> Result<usize, HsmError> {
        // In a real implementation, this would delete old events from the database
        Err(HsmError::NotImplemented)
    }
}

/// Check if an event matches a filter
fn matches_filter(event: &AuditEvent, filter: &AuditFilter) -> bool {
    // Filter by event type
    if let Some(event_type) = &filter.event_type {
        if event.event_type != event_type.to_string() {
            return false;
        }
    }
    
    // Filter by result
    if let Some(result) = &filter.result {
        if event.result != result.to_string() {
            return false;
        }
    }
    
    // Filter by severity
    if let Some(severity) = &filter.severity {
        if event.severity != severity.to_string() {
            return false;
        }
    }
    
    // Filter by actor
    if let Some(actor) = &filter.actor {
        match &event.actor {
            Some(event_actor) => {
                if event_actor != actor {
                    return false;
                }
            },
            None => return false,
        }
    }
    
    // Filter by operation ID
    if let Some(operation_id) = &filter.operation_id {
        match &event.operation_id {
            Some(event_operation_id) => {
                if event_operation_id != operation_id {
                    return false;
                }
            },
            None => return false,
        }
    }
    
    // Filter by key ID
    if let Some(key_id) = &filter.key_id {
        match &event.key_id {
            Some(event_key_id) => {
                if event_key_id != key_id {
                    return false;
                }
            },
            None => return false,
        }
    }
    
    // Filter by time range (start)
    if let Some(start_time) = &filter.start_time {
        if event.timestamp < *start_time {
            return false;
        }
    }
    
    // Filter by time range (end)
    if let Some(end_time) = &filter.end_time {
        if event.timestamp > *end_time {
            return false;
        }
    }
    
    true
}

/// Audit logger
pub struct AuditLogger {
    config: AuditLoggerConfig,
    storage: Arc<dyn AuditStorage>,
    operation_tracker: Mutex<HashMap<String, String>>, // Maps operation ID to actor
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new(config: AuditLoggerConfig) -> Result<Self, HsmError> {
        // Create storage based on configuration
        let storage: Arc<dyn AuditStorage> = match config.storage_type {
            AuditStorageType::Memory => {
                Arc::new(MemoryAuditStorage::new())
            },
            AuditStorageType::File => {
                if let Some(file_path) = &config.file_path {
                    Arc::new(FileAuditStorage::new(file_path.clone()))
                } else {
                    return Err(HsmError::InvalidParameters(
                        "File path is required for file storage".to_string()
                    ));
                }
            },
            AuditStorageType::Database => {
                if let Some(db_connection) = &config.db_connection {
                    Arc::new(DbAuditStorage::new(db_connection.clone()))
                } else {
                    return Err(HsmError::InvalidParameters(
                        "Database connection is required for database storage".to_string()
                    ));
                }
            },
        };
        
        Ok(Self {
            config,
            storage,
            operation_tracker: Mutex::new(HashMap::new()),
        })
    }
    
    /// Initialize the audit logger
    pub async fn initialize(&self) -> Result<(), HsmError> {
        if !self.config.enabled {
            return Ok(());
        }
        
        self.storage.initialize().await?;
        
        // Log initialization event
        let event = AuditEvent::success(AuditEventType::Initialize)
            .with_detail("storage_type", format!("{:?}", self.config.storage_type));
        
        self.storage.store_event(event).await?;
        
        Ok(())
    }
    
    /// Start a new operation and return the operation ID
    pub async fn start_operation(&self, event_type: AuditEventType, actor: Option<String>, key_id: Option<String>) -> Result<String, HsmError> {
        if !self.config.enabled {
            return Ok(Uuid::new_v4().to_string());
        }
        
        let operation_id = Uuid::new_v4().to_string();
        
        // Track the operation
        if let Some(actor) = actor.as_ref() {
            let mut tracker = self.operation_tracker.lock().await;
            tracker.insert(operation_id.clone(), actor.clone());
        }
        
        // Log start event
        let mut event = AuditEvent::in_progress(event_type)
            .with_operation_id(&operation_id);
        
        if let Some(actor) = actor {
            event = event.with_actor(actor);
        }
        
        if let Some(key_id) = key_id {
            event = event.with_key_id(key_id);
        }
        
        self.storage.store_event(event).await?;
        
        Ok(operation_id)
    }
    
    /// Log a success event for an operation
    pub async fn log_success(&self, event_type: AuditEventType, operation_id: &str, details: Option<HashMap<String, String>>) -> Result<(), HsmError> {
        if !self.config.enabled {
            return Ok(());
        }
        
        // Create event
        let mut event = AuditEvent::success(event_type)
            .with_operation_id(operation_id);
        
        // Add actor from tracker
        {
            let tracker = self.operation_tracker.lock().await;
            if let Some(actor) = tracker.get(operation_id) {
                event = event.with_actor(actor);
            }
        }
        
        // Add details
        if let Some(details) = details {
            for (key, value) in details {
                event = event.with_detail(key, value);
            }
        }
        
        // Store event
        self.storage.store_event(event).await?;
        
        // Clean up tracker
        {
            let mut tracker = self.operation_tracker.lock().await;
            tracker.remove(operation_id);
        }
        
        Ok(())
    }
    
    /// Log a failure event for an operation
    pub async fn log_failure(&self, event_type: AuditEventType, operation_id: &str, error: impl Into<String>, details: Option<HashMap<String, String>>) -> Result<(), HsmError> {
        if !self.config.enabled {
            return Ok(());
        }
        
        // Create event
        let mut event = AuditEvent::failure(event_type, error)
            .with_operation_id(operation_id);
        
        // Add actor from tracker
        {
            let tracker = self.operation_tracker.lock().await;
            if let Some(actor) = tracker.get(operation_id) {
                event = event.with_actor(actor);
            }
        }
        
        // Add details
        if let Some(details) = details {
            for (key, value) in details {
                event = event.with_detail(key, value);
            }
        }
        
        // Store event
        self.storage.store_event(event).await?;
        
        // Clean up tracker
        {
            let mut tracker = self.operation_tracker.lock().await;
            tracker.remove(operation_id);
        }
        
        Ok(())
    }
    
    /// Log a simple event (not part of an operation)
    pub async fn log_event(&self, event: AuditEvent) -> Result<(), HsmError> {
        if !self.config.enabled {
            return Ok(());
        }
        
        self.storage.store_event(event).await
    }
    
    /// Get audit events
    pub async fn get_events(&self, filter: AuditFilter) -> Result<Vec<AuditEvent>, HsmError> {
        if !self.config.enabled {
            return Ok(Vec::new());
        }
        
        self.storage.get_events(filter).await
    }
    
    /// Count audit events
    pub async fn count_events(&self, filter: AuditFilter) -> Result<usize, HsmError> {
        if !self.config.enabled {
            return Ok(0);
        }
        
        self.storage.count_events(filter).await
    }
    
    /// Clean up old events
    pub async fn cleanup(&self) -> Result<usize, HsmError> {
        if !self.config.enabled {
            return Ok(0);
        }
        
        self.storage.cleanup(self.config.retention_days, self.config.max_events).await
    }
} 