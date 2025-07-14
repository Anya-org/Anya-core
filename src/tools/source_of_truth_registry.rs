// Canonical Source of Truth Registry Implementation
// This module implements the PRD requirements for work item tracking
// and duplication elimination enforcement

use blake3;
use dashmap::DashMap; // Removed unused DashSet import
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicU32, AtomicU64, Ordering},
    Arc,
};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Errors related to Source of Truth Registry operations
#[derive(Debug, Error)]
pub enum SourceOfTruthError {
    #[error("Work item not found: {0}")]
    WorkItemNotFound(String),

    #[error("Duplication detected: {0}")]
    DuplicationDetected(String),

    #[error("Invalid work item ID format: {0}")]
    InvalidWorkItemId(String),

    #[error("Canonical document conflict: {0}")]
    CanonicalConflict(String),

    #[error("Registry corruption detected: {0}")]
    RegistryCorruption(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Work item status tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkStatus {
    Planning,
    InProgress,
    CodeReview,
    Testing,
    Completed,
    Blocked(String), // Reason for blocking
}

/// Duplication check status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DuplicationCheckStatus {
    Passed,
    Failed(String), // Duplication details
    NotChecked,
}

/// Canonical document status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CanonicalStatus {
    Draft,
    Review,
    Verified,
    Deprecated,
}

/// Work item tracking structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkItem {
    pub id: String,
    pub title: String,
    pub status: WorkStatus,
    pub component: String,
    pub files_modified: Vec<String>,
    pub duplication_check: DuplicationCheckStatus,
    pub source_of_truth_updated: bool,
    pub verification_hash: [u8; 32],
    pub completion_timestamp: Option<u64>,
    pub evidence_link: String,
    pub dependencies: Vec<String>,
    pub blockers: Vec<String>,
    pub created: u64,
    pub last_updated: u64,
}

/// Canonical document entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalDocument {
    pub file_path: String,
    pub work_item_id: String,
    pub verification_hash: [u8; 32],
    pub last_updated: u64,
    pub canonical_status: CanonicalStatus,
    pub authority_level: u8, // 1-10, 10 being highest authority
}

/// Source of Truth Registry entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceOfTruthEntry {
    pub file_path: String,
    pub work_item_id: String,
    pub verification_hash: [u8; 32],
    pub last_updated: u64,
    pub canonical_status: CanonicalStatus,
}

/// Duplication detection entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicationEntry {
    pub content_hash: [u8; 32],
    pub file_path: String,
    pub function_signature: Option<String>,
    pub first_occurrence: u64, // timestamp
}

/// Function signature for duplication detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
    pub visibility: String,
    pub file_path: String,
    pub line_number: u32,
}

/// Code fingerprint for advanced duplication detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeFingerprint {
    pub content_hash: [u8; 32],
    pub normalized_hash: [u8; 32], // Hash after removing whitespace/comments
    pub function_count: u32,
    pub line_count: u32,
    pub file_path: String,
}

/// Documentation entry for duplication checking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationEntry {
    pub content_hash: [u8; 32],
    pub title: String,
    pub file_path: String,
    pub section: String,
}

/// Main Source of Truth Registry
#[derive(Debug)]
pub struct SourceOfTruthRegistry {
    /// Registry of all canonical documents
    canonical_documents: DashMap<String, CanonicalDocument>,
    /// Work item tracking
    work_items: DashMap<String, WorkItem>,
    /// Duplication prevention index
    duplication_index: DashMap<String, DuplicationEntry>,
    /// Function signature index
    function_signatures: DashMap<String, FunctionSignature>,
    /// Code fingerprint index
    #[allow(dead_code)]
    code_fingerprints: DashMap<String, CodeFingerprint>,
    /// Documentation content index
    #[allow(dead_code)]
    documentation_index: DashMap<String, DocumentationEntry>,
    /// Last registry update
    last_updated: AtomicU64,
    /// Registry version
    version: AtomicU32,
    /// Registry file path
    registry_path: String,
}

impl SourceOfTruthRegistry {
    /// Create new registry instance
    pub async fn new(registry_path: String) -> Result<Self, SourceOfTruthError> {
        let registry = Self {
            canonical_documents: DashMap::new(),
            work_items: DashMap::new(),
            duplication_index: DashMap::new(),
            function_signatures: DashMap::new(),
            code_fingerprints: DashMap::new(),
            documentation_index: DashMap::new(),
            last_updated: AtomicU64::new(Self::current_timestamp()),
            version: AtomicU32::new(1),
            registry_path,
        };

        // Create registry directory if it doesn't exist
        if let Some(parent) = std::path::Path::new(&registry.registry_path).parent() {
            fs::create_dir_all(parent).await?;
        }

        // Load existing registry if it exists
        if registry.load_from_disk().await.is_err() {
            // If loading fails, initialize with empty registry
            registry.save_to_disk().await?;
        }

        Ok(registry)
    }

