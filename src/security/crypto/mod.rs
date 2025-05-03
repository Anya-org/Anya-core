//! Cryptographic Utilities Module
//! 
//! [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
//!
//! This module provides cryptographic utilities for the Bitcoin security framework,
//! implementing the Bitcoin Development Framework v2.5 requirements.

use std::error::Error as StdError;

// Random number generation
pub mod random;

// Re-export random number generator functions
pub use random::{random_bytes, random_u64, random_u32, random_usize, 
                 random_f64, random_in_range, random_bool, shuffle, reseed};

// Module stubs for planned modules (implementation pending)
/// Symmetric encryption module (AES, ChaCha20)
pub mod symmetric;

/// Asymmetric encryption module (RSA, ECC)
pub mod asymmetric {
    /// Placeholder for asymmetric encryption module
    /// To be implemented with RSA, ECC support
    pub fn placeholder() {
        // Placeholder function until module is implemented
    }
}

/// Hashing functions module (SHA256, SHA512, RIPEMD160)
pub mod hash {
    /// Placeholder for hash module
    /// To be implemented with SHA256, SHA512, RIPEMD160
    pub fn placeholder() {
        // Placeholder function until module is implemented
    }
}

/// Digital signatures module (ECDSA, Schnorr)
pub mod signature {
    /// Placeholder for signature module
    /// To be implemented with ECDSA, Schnorr
    pub fn placeholder() {
        // Placeholder function until module is implemented
    }
}

/// Key derivation functions module (PBKDF2, Argon2, scrypt)
pub mod kdf {
    /// Placeholder for KDF module
    /// To be implemented with PBKDF2, Argon2, scrypt
    pub fn placeholder() {
        // Placeholder function until module is implemented
    }
}

// Bitcoin-specific cryptographic modules (BIP-340/341/342)
pub mod schnorr;
pub mod sha256;

// Re-export commonly used types and functions
pub use schnorr::verify_signature;
pub use schnorr::sign_message;
pub use sha256::hash;
pub use sha256::double_hash;

// Constants for Bitcoin cryptography
pub const SCHNORR_SIGNATURE_SIZE: usize = 64;
pub const TAPROOT_PUBLIC_KEY_SIZE: usize = 32;

/// Helper function to generate a secure cryptographic key of specified length
/// 
/// # Arguments
/// * `length_bytes` - Length of the key in bytes
/// 
/// # Returns
/// A vector containing the generated key
pub fn generate_key(length_bytes: usize) -> Vec<u8> {
    random_bytes(length_bytes)
}

/// Helper function to generate a secure initialization vector
/// 
/// # Arguments
/// * `length_bytes` - Length of the IV in bytes
/// 
/// # Returns
/// A vector containing the generated IV
pub fn generate_iv(length_bytes: usize) -> Vec<u8> {
    random_bytes(length_bytes)
}

/// Helper function to generate a secure nonce
/// 
/// # Arguments
/// * `length_bytes` - Length of the nonce in bytes
/// 
/// # Returns
/// A vector containing the generated nonce
pub fn generate_nonce(length_bytes: usize) -> Vec<u8> {
    random_bytes(length_bytes)
} 
