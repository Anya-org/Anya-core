// Symmetric Encryption Module
// [AIR-2][AIS-2][BPC-2][AIT-2][RES-2]
//
// This module provides symmetric encryption utilities using modern algorithms.
use aes_gcm::{aead::Aead as AesAead, Aes256Gcm};
/// Supports AES-256 (GCM, CBC, CTR modes) and ChaCha20-Poly1305.
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    ChaCha20Poly1305, Key,
};
use thiserror::Error;

use crate::security::crypto::random;

/// Symmetric encryption error type
#[derive(Debug, Error)]
pub enum SymmetricError {
    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("Invalid key error: {0}")]
    InvalidKeyError(String),

    #[error("Invalid data error: {0}")]
    InvalidDataError(String),

    #[error("Invalid nonce error: {0}")]
    InvalidNonceError(String),

    #[error("Other error: {0}")]
    OtherError(String),
}

/// Symmetric encryption algorithm type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymmetricAlgorithm {
    /// AES-256 in GCM mode
    Aes256Gcm,
    /// AES-256 in CBC mode
    Aes256Cbc,
    /// AES-256 in CTR mode
    Aes256Ctr,
    /// ChaCha20-Poly1305
    ChaCha20Poly1305,
}

/// Symmetric encryption/decryption handler
#[derive(Debug)]
pub struct SymmetricCrypto {
    /// Algorithm to use
    algorithm: SymmetricAlgorithm,
}

impl SymmetricCrypto {
    /// Create a new symmetric crypto handler
    pub fn new(algorithm: SymmetricAlgorithm) -> Self {
        Self { algorithm }
    }

    /// Generate a random key suitable for the selected algorithm
    pub fn generate_key(&self) -> Vec<u8> {
        match self.algorithm {
            SymmetricAlgorithm::Aes256Gcm
            | SymmetricAlgorithm::Aes256Cbc
            | SymmetricAlgorithm::Aes256Ctr => {
                // For AES-256, we need a 32-byte key
                random::random_bytes(32)
            }
            SymmetricAlgorithm::ChaCha20Poly1305 => {
                // For ChaCha20-Poly1305, we need a 32-byte key
                random::random_bytes(32)
            }
        }
    }

    /// Generate a random nonce/IV suitable for the selected algorithm
    pub fn generate_nonce(&self) -> Vec<u8> {
        match self.algorithm {
            SymmetricAlgorithm::Aes256Gcm => {
                // For AES-GCM, standard nonce size is 12 bytes
                random::random_bytes(12)
            }
            SymmetricAlgorithm::Aes256Cbc => {
                // For AES-CBC, IV size is 16 bytes (block size)
                random::random_bytes(16)
            }
            SymmetricAlgorithm::Aes256Ctr => {
                // For AES-CTR, nonce size is 16 bytes (block size)
                random::random_bytes(16)
            }
            SymmetricAlgorithm::ChaCha20Poly1305 => {
                // For ChaCha20-Poly1305, nonce size is 12 bytes
                random::random_bytes(12)
            }
        }
    }

    /// Encrypt data using the selected algorithm
    pub fn encrypt(
        &self,
        key: &[u8],
        nonce: &[u8],
        plaintext: &[u8],
        aad: Option<&[u8]>,
    ) -> Result<Vec<u8>, SymmetricError> {
        match self.algorithm {
            SymmetricAlgorithm::Aes256Gcm => self.encrypt_aes_gcm(key, nonce, plaintext, aad),
            SymmetricAlgorithm::ChaCha20Poly1305 => {
                self.encrypt_chacha20_poly1305(key, nonce, plaintext, aad)
            }
            _ => Err(SymmetricError::EncryptionError(format!(
                "Algorithm {:?} not yet implemented",
                self.algorithm
            ))),
        }
    }