    /// Generate unique work item ID
    pub fn generate_work_item_id(&self) -> String {
        let now = chrono::Utc::now();
        let date_prefix = now.format("%Y-%m-%d").to_string();

        // Find next sequence number for today
        let mut sequence = 1;
        loop {
            let candidate_id = format!("WI-{date_prefix}-{sequence}");
            if !self.work_items.contains_key(&candidate_id) {
                return candidate_id;
            }
            sequence += 1;
        }
    }

    /// Create new work item with comprehensive validation
    pub async fn create_work_item(
        &self,
        title: String,
        component: String,
    ) -> Result<WorkItem, SourceOfTruthError> {
        // 1. Generate unique ID
        let work_id = self.generate_work_item_id();

        // 2. Run pre-work duplication check
        let duplication_status = self.check_work_item_duplication(&title, &component).await?;
        if matches!(duplication_status, DuplicationCheckStatus::Failed(_)) {
            return Err(SourceOfTruthError::DuplicationDetected(format!(
                "Work item title or component already exists: {title}"
            )));
        }

        // 3. Create work item
        let work_item = WorkItem {
            id: work_id.clone(),
            title,
            status: WorkStatus::Planning,
            component,
            files_modified: Vec::new(),
            duplication_check: duplication_status,
            source_of_truth_updated: false,
            verification_hash: [0u8; 32],
            completion_timestamp: None,
            evidence_link: String::new(),
            dependencies: Vec::new(),
            blockers: Vec::new(),
            created: Self::current_timestamp(),
            last_updated: Self::current_timestamp(),
        };

        // 4. Register in registry
        self.work_items.insert(work_id.clone(), work_item.clone());
        self.update_last_modified();

        // 5. Save to disk
        self.save_to_disk().await?;

        Ok(work_item)
    }

    /// Update work item status with validation
    pub async fn update_work_item_status(
        &self,
        work_id: &str,
        new_status: WorkStatus,
    ) -> Result<(), SourceOfTruthError> {
        // 1. Validate work item exists
        let mut work_item = self
            .work_items
            .get_mut(work_id)
            .ok_or_else(|| SourceOfTruthError::WorkItemNotFound(work_id.to_string()))?;

        // 2. Validate status transition
        self.validate_status_transition(&work_item.status, &new_status)?;

        // 3. Update work item
        work_item.status = new_status.clone();
        work_item.last_updated = Self::current_timestamp();

        // 4. Handle completion
        if matches!(new_status, WorkStatus::Completed) {
            work_item.completion_timestamp = Some(Self::current_timestamp());
            work_item.verification_hash = self.generate_verification_hash(&work_item).await?;
            work_item.source_of_truth_updated = true;
        }

        self.update_last_modified();
        self.save_to_disk().await?;

        Ok(())
    }

    /// Comprehensive duplication check for code files
    pub async fn check_code_duplication(
        &self,
        file_path: &str,
        content: &str,
    ) -> Result<DuplicationCheckStatus, SourceOfTruthError> {
        // 1. Generate content fingerprint
        let content_hash = blake3::hash(content.as_bytes()).into();

        // 2. Check for exact content duplication
        for entry in self.duplication_index.iter() {
            if entry.value().content_hash == content_hash && entry.value().file_path != file_path {
                return Ok(DuplicationCheckStatus::Failed(format!(
                    "Exact content duplication found in {}",
                    entry.value().file_path
                )));
            }
        }

        // 3. Check function signature duplication
        let functions = self.extract_rust_functions(content)?;
        for function in functions {
            let signature_key = format!("{}::{}", function.name, function.parameters.join(","));
            if let Some(existing) = self.function_signatures.get(&signature_key) {
                if existing.file_path != file_path {
                    return Ok(DuplicationCheckStatus::Failed(format!(
                        "Function signature duplication: {} in {}",
                        signature_key, existing.file_path
                    )));
                }
            }
        }

        // 4. Update indexes
        self.duplication_index.insert(
            file_path.to_string(),
            DuplicationEntry {
                content_hash,
                file_path: file_path.to_string(),
                function_signature: None,
                first_occurrence: Self::current_timestamp(),
            },
        );

        Ok(DuplicationCheckStatus::Passed)
    }

