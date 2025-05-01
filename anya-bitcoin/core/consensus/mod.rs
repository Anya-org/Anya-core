pub mod validation;
pub mod rules;
pub mod params;

// Re-export commonly used items
pub use validation::{validate_block_header, validate_block_hash};
pub use rules::{check_consensus_rules, verify_pow};
pub use params::ConsensusParams; 