//! Silent Payment Sender Implementation
//!
//! Implements the sender side of BIP-353 Silent Payments,
//! allowing for creating transactions with outputs that can be
//! detected by the receiver's scanner.

use bitcoin::secp256k1::{Secp256k1, SecretKey, XOnlyPublicKey};
use bitcoin::hashes::{sha256, Hash, HashEngine};
use bitcoin::{OutPoint, Transaction, TxOut, NetworkUnchecked, Amount};
use bitcoin::address::{NetworkChecked, NetworkUncheckedAddress};
use bitcoin::blockdata::script::{Builder, Script, ScriptBuf};
use bitcoin::opcodes::all::OP_1;
use crate::Result;
use crate::Error;
use super::{SilentPaymentAddress, derive_shared_secret};

/// Sender for creating Silent Payments according to BIP-353
///
/// Implements the sender side of the Silent Payments protocol,
/// generating Bitcoin outputs that can only be detected by the intended
/// receiver.
#[derive(Debug)]
pub struct SilentPaymentSender {
    /// The secp256k1 context for cryptographic operations
    secp: Secp256k1<bitcoin::secp256k1::All>,
}

impl SilentPaymentSender {
    /// Create a new Silent Payment sender
    pub fn new() -> Self {
        Self {
            secp: Secp256k1::new(),
        }
    }
    
    /// Create a transaction output to a Silent Payment address
    ///
    /// This function creates a P2TR output that can be detected
    /// by the receiver using their scan key.
    pub fn create_payment_output(
        &self,
        address: &SilentPaymentAddress,
        sender_secret: &SecretKey,
        input_outpoint: &OutPoint,
        amount: Amount,
    ) -> Result<TxOut> {
        // Get the receiver's public keys
        let receiver_scan_pubkey = &address.scan_pubkey;
        let receiver_spend_pubkey = &address.spend_pubkey;
        
        // Derive shared secret according to BIP-353
        let tweak = derive_shared_secret(
            receiver_scan_pubkey,
            receiver_spend_pubkey,
            sender_secret,
            input_outpoint,
        )?;
        
        // Apply the tweak to the receiver's spend key
        let output_key = receiver_spend_pubkey
            .add_tweak(&self.secp, &tweak.into())
            .map_err(|e| Error::Crypto(format!("Failed to tweak pubkey: {}", e)))?;
        
        // Create P2TR output script
        let output_script = ScriptBuf::new_p2tr(
            &self.secp,
            output_key,
            None, // No script path
        );
        
        Ok(TxOut {
            value: amount.to_sat(),
            script_pubkey: output_script,
        })
    }
    
    /// Create multiple outputs to a Silent Payment address
    ///
    /// Creates multiple outputs to the same Silent Payment address,
    /// ideal for batching multiple payments while maintaining privacy.
    pub fn create_multiple_outputs(
        &self,
        address: &SilentPaymentAddress,
        sender_secret: &SecretKey,
        input_outpoints: &[OutPoint],
        amounts: &[Amount],
    ) -> Result<Vec<TxOut>> {
        if input_outpoints.len() != amounts.len() {
            return Err(Error::ScanningError(
                "Number of inputs must match number of outputs".into()
            ));
        }
        
        let mut outputs = Vec::with_capacity(amounts.len());
        
        for (outpoint, amount) in input_outpoints.iter().zip(amounts.iter()) {
            let output = self.create_payment_output(
                address,
                sender_secret,
                outpoint,
                *amount,
            )?;
            
            outputs.push(output);
        }
        
        Ok(outputs)
    }
}

impl Default for SilentPaymentSender {
    fn default() -> Self {
        Self::new()
    }
} 