    /// Check for work item title/component duplication
    async fn check_work_item_duplication(
        &self,
        title: &str,
        component: &str,
    ) -> Result<DuplicationCheckStatus, SourceOfTruthError> {
        for item in self.work_items.iter() {
            let work_item = item.value();
            if work_item.title == title && work_item.component == component {
                return Ok(DuplicationCheckStatus::Failed(format!(
                    "Duplicate work item: {title} in {component}"
                )));
            }
        }
        Ok(DuplicationCheckStatus::Passed)
    }

    /// Extract Rust function signatures from source code
    fn extract_rust_functions(
        &self,
        content: &str,
    ) -> Result<Vec<FunctionSignature>, SourceOfTruthError> {
        let mut functions = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            if let Some(func) = self.parse_rust_function_signature(line, line_num as u32 + 1) {
                functions.push(func);
            }
        }

        Ok(functions)
    }

    /// Parse a single Rust function signature
    fn parse_rust_function_signature(
        &self,
        line: &str,
        line_number: u32,
    ) -> Option<FunctionSignature> {
        let trimmed = line.trim();

        // Simple regex-like parsing for Rust functions
        if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
            let visibility = if trimmed.starts_with("pub ") {
                "pub"
            } else {
                "private"
            };

            // Extract function name and parameters (simplified)
            if let Some(paren_start) = trimmed.find('(') {
                if let Some(fn_start) = trimmed.find("fn ") {
                    let name_start = fn_start + 3;
                    let name = trimmed[name_start..paren_start].trim().to_string();

                    // Extract parameters (simplified - just parameter names)
                    if let Some(paren_end) = trimmed.find(')') {
                        let param_str = &trimmed[paren_start + 1..paren_end];
                        let parameters: Vec<String> = param_str
                            .split(',')
                            .map(|p| p.trim().split(':').next().unwrap_or("").trim().to_string())
                            .filter(|p| !p.is_empty())
                            .collect();

                        // Extract return type
                        let return_type = trimmed.find("->").map(|arrow_pos| {
                            trimmed[arrow_pos + 2..]
                                .split_whitespace()
                                .next()
                                .unwrap_or("")
                                .to_string()
                        });

                        return Some(FunctionSignature {
                            name,
                            parameters,
                            return_type,
                            visibility: visibility.to_string(),
                            file_path: String::new(), // Set by caller
                            line_number,
                        });
                    }
                }
            }
        }

        None
    }

    /// Validate status transition is allowed
    fn validate_status_transition(
        &self,
        current: &WorkStatus,
        new: &WorkStatus,
    ) -> Result<(), SourceOfTruthError> {
        match (current, new) {
            (WorkStatus::Planning, WorkStatus::InProgress) => Ok(()),
            (WorkStatus::InProgress, WorkStatus::CodeReview) => Ok(()),
            (WorkStatus::CodeReview, WorkStatus::Testing) => Ok(()),
            (WorkStatus::Testing, WorkStatus::Completed) => Ok(()),
            (_, WorkStatus::Blocked(_)) => Ok(()), // Can always be blocked
            (WorkStatus::Blocked(_), _) => Ok(()), // Can transition from blocked to any state
            _ => Err(SourceOfTruthError::InvalidWorkItemId(format!(
                "Invalid status transition from {current:?} to {new:?}"
            ))),
        }
    }

    /// Generate verification hash for completed work item
    async fn generate_verification_hash(
        &self,
        work_item: &WorkItem,
    ) -> Result<[u8; 32], SourceOfTruthError> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(work_item.id.as_bytes());
        hasher.update(work_item.title.as_bytes());
        hasher.update(&work_item.completion_timestamp.unwrap_or(0).to_le_bytes());

        // Include hash of all modified files
        for file_path in &work_item.files_modified {
            if let Ok(content) = fs::read_to_string(file_path).await {
                hasher.update(content.as_bytes());
            }
        }

        Ok(hasher.finalize().into())
    }

    /// Get current timestamp in nanoseconds
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }

    /// Update last modified timestamp
    fn update_last_modified(&self) {
        self.last_updated
            .store(Self::current_timestamp(), Ordering::Relaxed);
        self.version.fetch_add(1, Ordering::Relaxed);
    }

    /// Save registry to disk
    async fn save_to_disk(&self) -> Result<(), SourceOfTruthError> {
        // Create a serializable version of the registry
        let registry_data = RegistryData {
            canonical_documents: self
                .canonical_documents
                .iter()
                .map(|entry| (entry.key().clone(), entry.value().clone()))
                .collect(),
            work_items: self
                .work_items
                .iter()
                .map(|entry| (entry.key().clone(), entry.value().clone()))
                .collect(),
            duplication_index: self
                .duplication_index
                .iter()
                .map(|entry| (entry.key().clone(), entry.value().clone()))
                .collect(),
            last_updated: self.last_updated.load(Ordering::Relaxed),
            version: self.version.load(Ordering::Relaxed),
        };

        let json_data = serde_json::to_string_pretty(&registry_data)?;
        let mut file = fs::File::create(&self.registry_path).await?;
        file.write_all(json_data.as_bytes()).await?;

        Ok(())
    }

    /// Load registry from disk
    async fn load_from_disk(&self) -> Result<(), SourceOfTruthError> {
        let mut file = fs::File::open(&self.registry_path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        let registry_data: RegistryData = serde_json::from_str(&contents)?;

        // Clear existing data
        self.canonical_documents.clear();
        self.work_items.clear();
        self.duplication_index.clear();

        // Load data
        for (key, value) in registry_data.canonical_documents {
            self.canonical_documents.insert(key, value);
        }
        for (key, value) in registry_data.work_items {
            self.work_items.insert(key, value);
        }
        for (key, value) in registry_data.duplication_index {
            self.duplication_index.insert(key, value);
        }

        self.last_updated
            .store(registry_data.last_updated, Ordering::Relaxed);
        self.version.store(registry_data.version, Ordering::Relaxed);

        Ok(())
    }
}

