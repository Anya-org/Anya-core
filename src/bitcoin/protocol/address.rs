// Bitcoin Address Utilities Module
// [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
//
// Address utilities according to Bitcoin Development Framework v2.5 requirements

use anyhow::{Result, Context, bail};
use bitcoin::{Address, Network, ScriptBuf, Script};
use bitcoin::address::NetworkUnchecked;
use bitcoin::secp256k1::{Secp256k1, XOnlyPublicKey};
use bitcoin::taproot::{TapLeaf, TapTree, LeafVersion};
use std::str::FromStr;

/// Address type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressType {
    /// P2PKH (Pay to Public Key Hash)
    P2PKH,
    
    /// P2SH (Pay to Script Hash)
    P2SH,
    
    /// P2WPKH (Pay to Witness Public Key Hash)
    P2WPKH,
    
    /// P2WSH (Pay to Witness Script Hash)
    P2WSH,
    
    /// P2TR (Pay to Taproot)
    P2TR,
    
    /// Unknown address type
    Unknown,
}

/// Get the type of an address
pub fn get_address_type(address: &Address) -> AddressType {
    if address.is_p2pkh() {
        AddressType::P2PKH
    } else if address.is_p2sh() {
        AddressType::P2SH
    } else if address.is_p2wpkh() {
        AddressType::P2WPKH
    } else if address.is_p2wsh() {
        AddressType::P2WSH
    } else if address.is_p2tr() {
        AddressType::P2TR
    } else {
        AddressType::Unknown
    }
}

/// Check if an address is a SegWit address
pub fn is_segwit_address(address: &Address) -> bool {
    address.is_p2wpkh() || address.is_p2wsh() || address.is_p2tr()
}

/// Check if an address is a Taproot address
pub fn is_taproot_address(address: &Address) -> bool {
    address.is_p2tr()
}

/// Get the script pubkey for an address
pub fn get_script_pubkey(address: &Address) -> ScriptBuf {
    address.script_pubkey()
}

/// Create a P2PKH address from a public key hash
pub fn create_p2pkh_address(pubkey_hash: &[u8; 20], network: Network) -> Result<Address> {
    Address::p2pkh(
        bitcoin::PubkeyHash::from_slice(pubkey_hash)
            .context("Invalid public key hash")?,
        network,
    ).context("Failed to create P2PKH address")
}

/// Create a P2SH address from a script
pub fn create_p2sh_address(script: &Script, network: Network) -> Result<Address> {
    Address::p2sh(
        &bitcoin::ScriptHash::from_script(script)
            .context("Failed to hash script")?,
        network,
    ).context("Failed to create P2SH address")
}

/// Create a P2WPKH address from a public key hash
pub fn create_p2wpkh_address(pubkey_hash: &[u8; 20], network: Network) -> Result<Address> {
    Address::p2wpkh(
        &bitcoin::WPubkeyHash::from_slice(pubkey_hash)
            .context("Invalid witness public key hash")?,
        network,
    ).context("Failed to create P2WPKH address")
}

/// Create a P2WSH address from a script
pub fn create_p2wsh_address(script: &Script, network: Network) -> Result<Address> {
    Address::p2wsh(
        &bitcoin::WScriptHash::from_script(script)
            .context("Failed to hash witness script")?,
        network,
    ).context("Failed to create P2WSH address")
}

/// Create a P2TR address from an internal key
pub fn create_p2tr_address(internal_key: &XOnlyPublicKey, network: Network) -> Result<Address> {
    let secp = Secp256k1::new();
    Address::p2tr(
        &secp,
        *internal_key,
        None,
        network,
    ).context("Failed to create P2TR address")
}

/// Create a P2TR address with a Taproot script tree
pub fn create_p2tr_address_with_scripts(
    internal_key: &XOnlyPublicKey,
    scripts: &[ScriptBuf],
    network: Network,
) -> Result<Address> {
    let secp = Secp256k1::new();
    
    // Create a Taproot tree with the scripts
    let mut builder = TapTree::builder();
    
    for (i, script) in scripts.iter().enumerate() {
        let leaf = TapLeaf::new(LeafVersion::TapScript, script.clone());
        builder = builder.add_leaf(i as u8, leaf)
            .context("Failed to add leaf to Taproot tree")?;
    }
    
    // Finalize the tree
    let tap_tree = builder.finalize(&secp, *internal_key)
        .context("Failed to finalize Taproot tree")?;
    
    // Get the output key
    let output_key = tap_tree.output_key();
    
    // Create the address
    Address::p2tr(
        &secp,
        output_key,
        None,
        network,
    ).context("Failed to create P2TR address with scripts")
}

/// Parse an address string into an Address object
pub fn parse_address(address: &str, network: Network) -> Result<Address> {
    // Parse as generic address first
    match Address::from_str(address) {
        Ok(address_unchecked) => {
            // Check network
            match address_unchecked.require_network(network) {
                Ok(address) => Ok(address),
                Err(_) => {
                    bail!("Address network mismatch: expected {:?}", network)
                }
            }
        },
        Err(e) => {
            bail!("Invalid address: {}", e)
        }
    }
}

/// Get the recommended address type for a given network
pub fn recommended_address_type(network: Network) -> AddressType {
    // P2TR is recommended for mainnet and testnet
    // P2WPKH is recommended for regtest for compatibility
    match network {
        Network::Bitcoin | Network::Testnet | Network::Signet => AddressType::P2TR,
        Network::Regtest => AddressType::P2WPKH,
    }
}

/// Format an address for display
pub fn format_address(address: &Address) -> String {
    // Get address type
    let address_type = get_address_type(address);
    
    // Format with prefix
    match address_type {
        AddressType::P2PKH => format!("P2PKH: {}", address),
        AddressType::P2SH => format!("P2SH: {}", address),
        AddressType::P2WPKH => format!("P2WPKH: {}", address),
        AddressType::P2WSH => format!("P2WSH: {}", address),
        AddressType::P2TR => format!("P2TR: {}", address),
        AddressType::Unknown => format!("Unknown: {}", address),
    }
} 