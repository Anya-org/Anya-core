// src/bitcoin/wallet/bip32.rs

use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::util::bip32::{ExtendedPrivKey, DerivationPath};
use crate::bitcoin::error::BitcoinError;
use crate::AnyaResult;

/// Generate a new seed from an optional password
pub fn generate_seed(password: &str) -> AnyaResult<[u8; 64]> {
    // Generate a new BIP39 mnemonic
    let mnemonic = bip39::Mnemonic::new(bip39::MnemonicType::Words12, bip39::Language::English);
    
    // Convert mnemonic to seed with optional password
    let seed = mnemonic.to_seed(password);
    
    // Print the mnemonic for backup purposes (in a real implementation, this should be returned to the user)
    println!("Generated new mnemonic: {}", mnemonic.phrase());
    println!("IMPORTANT: Write down this mnemonic and keep it in a safe place!");
    
    // Convert to fixed-size array
    let mut seed_bytes = [0u8; 64];
    seed_bytes.copy_from_slice(&seed[0..64]);
    
    Ok(seed_bytes)
}

/// Generate a seed from an existing mnemonic phrase and optional password
pub fn seed_from_mnemonic(mnemonic_phrase: &str, password: &str) -> AnyaResult<[u8; 64]> {
    // Parse the mnemonic
    let mnemonic = bip39::Mnemonic::from_phrase(mnemonic_phrase, bip39::Language::English)
        .map_err(|e| BitcoinError::Wallet(format!("Invalid mnemonic: {}", e)))?;
    
    // Convert mnemonic to seed with optional password
    let seed = mnemonic.to_seed(password);
    
    // Convert to fixed-size array
    let mut seed_bytes = [0u8; 64];
    seed_bytes.copy_from_slice(&seed[0..64]);
    
    Ok(seed_bytes)
}

/// Derive a private key from a seed and derivation path
pub fn derive_key_from_seed(seed: &[u8; 64], path: &str) -> AnyaResult<SecretKey> {
    // Create a secp256k1 context
    let secp = Secp256k1::new();
    
    // Parse the path
    let derivation_path = DerivationPath::from_str(path)
        .map_err(|e| BitcoinError::Wallet(format!("Invalid derivation path: {}", e)))?;
    
    // Create a master key from the seed
    let master_key = ExtendedPrivKey::new_master(bitcoin::Network::Bitcoin, seed)
        .map_err(|e| BitcoinError::Wallet(format!("Failed to create master key: {}", e)))?;
    
    // Derive the child key
    let child_key = master_key.derive_priv(&secp, &derivation_path)
        .map_err(|e| BitcoinError::Wallet(format!("Failed to derive key: {}", e)))?;
    
    Ok(child_key.private_key)
}

/// Parse a BIP32 extended private key from string
pub fn parse_xpriv(xpriv: &str) -> AnyaResult<ExtendedPrivKey> {
    ExtendedPrivKey::from_str(xpriv)
        .map_err(|e| BitcoinError::Wallet(format!("Invalid extended private key: {}", e)).into())
}

/// Format a BIP32 extended private key as string
pub fn format_xpriv(xpriv: &ExtendedPrivKey) -> String {
    xpriv.to_string()
} 