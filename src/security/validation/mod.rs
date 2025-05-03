// Validation module
pub mod transaction;
pub mod taproot;

// Re-export commonly used types
pub use transaction::validate_transaction;
pub use taproot::validate_taproot_transaction;

/// Validates a transaction against Bitcoin protocol rules
pub fn validate(tx_bytes: &[u8]) -> bool {
    match transaction::validate_transaction(tx_bytes) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validation() {
        let dummy_tx = vec![0u8; 100]; // Placeholder
        
        // Should pass validation
        assert!(validate(&dummy_tx));
    }
} 