use anya_core::bitcoin::{
    validation::{TransactionValidator, ValidationError},
    protocol::{BitcoinProtocol, BPCLevel},
    taproot::TaprootValidator
};
use bitcoin::{Transaction, Script};
use std::path::Path;
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_transaction_validator_creation() {
    let validator = TransactionValidator::new();
    assert_eq!(validator.protocol.get_level(), BPCLevel::BPC3);
}

#[test]
fn test_transaction_validator_with_level() {
    let validator = TransactionValidator::with_level(BPCLevel::BPC2);
    assert_eq!(validator.protocol.get_level(), BPCLevel::BPC2);
}

#[test]
fn test_validate_from_file() {
    // Create a temporary file with transaction data
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    file.write_all(b"mock transaction data").expect("Failed to write to temp file");
    let path = file.path();
    
    let validator = TransactionValidator::new();
    let result = validator.validate_from_file(path);
    assert!(result.is_ok());
}

#[test]
fn test_validate_taproot_transaction() {
    // In a real test, we would create a valid Taproot transaction
    // For now, we'll mock this with a dummy transaction
    let tx = Transaction {
        version: 2,
        lock_time: 0,
        input: vec![],
        output: vec![],
    };
    
    // Since our test transaction has no witness data, this should fail
    let validator = TransactionValidator::new();
    let result = validator.validate_taproot_transaction(&tx);
    assert!(matches!(result, Err(ValidationError::Taproot(_))));
}

#[test]
fn test_bpc_levels() {
    // Test that different BPC levels apply different validation rules
    let bpc2_validator = TransactionValidator::with_level(BPCLevel::BPC2);
    let bpc3_validator = TransactionValidator::with_level(BPCLevel::BPC3);
    
    // In a real test, we'd create transactions that pass BPC2 but fail BPC3
    // For now, we'll just verify the levels are set correctly
    assert_eq!(bpc2_validator.protocol.get_level(), BPCLevel::BPC2);
    assert_eq!(bpc3_validator.protocol.get_level(), BPCLevel::BPC3);
} 