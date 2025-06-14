#[test]
fn verify_bip341_implementation() {
    let config = BitcoinConfig::default();
    assert!(
        config.supports_bip("BIP-341").unwrap(),
        "BIP-341 (Taproot) not properly implemented"
    );

    let silent_leaf = hex::decode(BIP341_SILENT_LEAF.trim_start_matches("0x"))
        .expect("Invalid SILENT_LEAF format");
    assert_eq!(silent_leaf.len(), 32, "SILENT_LEAF must be 32 bytes");
}
