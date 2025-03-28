#![feature(edition2021)]
#[test]
fn test_hsm_taproot_compliance() {
    let provider = Yubihsm2Provider::new(BIP341_SILENT_LEAF);
    let sig = provider.sign_taproot(msg);
    assert!(verify_schnorr_signature(sig, pubkey));
} 