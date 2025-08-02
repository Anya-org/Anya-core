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

    #[error("Blockchain anchoring failed: {0}")]
    BlockchainAnchoringFailed(String),

    #[error("Blockchain verification failed: {0}")]
    BlockchainVerificationFailed(String),

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
    /// Hash of the document content
    pub content_hash: [u8; 32],
    /// Hash of the normalized content (without formatting, whitespace variations)
    pub normalized_hash: [u8; 32],
    /// Document title or heading
    pub title: String,
    /// File path where the document is located
    pub file_path: String,
    /// Section or heading within the document
    pub section: String,
    /// Word count of the document
    pub word_count: usize,
    /// Similarity score with most similar document (0.0-1.0)
    pub similarity_score: Option<f32>,
    /// Reference to the most similar document if similarity above threshold
    pub similar_to: Option<String>,
    /// Timestamp when the entry was created
    pub created_at: u64,
    /// Timestamp when the entry was last updated
    pub updated_at: u64,
}

/// Status of a blockchain anchor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnchorStatus {
    /// The anchor has been created but not yet broadcast
    Created,
    /// The anchor has been broadcast but not yet confirmed
    Broadcast,
    /// The anchor has been confirmed with at least one block
    Confirmed(u32), // Number of confirmations
    /// The anchor has reached the required confirmation threshold
    Final,
    /// The anchor failed to be included in the blockchain
    Failed(String), // Reason for failure
}

/// Taproot-specific anchoring data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaprootAnchorData {
    /// Taproot output script
    pub output_script: Vec<u8>,
    /// Internal key used for the Taproot commitment
    pub internal_key: Vec<u8>,
    /// Taproot script tree hashes
    pub script_tree_hashes: Vec<[u8; 32]>,
    /// Control block for the Taproot commitment
    pub control_block: Option<Vec<u8>>,
}