    /// Decrypt data using the selected algorithm
    pub fn decrypt(
        &self,
        key: &[u8],
        nonce: &[u8],
        ciphertext: &[u8],
        aad: Option<&[u8]>,
    ) -> Result<Vec<u8>, SymmetricError> {
        match self.algorithm {
            SymmetricAlgorithm::Aes256Gcm => self.decrypt_aes_gcm(key, nonce, ciphertext, aad),
            SymmetricAlgorithm::ChaCha20Poly1305 => {
                self.decrypt_chacha20_poly1305(key, nonce, ciphertext, aad)
            }
            _ => Err(SymmetricError::DecryptionError(format!(
                "Algorithm {:?} not yet implemented",
                self.algorithm
            ))),
        }
    }

    /// Encrypt data using AES-GCM
    fn encrypt_aes_gcm(
        &self,
        key: &[u8],
        nonce: &[u8],
        plaintext: &[u8],
        aad: Option<&[u8]>,
    ) -> Result<Vec<u8>, SymmetricError> {
        // Validate key and nonce
        if key.len() != 32 {
            return Err(SymmetricError::InvalidKeyError(format!(
                "AES-256-GCM requires a 32-byte key, got {}",
                key.len()
            )));
        }

        if nonce.len() != 12 {
            return Err(SymmetricError::InvalidNonceError(format!(
                "AES-256-GCM requires a 12-byte nonce, got {}",
                nonce.len()
            )));
        }

        // Create cipher
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| SymmetricError::EncryptionError(e.to_string()))?;

        // Create nonce
        let nonce = aes_gcm::Nonce::from_slice(nonce);

        // Encrypt
        let payload = if let Some(aad_data) = aad {
            aes_gcm::aead::Payload {
                msg: plaintext,
                aad: aad_data,
            }
        } else {
            aes_gcm::aead::Payload {
                msg: plaintext,
                aad: &[],
            }
        };

        cipher
            .encrypt(nonce, payload)
            .map_err(|e| SymmetricError::EncryptionError(e.to_string()))
    }

    /// Decrypt data using AES-GCM
    fn decrypt_aes_gcm(
        &self,
        key: &[u8],
        nonce: &[u8],
        ciphertext: &[u8],
        aad: Option<&[u8]>,
    ) -> Result<Vec<u8>, SymmetricError> {
        // Validate key and nonce
        if key.len() != 32 {
            return Err(SymmetricError::InvalidKeyError(format!(
                "AES-256-GCM requires a 32-byte key, got {}",
                key.len()
            )));
        }

        if nonce.len() != 12 {
            return Err(SymmetricError::InvalidNonceError(format!(
                "AES-256-GCM requires a 12-byte nonce, got {}",
                nonce.len()
            )));
        }

        // Create cipher
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| SymmetricError::DecryptionError(e.to_string()))?;

        // Create nonce
        let nonce = aes_gcm::Nonce::from_slice(nonce);

        // Decrypt
        let payload = if let Some(aad_data) = aad {
            aes_gcm::aead::Payload {
                msg: ciphertext,
                aad: aad_data,
            }
        } else {
            aes_gcm::aead::Payload {
                msg: ciphertext,
                aad: &[],
            }
        };

        cipher
            .decrypt(nonce, payload)
            .map_err(|e| SymmetricError::DecryptionError(e.to_string()))
    }

    /// Encrypt data using ChaCha20-Poly1305
    fn encrypt_chacha20_poly1305(
        &self,
        key: &[u8],
        nonce: &[u8],
        plaintext: &[u8],
        aad: Option<&[u8]>,
    ) -> Result<Vec<u8>, SymmetricError> {
        // Validate key and nonce
        if key.len() != 32 {
            return Err(SymmetricError::InvalidKeyError(format!(
                "ChaCha20-Poly1305 requires a 32-byte key, got {}",
                key.len()
            )));
        }

        if nonce.len() != 12 {
            return Err(SymmetricError::InvalidNonceError(format!(
                "ChaCha20-Poly1305 requires a 12-byte nonce, got {}",
                nonce.len()
            )));
        }

        // Create cipher
        let key = Key::from_slice(key);
        let cipher = ChaCha20Poly1305::new(key);

        // Create nonce
        let nonce = chacha20poly1305::Nonce::from_slice(nonce);

        // Encrypt with payload
        let payload = if let Some(aad_data) = aad {
            Payload {
                msg: plaintext,
                aad: aad_data,
            }
        } else {
            Payload {
                msg: plaintext,
                aad: &[],
            }
        };

        cipher
            .encrypt(nonce, payload)
            .map_err(|e| SymmetricError::EncryptionError(e.to_string()))
    }

    /// Decrypt data using ChaCha20-Poly1305
    fn decrypt_chacha20_poly1305(
        &self,
        key: &[u8],
        nonce: &[u8],
        ciphertext: &[u8],
        aad: Option<&[u8]>,
    ) -> Result<Vec<u8>, SymmetricError> {
        // Validate key and nonce
        if key.len() != 32 {
            return Err(SymmetricError::InvalidKeyError(format!(
                "ChaCha20-Poly1305 requires a 32-byte key, got {}",
                key.len()
            )));
        }

        if nonce.len() != 12 {
            return Err(SymmetricError::InvalidNonceError(format!(
                "ChaCha20-Poly1305 requires a 12-byte nonce, got {}",
                nonce.len()
            )));
        }

        // Create cipher
        let key = Key::from_slice(key);
        let cipher = ChaCha20Poly1305::new(key);

        // Create nonce
        let nonce = chacha20poly1305::Nonce::from_slice(nonce);

        // Decrypt with payload
        let payload = if let Some(aad_data) = aad {
            Payload {
                msg: ciphertext,
                aad: aad_data,
            }
        } else {
            Payload {
                msg: ciphertext,
                aad: &[],
            }
        };

        cipher
            .decrypt(nonce, payload)
            .map_err(|e| SymmetricError::DecryptionError(e.to_string()))
    }
}

