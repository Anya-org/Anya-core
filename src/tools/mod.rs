//! Tools Module [AIR-3][AIS-3][BPC-3][AIT-3]
//! 
//! This module provides various utility tools for the Anya Core system,
//! following official Bitcoin Improvement Proposals (BIPs).

pub mod commit_tracker;
pub mod markdown;

// Re-export commonly used tools
pub use markdown::{DocumentationValidator, DocError};
pub use commit_tracker::{CommitInfo, update_ai_labelling_file};