/// Serializable registry data structure
#[derive(Debug, Serialize, Deserialize)]
struct RegistryData {
    canonical_documents: HashMap<String, CanonicalDocument>,
    work_items: HashMap<String, WorkItem>,
    duplication_index: HashMap<String, DuplicationEntry>,
    last_updated: u64,
    version: u32,
}

/// Global registry instance
static GLOBAL_REGISTRY: once_cell::sync::Lazy<
    Arc<tokio::sync::RwLock<Option<SourceOfTruthRegistry>>>,
> = once_cell::sync::Lazy::new(|| Arc::new(tokio::sync::RwLock::new(None)));

/// Initialize global registry
pub async fn initialize_global_registry(registry_path: String) -> Result<(), SourceOfTruthError> {
    let registry = SourceOfTruthRegistry::new(registry_path).await?;
    let mut global = GLOBAL_REGISTRY.write().await;
    *global = Some(registry);
    Ok(())
}

/// Get reference to global registry
pub async fn get_global_registry() -> Arc<tokio::sync::RwLock<Option<SourceOfTruthRegistry>>> {
    GLOBAL_REGISTRY.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_work_item_creation() {
        let temp_dir = tempdir().unwrap();
        let registry_path = temp_dir
            .path()
            .join("registry.json")
            .to_string_lossy()
            .to_string();

        let registry = SourceOfTruthRegistry::new(registry_path).await.unwrap();

        let work_item = registry
            .create_work_item("Test work item".to_string(), "test_component".to_string())
            .await
            .unwrap();

        assert!(work_item.id.starts_with("WI-"));
        assert_eq!(work_item.status, WorkStatus::Planning);
        assert_eq!(work_item.duplication_check, DuplicationCheckStatus::Passed);
    }

    #[tokio::test]
    async fn test_duplication_detection() {
        let temp_dir = tempdir().unwrap();
        let registry_path = temp_dir
            .path()
            .join("registry.json")
            .to_string_lossy()
            .to_string();

        let registry = SourceOfTruthRegistry::new(registry_path).await.unwrap();

        // First work item should pass
        let _work_item1 = registry
            .create_work_item("Unique title".to_string(), "component1".to_string())
            .await
            .unwrap();

        // Duplicate should fail
        let result = registry
            .create_work_item("Unique title".to_string(), "component1".to_string())
            .await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SourceOfTruthError::DuplicationDetected(_)
        ));
    }

    #[tokio::test]
    async fn test_status_transitions() {
        let temp_dir = tempdir().unwrap();
        let registry_path = temp_dir
            .path()
            .join("registry.json")
            .to_string_lossy()
            .to_string();

        let registry = SourceOfTruthRegistry::new(registry_path).await.unwrap();

        let work_item = registry
            .create_work_item("Status test".to_string(), "test_component".to_string())
            .await
            .unwrap();

        // Valid transition
        registry
            .update_work_item_status(&work_item.id, WorkStatus::InProgress)
            .await
            .unwrap();

        // Invalid transition should fail
        let result = registry
            .update_work_item_status(&work_item.id, WorkStatus::Completed)
            .await;
        assert!(result.is_err());
    }
}
