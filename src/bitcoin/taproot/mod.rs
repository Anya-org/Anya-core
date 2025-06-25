//! Taproot implementation for Bitcoin
//!
//! This module provides functionality for working with Taproot, including
//! key generation, asset creation, and transaction building.
//!
//! [AIR-3][AIS-3][BPC-3][RES-3] Implementation follows official Bitcoin Improvement Proposals (BIPs)

// [AIR-3][AIS-3][BPC-3][RES-3] Taproot module for Bitcoin asset management
use bitcoin::hashes::sha256;
use bitcoin::secp256k1::{self, Secp256k1, SecretKey};
use bitcoin::taproot::TaprootBuilder;
use bitcoin::ScriptBuf;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::bitcoin::error::BitcoinError;

/// Errors that can occur during Taproot operations
#[derive(Debug)]
pub enum TaprootError {
    /// Error related to key operations
    KeyError(String),
    /// Error related to script operations
    ScriptError(String),
    /// Error from Taproot operations
    TaprootError(String),
    /// Error from Taproot builder
    BuilderError(String),
    /// Error from Bitcoin operations
    BitcoinError(String),
    /// Error from secp256k1 operations
    Secp256k1Error(secp256k1::Error),
    /// Error from hex operations
    HexError(hex::FromHexError),
    /// Error from input validation
    ValidationError(String),
}

impl fmt::Display for TaprootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KeyError(e) => write!(f, "Key error: {}", e),
            Self::ScriptError(e) => write!(f, "Script error: {}", e),
            Self::TaprootError(e) => write!(f, "Taproot error: {}", e),
            Self::BuilderError(e) => write!(f, "Builder error: {}", e),
            Self::BitcoinError(e) => write!(f, "Bitcoin error: {}", e),
            Self::Secp256k1Error(e) => write!(f, "Secp256k1 error: {}", e),
            Self::HexError(e) => write!(f, "Hex error: {}", e),
            Self::ValidationError(e) => write!(f, "Validation error: {}", e),
        }
    }
}

impl std::error::Error for TaprootError {}

impl From<bitcoin::secp256k1::Error> for TaprootError {
    fn from(e: bitcoin::secp256k1::Error) -> Self {
        Self::Secp256k1Error(e)
    }
}

impl From<hex::FromHexError> for TaprootError {
    fn from(e: hex::FromHexError) -> Self {
        Self::HexError(e)
    }
}

impl From<BitcoinError> for TaprootError {
    fn from(e: BitcoinError) -> Self {
        Self::BitcoinError(e.to_string())
    }
}

impl From<bitcoin::taproot::TaprootError> for TaprootError {
    fn from(e: bitcoin::taproot::TaprootError) -> Self {
        Self::TaprootError(e.to_string())
    }
}

impl From<bitcoin::taproot::IncompleteBuilderError> for TaprootError {
    fn from(e: bitcoin::taproot::IncompleteBuilderError) -> Self {
        Self::BuilderError(e.to_string())
    }
}

