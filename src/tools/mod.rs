//! Tools Module [AIR-3][AIS-3][BPC-3][AIT-3]
//!
//! This module provides various utility tools for the Anya Core system,
//! following official Bitcoin Improvement Proposals (BIPs) and canonical
//! Source of Truth Registry standards.

pub mod commit_tracker;
pub mod doc_duplication_scanner;
pub mod doc_duplication_scanner_cli;
pub mod markdown;
pub mod source_of_truth_registry;

// Re-export commonly used tools
pub use commit_tracker::{update_ai_labelling_file, CommitInfo};
pub use markdown::{DocError, DocumentationValidator};
pub use source_of_truth_registry::{
    get_global_registry, initialize_global_registry, CanonicalDocument, CanonicalStatus,
    DuplicationCheckStatus, SourceOfTruthError, SourceOfTruthRegistry, WorkItem, WorkStatus,
};