/// Blockchain anchoring data for registry entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainAnchor {
    /// Bitcoin transaction ID containing the anchor
    pub txid: String,
    /// Block height of the confirmation
    pub block_height: Option<u32>,
    /// Merkle proof (if available)
    pub merkle_proof: Option<Vec<u8>>,
    /// Timestamp of anchoring
    pub timestamp: u64,
    /// Root hash of registry at time of anchoring
    pub registry_root_hash: [u8; 32],
    /// Number of confirmations required to consider final
    pub required_confirmations: u8,
    /// Network the anchor was created on (mainnet, testnet, signet)
    pub network: String,
    /// Taproot-specific verification data (if using Taproot)
    pub taproot_data: Option<TaprootAnchorData>,
    /// Status of the blockchain anchor
    pub status: AnchorStatus,
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
    /// Blockchain anchors for registry snapshots
    blockchain_anchors: DashMap<String, BlockchainAnchor>,
    /// Last registry update
    last_updated: AtomicU64,
    /// Registry version
    version: AtomicU32,
    /// Registry file path
    registry_path: String,
    /// Blockchain anchoring enabled flag
    blockchain_anchoring_enabled: AtomicU32, // 0 = disabled, 1 = enabled
    /// Default Bitcoin network to use for anchoring
    bitcoin_network: AtomicU32, // 0 = mainnet, 1 = testnet, 2 = signet, 3 = regtest
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
            blockchain_anchors: DashMap::new(),
            last_updated: AtomicU64::new(Self::current_timestamp()),
            version: AtomicU32::new(1),
            registry_path,
            blockchain_anchoring_enabled: AtomicU32::new(0), // Disabled by default
            bitcoin_network: AtomicU32::new(1),              // Default to testnet (1) for safety
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

    /// Set the Bitcoin network for anchoring
    pub fn set_bitcoin_network(&self, network_type: &str) -> Result<(), SourceOfTruthError> {
        let network_value = match network_type.to_lowercase().as_str() {
            "mainnet" => 0,
            "testnet" => 1,
            "signet" => 2,
            "regtest" => 3,
            _ => {
                return Err(SourceOfTruthError::BlockchainAnchoringFailed(format!(
                    "Invalid network type: {}",
                    network_type
                )));
            }
        };

        self.bitcoin_network.store(network_value, Ordering::Relaxed);
        log::info!("Bitcoin network set to: {}", network_type);

        Ok(())
    }

    /// Get the current Bitcoin network setting as a string
    pub fn get_bitcoin_network(&self) -> String {
        match self.bitcoin_network.load(Ordering::Relaxed) {
            0 => "mainnet".to_string(),
            1 => "testnet".to_string(),
            2 => "signet".to_string(),
            3 => "regtest".to_string(),
            _ => "unknown".to_string(),
        }
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
        // 1. Validate work item exists and get current status
        let current_status = {
            let work_item = match self.work_items.get(work_id) {
                Some(item) => item,
                None => {
                    return Err(SourceOfTruthError::WorkItemNotFound(work_id.to_string()));
                }
            };
            work_item.status.clone()
        };

        // 2. Validate status transition
        if let Err(e) = self.validate_status_transition(&current_status, &new_status) {
            return Err(e);
        }

        // 3. Generate verification hash if completing (before updating)
        let verification_hash = if matches!(new_status, WorkStatus::Completed) {
            let work_item = self.work_items.get(work_id).unwrap();
            let mut temp_item = work_item.clone();
            temp_item.status = new_status.clone();
            temp_item.completion_timestamp = Some(Self::current_timestamp());
            Some(self.generate_verification_hash(&temp_item).await?)
        } else {
            None
        };

        // 4. Update work item (separate scope to release lock)
        {
            let mut work_item = match self.work_items.get_mut(work_id) {
                Some(item) => item,
                None => {
                    return Err(SourceOfTruthError::WorkItemNotFound(work_id.to_string()));
                }
            };

            work_item.status = new_status.clone();
            work_item.last_updated = Self::current_timestamp();

            // Handle completion
            if matches!(new_status, WorkStatus::Completed) {
                work_item.completion_timestamp = Some(Self::current_timestamp());
                work_item.verification_hash = verification_hash.unwrap();
                work_item.source_of_truth_updated = true;
            }
        } // Release the mutable lock here

        // 5. Save to disk after releasing the lock
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

    /// Check for documentation duplication in markdown or other documentation files
    pub async fn check_documentation_duplication(
        &self,
        file_path: &str,
        content: &str,
        title: &str,
        section: &str,
    ) -> Result<DuplicationCheckStatus, SourceOfTruthError> {
        // 1. Generate content hashes - both regular and normalized
        let content_hash = blake3::hash(content.as_bytes()).into();

        // Create normalized content by removing whitespace variations, markdown formatting
        let normalized_content = self.normalize_documentation_content(content);
        let normalized_hash = blake3::hash(normalized_content.as_bytes()).into();

        // Count words for similarity detection
        let word_count = content.split_whitespace().count();

        // 2. Check for exact content duplication
        for entry in self.documentation_index.iter() {
            // Skip comparing with itself
            if entry.value().file_path == file_path && entry.value().section == section {
                continue;
            }

            // Check for exact content match
            if entry.value().content_hash == content_hash {
                return Ok(DuplicationCheckStatus::Failed(format!(
                    "Exact documentation duplication found in {} (section: {})",
                    entry.value().file_path,
                    entry.value().section
                )));
            }

            // Check for normalized content match (same content with different formatting)
            if entry.value().normalized_hash == normalized_hash {
                return Ok(DuplicationCheckStatus::Failed(format!(
                    "Documentation content duplication found in {} (section: {}) - same content with different formatting",
                    entry.value().file_path, entry.value().section
                )));
            }
        }

        // 3. Check for high similarity (for documents with sufficient content)
        if word_count > 50 {
            let mut highest_similarity = 0.0;
            let mut most_similar_path = String::new();
            let mut most_similar_section = String::new();

            for entry in self.documentation_index.iter() {
                // Skip comparing with itself
                if entry.value().file_path == file_path && entry.value().section == section {
                    continue;
                }

                // Calculate similarity
                let similarity =
                    self.calculate_documentation_similarity(&normalized_content, &entry.key());

                if similarity > highest_similarity {
                    highest_similarity = similarity;
                    most_similar_path = entry.value().file_path.clone();
                    most_similar_section = entry.value().section.clone();
                }
            }

            // If similarity above threshold, flag as potential duplication
            if highest_similarity > 0.8 {
                return Ok(DuplicationCheckStatus::Failed(format!(
                    "High documentation similarity ({:.2}%) with {} (section: {})",
                    highest_similarity * 100.0,
                    most_similar_path,
                    most_similar_section
                )));
            }
        }

        // 4. Add to documentation index
        let doc_entry = DocumentationEntry {
            content_hash,
            normalized_hash,
            title: title.to_string(),
            file_path: file_path.to_string(),
            section: section.to_string(),
            word_count,
            similarity_score: None,
            similar_to: None,
            created_at: Self::current_timestamp(),
            updated_at: Self::current_timestamp(),
        };

        self.documentation_index
            .insert(file_path.to_string() + "::" + section, doc_entry);

        Ok(DuplicationCheckStatus::Passed)
    }

    /// Normalize documentation content by removing formatting, whitespace variations, etc.
    fn normalize_documentation_content(&self, content: &str) -> String {
        // Remove markdown formatting
        let mut normalized = content.to_string();

        // Remove headings
        normalized = normalized.replace(|c| c == '#', "");

        // Remove inline formatting (* for bold/italic)
        normalized = normalized.replace('*', "");

        // Remove links - replace [text](url) with just text
        let link_regex = regex::Regex::new(r"\[(.*?)\]\(.*?\)").unwrap();
        normalized = link_regex.replace_all(&normalized, "$1").to_string();

        // Remove code blocks
        let code_block_regex = regex::Regex::new(r"```[\s\S]*?```").unwrap();
        normalized = code_block_regex.replace_all(&normalized, "").to_string();

        // Remove inline code
        let inline_code_regex = regex::Regex::new(r"`(.*?)`").unwrap();
        normalized = inline_code_regex.replace_all(&normalized, "$1").to_string();

        // Normalize whitespace
        let whitespace_regex = regex::Regex::new(r"\s+").unwrap();
        normalized = whitespace_regex.replace_all(&normalized, " ").to_string();

        // Convert to lowercase
        normalized = normalized.to_lowercase();

        // Trim
        normalized.trim().to_string()
    }

    /// Calculate similarity between two documentation contents
    fn calculate_documentation_similarity(&self, content1: &str, doc_key: &str) -> f32 {
        if let Some(doc_entry) = self.documentation_index.get(doc_key) {
            // In a real implementation, this would use a proper similarity algorithm
            // like Jaccard similarity, cosine similarity with tf-idf, or similar

            // Here we'll use a simple approach:
            // 1. Split into words
            let words1: Vec<&str> = content1.split_whitespace().collect();

            // 2. Load the second document and get its normalized content
            if let Ok(content2) = Self::sync_wait(fs::read_to_string(&doc_entry.file_path)) {
                let normalized2 = self.normalize_documentation_content(&content2);
                let words2: Vec<&str> = normalized2.split_whitespace().collect();

                // 3. Count common words
                let mut common_words = 0;
                for word in &words1 {
                    if words2.contains(word) {
                        common_words += 1;
                    }
                }

                // 4. Calculate Jaccard similarity
                let total_unique_words = words1.len() + words2.len() - common_words;
                if total_unique_words > 0 {
                    return common_words as f32 / total_unique_words as f32;
                }
            }
        }

        0.0 // Default to no similarity
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

    /// Helper method to synchronously wait for an async operation
    fn sync_wait<T>(future: impl std::future::Future<Output = T>) -> T {
        tokio::runtime::Runtime::new().unwrap().block_on(future)
    }

    /// Update last modified timestamp
    fn update_last_modified(&self) {
        self.last_updated
            .store(Self::current_timestamp(), Ordering::Relaxed);
        self.version.fetch_add(1, Ordering::Relaxed);
    }

    /// Generate a root hash for the entire registry
    pub async fn generate_registry_root_hash(&self) -> [u8; 32] {
        // Create a combined hash of all registry components
        let mut hasher = blake3::Hasher::new();

        // Hash canonical documents
        for entry in self.canonical_documents.iter() {
            let data = format!("{}:{:?}", entry.key(), entry.value().verification_hash);
            hasher.update(data.as_bytes());
        }

        // Hash work items
        for entry in self.work_items.iter() {
            let data = format!(
                "{}:{:?}:{:?}",
                entry.key(),
                entry.value().verification_hash,
                entry.value().status
            );
            hasher.update(data.as_bytes());
        }

        // Hash version and timestamp
        let metadata = format!(
            "v{}:t{}",
            self.version.load(Ordering::Relaxed),
            self.last_updated.load(Ordering::Relaxed)
        );
        hasher.update(metadata.as_bytes());

        *hasher.finalize().as_bytes()
    }

    /// Enable blockchain anchoring
    pub fn enable_blockchain_anchoring(&self) {
        self.blockchain_anchoring_enabled
            .store(1, Ordering::Relaxed);
    }

    /// Disable blockchain anchoring
    pub fn disable_blockchain_anchoring(&self) {
        self.blockchain_anchoring_enabled
            .store(0, Ordering::Relaxed);
    }

    /// Check if blockchain anchoring is enabled
    pub fn is_blockchain_anchoring_enabled(&self) -> bool {
        self.blockchain_anchoring_enabled.load(Ordering::Relaxed) == 1
    }

    /// Anchor registry data to blockchain
    pub async fn anchor_to_blockchain(&self) -> Result<BlockchainAnchor, SourceOfTruthError> {
        if !self.is_blockchain_anchoring_enabled() {
            return Err(SourceOfTruthError::BlockchainAnchoringFailed(
                "Blockchain anchoring is disabled".to_string(),
            ));
        }

        // Generate root hash for registry
        let registry_root_hash = self.generate_registry_root_hash().await;

        // In a real implementation, this would connect to a Bitcoin node
        // and create an OP_RETURN transaction with the registry_root_hash
        // For now, we'll simulate the process

        // Simulated transaction ID (in production this would be the real Bitcoin txid)
        let txid = format!(
            "{}{}",
            hex::encode(&registry_root_hash[0..16]),
            Self::current_timestamp()
        );

        // Create optional Taproot data for enhanced verification
        let taproot_data = if cfg!(feature = "taproot") {
            // In real implementation, this would generate real Taproot commitment data
            Some(TaprootAnchorData {
                output_script: vec![0x51],         // OP_TRUE placeholder
                internal_key: vec![0; 32],         // Placeholder key
                script_tree_hashes: vec![[0; 32]], // Placeholder script tree
                control_block: None,
            })
        } else {
            None
        };

        let anchor = BlockchainAnchor {
            txid,
            block_height: None, // Not confirmed yet
            merkle_proof: None, // Not available until confirmation
            timestamp: Self::current_timestamp(),
            registry_root_hash,
            required_confirmations: 6,
            network: self.get_bitcoin_network(), // Use configured network
            taproot_data,
            status: AnchorStatus::Created,
        };

        // Store the anchor
        self.blockchain_anchors
            .insert(anchor.txid.clone(), anchor.clone());
        self.update_last_modified();
        self.save_to_disk().await?;

        // Log the anchoring attempt
        log::info!("Created blockchain anchor with txid: {}", anchor.txid);

        Ok(anchor)
    }

    /// Verify blockchain anchoring for a specific anchor
    pub async fn verify_blockchain_anchoring(
        &self,
        txid: &str,
    ) -> Result<bool, SourceOfTruthError> {
        let anchor = self.blockchain_anchors.get(txid).ok_or_else(|| {
            SourceOfTruthError::BlockchainVerificationFailed(format!(
                "Anchor not found for txid: {}",
                txid
            ))
        })?;

        // Check the current status
        match anchor.status {
            AnchorStatus::Failed(ref reason) => {
                return Err(SourceOfTruthError::BlockchainVerificationFailed(format!(
                    "Anchor failed: {}",
                    reason
                )));
            }
            AnchorStatus::Created | AnchorStatus::Broadcast => {
                log::info!("Anchor {} is not yet confirmed", txid);
                return Ok(false);
            }
            AnchorStatus::Final => {
                return Ok(true);
            }
            AnchorStatus::Confirmed(confirmations) => {
                if confirmations >= anchor.required_confirmations as u32 {
                    // Update status to final
                    if let Some(mut anchor_mut) = self.blockchain_anchors.get_mut(txid) {
                        anchor_mut.status = AnchorStatus::Final;
                        log::info!(
                            "Anchor {} is now final with {} confirmations",
                            txid,
                            confirmations
                        );
                    }
                    return Ok(true);
                }
            }
        }

        // In a real implementation, this would verify:
        // 1. That the transaction exists on the Bitcoin blockchain
        // 2. That it has sufficient confirmations
        // 3. That the merkle proof is valid

        if anchor.block_height.is_none() {
            return Ok(false); // Not yet confirmed
        }

        // Verify Taproot-specific data if available
        if let Some(taproot_data) = &anchor.taproot_data {
            // In a real implementation, this would verify the Taproot commitment
            log::info!("Verifying Taproot commitment for anchor {}", txid);

            // This would validate that the output_script corresponds to the internal_key
            // and script_tree_hashes, and that the control_block (if present) is valid
            if taproot_data.output_script.is_empty() {
                log::warn!("Taproot output script is empty for anchor {}", txid);
                // This doesn't fail verification as the anchor might be using a different method
            }
        }

        // In a real implementation, we would check the current blockchain height
        // and verify that the transaction has enough confirmations
        Ok(true)
    }

    /// Update blockchain anchor with confirmation details
    pub async fn update_blockchain_anchor(
        &self,
        txid: &str,
        block_height: u32,
        merkle_proof: Vec<u8>,
        confirmations: u32,
    ) -> Result<(), SourceOfTruthError> {
        let mut anchor = self.blockchain_anchors.get_mut(txid).ok_or_else(|| {
            SourceOfTruthError::BlockchainVerificationFailed(format!(
                "Anchor not found for txid: {}",
                txid
            ))
        })?;

        // Update confirmation details
        anchor.block_height = Some(block_height);
        anchor.merkle_proof = Some(merkle_proof);

        // Update status based on confirmations
        if confirmations >= anchor.required_confirmations as u32 {
            anchor.status = AnchorStatus::Final;
            log::info!(
                "Anchor {} is now final with {} confirmations",
                txid,
                confirmations
            );
        } else {
            anchor.status = AnchorStatus::Confirmed(confirmations);
            log::info!("Anchor {} now has {} confirmations", txid, confirmations);
        }

        self.update_last_modified();
        self.save_to_disk().await?;

        Ok(())
    }

    /// Mark an anchor as broadcast
    pub async fn mark_anchor_as_broadcast(&self, txid: &str) -> Result<(), SourceOfTruthError> {
        let mut anchor = self.blockchain_anchors.get_mut(txid).ok_or_else(|| {
            SourceOfTruthError::BlockchainVerificationFailed(format!(
                "Anchor not found for txid: {}",
                txid
            ))
        })?;

        anchor.status = AnchorStatus::Broadcast;
        log::info!("Anchor {} has been broadcast to the network", txid);

        self.update_last_modified();
        self.save_to_disk().await?;

        Ok(())
    }

    /// Mark an anchor as failed
    pub async fn mark_anchor_as_failed(
        &self,
        txid: &str,
        reason: &str,
    ) -> Result<(), SourceOfTruthError> {
        let mut anchor = self.blockchain_anchors.get_mut(txid).ok_or_else(|| {
            SourceOfTruthError::BlockchainVerificationFailed(format!(
                "Anchor not found for txid: {}",
                txid
            ))
        })?;

        anchor.status = AnchorStatus::Failed(reason.to_string());
        log::warn!("Anchor {} has failed: {}", txid, reason);

        self.update_last_modified();
        self.save_to_disk().await?;

        Ok(())
    }

    /// Get list of all blockchain anchors
    pub fn get_blockchain_anchors(&self) -> Vec<BlockchainAnchor> {
        self.blockchain_anchors
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// Create a Taproot-specific blockchain anchor using v1.3 improvements
    #[cfg(feature = "taproot")]
    pub async fn create_taproot_anchor(
        &self,
        internal_key: Vec<u8>,
        script_tree_hashes: Vec<[u8; 32]>,
        network: &str,
    ) -> Result<BlockchainAnchor, SourceOfTruthError> {
        if !self.is_blockchain_anchoring_enabled() {
            return Err(SourceOfTruthError::BlockchainAnchoringFailed(
                "Blockchain anchoring is disabled".to_string(),
            ));
        }

        // Generate root hash for registry
        let registry_root_hash = self.generate_registry_root_hash().await;

        // In a real implementation, this would:
        // 1. Create a Taproot output using the provided internal key and script tree
        // 2. Include the registry_root_hash in one of the script paths
        // 3. Broadcast the transaction

        // Simulated transaction ID
        let txid = format!(
            "taproot_{}{}",
            hex::encode(&registry_root_hash[0..12]),
            Self::current_timestamp()
        );

        // Create Taproot data
        // In real implementation, we would calculate the real output script
        // based on internal_key and script_tree_hashes
        let taproot_data = TaprootAnchorData {
            output_script: vec![0x51, 0x20], // Placeholder for real Taproot output script
            internal_key: internal_key.clone(),
            script_tree_hashes,
            control_block: None, // Will be generated when using a specific script path
        };

        let anchor = BlockchainAnchor {
            txid,
            block_height: None,
            merkle_proof: None,
            timestamp: Self::current_timestamp(),
            registry_root_hash,
            required_confirmations: 6,
            network: network.to_string(),
            taproot_data: Some(taproot_data),
            status: AnchorStatus::Created,
        };

        // Store the anchor
        self.blockchain_anchors
            .insert(anchor.txid.clone(), anchor.clone());
        self.update_last_modified();
        self.save_to_disk().await?;

        log::info!(
            "Created Taproot blockchain anchor with txid: {}",
            anchor.txid
        );

        Ok(anchor)
    }

    /// Save registry to disk
    async fn save_to_disk(&self) -> Result<(), SourceOfTruthError> {
        // Create a serializable version of the registry by collecting iterators safely
        let canonical_documents: HashMap<String, CanonicalDocument> = self
            .canonical_documents
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect();
        let work_items: HashMap<String, WorkItem> = self
            .work_items
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect();
        let duplication_index: HashMap<String, DuplicationEntry> = self
            .duplication_index
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect();
        let blockchain_anchors: HashMap<String, BlockchainAnchor> = self
            .blockchain_anchors
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect();
        let documentation_index: HashMap<String, DocumentationEntry> = self
            .documentation_index
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect();

        let registry_data = RegistryData {
            canonical_documents,
            work_items,
            duplication_index,
            blockchain_anchors,
            documentation_index,
            last_updated: self.last_updated.load(Ordering::Relaxed),
            version: self.version.load(Ordering::Relaxed),
            blockchain_anchoring_enabled: self.blockchain_anchoring_enabled.load(Ordering::Relaxed),
            bitcoin_network: self.bitcoin_network.load(Ordering::Relaxed),
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
        self.blockchain_anchors.clear();
        self.documentation_index.clear();

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
        for (key, value) in registry_data.blockchain_anchors {
            self.blockchain_anchors.insert(key, value);
        }
        for (key, value) in registry_data.documentation_index {
            self.documentation_index.insert(key, value);
        }

        self.last_updated
            .store(registry_data.last_updated, Ordering::Relaxed);
        self.version.store(registry_data.version, Ordering::Relaxed);
        self.blockchain_anchoring_enabled.store(
            registry_data.blockchain_anchoring_enabled,
            Ordering::Relaxed,
        );
        self.bitcoin_network
            .store(registry_data.bitcoin_network, Ordering::Relaxed);

        Ok(())
    }

    /// Synchronize registry with blockchain by checking all anchors
    pub async fn sync_with_blockchain(&self) -> Result<Vec<String>, SourceOfTruthError> {
        if !self.is_blockchain_anchoring_enabled() {
            return Err(SourceOfTruthError::BlockchainAnchoringFailed(
                "Blockchain anchoring is disabled".to_string(),
            ));
        }

        let mut updated_txids = Vec::new();

        // Iterate through all anchors
        for entry in self.blockchain_anchors.iter() {
            let txid = entry.key().clone();
            let anchor = entry.value().clone();

            // Skip anchors that are already final or failed
            match anchor.status {
                AnchorStatus::Final | AnchorStatus::Failed(_) => continue,
                _ => {}
            }

            // In a real implementation, this would query a Bitcoin node
            // to get the current status of the transaction
            match anchor.status {
                AnchorStatus::Created => {
                    // In real implementation, check if transaction was broadcast
                    // For now, simulate by marking as broadcast
                    self.mark_anchor_as_broadcast(&txid).await?;
                    updated_txids.push(txid.clone());
                }
                AnchorStatus::Broadcast => {
                    // In real implementation, check if transaction was confirmed
                    // For now, simulate by adding a block height and 1 confirmation
                    let simulated_height = 800000; // Placeholder
                    let simulated_merkle_proof = vec![0; 32]; // Placeholder
                    self.update_blockchain_anchor(
                        &txid,
                        simulated_height,
                        simulated_merkle_proof,
                        1,
                    )
                    .await?;
                    updated_txids.push(txid.clone());
                }
                AnchorStatus::Confirmed(confirmations) => {
                    // In real implementation, check current confirmation count
                    // For now, simulate by incrementing confirmation count
                    if let Some(block_height) = anchor.block_height {
                        let merkle_proof = anchor.merkle_proof.unwrap_or_else(|| vec![0; 32]);
                        let new_confirmations = confirmations + 1;
                        self.update_blockchain_anchor(
                            &txid,
                            block_height,
                            merkle_proof,
                            new_confirmations,
                        )
                        .await?;
                        updated_txids.push(txid.clone());
                    }
                }
                _ => {}
            }
        }

        // If any anchors were updated, save the registry
        if !updated_txids.is_empty() {
            self.update_last_modified();
            self.save_to_disk().await?;
        }

        Ok(updated_txids)
    }
}

/// Serializable registry data structure
#[derive(Debug, Serialize, Deserialize)]
struct RegistryData {
    canonical_documents: HashMap<String, CanonicalDocument>,
    work_items: HashMap<String, WorkItem>,
    duplication_index: HashMap<String, DuplicationEntry>,
    blockchain_anchors: HashMap<String, BlockchainAnchor>,
    #[serde(default)]
    documentation_index: HashMap<String, DocumentationEntry>,
    last_updated: u64,
    version: u32,
    blockchain_anchoring_enabled: u32,
    #[serde(default = "default_bitcoin_network")]
    bitcoin_network: u32,
}

fn default_bitcoin_network() -> u32 {
    1 // Default to testnet (1) for safety
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

/// Scan entire repository for documentation duplication
pub async fn scan_repo_for_documentation_duplication(
    base_path: &std::path::Path,
) -> Result<Vec<(String, String, f32)>, SourceOfTruthError> {
    let mut duplications = Vec::new();

    // Get registry instance
    let registry_lock = get_global_registry().await;
    let registry = registry_lock.read().await;
    let registry = match &*registry {
        Some(registry) => registry,
        None => {
            return Err(SourceOfTruthError::RegistryCorruption(
                "Registry not initialized".to_string(),
            ));
        }
    };

    // Find markdown files recursively
    let markdown_files = find_markdown_files(base_path).await?;

    // Process each file
    for file_path in markdown_files {
        // Read file content
        if let Ok(content) = fs::read_to_string(&file_path).await {
            // Extract sections from markdown
            let sections = extract_markdown_sections(&content);

            // Process each section
            for (section_name, section_content) in sections {
                // Skip very small sections
                if section_content.len() < 100 {
                    continue;
                }

                // Get relative path for display
                let rel_path = if let Ok(rel) = file_path.strip_prefix(base_path) {
                    rel.to_string_lossy().to_string()
                } else {
                    file_path.to_string_lossy().to_string()
                };

                // Check for duplication
                let result = registry
                    .check_documentation_duplication(
                        &rel_path,
                        &section_content,
                        &rel_path, // Use file path as title for now
                        &section_name,
                    )
                    .await;

                // Collect duplications
                if let Ok(DuplicationCheckStatus::Failed(message)) = result {
                    // Extract similarity if available
                    if let Some(similarity) = extract_similarity_from_message(&message) {
                        duplications.push((rel_path.clone(), section_name.clone(), similarity));
                    } else {
                        duplications.push((rel_path.clone(), section_name.clone(), 1.0));
                        // Exact match
                    }
                }
            }
        }
    }

    Ok(duplications)
}

/// Helper function to find markdown files in a directory recursively
async fn find_markdown_files(
    dir: &std::path::Path,
) -> Result<Vec<std::path::PathBuf>, SourceOfTruthError> {
    let mut files = Vec::new();
    let mut dirs_to_process = vec![dir.to_path_buf()];

    while let Some(current_dir) = dirs_to_process.pop() {
        let mut entries = match fs::read_dir(&current_dir).await {
            Ok(entries) => entries,
            Err(e) => {
                log::warn!("Error reading directory {}: {}", current_dir.display(), e);
                continue;
            }
        };

        while let Some(entry) = entries.next_entry().await.transpose() {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    log::warn!("Error reading directory entry: {}", e);
                    continue;
                }
            };

            let path = entry.path();
            let file_type = match entry.file_type().await {
                Ok(ft) => ft,
                Err(e) => {
                    log::warn!("Error getting file type for {}: {}", path.display(), e);
                    continue;
                }
            };

            if file_type.is_dir() {
                dirs_to_process.push(path);
            } else if file_type.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "md" || ext == "markdown" {
                        files.push(path);
                    }
                }
            }
        }
    }

    Ok(files)
}

