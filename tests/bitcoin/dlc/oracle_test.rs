// [AIR-3][AIS-3][BPC-3][AIT-3]
use anya_core::bitcoin::dlc::{
    oracle::{
        implement_non_interactive_oracle, create_privacy_preserving_oracle,
        OracleAttestationParams, SchnorrParams, Error
    },
    Oracle, OracleInfo
};
use bitcoin::PublicKey;
use bitcoin::hashes::hex::FromHex;
use std::collections::HashMap;

/// Test the creation of a privacy-preserving oracle
/// [AIT-3][BPC-3]
#[test]
fn test_create_privacy_preserving_oracle() {
    // Test creation of privacy-preserving oracle
    let result = create_privacy_preserving_oracle("TestOracle", "https://oracle.example.com");
    
    // With the fixed implementation, this should now succeed
    assert!(result.is_ok());
    
    let oracle = result.expect("Failed to create oracle");
    assert_eq!(oracle.info.name, "TestOracle");
    assert_eq!(oracle.info.endpoint, "https://oracle.example.com");
    assert!(oracle.info.properties.contains_key("non_interactive"));
    assert_eq!(oracle.info.properties.get("non_interactive").unwrap(), "true");
    assert!(oracle.info.properties.contains_key("schnorr_signatures"));
    assert!(oracle.info.properties.contains_key("musig_support"));
}

/// Test the implementation of a non-interactive oracle
/// [AIT-3][BPC-3]
#[test]
fn test_implement_non_interactive_oracle() {
    // Create a commitment and public key for testing
    let commitment = "0x8f3a123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    
    // Create a valid public key for testing
    let pubkey_hex = "02aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let bytes = Vec::from_hex(pubkey_hex).expect("Failed to parse hex");
    let pubkey = PublicKey::from_slice(&bytes).expect("Failed to create public key");
    
    // Test the non-interactive oracle implementation
    let result = implement_non_interactive_oracle(commitment, &pubkey);
    assert!(result.is_ok());
    
    // Verify the result
    let params = result.expect("Failed to implement non-interactive oracle");
    assert_eq!(params.commitment, commitment);
    assert_eq!(params.oracle_pubkey, pubkey);
} 