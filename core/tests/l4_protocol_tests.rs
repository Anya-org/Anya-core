#![feature(edition2021)]
//! Tests for Layer 4 protocol implementation
//! [AIR-3][AIS-3][BPC-3][AIP-3][RES-3]

#[cfg(test)]
mod tests {
    // Fix imports to use secp256k1 crate directly instead of through bitcoin
    use bitcoin::Network;
    use rand::thread_rng;
    use secp256k1::{Secp256k1, SecretKey};
    use std::str::FromStr;

    // Use fully qualified path to avoid the std::core vs our core crate conflict
    use crate::l4_protocol::{AnyaL4Protocol, DlcContract, BIP341_SILENT_LEAF};

    #[test]
    fn test_silent_leaf_pattern() {
        // Verify the silent leaf pattern constant matches specification
        assert_eq!(BIP341_SILENT_LEAF, "0x8f3a1c29566443e2e2d6e5a9a5a4e8d");
    }

    #[test]
    fn test_protocol_initialization() {
        // Test different initialization methods
        let l4_default = AnyaL4Protocol::new();
        assert_eq!(l4_default.network, Network::Testnet);

        let l4_mainnet = AnyaL4Protocol::with_network(Network::Bitcoin);
        assert_eq!(l4_mainnet.network, Network::Bitcoin);

        // Fix assertion to use public getter method instead of private field
        let l4_endpoint = AnyaL4Protocol::with_endpoint("https://blockstream.info/api/");
        assert!(l4_endpoint.get_endpoint().contains("blockstream.info"));
    }

    #[test]
    fn test_hsm_initialization() {
        let mut l4 = AnyaL4Protocol::new();

        // Initialize HSM
        let result = l4.init_hsm("software");
        assert!(result.is_ok());

        // Use is_hsm_initialized method instead of accessing private field
        assert!(l4.is_hsm_initialized());

        // Test HSM initialization with different types
        let result = l4.init_hsm("hardware");
        assert!(result.is_ok());
    }

    // Use a simple regular test instead of async tokio test for now
    #[test]
    fn test_dlc_contract_creation() {
        let l4 = AnyaL4Protocol::new();

        // Create a key pair for testing - use the correct API
        let pubkey_str = "03a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2";
        let pubkey = bitcoin::secp256k1::PublicKey::from_str(pubkey_str).unwrap();

        // Define outcomes
        let outcomes = vec![
            "btc_price_above_100k".to_string(),
            "btc_price_below_100k".to_string(),
        ];

        // Create a DLC contract - use synchronous version for testing
        let contract = l4.create_dlc_contract_sync(pubkey, outcomes.clone());

        // Verify the contract
        assert_eq!(contract.oracle_pubkey, pubkey);
        assert_eq!(contract.outcomes, outcomes);
        assert_eq!(contract.silent_leaf, BIP341_SILENT_LEAF);
        assert!(contract.taproot_script.is_some());
    }
}
