//! Validation port for Bitcoin components
//! 
//! This port defines the validation interface for Bitcoin components in the hexagonal architecture.
//! It provides methods for validating transactions, blocks, scripts, and signatures.

use bitcoin::{Block, Transaction, ScriptBuf, TxOut, PublicKey};
use std::collections::HashMap;

/// Error types for validation operations
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Transaction validation error: {0}")]
    Transaction(String),
    
    #[error("Block validation error: {0}")]
    Block(String),
    
    #[error("Script validation error: {0}")]
    Script(String),
    
    #[error("Signature validation error: {0}")]
    Signature(String),
    
    #[error("Taproot validation error: {0}")]
    Taproot(String),
    
    #[error("Consensus rule violation: {0}")]
    ConsensusRule(String),
}

/// Validation result type
pub type ValidationResult<T> = Result<T, ValidationError>;

/// Validation port interface
pub trait ValidationPort {
    /// Validate a transaction
    fn validate_transaction(&self, tx: &Transaction, utxos: &HashMap<TxOut, u32>) -> ValidationResult<()>;
    
    /// Validate a block
    fn validate_block(&self, block: &Block, height: u32) -> ValidationResult<()>;
    
    /// Validate a script
    fn validate_script(&self, script: &ScriptBuf, tx: &Transaction, input_index: usize) -> ValidationResult<()>;
    
    /// Validate a signature against a public key
    fn validate_signature(&self, 
                         signature: &[u8], 
                         message: &[u8], 
                         public_key: &PublicKey) -> ValidationResult<()>;
    
    /// Validate Taproot execution
    fn validate_taproot_execution(&self, 
                                 tx: &Transaction, 
                                 input_index: usize,
                                 control_block: &[u8]) -> ValidationResult<()>;
    
    /// Validate against consensus rules
    fn validate_consensus(&self, block: &Block, height: u32) -> ValidationResult<()>;
}

/// Extended validation methods for BIP-341 (Taproot)
pub trait TaprootValidationPort: ValidationPort {
    /// Validate a Taproot output
    fn validate_taproot_output(&self, tx_out: &TxOut) -> ValidationResult<()>;
    
    /// Validate a Taproot spend path
    fn validate_taproot_spend_path(&self, 
                                  tx: &Transaction, 
                                  input_index: usize,
                                  leaf_script: &ScriptBuf, 
                                  leaf_version: u8) -> ValidationResult<()>;
    
    /// Validate a Taproot script path
    fn validate_taproot_script_path(&self,
                                   tx: &Transaction,
                                   input_index: usize,
                                   internal_key: &PublicKey,
                                   merkle_proof: &[u8]) -> ValidationResult<()>;
}

/// Extended validation methods for Layer 2 protocols
pub trait Layer2ValidationPort: ValidationPort {
    /// Validate a Lightning Network HTLC
    fn validate_lightning_htlc(&self, 
                              tx: &Transaction, 
                              htlc_output_index: usize,
                              preimage: Option<&[u8]>,
                              timeout: u32) -> ValidationResult<()>;
    
    /// Validate a RGB commitment
    fn validate_rgb_commitment(&self, 
                              tx: &Transaction, 
                              commitment_data: &[u8],
                              schema_id: &str) -> ValidationResult<()>;
    
    /// Validate a DLC contract execution
    fn validate_dlc_execution(&self,
                             tx: &Transaction,
                             oracle_signatures: &[Vec<u8>],
                             outcomes: &[String]) -> ValidationResult<()>;
}