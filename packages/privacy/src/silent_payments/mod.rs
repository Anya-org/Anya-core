//! # Silent Payments (BIP-353) Implementation
//!
//! This module implements the Silent Payments protocol as defined in BIP-353,
//! allowing for enhanced privacy in Bitcoin transactions.
//!
//! [AIR-3][AIS-3][AIP-3][BPC-3]
//!
//! Silent Payments enable receivers to publish a static payment code while
//! ensuring that all on-chain payments are unlinkable by any third-party observer,
//! even if the observer knows all the receivers' addresses.

mod scanner;
mod sender;
mod address;
mod keys;
mod util;
mod storage;
mod hsm;

#[cfg(test)]
mod tests;

// Re-export public types
pub use address::SilentPaymentAddress;
pub use keys::KeyManager;
pub use scanner::{SilentPaymentScanner, SilentPaymentTelemetry};
pub use sender::SilentPaymentSender;
pub use hsm::{SilentPaymentHsm, HsmConfig, HsmProviderType};
pub use storage::SilentPaymentStorage;

use bitcoin::secp256k1::{Secp256k1, SecretKey, XOnlyPublicKey};
use bitcoin::hashes::{sha256, Hash, HashEngine};
use bitcoin::{OutPoint, Transaction, TxOut};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use crate::Result;

/// The prefix for Silent Payment addresses on mainnet
pub const MAINNET_PREFIX: &str = "sp";

/// The prefix for Silent Payment addresses on testnet
pub const TESTNET_PREFIX: &str = "tsp";

/// The prefix for Silent Payment addresses on regtest
pub const REGTEST_PREFIX: &str = "rsp";

/// Status information about a Silent Payment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SilentPaymentInfo {
    /// The Bitcoin transaction ID that contains this payment
    pub txid: bitcoin::Txid,
    
    /// The output index within the transaction
    pub vout: u32,
    
    /// The amount received in satoshis
    pub amount: u64,
    
    /// The Bitcoin block height when this payment was received
    pub block_height: Option<u32>,
    
    /// The spending status of this output
    pub spent: bool,
}

/// Derive a shared secret using BIP-353 algorithm
///
/// This is the core cryptographic operation for Silent Payments,
/// producing a shared secret between the sender and receiver.
pub fn derive_shared_secret(
    scan_pubkey: &XOnlyPublicKey,
    spend_pubkey: &XOnlyPublicKey,
    sender_secret: &SecretKey,
    outpoint: &OutPoint,
) -> Result<[u8; 32]> {
    let secp = Secp256k1::new();
    
    // Derive the shared point using ECDH
    let shared_point = bitcoin::secp256k1::PublicKey::from_x_only_public_key(*scan_pubkey, 
        bitcoin::secp256k1::Parity::Even) // We assume even parity for simplicity
        .mul_tweak(&secp, &sender_secret.into())
        .map_err(|e| crate::Error::Crypto(format!("ECDH computation failed: {}", e)))?;
    
    // Serialize the shared point
    let shared_point_bytes = shared_point.serialize();
    
    // Create SHA256(shared_point || outpoint || spend_pubkey)
    let mut engine = sha256::Hash::engine();
    engine.input(&shared_point_bytes);
    engine.input(&outpoint.txid[..]);
    engine.input(&outpoint.vout.to_le_bytes());
    engine.input(&spend_pubkey.serialize());
    
    // Finalize the hash
    let result = sha256::Hash::from_engine(engine);
    
    Ok(result.into_inner())
}

/// Serialize an outpoint to bytes for use in cryptographic operations
fn serialize_outpoint(outpoint: &OutPoint) -> Vec<u8> {
    let mut result = Vec::with_capacity(36);
    result.extend_from_slice(&outpoint.txid[..]);
    result.extend_from_slice(&outpoint.vout.to_le_bytes());
    result
}

/// Extract a public key from a P2TR script
pub fn extract_p2tr_pubkey(script: &bitcoin::Script) -> Option<XOnlyPublicKey> {
    // Check if it's a P2TR script pattern
    if script.len() == 34 && script[0] == 0x51 && script[1] == 0x20 {
        let pubkey_bytes = &script.as_bytes()[2..34];
        XOnlyPublicKey::from_slice(pubkey_bytes).ok()
    } else {
        None
    }
} 