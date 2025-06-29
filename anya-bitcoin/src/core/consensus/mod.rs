pub mod params;
pub mod rules;
pub mod types;
pub mod validation;

// Re-export commonly used items
pub use params::ConsensusParams;
pub use rules::{check_consensus_rules, verify_pow};
pub use types::{Proof, TransactionId, VerifyFlags};
pub use validation::{validate_block_hash, validate_block_header};

// Re-export Bitcoin types for convenience
pub use bitcoin::{Block, BlockHash, Transaction, Txid};

// Bitcoin consensus implementation
//
use crate::core::error::AnyaResult;
use async_trait::async_trait;

/// Block validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the block is valid
    pub is_valid: bool,
    /// Validation error message if invalid
    pub error: Option<String>,
    /// Block hash
    pub block_hash: BlockHash,
}

/// Consensus validator interface
#[async_trait]
pub trait Validator: Send + Sync {
    /// Validate a block according to consensus rules
    async fn validate_block(&self, block: &Block) -> AnyaResult<ValidationResult>;

    /// Validate a transaction according to consensus rules
    async fn validate_transaction(&self, tx: &Transaction) -> AnyaResult<bool>;

    /// Check if a transaction is in the UTXO set
    async fn is_transaction_in_utxo(&self, txid: &Txid) -> AnyaResult<bool>;
}

/// No-op validator implementation for testing
pub struct NoopValidator;

#[async_trait]
impl Validator for NoopValidator {
    async fn validate_block(&self, block: &Block) -> AnyaResult<ValidationResult> {
        Ok(ValidationResult {
            is_valid: true,
            error: None,
            block_hash: block.block_hash(),
        })
    }

    async fn validate_transaction(&self, _tx: &Transaction) -> AnyaResult<bool> {
        Ok(true)
    }

    async fn is_transaction_in_utxo(&self, _txid: &Txid) -> AnyaResult<bool> {
        Ok(false)
    }
}
