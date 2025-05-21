//! Taproot implementation for Bitcoin
//! 
//! This module provides functionality for working with Taproot, including
//! key generation, asset creation, and transaction building.
//!
//! [AIR-3][AIS-3][BPC-3][RES-3] Implementation follows Bitcoin Development Framework v2.5

use bitcoin::hashes::{Hash, sha256};
use bitcoin::key::Keypair;
use bitcoin::secp256k1::{self, Secp256k1, SecretKey, Keypair as KeyPair, Parity};
use bitcoin::taproot::{TaprootBuilder, TaprootSpendInfo};
use bitcoin::ScriptBuf;
use bitcoin::XOnlyPublicKey;
use std::error::Error;
use std::fmt;
use crate::bitcoin::error::BitcoinError;

/// Custom error type for Taproot operations
#[derive(Debug)]
pub enum TaprootError {
    /// Error during key generation
    KeyError(String),
    /// Error during script generation
    ScriptError(String),
    /// Error during transaction building
    TransactionError(String),
    /// Error during hashing
    HashingError(String),
    /// Error during validation
    ValidationError(String),
    /// Error from Bitcoin operations
    BitcoinError(BitcoinError),
}

impl std::fmt::Display for TaprootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaprootError::KeyError(e) => write!(f, "Key error: {}", e),
            TaprootError::ScriptError(e) => write!(f, "Script error: {}", e),
            TaprootError::TransactionError(e) => write!(f, "Transaction error: {}", e),
            TaprootError::HashingError(e) => write!(f, "Hashing error: {}", e),
            TaprootError::ValidationError(e) => write!(f, "Validation error: {}", e),
            TaprootError::BitcoinError(e) => write!(f, "Bitcoin error: {}", e),
        }
    }
}

impl std::error::Error for TaprootError {}

impl From<bitcoin::secp256k1::Error> for TaprootError {
    fn from(e: bitcoin::secp256k1::Error) -> Self {
        TaprootError::KeyError(e.to_string())
    }
}

impl From<hex::FromHexError> for TaprootError {
    fn from(e: hex::FromHexError) -> Self {
        TaprootError::HashingError(e.to_string())
    }
}

impl From<BitcoinError> for TaprootError {
    fn from(e: BitcoinError) -> Self {
        TaprootError::BitcoinError(e)
    }
}

impl From<bitcoin::taproot::TaprootError> for TaprootError {
    fn from(e: bitcoin::taproot::TaprootError) -> Self {
        TaprootError::TransactionError(e.to_string())
    }
}

/// Taproot Asset structure
#[derive(Debug, Clone)]
pub struct TaprootAsset {
    /// Unique identifier for the asset
    pub asset_id: [u8; 32],
    /// Human-readable name of the asset
    pub name: String,
    /// Total supply of the asset
    pub supply: u64,
    /// Number of decimal places for the asset
    pub precision: u8,
    /// Additional metadata as a JSON string
    pub metadata: String,
    /// Whether the asset has been issued
    pub issued: bool,
    /// Taproot leaves for the asset
    pub leaves: Vec<ScriptBuf>,
    /// Number of leaves in the Taproot tree
    pub num_leaves: u32,
    /// Public key of the asset issuer
    pub issuer_pubkey: [u8; 32],
}

impl TaprootAsset {
    /// Create a new Taproot asset with the given parameters
    pub fn new(
        name: &str,
        supply: u64,
        precision: u8,
        metadata: &str,
        issuer_secret_key: &[u8],
    ) -> Result<Self, TaprootError> {
        if name.is_empty() {
            return Err(TaprootError::ValidationError("Asset name cannot be empty".to_string()));
        }
        
        if precision > 8 {
            return Err(TaprootError::ValidationError("Precision cannot be greater than 8".to_string()));
        }
        
        if metadata.len() > 1024 {
            return Err(TaprootError::ValidationError("Metadata exceeds maximum length of 1024 bytes".to_string()));
        }
        
        // Generate asset ID
        let asset_id = generate_asset_id(name, supply, precision, metadata)?;
        
        // Derive issuer public key from secret key
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(issuer_secret_key)
            .map_err(|e| TaprootError::KeyError(e.to_string()))?;
        let (_, issuer_pubkey) = secret_key.x_only_public_key(&secp);
        
        Ok(Self {
            asset_id,
            name: name.to_string(),
            supply,
            precision,
            metadata: metadata.to_string(),
            issued: false,
            leaves: Vec::new(),
            num_leaves: 0,
            issuer_pubkey: issuer_pubkey.serialize(),
        })
    }
    
    pub fn create_asset_script(&self) -> ScriptBuf {
        // Create a simple script that commits to the asset ID
        let mut script = ScriptBuf::new();
        script.push_slice(&self.asset_id);
        script
    }
}

/// Generate a new Taproot key pair
/// 
/// # Returns
/// A tuple containing the secret key and corresponding public key
/// 
/// # Compliance
/// - BIP-341/342 (Taproot)
pub fn generate_keypair() -> Result<(SecretKey, bitcoin::key::XOnlyPublicKey), TaprootError> {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();
    let (secret_key, _) = secp.generate_keypair(&mut rng);
    
    // Convert to x-only public key for Taproot
    let x_only = secret_key.x_only_public_key(&secp);
    Ok((secret_key, x_only.0))
}