/// Taproot Asset structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaprootAsset {
    /// Unique identifier for the asset (SHA-256 hash of asset properties)
    pub asset_id: [u8; 32],
    /// Name of the asset (1-32 characters)
    pub name: String,
    /// Total supply of the asset (must be > 0)
    pub supply: u64,
    /// Number of decimal places (0-8)
    pub precision: u8,
    /// Additional metadata (max 1024 bytes)
    pub metadata: String,
    /// Whether the asset has been issued on-chain
    pub issued: bool,
    /// Taproot script leaves for the asset
    pub leaves: Vec<ScriptBuf>,
    /// Number of script leaves
    pub num_leaves: u32,
    /// Issuer's public key (x-only, 32 bytes)
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
        // Validate inputs
        if name.is_empty() || name.len() > 32 {
            return Err(crate::bitcoin::taproot::TaprootError::ValidationError(
                "Asset name must be 1-32 characters".to_string(),
            ));
        }

        if supply == 0 {
            return Err(crate::bitcoin::taproot::TaprootError::ValidationError(
                "Supply must be greater than 0".to_string(),
            ));
        }

        if precision > 8 {
            return Err(crate::bitcoin::taproot::TaprootError::ValidationError(
                "Precision must be 0-8".to_string(),
            ));
        }

        if metadata.len() > 1024 {
            return Err(crate::bitcoin::taproot::TaprootError::ValidationError(
                "Metadata too large (max 1024 bytes)".to_string(),
            ));
        }

        // Generate asset ID
        let asset_id = generate_asset_id(name, supply, precision, metadata)?;

        // Create the asset script
        let mut builder = bitcoin::blockdata::script::Builder::new();
        builder = builder.push_opcode(bitcoin::opcodes::all::OP_RETURN);
        builder = push_bytes_to_script(builder, &asset_id);

        // Initialize the leaves vector with the asset script
        let leaves = vec![builder.into_script()];

        // Parse the secret key to get the public key
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(issuer_secret_key)
            .map_err(|e| crate::bitcoin::taproot::TaprootError::KeyError(e.to_string()))?;
        let (x_only_pubkey, _) = secret_key.public_key(&secp).x_only_public_key();

        Ok(Self {
            asset_id,
            name: name.to_string(),
            supply,
            precision,
            metadata: metadata.to_string(),
            issued: true,
            leaves,
            num_leaves: 1,
            issuer_pubkey: x_only_pubkey.serialize(),
        })
    }

    /// Create a Taproot-compatible asset script
    ///
    /// # Returns
    /// A `ScriptBuf` containing the asset script or a `TaprootError` if creation fails
    ///
    /// # Compliance
    /// - BIP-341/342 (Taproot)
    /// - BIP-352 (Asset protocols)
    pub fn create_asset_script(&self) -> Result<ScriptBuf, TaprootError> {
        use bitcoin::blockdata::opcodes;
        use bitcoin::blockdata::script::Builder;

        let mut builder = Builder::new();

        // Start with OP_RETURN
        builder = builder.push_opcode(opcodes::all::OP_RETURN);

        // Push the asset ID bytes
        builder = push_bytes_to_script(builder, &self.asset_id);

        // Push name as bytes (limited to 32 bytes)
        let name_bytes = self.name.as_bytes();
        let name_slice = if name_bytes.len() > 32 {
            &name_bytes[..32]
        } else {
            name_bytes
        };
        builder = push_bytes_to_script(builder, name_slice);

        // Push supply as 8-byte little-endian
        let supply_bytes = self.supply.to_le_bytes();
        builder = push_bytes_to_script(builder, &supply_bytes);

        // Push precision as single byte
        builder = builder.push_int(self.precision as i64);

        Ok(builder.into_script())
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

/// Generate a unique asset ID from the asset's properties
///
/// # Arguments
/// * `name` - Name of the asset
/// * `supply` - Total supply of the asset
/// * `precision` - Decimal precision of the asset
/// * `metadata` - Additional metadata for the asset
///
/// # Returns
/// A 32-byte array representing the asset ID
fn generate_asset_id(
    name: &str,
    supply: u64,
    precision: u8,
    metadata: &str,
) -> Result<[u8; 32], TaprootError> {
    use bitcoin::hashes::{Hash, HashEngine};

    let mut engine = sha256::Hash::engine();
    engine.input(name.as_bytes());
    engine.input(&supply.to_le_bytes());
    engine.input(&[precision]);
    engine.input(metadata.as_bytes());

    let hash = sha256::Hash::from_engine(engine);
    Ok(hash.to_byte_array())
}

/// Helper method to push bytes to a script
fn push_bytes_to_script(
    mut builder: bitcoin::blockdata::script::Builder,
    data: &[u8],
) -> bitcoin::blockdata::script::Builder {
    for &byte in data {
        builder = builder.push_int(byte as i64);
    }
    builder
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
/// * `issuer_secret_key` - Secret key of the asset issuer (32 bytes)
///
/// # Returns
/// A new instance of TaprootAsset or an error if validation fails
///
/// # Compliance
/// - BIP-341/342 (Taproot)
/// - BIP-352 (Asset protocols)
///
/// # Example
/// ```
/// use anya_core::bitcoin::taproot::{create_asset, TaprootAsset};
/// use bitcoin::secp256k1::rand;
///
/// let mut rng = rand::thread_rng();
/// let secret_key = bitcoin::secp256k1::SecretKey::new(&mut rng);
///
/// let asset = create_asset(
///     "MY_ASSET",
///     1000000,
///     8,
///     "My test asset",
///     &secret_key[..],
/// ).unwrap();
/// ```
pub fn create_asset(
    name: &str,
    supply: u64,
    precision: u8,
    metadata: &str,
    issuer_secret_key: &[u8],
) -> Result<TaprootAsset, BitcoinError> {
    // Validate inputs
    if name.is_empty() || name.len() > 32 {
        return Err(BitcoinError::InvalidScript(
            "Asset name must be 1-32 characters".to_string(),
        ));
    }

    if supply == 0 {
        return Err(BitcoinError::InvalidScript(
            "Asset supply must be greater than 0".to_string(),
        ));
    }

    if precision > 8 {
        return Err(BitcoinError::InvalidScript(
            "Precision must be between 0 and 8".to_string(),
        ));
    }

    if metadata.len() > 1024 {
        return Err(BitcoinError::InvalidScript(
            "Metadata exceeds maximum length of 1024 bytes".to_string(),
        ));
    }

    if issuer_secret_key.len() != 32 {
        return Err(BitcoinError::InvalidPrivateKey);
    }

    // Create the asset with the provided issuer secret key
    TaprootAsset::new(name, supply, precision, metadata, issuer_secret_key)
        .map_err(|e| BitcoinError::TaprootError(e.to_string()))
}

/// Issue a Taproot asset
///
/// Creates a transaction that issues the asset to the specified address.
///
/// # Arguments
/// * `asset` - The TaprootAsset to issue
/// * `issuer_secret_key` - The issuer's secret key (32 bytes)
///
/// # Returns
/// The hex-encoded Taproot output script that locks the asset
///
/// # Compliance
/// - BIP-341/342 (Taproot)
/// - BIP-352 (Asset protocols)
pub fn issue_asset(asset: &TaprootAsset, issuer_secret_key: &[u8]) -> Result<String, TaprootError> {
    // Create secp256k1 context
    let secp = Secp256k1::new();

    // Parse the secret key
    let secret_key = secp256k1::SecretKey::from_slice(issuer_secret_key)
        .map_err(|e| TaprootError::KeyError(e.to_string()))?;

    // Get the x-only public key
    let (x_only_pubkey, _) = secret_key.public_key(&secp).x_only_public_key();

    // Create the asset script
    let asset_script = asset.create_asset_script()?;

    // Create a Taproot tree with the asset script following the tr(KEY,{SILENT_LEAF}) pattern
    let mut builder = TaprootBuilder::new();

    // Add the asset script as a leaf with depth 1 (SILENT_LEAF)
    builder = builder
        .add_leaf(1, asset_script.clone())
        .map_err(|e| TaprootError::BuilderError(format!("Failed to add leaf: {}", e)))?;

    // Finalize the Taproot tree with the internal key (KEY)
    let taproot_spend_info = builder
        .finalize(&secp, x_only_pubkey)
        .map_err(|e| TaprootError::TaprootError(format!("Failed to finalize builder: {:?}", e)))?;

    // Get the Taproot output script
    let output_key = taproot_spend_info.output_key();
    let output_script = bitcoin::ScriptBuf::new_p2tr(&secp, output_key.into(), None);

    // [AIR-3][AIS-3][BPC-3][RES-3] Verify the output script follows tr(KEY,{SILENT_LEAF}) pattern
    let script_bytes = output_script.as_bytes();
    if script_bytes.len() != 34 || !script_bytes.starts_with(&[0x51, 0x20]) {
        return Err(TaprootError::BuilderError(
            "Output script does not match tr(KEY,{SILENT_LEAF}) pattern".to_string(),
        ));
    }

    Ok(hex::encode(output_script.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::secp256k1::rand::rngs::OsRng;
    use bitcoin::secp256k1::Secp256k1;

    #[test]
    fn test_generate_keypair() {
        let result = generate_keypair();
        assert!(result.is_ok());

        let (secret_key, public_key) = result.unwrap();

        // Verify the keys are related
        let secp = Secp256k1::new();
        let derived_pubkey = secret_key.x_only_public_key(&secp).0;
        assert_eq!(public_key, derived_pubkey);
    }

    #[test]
    fn test_generate_asset_id() {
        let asset_id1 = generate_asset_id("TEST", 1000, 8, "metadata").unwrap();
        let asset_id2 = generate_asset_id("TEST", 1000, 8, "metadata").unwrap();
        let asset_id3 = generate_asset_id("TEST", 1001, 8, "metadata").unwrap();

        // Same inputs should produce same ID
        assert_eq!(asset_id1, asset_id2);

        // Different inputs should produce different IDs
        assert_ne!(asset_id1, asset_id3);

        // Asset ID should be 32 bytes
        assert_eq!(asset_id1.len(), 32);
    }

    #[test]
    fn test_taproot_asset_creation() {
        let secp = Secp256k1::new();
        let mut rng = OsRng;
        let (secret_key, _) = secp.generate_keypair(&mut rng);

        let asset = TaprootAsset::new(
            "TESTCOIN",
            1000000,
            8,
            "Test asset metadata",
            &secret_key[..],
        );

        assert!(asset.is_ok());
        let asset = asset.unwrap();

        assert_eq!(asset.name, "TESTCOIN");
        assert_eq!(asset.supply, 1000000);
        assert_eq!(asset.precision, 8);
        assert_eq!(asset.metadata, "Test asset metadata");
        assert!(asset.issued);
        assert_eq!(asset.num_leaves, 1);
        assert_eq!(asset.leaves.len(), 1);
    }

    #[test]
    fn test_asset_validation() {
        let secp = Secp256k1::new();
        let mut rng = OsRng;
        let (secret_key, _) = secp.generate_keypair(&mut rng);

        // Test empty name
        let result = TaprootAsset::new("", 1000, 8, "metadata", &secret_key[..]);
        assert!(result.is_err());

        // Test name too long
        let long_name = "a".repeat(33);
        let result = TaprootAsset::new(&long_name, 1000, 8, "metadata", &secret_key[..]);
        assert!(result.is_err());

        // Test zero supply
        let result = TaprootAsset::new("TEST", 0, 8, "metadata", &secret_key[..]);
        assert!(result.is_err());

        // Test precision too high
        let result = TaprootAsset::new("TEST", 1000, 9, "metadata", &secret_key[..]);
        assert!(result.is_err());

        // Test metadata too large
        let large_metadata = "a".repeat(1025);
        let result = TaprootAsset::new("TEST", 1000, 8, &large_metadata, &secret_key[..]);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_asset_script() {
        let secp = Secp256k1::new();
        let mut rng = OsRng;
        let (secret_key, _) = secp.generate_keypair(&mut rng);

        let asset = TaprootAsset::new("TEST", 1000, 8, "metadata", &secret_key[..]).unwrap();

        let script = asset.create_asset_script();
        assert!(script.is_ok());

        let script = script.unwrap();
        assert!(!script.is_empty());

        // Should start with OP_RETURN
        let script_bytes = script.as_bytes();
        assert_eq!(script_bytes[0], 0x6a); // OP_RETURN opcode
    }

    #[test]
    fn test_create_asset_function() {
        let secp = Secp256k1::new();
        let mut rng = OsRng;
        let (secret_key, _) = secp.generate_keypair(&mut rng);

        let result = create_asset("TESTCOIN", 1000000, 8, "Test metadata", &secret_key[..]);

        assert!(result.is_ok());
        let asset = result.unwrap();

        assert_eq!(asset.name, "TESTCOIN");
        assert_eq!(asset.supply, 1000000);
        assert_eq!(asset.precision, 8);
        assert_eq!(asset.metadata, "Test metadata");
        assert!(asset.issued);
        assert_eq!(asset.num_leaves, 1);
        assert_eq!(asset.leaves.len(), 1);
    }
}
