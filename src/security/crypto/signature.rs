//! Digital Signature Module
//!
//! This module provides cryptographic signature functionality for Bitcoin security.

use std::error::Error;

/// Supported signature algorithms
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SignatureAlgorithm {
    /// ECDSA with secp256k1 curve (Bitcoin standard)
    EcdsaSecp256k1,
    /// Schnorr signature with secp256k1 curve (Bitcoin Taproot)
    SchnorrSecp256k1,
    /// EdDSA with Curve25519 (Ed25519)
    Ed25519,
}

/// A digital signature
#[derive(Debug, Clone)]
pub struct Signature {
    /// Raw signature bytes
    pub bytes: Vec<u8>,
    /// The algorithm used to create this signature
    pub algorithm: SignatureAlgorithm,
}

/// Sign a message with the provided private key
pub fn sign(
    message: &[u8],
    private_key: &[u8],
    algorithm: SignatureAlgorithm,
) -> Result<Signature, Box<dyn Error>> {
    use secp256k1::hashes::{sha256, Hash};
    use secp256k1::{Keypair, Message, Secp256k1, SecretKey};

    log::debug!(
        "Signing message of length {} with key length {}",
        message.len(),
        private_key.len()
    );

    match algorithm {
        SignatureAlgorithm::EcdsaSecp256k1 => {
            let secp = Secp256k1::new();
            let secret_key = SecretKey::from_slice(private_key)
                .map_err(|e| format!("Invalid private key: {}", e))?;

            // Hash the message using SHA256
            let msg_hash = sha256::Hash::hash(message);
            let msg = Message::from_digest_slice(&msg_hash[..])
                .map_err(|e| format!("Invalid message hash: {}", e))?;

            // Sign with ECDSA
            let signature = secp.sign_ecdsa(&msg, &secret_key);

            Ok(Signature {
                bytes: signature.serialize_compact().to_vec(),
                algorithm,
            })
        }
        SignatureAlgorithm::SchnorrSecp256k1 => {
            let secp = Secp256k1::new();
            let secret_key = SecretKey::from_slice(private_key)
                .map_err(|e| format!("Invalid private key: {}", e))?;

            let key_pair = Keypair::from_secret_key(&secp, &secret_key);

            // Hash the message using SHA256
            let msg_hash = sha256::Hash::hash(message);
            let msg = Message::from_digest_slice(&msg_hash[..])
                .map_err(|e| format!("Invalid message hash: {}", e))?;

            // Sign with Schnorr
            let signature = secp.sign_schnorr(&msg, &key_pair);

            Ok(Signature {
                bytes: signature.as_ref().to_vec(),
                algorithm,
            })
        }
        SignatureAlgorithm::Ed25519 => {
            // For now, fallback to ECDSA secp256k1
            // TODO: Add proper Ed25519 support
            sign(message, private_key, SignatureAlgorithm::EcdsaSecp256k1)
        }
    }
}

/// Verify a signature against a message and public key
pub fn verify(
    message: &[u8],
    signature: &Signature,
    public_key: &[u8],
) -> Result<bool, Box<dyn Error>> {
    use secp256k1::hashes::{sha256, Hash};
    use secp256k1::{ecdsa, Message, PublicKey, Secp256k1, XOnlyPublicKey};

    match signature.algorithm {
        SignatureAlgorithm::EcdsaSecp256k1 => {
            let secp = Secp256k1::verification_only();

            let public_key = PublicKey::from_slice(public_key)
                .map_err(|e| format!("Invalid public key: {}", e))?;

            let signature = ecdsa::Signature::from_compact(&signature.bytes)
                .map_err(|e| format!("Invalid signature: {}", e))?;

            // Hash the message using SHA256
            let msg_hash = sha256::Hash::hash(message);
            let msg = Message::from_digest_slice(&msg_hash[..])
                .map_err(|e| format!("Invalid message hash: {}", e))?;

            Ok(secp.verify_ecdsa(&msg, &signature, &public_key).is_ok())
        }
        SignatureAlgorithm::SchnorrSecp256k1 => {
            let secp = Secp256k1::verification_only();

            // For Schnorr, public key should be x-only (32 bytes)
            let xonly_pk = if public_key.len() == 32 {
                XOnlyPublicKey::from_slice(public_key)
                    .map_err(|e| format!("Invalid x-only public key: {}", e))?
            } else {
                // If it's a full public key, extract x-only part
                let full_pk = PublicKey::from_slice(public_key)
                    .map_err(|e| format!("Invalid public key: {}", e))?;
                full_pk.x_only_public_key().0
            };

            let signature = secp256k1::schnorr::Signature::from_slice(&signature.bytes)
                .map_err(|e| format!("Invalid Schnorr signature: {}", e))?;

            // Hash the message using SHA256
            let msg_hash = sha256::Hash::hash(message);
            let msg = Message::from_digest_slice(&msg_hash[..])
                .map_err(|e| format!("Invalid message hash: {}", e))?;

            Ok(secp.verify_schnorr(&signature, &msg, &xonly_pk).is_ok())
        }
        SignatureAlgorithm::Ed25519 => {
            // For now, fallback to ECDSA verification
            // TODO: Add proper Ed25519 support
            let mut ecdsa_sig = signature.clone();
            ecdsa_sig.algorithm = SignatureAlgorithm::EcdsaSecp256k1;
            verify(message, &ecdsa_sig, public_key)
        }
    }
}

/// Sign a message with ECDSA using the secp256k1 curve (Bitcoin standard)
pub fn ecdsa_sign_secp256k1(
    message: &[u8],
    private_key: &[u8],
) -> Result<Signature, Box<dyn Error>> {
    sign(message, private_key, SignatureAlgorithm::EcdsaSecp256k1)
}

/// Verify an ECDSA signature using the secp256k1 curve (Bitcoin standard)
pub fn ecdsa_verify_secp256k1(
    message: &[u8],
    signature: &[u8],
    public_key: &[u8],
) -> Result<bool, Box<dyn Error>> {
    let sig = Signature {
        bytes: signature.to_vec(),
        algorithm: SignatureAlgorithm::EcdsaSecp256k1,
    };
    verify(message, &sig, public_key)
}

/// Sign a message with Schnorr using the secp256k1 curve (Bitcoin Taproot)
pub fn schnorr_sign_secp256k1(
    message: &[u8],
    private_key: &[u8],
) -> Result<Signature, Box<dyn Error>> {
    sign(message, private_key, SignatureAlgorithm::SchnorrSecp256k1)
}

/// Verify a Schnorr signature using the secp256k1 curve (Bitcoin Taproot)
pub fn schnorr_verify_secp256k1(
    message: &[u8],
    signature: &[u8],
    public_key: &[u8],
) -> Result<bool, Box<dyn Error>> {
    let sig = Signature {
        bytes: signature.to_vec(),
        algorithm: SignatureAlgorithm::SchnorrSecp256k1,
    };
    verify(message, &sig, public_key)
}