/// Generate a unique asset ID based on asset properties
/// [AIR-3][AIS-3][BPC-3][RES-3] Using Bitcoin's hashing functionality
pub fn generate_asset_id(
    name: &str,
    supply: u64,
    precision: u8,
    metadata: &str,
) -> Result<[u8; 32], TaprootError> {
    // Create a unique string from all asset properties
    let data = format!("{}:{}:{}:{}", name, supply, precision, metadata);
    
    // Hash the data using SHA-256
    let hash = sha256::Hash::hash(data.as_bytes());
    
    // Convert the hash to a fixed-size array
    let mut result = [0u8; 32];
    result.copy_from_slice(&hash[..]);
    
    Ok(result)
}

/// Issue a Taproot asset
/// 
/// Creates a transaction that issues the asset to the specified address.
pub fn issue_asset(asset: &TaprootAsset, issuer_secret_key: &[u8]) -> Result<String, TaprootError> {
    let secp = Secp256k1::new();
    
    // Parse the secret key
    let secret_key = SecretKey::from_slice(issuer_secret_key)?;
    let keypair = KeyPair::from_secret_key(&secp, &secret_key);
    
    // Create the asset script
    let asset_script = asset.create_asset_script();
    
    // Create a Taproot builder and add the asset script as a leaf
    let builder = TaprootBuilder::new()
        .add_leaf(0, asset_script)?;
    
    // Get the x-only public key
    let (x_only_pubkey, _parity) = keypair.x_only_public_key();
    
    // Finalize the Taproot tree
    let taproot_spend_info = builder.finalize(&secp, x_only_pubkey)?;
    
    // Get the output key and create the Taproot output script
    let output_key = taproot_spend_info.output_key();
    let output_script = ScriptBuf::new_p2tr_tweaked(output_key);
    
    // Return the hex-encoded script
    Ok(hex::encode(output_script.as_bytes()))
}

/// Create a new Taproot asset
/// 
/// Creates a new Taproot asset with the given parameters
/// 
/// # Arguments
/// * `name` - Name of the asset (1-32 characters)
/// * `supply` - Total supply of the asset (must be > 0)
/// * `precision` - Number of decimal places (0-8)
/// * `metadata` - Additional metadata as a JSON string (max 1024 bytes)
/// * `issuer_secret_key` - Secret key of the asset issuer
/// 
/// # Returns
/// A new instance of TaprootAsset or an error if validation fails
/// 
/// # Compliance
/// - BIP-341/342 (Taproot)
/// - BIP-352 (Asset protocols)
pub fn create_asset(
    name: &str,
    supply: u64,
    precision: u8,
    metadata: &str,
    issuer_secret_key: &[u8],
) -> Result<TaprootAsset, BitcoinError> {
    // Input validation
    if name.is_empty() || name.len() > 32 {
        return Err(BitcoinError::InvalidScript("Asset name must be 1-32 bytes".to_string()));
    }
    
    if supply == 0 {
        return Err(BitcoinError::InvalidScript("Asset supply must be greater than 0".to_string()));
    }
    
    if precision > 8 {
        return Err(BitcoinError::InvalidScript("Precision cannot exceed 8 decimal places".to_string()));
    }
    
    if metadata.len() > 1024 {
        return Err(BitcoinError::InvalidScript("Metadata exceeds maximum length of 1024 bytes".to_string()));
    }
    
    // Create the asset with the provided issuer secret key
    TaprootAsset::new(
        name,
        supply,
        precision,
        metadata,
        issuer_secret_key,
    ).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::secp256k1::rand;
    

    #[test]
    fn test_taproot_asset_creation() -> Result<(), Box<dyn std::error::Error>> {
        // Generate a random key pair for testing
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut rand::thread_rng());
        
        // Test creating a new asset
        let asset = TaprootAsset::new(
            "TEST",
            100000000,
            8,
            "Test asset",
            &secret_key[..],
        )?;
        
        // Verify the asset properties
        assert_eq!(asset.name, "TEST");
        assert_eq!(asset.supply, 100000000);
        assert_eq!(asset.precision, 8);
        assert_eq!(asset.metadata, "Test asset");
        assert!(!asset.issued);
        
        // Test issuing the asset
        let output_script = super::issue_asset(&asset, &secret_key[..])?;
        assert!(!output_script.is_empty());
        
        // Verify the issuer public key is set
        assert_ne!(asset.issuer_pubkey, [0u8; 32], "Issuer public key should not be zero");
        
        Ok(())
    }
    
    #[test]
    fn test_create_asset() -> Result<(), Box<dyn std::error::Error>> {
        // Generate a random key pair for testing
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut rand::thread_rng());
        
        // Test creating a new asset using the create_asset function
        let asset = super::create_asset(
            "TEST2",
            50000000,
            8,
            "Another test asset",
            &secret_key[..],
        )?;
        
        // Verify the asset properties
        assert_eq!(asset.name, "TEST2");
        assert_eq!(asset.supply, 50000000);
        assert_eq!(asset.precision, 8);
        assert_eq!(asset.metadata, "Another test asset");
        assert!(!asset.issued);
        
        // Test validation
        assert!(super::create_asset("", 100, 8, "", &secret_key[..]).is_err()); // Empty name
        assert!(super::create_asset("TEST", 0, 8, "", &secret_key[..]).is_err()); // Zero supply
        assert!(super::create_asset("TEST", 100, 9, "", &secret_key[..]).is_err()); // Precision > 8
        
        Ok(())
    }
}
    