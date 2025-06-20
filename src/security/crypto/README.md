# Cryptographic Security Module [AIR-3][AIS-3][BPC-3]

This directory contains the cryptographic security implementations for Anya Core, following official Bitcoin Improvement Proposals (BIPs) standards.

## Overview

The cryptographic module provides secure implementations of cryptographic algorithms needed for Bitcoin operations, including symmetric and asymmetric encryption, hashing, key derivation, and digital signatures.

## Key Components

### Symmetric Encryption

- **AES-GCM**: Authenticated encryption with associated data
- **ChaCha20-Poly1305**: High-performance authenticated encryption
- **AES-CBC/CTR**: Block cipher modes of operation

### Asymmetric Cryptography

- **ECDSA**: Elliptic Curve Digital Signature Algorithm
- **Schnorr Signatures**: BIP-340 compliant Schnorr implementation
- **RSA**: For legacy compatibility where needed

### Hashing Functions

- **SHA-256/512**: Secure hash algorithms
- **RIPEMD-160**: For Bitcoin address generation
- **SHA-3**: Next-generation secure hash

### Key Derivation

- **PBKDF2**: Password-Based Key Derivation Function
- **Argon2**: Modern key derivation with tunable parameters
- **scrypt**: Memory-hard key derivation

### Random Number Generation

- **Secure RNG**: Cryptographically secure random number generation
- **Deterministic RNG**: For reproducible testing

## Architecture

The cryptographic module follows a clean architecture pattern:

- Core cryptographic primitives as domain entities
- Service layer for complex operations
- Adapters for specific cryptographic libraries
- Cross-cutting concerns like validation and logging

## Implementation Details

### Security Considerations

All cryptographic implementations adhere to the following principles:

1. **Constant-Time Operations**: Resistant to timing attacks
2. **No Side-Channel Leakage**: Protected against side-channel attacks
3. **Memory Safety**: Secure memory handling for sensitive data
4. **Modern Algorithms**: Using current best practices and standards
5. **Defense in Depth**: Multiple layers of protection

### Error Handling

Cryptographic errors are handled using a dedicated error type hierarchy:

```rust
#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    
    #[error("Decryption error: {0}")]
    DecryptionError(String),
    
    #[error("Invalid key error: {0}")]
    InvalidKeyError(String),
    
    #[error("Signature error: {0}")]
    SignatureError(String),
    
    #[error("Hash error: {0}")]
    HashError(String),
    
    #[error("Random generation error: {0}")]
    RandomError(String),
}
```

## Usage Examples

### Symmetric Encryption

```rust
use anya_core::security::crypto::symmetric::{SymmetricCrypto, SymmetricAlgorithm};

// Create a symmetric crypto handler using AES-GCM
let crypto = SymmetricCrypto::new(SymmetricAlgorithm::Aes256Gcm);

// Generate a random key and nonce
let key = crypto.generate_key();
let nonce = crypto.generate_nonce();

// Encrypt data
let plaintext = "Sensitive data";
let ciphertext = crypto.encrypt(&key, &nonce, plaintext.as_bytes(), Some(b"associated data"))?;

// Decrypt data
let decrypted = crypto.decrypt(&key, &nonce, &ciphertext, Some(b"associated data"))?;
assert_eq!(plaintext.as_bytes(), &decrypted[..]);
```

### Digital Signatures

```rust
use anya_core::security::crypto::signature::{Signer, SigningAlgorithm};

// Create a Schnorr signer
let signer = Signer::new(SigningAlgorithm::Schnorr);

// Generate a key pair
let (private_key, public_key) = signer.generate_key_pair()?;

// Sign a message
let message = "Message to sign";
let signature = signer.sign(&private_key, message.as_bytes())?;

// Verify the signature
let is_valid = signer.verify(&public_key, message.as_bytes(), &signature)?;
assert!(is_valid);
```

## Bitcoin Protocol Compliance

The cryptographic module adheres to Bitcoin protocol standards:

- BIP-340: Schnorr Signatures for Bitcoin
- BIP-341: Taproot: SegWit version 1 spending rules
- BIP-342: Validation of Taproot Scripts
- BIP-174: Partially Signed Bitcoin Transaction Format

## Documentation

For more information, see:

- [Security Guidelines](../../../docs/SECURITY.md)
- [Cryptographic Standards](https://developers.bitcoin.org/reference/transactions.html)
- [Implementation Status](../../../docs/IMPLEMENTATION_MILESTONES.md)

## Version Information

- Current Version: 3.1.0
- Last Updated: 2025-04-29
- Bitcoin Improvement Proposals (BIPs): Latest standards

*This component complies with [AI Labeling Standards](../../../docs/AI_LABELING.md)* 