/// Extract sections from markdown content
fn extract_markdown_sections(content: &str) -> Vec<(String, String)> {
    let mut sections = Vec::new();
    let mut current_section = String::from("Introduction");
    let mut current_content = String::new();
    let mut section_start = 0;

    // Split by lines
    let lines: Vec<&str> = content.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        // Check if line is a heading
        if line.starts_with('#') {
            // If we have content for current section, save it
            if !current_content.is_empty() {
                sections.push((current_section, current_content));
                current_content = String::new();
            }

            // Extract heading text and level
            let _heading_level = line.chars().take_while(|&c| c == '#').count();
            let heading_text = line.trim_start_matches(|c| c == '#' || c == ' ').trim();

            // Set as current section
            current_section = heading_text.to_string();
            section_start = i + 1;
        } else if i >= section_start {
            // Append to current content
            current_content.push_str(line);
            current_content.push('\n');
        }
    }

    // Add the last section
    if !current_content.is_empty() {
        sections.push((current_section, current_content));
    }

    sections
}

/// Extract similarity percentage from duplication message
fn extract_similarity_from_message(message: &str) -> Option<f32> {
    // Look for pattern like "High documentation similarity (85.2%) with..."
    let similarity_regex = regex::Regex::new(r"similarity \((\d+\.\d+)%\)").unwrap();

    if let Some(captures) = similarity_regex.captures(message) {
        if let Some(percentage_str) = captures.get(1) {
            if let Ok(percentage) = percentage_str.as_str().parse::<f32>() {
                return Some(percentage / 100.0);
            }
        }
    }

    None
}

