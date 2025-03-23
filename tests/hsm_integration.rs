//! YubiHSM2 Integration Tests
#![cfg(feature = "hsm")]

use anya_core::hsm::{YubiHSM, HsmSigner};
use bitcoin::psbt::PartiallySignedTransaction;

#[test]
fn test_hsm_signing() {
    let hsm = YubiHSM::connect("mock://localhost").unwrap();
    let signer = HsmSigner::new(hsm, "secp256k1").unwrap();
    
    let mut psbt = load_test_psbt();
    signer.sign_psbt(&mut psbt).unwrap();
    
    assert!(psbt.finalized(), "HSM failed to properly sign PSBT");
    verify_taproot_signature(&psbt);
}

#[cfg(not(feature = "hsm"))]
mod hsm_mock {
    // Mock implementation for CI environments
} 