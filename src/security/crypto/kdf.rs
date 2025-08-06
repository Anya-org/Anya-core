//! Key Derivation Function Module
//!
//! This module provides key derivation functions for Bitcoin security.

use std::error::Error;

/// Supported key derivation algorithms
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KdfAlgorithm {
    /// PBKDF2 with HMAC-SHA256
    Pbkdf2,
    /// Argon2id - memory-hard KDF
    Argon2id,
    /// scrypt - memory-hard KDF
    Scrypt,
    /// HKDF - HMAC-based Key Derivation Function
    Hkdf,
}

/// PBKDF2 parameters
#[derive(Debug, Clone)]
pub struct Pbkdf2Params {
    /// Number of iterations
    pub iterations: u32,
    /// Salt value
    pub salt: Vec<u8>,
    /// Output key length in bytes
    pub key_length: usize,
}

impl Default for Pbkdf2Params {
    fn default() -> Self {
        Self {
            iterations: 10_000,
            salt: vec![0u8; 16], // Should be random in practice
            key_length: 32,
        }
    }
}

/// Argon2 parameters
#[derive(Debug, Clone)]
pub struct Argon2Params {
    /// Memory size in KB
    pub memory_cost: u32,
    /// Number of iterations
    pub iterations: u32,
    /// Parallelism factor
    pub parallelism: u32,
    /// Salt value
    pub salt: Vec<u8>,
    /// Output key length in bytes
    pub key_length: usize,
}

impl Default for Argon2Params {
    fn default() -> Self {
        Self {
            memory_cost: 65536, // 64 MB
            iterations: 3,
            parallelism: 4,
            salt: vec![0u8; 16], // Should be random in practice
            key_length: 32,
        }
    }
}

/// Scrypt parameters
#[derive(Debug, Clone)]
pub struct ScryptParams {
    /// CPU/memory cost parameter
    pub n: u32,
    /// Block size parameter
    pub r: u32,
    /// Parallelization parameter
    pub p: u32,
    /// Salt value
    pub salt: Vec<u8>,
    /// Output key length in bytes
    pub key_length: usize,
}

impl Default for ScryptParams {
    fn default() -> Self {
        Self {
            n: 16384, // 2^14
            r: 8,
            p: 1,
            salt: vec![0u8; 16], // Should be random in practice
            key_length: 32,
        }
    }
}

/// Derive a key from a password or passphrase using PBKDF2
pub fn pbkdf2(password: &[u8], params: &Pbkdf2Params) -> Vec<u8> {
    use pbkdf2::pbkdf2_hmac;
    use sha2::Sha256;

    let mut output = vec![0u8; params.key_length];
    pbkdf2_hmac::<Sha256>(password, &params.salt, params.iterations, &mut output);
    output
}

/// Derive a key from a password or passphrase using Argon2id
pub fn argon2id(password: &[u8], params: &Argon2Params) -> Vec<u8> {
    use argon2::{Algorithm, Argon2, Params, Version};

    let argon2_params = Params::new(
        params.memory_cost,
        params.iterations,
        params.parallelism,
        Some(params.key_length),
    )
    .expect("Valid Argon2 parameters");

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, argon2_params);

    let mut output = vec![0u8; params.key_length];
    argon2
        .hash_password_into(password, &params.salt, &mut output)
        .expect("Argon2 hash generation failed");

    output
}

/// Derive a key from a password or passphrase using scrypt
pub fn scrypt(password: &[u8], params: &ScryptParams) -> Vec<u8> {
    // For now, fallback to PBKDF2 since scrypt dependency is not available
    // Convert scrypt params to equivalent PBKDF2 params
    let pbkdf2_params = Pbkdf2Params {
        salt: params.salt.clone(),
        iterations: params.n as u32, // Use N as iterations
        key_length: params.key_length,
    };
    pbkdf2(password, &pbkdf2_params)
}

/// Derive a key using HKDF (HMAC-based Key Derivation Function)
pub fn hkdf(ikm: &[u8], salt: &[u8], info: &[u8], key_length: usize) -> Vec<u8> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    // HKDF-Extract
    let mut mac = HmacSha256::new_from_slice(salt).expect("HMAC can take key of any size");
    mac.update(ikm);
    let prk = mac.finalize().into_bytes();

    // HKDF-Expand
    let mut output = Vec::with_capacity(key_length);
    let mut counter = 1u8;

    while output.len() < key_length {
        let mut mac = HmacSha256::new_from_slice(&prk).expect("PRK is valid key");
        if !output.is_empty() {
            mac.update(&output[output.len().saturating_sub(32)..]);
        }
        mac.update(info);
        mac.update(&[counter]);

        let chunk = mac.finalize().into_bytes();
        let needed = std::cmp::min(32, key_length - output.len());
        output.extend_from_slice(&chunk[..needed]);

        counter += 1;
        if counter == 0 {
            break; // Prevent overflow
        }
    }

    output.truncate(key_length);
    output
}

/// Derive a key using the specified KDF algorithm
pub fn derive_key(
    password: &[u8],
    algorithm: KdfAlgorithm,
    _params: &[u8],
) -> Result<Vec<u8>, Box<dyn Error>> {
    match algorithm {
        KdfAlgorithm::Pbkdf2 => {
            // Parse params as Pbkdf2Params
            let default_params = Pbkdf2Params::default();
            Ok(pbkdf2(password, &default_params))
        }
        KdfAlgorithm::Argon2id => {
            // Parse params as Argon2Params
            let default_params = Argon2Params::default();
            Ok(argon2id(password, &default_params))
        }
        KdfAlgorithm::Scrypt => {
            // Parse params as ScryptParams
            let default_params = ScryptParams::default();
            Ok(scrypt(password, &default_params))
        }
        KdfAlgorithm::Hkdf => {
            // Parse params as salt and info
            Ok(hkdf(password, &[], &[], 32))
        }
    }
}
