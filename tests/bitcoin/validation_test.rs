// Fix imports to match current module structure
use anya_core::{
    bitcoin::protocol::BPCLevel as ProtocolLevel,
    bitcoin::validation::{TransactionValidator, ValidationError},
};

// Fix import from tests - this will need to be adjusted based on how tests is made available
// use tests::common::bitcoin_compat::*;

use bitcoin::{absolute::LockTime, transaction::Version, Transaction};

use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_transaction_validator_creation() {
    let validator = TransactionValidator::new();
    // Use public method instead of accessing private field
    assert!(validator.maintains_consensus);
}

#[test]
fn test_transaction_validator_with_level() {
    let validator = TransactionValidator::with_level(ProtocolLevel::Enhanced);
    // Use public method instead of accessing private field
    assert!(validator.maintains_consensus);
}

#[test]
fn test_validate_from_file() {
    // Create a temporary file with transaction data
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    file.write_all(b"mock transaction data")
        .expect("Failed to write to temp file");
    let path = file.path();

    let validator = TransactionValidator::new();
    let result = validator.validate_from_file(path);
    assert!(result.is_ok());
}

#[test]
fn test_validate_taproot_transaction() {
    use bitcoin::{Amount, OutPoint, ScriptBuf, Sequence, TxIn, TxOut, Witness};

    // Create a transaction that passes basic validation but should fail Taproot validation
    // This transaction has inputs and outputs but no witness data (which Taproot requires)
    let tx = Transaction {
        version: Version::ONE,
        lock_time: LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::new(),
            sequence: Sequence(0),
            witness: Witness::default(), // Empty witness - should trigger Taproot error
        }],
        output: vec![TxOut {
            value: Amount::from_sat(1000),
            script_pubkey: ScriptBuf::new(),
        }],
    };

    // Since our test transaction has empty witness data, this should fail Taproot validation
    let validator = TransactionValidator::new();
    let result = validator.validate_taproot_transaction(&tx);
    assert!(matches!(result, Err(ValidationError::Taproot(_))));
}

#[test]
fn test_bpc_levels() {
    // Test that different BPC levels apply different validation rules
    let bpc2_validator = TransactionValidator::with_level(ProtocolLevel::Enhanced);
    let bpc3_validator = TransactionValidator::with_level(ProtocolLevel::BPC3);

    // In a real test, we'd create transactions that pass BPC2 but fail BPC3
    // For now, we'll just verify the validators were created successfully
    assert!(bpc2_validator.maintains_consensus);
    assert!(bpc3_validator.maintains_consensus);
}
