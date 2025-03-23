// Protocol Test Suite v2.5
// [AIT-3][BPC-3][RES-3]

use anya_core::protocols::*;
use bitcoin::{consensus, secp256k1, Transaction};
use lightning::offers::offer::Offer;

#[tokio::test]
async fn test_full_protocol_stack() {
    let config = ProtocolConfig::test_default();
    let protocol = ProtocolManager::new(config).unwrap();
    
    // Test BIP-341/342 (Taproot)
    let taproot_tx = protocol.taproot_engine.build_taproot_transaction(
        vec![test_input()],
        vec![test_output()],
        FeeRate::default()
    ).unwrap();
    assert!(protocol.segwit_verifier.verify_taproot(&taproot_tx).is_ok());

    // Test BOLT 12 Offers
    let offer = protocol.ldk_node.create_offer(OfferRequest {
        amount_msat: 100_000,
        description: "Test".into(),
        expiry_secs: 3600,
    }).unwrap();
    let invoice = protocol.ldk_node.request_invoice_from_offer(&offer).unwrap();
    let payment_hash = protocol.ldk_node.send_payment_for_offer(&offer).unwrap();
    assert!(!payment_hash.is_empty());

    // Test Cross-chain SPV
    let proof = test_spv_proof();
    assert!(protocol.spv_verifier.verify_cross_chain_swap(proof, "BTC").unwrap());
}

#[test]
fn test_ai_label_compliance() {
    let verifier = LabelValidator::new();
    
    let components = vec![
        Component::new("TaprootEngine", ComponentCategory::Consensus),
        Component::new("LdkNode", ComponentCategory::Network),
        Component::new("SpvVerifier", ComponentCategory::CrossChain),
    ];

    for component in components {
        verifier.validate_component(&component).unwrap();
    }
}

#[tokio::test]
async fn test_lightning_security() {
    let node = setup_lightning_node().await;
    
    // Test key rotation
    node.rotate_node_keys().unwrap();
    
    // Validate watchtower coverage
    node.validate_watchtower_coverage().unwrap();
    
    // Test anti-jamming
    let channel_id = node.open_test_channel().await.unwrap();
    node.apply_anti_jamming(channel_id).unwrap();
}

// Helper functions
fn test_input() -> TaprootInput {
    TaprootInput {
        internal_key: secp256k1::PublicKey::new(),
        script: Script::new(),
        leaf_version: 0xc0,
    }
}

fn test_output() -> TxOut {
    TxOut {
        value: 10_000,
        script_pubkey: Script::new(),
    }
}

fn test_spv_proof() -> CrossChainProof {
    CrossChainProof {
        tx_hash: [0; 32],
        merkle_root: [0; 32],
        proof: vec![],
    }
} 