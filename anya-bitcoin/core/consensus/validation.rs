//! Bitcoin transaction validation [AIS-3][BPC-3][DAO-3]
//! This module re-exports from the main validation module for backward compatibility

// Re-export from main validation module
pub use crate::bitcoin::validation::{
    TransactionValidator, 
    ValidationError,
    HistoricalTransactionDB,
    VerificationRecord,
};

/// Legacy helper for file-based validation for backward compatibility
pub fn validate_from_file(path: &std::path::Path) -> Result<(), ValidationError> {
    use std::fs;
    use bitcoin::consensus::deserialize;
    
    let data = fs::read(path)?;
    let tx: bitcoin::Transaction = deserialize(&data)
        .map_err(|e| ValidationError::Failed(format!("Failed to deserialize transaction: {}", e)))?;
    
    let validator = TransactionValidator::new();
    validator.validate(&tx)
} 
