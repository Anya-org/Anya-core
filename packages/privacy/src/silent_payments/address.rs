//! Silent Payment Address Implementation
//!
//! This module defines the address format for Silent Payments as specified
//! in BIP-353, with strong type safety and serialization capabilities.

use bitcoin::bech32::{self, ToBase32, Variant};
use bitcoin::network::constants::Network;
use bitcoin::secp256k1::XOnlyPublicKey;
use serde::{Deserialize, Serialize, Deserializer, Serializer};
use serde::de::{self, Visitor};
use std::fmt;
use std::str::FromStr;
use crate::Result;
use crate::Error;
use super::{MAINNET_PREFIX, TESTNET_PREFIX, REGTEST_PREFIX};

/// A Silent Payment address as defined in BIP-353
///
/// Contains both the scan and spend public keys with network information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SilentPaymentAddress {
    /// The scan public key
    pub scan_pubkey: XOnlyPublicKey,
    
    /// The spend public key
    pub spend_pubkey: XOnlyPublicKey,
    
    /// The Bitcoin network this address is for
    pub network: Network,
}

impl SilentPaymentAddress {
    /// Create a new Silent Payment address
    pub fn new(scan_pubkey: XOnlyPublicKey, spend_pubkey: XOnlyPublicKey, network: Network) -> Self {
        Self {
            scan_pubkey,
            spend_pubkey,
            network,
        }
    }
    
    /// Get the appropriate prefix for the network
    fn prefix(&self) -> &'static str {
        match self.network {
            Network::Bitcoin => MAINNET_PREFIX,
            Network::Testnet => TESTNET_PREFIX,
            Network::Regtest => REGTEST_PREFIX,
            _ => TESTNET_PREFIX, // Default to testnet for other networks
        }
    }
    
    /// Encode the address as a Bech32 string
    pub fn to_string(&self) -> String {
        // Serialize both public keys
        let mut data = Vec::with_capacity(64);
        data.extend_from_slice(&self.scan_pubkey.serialize());
        data.extend_from_slice(&self.spend_pubkey.serialize());
        
        // Encode as Bech32m
        let base32_data = data.to_base32();
        bech32::encode(self.prefix(), base32_data, Variant::Bech32m)
            .expect("Should never fail with valid data")
    }
}

impl FromStr for SilentPaymentAddress {
    type Err = Error;
    
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // Decode the Bech32m address
        let (hrp, data, variant) = bech32::decode(s)
            .map_err(|e| Error::InvalidAddress(format!("Invalid Bech32m encoding: {}", e)))?;
        
        // Verify this is a Silent Payment address with proper variant
        if variant != Variant::Bech32m {
            return Err(Error::InvalidAddress("Not a Bech32m address".into()));
        }
        
        // Determine network from prefix
        let network = match hrp.as_str() {
            prefix if prefix == MAINNET_PREFIX => Network::Bitcoin,
            prefix if prefix == TESTNET_PREFIX => Network::Testnet,
            prefix if prefix == REGTEST_PREFIX => Network::Regtest,
            _ => return Err(Error::InvalidAddress(format!("Unknown prefix: {}", hrp))),
        };
        
        // Decode from base32
        let data = Vec::from_base32(&data)
            .map_err(|e| Error::InvalidAddress(format!("Invalid base32 data: {}", e)))?;
        
        // Verify data length
        if data.len() != 64 {
            return Err(Error::InvalidAddress(format!(
                "Invalid data length: {}, expected 64 bytes", data.len()
            )));
        }
        
        // Extract pubkeys
        let scan_pubkey = XOnlyPublicKey::from_slice(&data[0..32])
            .map_err(|e| Error::InvalidAddress(format!("Invalid scan pubkey: {}", e)))?;
        
        let spend_pubkey = XOnlyPublicKey::from_slice(&data[32..64])
            .map_err(|e| Error::InvalidAddress(format!("Invalid spend pubkey: {}", e)))?;
        
        Ok(Self {
            scan_pubkey,
            spend_pubkey,
            network,
        })
    }
}

impl fmt::Display for SilentPaymentAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// Custom serialization for Serde
impl Serialize for SilentPaymentAddress {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct SilentPaymentAddressVisitor;

impl<'de> Visitor<'de> for SilentPaymentAddressVisitor {
    type Value = SilentPaymentAddress;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid Silent Payment address string")
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        SilentPaymentAddress::from_str(value).map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for SilentPaymentAddress {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SilentPaymentAddressVisitor)
    }
} 