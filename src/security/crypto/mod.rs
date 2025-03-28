#![feature(edition2021)]
//! Cryptographic Utilities Module
//! [AIR-1][AIS-1][BPC-1][AIT-1][RES-1]
//!
//! This module provides cryptographic utilities for the Bitcoin security framework.

pub mod random;

// Re-export random number generator functions
pub use random::{random_bytes, random_u64, random_u32, random_usize, 
                 random_f64, random_in_range, random_bool, shuffle, reseed};

// Module for symmetric encryption (AES, ChaCha20)
pub mod symmetric;

// Module for asymmetric encryption (RSA, ECC)
pub mod asymmetric;

// Module for hashing functions (SHA256, SHA512, RIPEMD160)
pub mod hash;

// Module for digital signatures (ECDSA, Schnorr)
pub mod signature;

// Module for key derivation functions (PBKDF2, Argon2, scrypt)
pub mod kdf;

/// Helper function to generate a secure cryptographic key of specified length
pub fn generate_key(length_bytes: usize) -> Vec<u8> {
    random_bytes(length_bytes)
}

/// Helper function to generate a secure initialization vector
pub fn generate_iv(length_bytes: usize) -> Vec<u8> {
    random_bytes(length_bytes)
}

/// Helper function to generate a secure nonce
pub fn generate_nonce(length_bytes: usize) -> Vec<u8> {
    random_bytes(length_bytes)
} 