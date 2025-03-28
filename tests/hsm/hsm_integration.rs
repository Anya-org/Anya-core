#![feature(edition2021)]
//! Comprehensive HSM Test Suite
#![cfg(feature = "hsm")]

use anya_core::{hsm::{YubiHSM, HsmSigner}, bitcoin::psbt::PartiallySignedTransaction};
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn test_hsm_key_lifecycle() {
    let hsm = YubiHSM::connect("mock://localhost").unwrap();
    let signer = HsmSigner::new(hsm, "secp256k1").unwrap();
    
    // Key generation test
    let (pubkey, handle) = signer.generate_taproot_key("test-key").await.unwrap();
    assert!(!pubkey.to_string().is_empty(), "Invalid public key generated");
    
    // Signing test
    let mut psbt = load_test_psbt();
    signer.sign_psbt(&mut psbt).await.unwrap();
    assert!(psbt.finalized(), "PSBT not properly finalized");
    
    // Key deletion test
    signer.delete_key(&handle).await.unwrap();
    assert!(signer.get_key(&handle).await.is_err(), "Key not deleted");
}

#[tokio::test]
async fn test_hsm_failure_scenarios() {
    // Test invalid HSM URL
    let hsm = YubiHSM::connect("invalid://url").await;
    assert!(hsm.is_err(), "Should reject invalid HSM URL");

    // Test operation timeout
    let hsm = YubiHSM::connect("mock://localhost?timeout=100").unwrap();
    let signer = HsmSigner::new(hsm, "secp256k1").unwrap();
    
    let result = timeout(Duration::from_millis(50), signer.generate_taproot_key("timeout-test"))
        .await;
    assert!(result.is_err(), "Timeout not triggered");
} 