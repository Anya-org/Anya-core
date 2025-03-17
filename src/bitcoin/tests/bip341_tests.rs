//! Tests for the BIP-341 (Taproot) implementation
//! [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]

use crate::bitcoin::bip341::{
    TaprootLeaf, TaprootMerkleTree, Bip341Taproot, TaprootSpend, TaprootOutput,
    LeafVersion, Bip341Error,
};
use crate::bitcoin::bip340::{XOnlyPublicKey, SchnorrSignature, Bip340Schnorr};
use bitcoin::hashes::{sha256, Hash};

// Helper function to create a test key
fn create_test_key(seed: u8) -> XOnlyPublicKey {
    let mut key_bytes = [0u8; 32];
    key_bytes.iter_mut().for_each(|b| *b = seed);
    XOnlyPublicKey::from_bytes(key_bytes)
}

#[test]
fn test_leaf_creation() {
    // Create a leaf with default version
    let script = vec![0x51, 0x21, 0x03]; // OP_1 OP_SIZE OP_PUSH3
    let leaf = TaprootLeaf::new(LeafVersion::Default, script.clone());
    
    // Verify version and script
    assert_eq!(leaf.version, LeafVersion::Default);
    assert_eq!(leaf.script, script);
    
    // Create a leaf with future version
    let future_version = 0xc1;
    let leaf2 = TaprootLeaf::new(LeafVersion::Future(future_version), script.clone());
    
    // Verify version and script
    assert_eq!(leaf2.version, LeafVersion::Future(future_version));
    assert_eq!(leaf2.script, script);
}

#[test]
fn test_leaf_hash() {
    // Create two different leaves
    let script1 = vec![0x51]; // OP_1
    let script2 = vec![0x52]; // OP_2
    let leaf1 = TaprootLeaf::new(LeafVersion::Default, script1);
    let leaf2 = TaprootLeaf::new(LeafVersion::Default, script2);
    
    // Compute hashes
    let hash1 = leaf1.compute_leaf_hash();
    let hash2 = leaf2.compute_leaf_hash();
    
    // Verify hashes are different
    assert_ne!(hash1, hash2);
    
    // Verify hash for the same leaf is consistent
    let hash1_again = leaf1.compute_leaf_hash();
    assert_eq!(hash1, hash1_again);
}

#[test]
fn test_merkle_tree_creation() {
    // Create a new Merkle tree
    let mut tree = TaprootMerkleTree::new();
    
    // Verify it's empty
    assert!(tree.leaves.is_empty());
    
    // Add a leaf
    let script = vec![0x51]; // OP_1
    let leaf = TaprootLeaf::new(LeafVersion::Default, script);
    tree.add_leaf(0, leaf);
    
    // Verify it has one leaf
    assert_eq!(tree.leaves.len(), 1);
}

#[test]
fn test_merkle_root_single_leaf() {
    // Create a new Merkle tree with a single leaf
    let mut tree = TaprootMerkleTree::new();
    let script = vec![0x51]; // OP_1
    let leaf = TaprootLeaf::new(LeafVersion::Default, script);
    tree.add_leaf(0, leaf.clone());
    
    // Compute root hash
    let root_hash = tree.root_hash();
    
    // Verify root hash equals leaf hash for a single leaf
    let leaf_hash = leaf.compute_leaf_hash();
    assert_eq!(root_hash, leaf_hash);
}

#[test]
fn test_merkle_root_multiple_leaves() {
    // Create a new Merkle tree with multiple leaves
    let mut tree = TaprootMerkleTree::new();
    
    // Add two leaves
    let script1 = vec![0x51]; // OP_1
    let script2 = vec![0x52]; // OP_2
    let leaf1 = TaprootLeaf::new(LeafVersion::Default, script1);
    let leaf2 = TaprootLeaf::new(LeafVersion::Default, script2);
    
    tree.add_leaf(0, leaf1);
    tree.add_leaf(1, leaf2);
    
    // Compute root hash
    let root_hash = tree.root_hash();
    
    // Verify root hash is not all zeros
    assert!(!root_hash.iter().all(|&b| b == 0));
    
    // Root hash should be stable for multiple calls
    let root_hash2 = tree.root_hash();
    assert_eq!(root_hash, root_hash2);
}

#[test]
fn test_merkle_proof() {
    // Create a new Merkle tree with multiple leaves
    let mut tree = TaprootMerkleTree::new();
    
    // Add four leaves
    for i in 0..4 {
        let script = vec![(0x51 + i) as u8]; // OP_1, OP_2, OP_3, OP_4
        let leaf = TaprootLeaf::new(LeafVersion::Default, script);
        tree.add_leaf(i, leaf);
    }
    
    // Compute root hash to build the tree
    let _root_hash = tree.root_hash();
    
    // Get proof for leaf 0
    let proof0 = tree.get_proof(0);
    
    // Should have log2(4) = 2 proof elements for a balanced tree
    assert_eq!(proof0.len(), 2);
    
    // Get proof for leaf 3
    let proof3 = tree.get_proof(3);
    
    // Should also have 2 proof elements
    assert_eq!(proof3.len(), 2);
    
    // Get proof for non-existent leaf
    let empty_proof = tree.get_proof(10);
    
    // Should be empty
    assert!(empty_proof.is_empty());
}

