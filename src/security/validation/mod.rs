// Security validation module
// This module now re-exports functionality from the consolidated Bitcoin validation

// Re-export commonly used types from the Bitcoin module
pub use crate::bitcoin::validation::{TransactionValidator, ValidationError};

/// Validates a transaction against Bitcoin protocol rules
/// This is a convenience wrapper around the core Bitcoin validation
pub fn validate(tx_bytes: &[u8]) -> bool {
    use bitcoin::consensus::deserialize;
    
    // Try to deserialize the transaction
    match deserialize::<bitcoin::Transaction>(tx_bytes) {
        Ok(tx) => {
            // Use the consolidated validator
            let validator = crate::bitcoin::validation::TransactionValidator::new();
            validator.validate(&tx).is_ok()
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::consensus::serialize;
    
    #[test]
    fn test_validation() {
        // Create a simple valid transaction
        let tx = crate::tests::common::test_utilities::TestTransactionFactory::create_dummy_transaction();
        let tx_bytes = serialize(&tx);
        
        // Should pass validation
        assert!(validate(&tx_bytes));
    }
} 