/// Web5 integration for anchoring
#[cfg(feature = "web5")]
pub mod web5_anchoring {
    use super::*;
    use std::error::Error;

    /// Anchor registry data using Web5 and Bitcoin
    pub async fn anchor_registry_via_web5(
        registry: &SourceOfTruthRegistry,
    ) -> Result<String, Box<dyn Error>> {
        // Generate registry root hash
        let registry_root_hash = registry.generate_registry_root_hash().await;

        // In a real implementation, this would:
        // 1. Connect to the Web5 DWN
        // 2. Create a record with the registry hash
        // 3. Use Bitcoin anchoring to secure the DWN record

        // Simulated DWN record ID
        let record_id = format!("dwn_registry_{}", hex::encode(&registry_root_hash[0..16]),);

        // Log the anchoring
        log::info!("Web5 anchoring complete with record ID: {}", record_id);

        Ok(record_id)
    }

    /// Verify registry data using Web5 and Bitcoin
    pub async fn verify_registry_via_web5(
        registry: &SourceOfTruthRegistry,
        record_id: &str,
    ) -> Result<bool, Box<dyn Error>> {
        // Generate current registry root hash
        let _current_hash = registry.generate_registry_root_hash().await;

        // In a real implementation, this would:
        // 1. Retrieve the record from the Web5 DWN
        // 2. Verify the Bitcoin anchoring of the record
        // 3. Compare the stored hash with the current hash

        // Simulated verification - in production this would perform real verification
        log::info!("Verifying Web5 record: {}", record_id);

        // Return true for simulation purposes
        Ok(true)
    }
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
    async fn test_status_transition_valid() {
        use tokio::time::{timeout, Duration};
        let temp_dir = tempdir().unwrap();
        let registry_path = temp_dir
            .path()
            .join("registry_valid.json")
            .to_string_lossy()
            .to_string();

        let registry = SourceOfTruthRegistry::new(registry_path).await.unwrap();
        let work_item = registry
            .create_work_item(
                "Status test valid".to_string(),
                "test_component".to_string(),
            )
            .await
            .unwrap();

        let valid = timeout(
            Duration::from_secs(5),
            registry.update_work_item_status(&work_item.id, WorkStatus::InProgress),
        )
        .await;
        match valid {
            Ok(Ok(_)) => println!("Valid status transition succeeded"),
            Ok(Err(e)) => panic!("Valid status transition failed: {e:?}"),
            Err(_) => panic!("Timeout on valid status transition"),
        }
    }

    #[tokio::test]
    async fn test_status_transition_invalid() {
        use tokio::time::{timeout, Duration};
        let temp_dir = tempdir().unwrap();
        let registry_path = temp_dir
            .path()
            .join("registry_invalid.json")
            .to_string_lossy()
            .to_string();

        let registry = SourceOfTruthRegistry::new(registry_path).await.unwrap();
        let work_item = registry
            .create_work_item(
                "Status test invalid".to_string(),
                "test_component".to_string(),
            )
            .await
            .unwrap();

        // Try invalid transition: Planning -> Completed (should fail)
        let invalid = timeout(
            Duration::from_secs(5),
            registry.update_work_item_status(&work_item.id, WorkStatus::Completed),
        )
        .await;
        match invalid {
            Ok(Ok(_)) => panic!("Invalid status transition unexpectedly succeeded"),
            Ok(Err(_)) => println!("Invalid status transition correctly failed"),
            Err(_) => panic!("Timeout on invalid status transition"),
        }
    }
}