#[test]
fn test_taproot_output_creation() {
    // Create a Taproot implementation
    let taproot = Bip341Taproot::new();
    
    // Create an internal key
    let internal_key = create_test_key(42);
    
    // Create a Merkle tree with a single leaf
    let mut tree = TaprootMerkleTree::new();
    let script = vec![0x51]; // OP_1
    let leaf = TaprootLeaf::new(LeafVersion::Default, script);
    tree.add_leaf(0, leaf);
    
    // Get the Merkle root
    let merkle_root = Some(tree.root_hash());
    
    // Create a Taproot output
    let output = taproot.create_taproot_output(internal_key, merkle_root).unwrap();
    
    // Verify output contains the correct internal key and Merkle root
    assert_eq!(output.internal_key.to_bytes(), internal_key.to_bytes());
    assert_eq!(output.merkle_root, merkle_root);
    
    // Output key should be different from internal key
    assert_ne!(output.output_key.to_bytes(), internal_key.to_bytes());
}

#[test]
fn test_taproot_output_creation_no_scripts() {
    // Create a Taproot implementation
    let taproot = Bip341Taproot::new();
    
    // Create an internal key
    let internal_key = create_test_key(42);
    
    // Create a Taproot output with no script tree
    let output = taproot.create_taproot_output(internal_key, None).unwrap();
    
    // Verify output contains the correct internal key and no Merkle root
    assert_eq!(output.internal_key.to_bytes(), internal_key.to_bytes());
    assert_eq!(output.merkle_root, None);
    
    // Output key should still be different from internal key (key-only tweak)
    assert_ne!(output.output_key.to_bytes(), internal_key.to_bytes());
}

#[test]
fn test_silent_leaf_creation() {
    // Create a Taproot implementation
    let taproot = Bip341Taproot::new();
    
    // Create a silent leaf
    let silent_leaf = taproot.create_silent_leaf();
    
    // Verify the leaf has the default version
    assert_eq!(silent_leaf.version, LeafVersion::Default);
    
    // Compute the hash of the silent leaf
    let silent_leaf_hash = taproot.silent_leaf_hash();
    
    // Hash should be consistent
    assert_eq!(silent_leaf_hash, silent_leaf.compute_leaf_hash());
}

#[test]
fn test_taproot_tweak() {
    // Create a Taproot implementation
    let taproot = Bip341Taproot::new();
    
    // Create an internal key
    let internal_key = create_test_key(42);
    
    // Create a Merkle tree
    let mut tree = TaprootMerkleTree::new();
    let script = vec![0x51]; // OP_1
    let leaf = TaprootLeaf::new(LeafVersion::Default, script);
    tree.add_leaf(0, leaf);
    
    // Get Merkle root
    let merkle_root = Some(tree.root_hash());
    
    // Compute tweak
    let tweak = taproot.compute_taproot_tweak(&internal_key, merkle_root);
    
    // Tweak should be non-zero
    assert!(!tweak.iter().all(|&b| b == 0));
    
    // Tweak should be deterministic
    let tweak2 = taproot.compute_taproot_tweak(&internal_key, merkle_root);
    assert_eq!(tweak, tweak2);
    
    // Tweak with no script should be different
    let tweak_no_script = taproot.compute_taproot_tweak(&internal_key, None);
    assert_ne!(tweak, tweak_no_script);
}

// This is a simplified integration test for key path spending
// In a real implementation, this would use actual Bitcoin transactions
#[test]
fn test_key_path_spending() {
    // Create a Taproot implementation
    let taproot = Bip341Taproot::new();
    let schnorr = Bip340Schnorr::new();
    
    // Create an internal key
    let key_pair = schnorr.generate_key_pair();
    let internal_key = key_pair.public_key;
    
    // Create a Taproot output
    let output = taproot.create_taproot_output(internal_key, None).unwrap();
    
    // Create a message to sign (in real world, this would be a transaction)
    let message = b"Test message for key path spending";
    
    // Sign the message using the internal key
    let signature = schnorr.sign(&key_pair, message).unwrap();
    
    // Create a key path spend
    let spend = TaprootSpend::KeyPath {
        output_key: output.output_key,
        signature,
    };
    
    // Verify the spend
    let result = taproot.verify_spend(&spend, message);
    
    // Should succeed for a legitimate spend
    assert!(result.is_ok());
    
    // For key path spending, we expect the result to be true
    assert!(result.unwrap());
}

// Integration test for script path spending would be more complex
// and requires actual transaction construction, which is beyond the scope
// of this unit test file. In a real implementation, you would have
// tests that construct and verify actual Bitcoin transactions.

#[test]
fn test_version_conversions() {
    // Test default version
    let default_version = LeafVersion::Default;
    let default_byte: u8 = default_version.into();
    assert_eq!(default_byte, 0xc0);
    
    // Test conversion from byte to version
    let version_from_byte = LeafVersion::from(default_byte);
    assert_eq!(version_from_byte, LeafVersion::Default);
    
    // Test future version
    let future_byte = 0xc1;
    let future_version = LeafVersion::Future(future_byte);
    let byte_from_future: u8 = future_version.into();
    assert_eq!(byte_from_future, 0xc1);
    
    // Test conversion from byte to future version
    let version_from_future_byte = LeafVersion::from(future_byte);
    match version_from_future_byte {
        LeafVersion::Future(v) => assert_eq!(v, future_byte),
        _ => panic!("Expected Future variant"),
    }
} 