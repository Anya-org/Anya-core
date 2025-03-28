#!/bin/bash
set -e

echo "Creating test cases for Layer 4 protocol..."

# Create a directory for tests
mkdir -p core/tests

# Create test for BIP compliance
cat > core/tests/l4_protocol_tests.rs << 'RUST'
//! Tests for Layer 4 protocol implementation
//! [AIR-3][AIS-3][BPC-3][AIP-3][RES-3]

#[cfg(test)]
mod tests {
    use bitcoin::Network;
    use bitcoin::secp256k1::{Secp256k1, KeyPair, PublicKey};
    use core::l4_protocol::{AnyaL4Protocol, DlcContract, BIP341_SILENT_LEAF};
    
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
        
        let l4_endpoint = AnyaL4Protocol::with_endpoint("https://blockstream.info/api/");
        assert_eq!(l4_endpoint.rpc_adapter.endpoints[0], "https://blockstream.info/api/");
    }
    
    #[test]
    fn test_hsm_initialization() {
        let mut l4 = AnyaL4Protocol::new();
        
        // Initialize HSM
        let result = l4.init_hsm("software");
        assert!(result.is_ok());
        assert!(l4.hsm_initialized);
        
        // Test HSM initialization with different types
        let result = l4.init_hsm("hardware");
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_dlc_contract_creation() {
        let l4 = AnyaL4Protocol::new();
        
        // Create a key pair for testing
        let secp = Secp256k1::new();
        let keypair = KeyPair::new(&secp, &mut rand::thread_rng());
        let pubkey = keypair.public_key();
        
        // Define outcomes
        let outcomes = vec![
            "btc_price_above_100k".to_string(),
            "btc_price_below_100k".to_string(),
        ];
        
        // Create a DLC contract
        let contract = l4.create_dlc_contract(pubkey, outcomes.clone()).await.unwrap();
        
        // Verify the contract
        assert_eq!(contract.oracle_pubkey, pubkey);
        assert_eq!(contract.outcomes, outcomes);
        assert_eq!(contract.silent_leaf, BIP341_SILENT_LEAF);
        assert!(contract.taproot_script.is_some());
        
        // Test JSON serialization
        let json = contract.to_json();
        assert!(json.get("oracle_pubkey").is_some());
        assert!(json.get("outcomes").is_some());
        assert!(json.get("silent_leaf").is_some());
    }
}
RUST

# Create a shell script to run the tests
cat > test_l4_protocol.sh << 'BASH'
#!/bin/bash
set -e

echo "Running Layer 4 protocol tests..."

# Build the project
cargo build

# Run the unit tests
cargo test --lib

# Run the integration tests
cargo test --test l4_protocol_tests

echo "Running anya-core with different arguments..."

# Test default behavior
echo -e "\nRunning with default settings:"
cargo run --bin anya-core

# Test with network specification
echo -e "\nRunning on mainnet:"
cargo run --bin anya-core -- --network mainnet

# Test help command
echo -e "\nShowing help:"
cargo run --bin anya-core -- help

# Test DLC contract creation
echo -e "\nCreating DLC contract:"
cargo run --bin anya-core -- create-dlc 03a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2 outcome1,outcome2

echo "All tests completed successfully!"
BASH

chmod +x test_l4_protocol.sh

echo "Test cases created successfully. Run ./test_l4_protocol.sh to execute them."
