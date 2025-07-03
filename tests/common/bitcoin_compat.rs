//! Bitcoin compatibility module for tests
//!
//! This module provides compatibility shims and re-exports for Bitcoin-related
//! types and functionality used in tests.

use bitcoin::*;
use anya_core::bitcoin::{interface::*, validation::*};

// Re-export common Bitcoin types for tests
pub use bitcoin::{
    Transaction, TxIn, TxOut, Script, OutPoint, Amount, Block,
    secp256k1::{Secp256k1, SecretKey, PublicKey},
    key::{TweakedKeypair, TweakedPublicKey, UntweakedPublicKey, XOnlyPublicKey},
    taproot::{TapLeafHash, TapNodeHash, ControlBlock},
    hashes::{sha256, Hash, HashEngine},
};

// Re-export our custom types
pub use anya_core::bitcoin::{
    protocol::{BitcoinProtocol, BPCLevel as ProtocolLevel},
    validation::{TransactionValidator, MempoolBatchVerifier},
};

// Compatibility types for tests
#[derive(Debug, Clone)]
pub struct TaprootInput {
    pub prev_out: OutPoint,
    pub amount: Amount,
    pub script: ScriptBuf,
    pub internal_key: XOnlyPublicKey,
}

#[derive(Debug, Clone)]
pub struct CrossChainProof {
    pub chain_id: String,
    pub block_hash: String,
    pub merkle_proof: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct OfferRequest {
    pub amount: Amount,
    pub description: String,
}

// Mock functions for testing
pub fn create_test_transaction() -> Transaction {
    use bitcoin::consensus::deserialize;
    // A simple test transaction (coinbase)
    let hex = "01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff08044c86041b020602ffffffff0100f2052a010000004341041b0e8c2567c12536aa13357b79a073dc4444acb83c4ec7a0e2f99dd7457516c5817242da796924ca4e99947d087fedf9ce467cb9f7c6287078f801df276fdf84424ac00000000";
    let bytes = hex::decode(hex).unwrap();
    deserialize(&bytes).unwrap()
}

pub fn create_test_secp_context() -> Secp256k1<bitcoin::secp256k1::All> {
    Secp256k1::new()
}

pub fn create_test_keypair() -> (SecretKey, PublicKey) {
    let secp = create_test_secp_context();
    let secret_key = SecretKey::from_slice(&[1u8; 32]).unwrap();
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    (secret_key, public_key)
}

// Blockchain header compatibility
pub type BlockHeader = bitcoin::block::Header;

// Fee rate compatibility
pub use bitcoin::FeeRate;

// Script compatibility
pub use bitcoin::{ScriptBuf, Script as ScriptRef};
