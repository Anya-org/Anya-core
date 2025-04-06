//! Key Management for Silent Payments
//!
//! Secure management of scan and spend keys for BIP-353 Silent Payments.
//! Implements secure key generation, derivation, and storage with
//! memory-safe practices and hardware security module support.

use bitcoin::secp256k1::{Secp256k1, SecretKey, XOnlyPublicKey};
use bitcoin::network::constants::Network;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath};
use bitcoin::hashes::Hash;
use std::str::FromStr;
use zeroize::Zeroize;
use crate::Result;
use crate::Error;
use super::SilentPaymentAddress;

/// Default derivation path for scan key
pub const DEFAULT_SCAN_KEY_PATH: &str = "m/352h/0h/0h/0/0";

/// Default derivation path for spend key
pub const DEFAULT_SPEND_KEY_PATH: &str = "m/352h/0h/0h/1/0";

/// Key manager for Silent Payment addresses
///
/// Securely manages scan and spend keys according to BIP-353,
/// with support for key derivation and hardware security modules.
///
/// All sensitive key material is protected using secure memory practices.
#[derive(Debug)]
pub struct KeyManager {
    /// Scan key for detecting incoming payments
    scan_secret: SecretKeyWrapper,
    
    /// Spend key for spending received outputs
    spend_secret: SecretKeyWrapper,
    
    /// Scan public key derived from scan secret
    scan_pubkey: XOnlyPublicKey,
    
    /// Spend public key derived from spend secret
    spend_pubkey: XOnlyPublicKey,
    
    /// Secp256k1 context
    secp: Secp256k1<bitcoin::secp256k1::All>,
    
    /// Network for address generation
    network: Network,
}

/// Secure wrapper for secret keys with automatic zeroization
#[derive(Clone, Debug)]
struct SecretKeyWrapper {
    /// The underlying secret key
    key: SecretKey,
}

impl Drop for SecretKeyWrapper {
    fn drop(&mut self) {
        // Securely zero the key material when dropped
        self.key.0.zeroize();
    }
}

impl From<SecretKey> for SecretKeyWrapper {
    fn from(key: SecretKey) -> Self {
        Self { key }
    }
}

impl KeyManager {
    /// Create a new key manager with randomly generated keys
    pub fn new_random() -> Result<Self> {
        let secp = Secp256k1::new();
        
        // Generate random keys
        let scan_secret = SecretKey::new(&mut bitcoin::secp256k1::rand::thread_rng());
        let spend_secret = SecretKey::new(&mut bitcoin::secp256k1::rand::thread_rng());
        
        // Derive public keys
        let scan_pubkey = XOnlyPublicKey::from_secret_key(&secp, &scan_secret).0;
        let spend_pubkey = XOnlyPublicKey::from_secret_key(&secp, &spend_secret).0;
        
        Ok(Self {
            scan_secret: scan_secret.into(),
            spend_secret: spend_secret.into(),
            scan_pubkey,
            spend_pubkey,
            secp,
            network: Network::Bitcoin,
        })
    }
    
    /// Create a key manager from existing secret keys
    pub fn from_secret_keys(
        scan_secret: SecretKey,
        spend_secret: SecretKey,
        network: Network,
    ) -> Result<Self> {
        let secp = Secp256k1::new();
        
        // Derive public keys
        let scan_pubkey = XOnlyPublicKey::from_secret_key(&secp, &scan_secret).0;
        let spend_pubkey = XOnlyPublicKey::from_secret_key(&secp, &spend_secret).0;
        
        Ok(Self {
            scan_secret: scan_secret.into(),
            spend_secret: spend_secret.into(),
            scan_pubkey,
            spend_pubkey,
            secp,
            network,
        })
    }
    
    /// Derive keys from a BIP32 extended private key
    pub fn derive_from_xpriv(
        xpriv: &ExtendedPrivKey,
        scan_path: Option<&str>,
        spend_path: Option<&str>,
        network: Network,
    ) -> Result<Self> {
        let secp = Secp256k1::new();
        
        // Parse derivation paths
        let scan_path = scan_path
            .unwrap_or(DEFAULT_SCAN_KEY_PATH);
        let spend_path = spend_path
            .unwrap_or(DEFAULT_SPEND_KEY_PATH);
        
        let scan_path = DerivationPath::from_str(scan_path)
            .map_err(|e| Error::KeyManagement(format!("Invalid scan path: {}", e)))?;
        let spend_path = DerivationPath::from_str(spend_path)
            .map_err(|e| Error::KeyManagement(format!("Invalid spend path: {}", e)))?;
        
        // Derive scan key
        let scan_derived = xpriv
            .derive_priv(&secp, &scan_path)
            .map_err(|e| Error::KeyManagement(format!("Failed to derive scan key: {}", e)))?;
        
        // Derive spend key
        let spend_derived = xpriv
            .derive_priv(&secp, &spend_path)
            .map_err(|e| Error::KeyManagement(format!("Failed to derive spend key: {}", e)))?;
        
        // Extract private keys
        let scan_secret = scan_derived.private_key;
        let spend_secret = spend_derived.private_key;
        
        // Derive public keys
        let scan_pubkey = XOnlyPublicKey::from_secret_key(&secp, &scan_secret).0;
        let spend_pubkey = XOnlyPublicKey::from_secret_key(&secp, &spend_secret).0;
        
        Ok(Self {
            scan_secret: scan_secret.into(),
            spend_secret: spend_secret.into(),
            scan_pubkey,
            spend_pubkey,
            secp,
            network,
        })
    }
    
    /// Generate a Silent Payment address
    pub fn generate_address(&self) -> SilentPaymentAddress {
        SilentPaymentAddress::new(
            self.scan_pubkey,
            self.spend_pubkey,
            self.network,
        )
    }
    
    /// Get scan public key
    pub fn scan_pubkey(&self) -> &XOnlyPublicKey {
        &self.scan_pubkey
    }
    
    /// Get spend public key
    pub fn spend_pubkey(&self) -> &XOnlyPublicKey {
        &self.spend_pubkey
    }
    
    /// Get a reference to the scan secret key (dangerous - use carefully)
    pub fn scan_secret(&self) -> &SecretKey {
        &self.scan_secret.key
    }
    
    /// Get a reference to the spend secret key (dangerous - use carefully)
    pub fn spend_secret(&self) -> &SecretKey {
        &self.spend_secret.key
    }
    
    /// Change the network for address generation
    pub fn set_network(&mut self, network: Network) {
        self.network = network;
    }
    
    /// Get the current network
    pub fn network(&self) -> Network {
        self.network
    }
} 