/// Helper function to create an AES-256-GCM cipher
pub fn create_aes_256_gcm() -> SymmetricCrypto {
    SymmetricCrypto::new(SymmetricAlgorithm::Aes256Gcm)
}

/// Helper function to create a ChaCha20-Poly1305 cipher
pub fn create_chacha20_poly1305() -> SymmetricCrypto {
    SymmetricCrypto::new(SymmetricAlgorithm::ChaCha20Poly1305)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_gcm() -> Result<(), Box<dyn std::error::Error>> {
        let crypto = SymmetricCrypto::new(SymmetricAlgorithm::Aes256Gcm);

        // Generate key and nonce
        let key = crypto.generate_key();
        let nonce = crypto.generate_nonce();

        // Test data
        let plaintext = b"This is a test message";
        let aad = b"Additional authenticated data";

        // Encrypt
        let ciphertext = crypto.encrypt(&key, &nonce, plaintext, Some(aad))?;

        // Verify not the same as plaintext
        assert_ne!(&ciphertext, plaintext);

        // Decrypt
        let decrypted = crypto.decrypt(&key, &nonce, &ciphertext, Some(aad))?;

        // Verify decrypted matches original
        assert_eq!(&decrypted, plaintext);

        // Verify decryption fails with wrong AAD
        let wrong_aad = b"Wrong additional data";
        let result = crypto.decrypt(&key, &nonce, &ciphertext, Some(wrong_aad));
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_chacha20_poly1305() -> Result<(), Box<dyn std::error::Error>> {
        let crypto = SymmetricCrypto::new(SymmetricAlgorithm::ChaCha20Poly1305);

        // Generate key and nonce
        let key = crypto.generate_key();
        let nonce = crypto.generate_nonce();

        // Test data
        let plaintext = b"This is a test message for ChaCha20-Poly1305";

        // Encrypt
        let ciphertext = crypto.encrypt(&key, &nonce, plaintext, None)?;

        // Verify not the same as plaintext
        assert_ne!(&ciphertext, plaintext);

        // Decrypt
        let decrypted = crypto.decrypt(&key, &nonce, &ciphertext, None)?;

        // Verify decrypted matches original
        assert_eq!(&decrypted, plaintext);

        Ok(())
    }

    #[test]
    fn test_helper_functions() {
        let aes_crypto = create_aes_256_gcm();
        let chacha_crypto = create_chacha20_poly1305();

        assert_eq!(aes_crypto.algorithm, SymmetricAlgorithm::Aes256Gcm);
        assert_eq!(
            chacha_crypto.algorithm,
            SymmetricAlgorithm::ChaCha20Poly1305
        );
    }
}
