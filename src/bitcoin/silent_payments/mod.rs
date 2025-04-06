// BIP353 Silent Payment Address Generation
pub fn generate_silent_address(
    secp: &Secp256k1<All>,
    scan_key: &SecretKey,
    spend_key: &SecretKey
) -> Address {
    let combined = secp.tweak_add_assign(scan_key, spend_key).unwrap();
    Address::p2tr(secp, combined, None, Network::Bitcoin)
} 