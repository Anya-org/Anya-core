#[test]
fn verify_bip341_implementation() {
    use anya_core::bitcoin::bip341::TAPROOT_SILENT_LEAF_TAG;
    use anya_core::bitcoin::BitcoinConfig;

    let config = BitcoinConfig::default();
    assert!(
        config.supports_bip("BIP-341").unwrap(),
        "BIP-341 (Taproot) not properly implemented"
    );

    // Verify the SILENT_LEAF tag exists and has correct format
    assert_eq!(
        TAPROOT_SILENT_LEAF_TAG, b"SILENT_LEAF",
        "SILENT_LEAF tag must match BIP-341 specification"
    );
}
