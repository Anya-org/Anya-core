//! Bitcoin standard script templates
//!
//! This module provides standard Bitcoin script templates and patterns
//! following Bitcoin Core principles, with support for Taproot and related BIPs.
//! It includes templates for P2PKH, P2SH, P2WPKH, P2WSH, and P2TR (Taproot).

use std::fmt;
use thiserror::Error;
use log::{debug, info, warn};
use bitcoin::{Script, ScriptBuf, Address, Network, TxOut, PublicKey, BlockHash, Transaction};

use crate::core::error::AnyaResult;

/// Maximum number of public keys in a multisig script
pub const MAX_PUBKEYS_PER_MULTISIG: usize = 16;

/// Standard script types in Bitcoin
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptType {
    /// Non-standard script
    NonStandard,
    /// Pay to Public Key
    PubKey,
    /// Pay to Public Key Hash (P2PKH)
    PubKeyHash,
    /// Pay to Script Hash (P2SH)
    ScriptHash,
    /// SegWit v0 Pay to Public Key Hash (P2WPKH)
    WitnessV0PubKeyHash,
    /// SegWit v0 Pay to Script Hash (P2WSH)
    WitnessV0ScriptHash,
    /// SegWit v1 Pay to Taproot (P2TR)
    WitnessV1Taproot,
    /// Multiple signature script (legacy)
    MultiSig,
    /// OP_RETURN data carrier
    NullData,
}

impl fmt::Display for ScriptType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScriptType::NonStandard => write!(f, "nonstandard"),
            ScriptType::PubKey => write!(f, "pubkey"),
            ScriptType::PubKeyHash => write!(f, "pubkeyhash"),
            ScriptType::ScriptHash => write!(f, "scripthash"),
            ScriptType::WitnessV0PubKeyHash => write!(f, "witness_v0_keyhash"),
            ScriptType::WitnessV0ScriptHash => write!(f, "witness_v0_scripthash"),
            ScriptType::WitnessV1Taproot => write!(f, "witness_v1_taproot"),
            ScriptType::MultiSig => write!(f, "multisig"),
            ScriptType::NullData => write!(f, "nulldata"),
        }
    }
}

/// Script matching and classification errors
#[derive(Debug, Error)]
pub enum ScriptClassifyError {
    #[error("Invalid script format: {0}")]
    InvalidFormat(String),
    
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),
    
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    
    #[error("Invalid OP_RETURN data: {0}")]
    InvalidOpReturn(String),
    
    #[error("Script too large")]
    ScriptTooLarge,
    
    #[error("General error: {0}")]
    General(String),
}

/// Standard script and address creation
pub struct StandardScripts;

impl StandardScripts {
    /// Classify a script into a standard type
    pub fn classify_script(script: &Script) -> ScriptType {
        let script_bytes = script.as_bytes();
        
        // Empty script
        if script_bytes.is_empty() {
            return ScriptType::NonStandard;
        }
        
        // P2PKH: OP_DUP OP_HASH160 <20-byte hash> OP_EQUALVERIFY OP_CHECKSIG
        if script_bytes.len() == 25 &&
           script_bytes[0] == 0x76 && // OP_DUP
           script_bytes[1] == 0xa9 && // OP_HASH160
           script_bytes[2] == 0x14 && // Push 20 bytes
           script_bytes[23] == 0x88 && // OP_EQUALVERIFY
           script_bytes[24] == 0xac {  // OP_CHECKSIG
            return ScriptType::PubKeyHash;
        }
        
        // P2SH: OP_HASH160 <20-byte hash> OP_EQUAL
        if script_bytes.len() == 23 &&
           script_bytes[0] == 0xa9 && // OP_HASH160
           script_bytes[1] == 0x14 && // Push 20 bytes
           script_bytes[22] == 0x87 {  // OP_EQUAL
            return ScriptType::ScriptHash;
        }
        
        // P2WPKH: OP_0 <20-byte hash>
        if script_bytes.len() == 22 &&
           script_bytes[0] == 0x00 && // OP_0
           script_bytes[1] == 0x14 {  // Push 20 bytes
            return ScriptType::WitnessV0PubKeyHash;
        }
        
        // P2WSH: OP_0 <32-byte hash>
        if script_bytes.len() == 34 &&
           script_bytes[0] == 0x00 && // OP_0
           script_bytes[1] == 0x20 {  // Push 32 bytes
            return ScriptType::WitnessV0ScriptHash;
        }
        
        // P2TR: OP_1 <32-byte x-only pubkey>
        if script_bytes.len() == 34 &&
           script_bytes[0] == 0x51 && // OP_1
           script_bytes[1] == 0x20 {  // Push 32 bytes
            return ScriptType::WitnessV1Taproot;
        }
        
        // P2PK: <pubkey> OP_CHECKSIG
        if (script_bytes.len() == 35 || script_bytes.len() == 67) &&
           (script_bytes[0] == 0x21 || script_bytes[0] == 0x41) && // Push 33 or 65 bytes
           script_bytes[script_bytes.len() - 1] == 0xac {          // OP_CHECKSIG
            return ScriptType::PubKey;
        }
        
        // OP_RETURN: OP_RETURN <data>
        if !script_bytes.is_empty() && script_bytes[0] == 0x6a { // OP_RETURN
            return ScriptType::NullData;
        }
        
        // Check for multisig pattern
        if Self::is_multisig(script) {
            return ScriptType::MultiSig;
        }
        
        // Default to non-standard
        ScriptType::NonStandard
    }
    
    /// Create a P2PKH script from a public key hash
    pub fn create_p2pkh(pubkey_hash: &[u8]) -> Result<ScriptBuf, ScriptClassifyError> {
        if pubkey_hash.len() != 20 {
            return Err(ScriptClassifyError::InvalidFormat(
                "P2PKH requires a 20-byte pubkey hash".to_string()
            ));
        }
        
        // OP_DUP OP_HASH160 <pubkey_hash> OP_EQUALVERIFY OP_CHECKSIG
        let mut script = Vec::with_capacity(25);
        script.push(0x76); // OP_DUP
        script.push(0xa9); // OP_HASH160
        script.push(0x14); // Push 20 bytes
        script.extend_from_slice(pubkey_hash);
        script.push(0x88); // OP_EQUALVERIFY
        script.push(0xac); // OP_CHECKSIG
        
        Ok(ScriptBuf::from(script))
    }
    
    /// Create a P2SH script from a script hash
    pub fn create_p2sh(script_hash: &[u8]) -> Result<ScriptBuf, ScriptClassifyError> {
        if script_hash.len() != 20 {
            return Err(ScriptClassifyError::InvalidFormat(
                "P2SH requires a 20-byte script hash".to_string()
            ));
        }
        
        // OP_HASH160 <script_hash> OP_EQUAL
        let mut script = Vec::with_capacity(23);
        script.push(0xa9); // OP_HASH160
        script.push(0x14); // Push 20 bytes
        script.extend_from_slice(script_hash);
        script.push(0x87); // OP_EQUAL
        
        Ok(ScriptBuf::from(script))
    }
    
    /// Create a P2WPKH script from a public key hash
    pub fn create_p2wpkh(pubkey_hash: &[u8]) -> Result<ScriptBuf, ScriptClassifyError> {
        if pubkey_hash.len() != 20 {
            return Err(ScriptClassifyError::InvalidFormat(
                "P2WPKH requires a 20-byte pubkey hash".to_string()
            ));
        }
        
        // OP_0 <pubkey_hash>
        let mut script = Vec::with_capacity(22);
        script.push(0x00); // OP_0
        script.push(0x14); // Push 20 bytes
        script.extend_from_slice(pubkey_hash);
        
        Ok(ScriptBuf::from(script))
    }
    
    /// Create a P2WSH script from a witness script hash
    pub fn create_p2wsh(witness_script_hash: &[u8]) -> Result<ScriptBuf, ScriptClassifyError> {
        if witness_script_hash.len() != 32 {
            return Err(ScriptClassifyError::InvalidFormat(
                "P2WSH requires a 32-byte witness script hash".to_string()
            ));
        }
        
        // OP_0 <witness_script_hash>
        let mut script = Vec::with_capacity(34);
        script.push(0x00); // OP_0
        script.push(0x20); // Push 32 bytes
        script.extend_from_slice(witness_script_hash);
        
        Ok(ScriptBuf::from(script))
    }
    
    /// Create a P2TR (Taproot) script from an x-only public key
    pub fn create_p2tr(x_only_pubkey: &[u8]) -> Result<ScriptBuf, ScriptClassifyError> {
        if x_only_pubkey.len() != 32 {
            return Err(ScriptClassifyError::InvalidFormat(
                "P2TR requires a 32-byte x-only pubkey".to_string()
            ));
        }
        
        // OP_1 <x_only_pubkey>
        let mut script = Vec::with_capacity(34);
        script.push(0x51); // OP_1
        script.push(0x20); // Push 32 bytes
        script.extend_from_slice(x_only_pubkey);
        
        Ok(ScriptBuf::from(script))
    }
    
    /// Create an OP_RETURN script with data
    pub fn create_op_return(data: &[u8]) -> Result<ScriptBuf, ScriptClassifyError> {
        if data.len() > 80 {
            return Err(ScriptClassifyError::ScriptTooLarge);
        }
        
        // OP_RETURN <data>
        let mut script = Vec::new();
        script.push(0x6a); // OP_RETURN
        
        // Encode the data push correctly based on size
        if data.len() <= 75 {
            script.push(data.len() as u8); // Direct push
        } else {
            script.push(0x4c); // OP_PUSHDATA1
            script.push(data.len() as u8);
        }
        
        script.extend_from_slice(data);
        
        Ok(ScriptBuf::from(script))
    }
    
    /// Create a multisig script
    pub fn create_multisig(required: usize, pubkeys: &[&[u8]]) -> Result<ScriptBuf, ScriptClassifyError> {
        if required == 0 || required > pubkeys.len() || pubkeys.len() > MAX_PUBKEYS_PER_MULTISIG {
            return Err(ScriptClassifyError::InvalidFormat(format!(
                "Multisig requires 1-{} signatures from 1-{} keys",
                MAX_PUBKEYS_PER_MULTISIG, MAX_PUBKEYS_PER_MULTISIG
            )));
        }
        
        // Start with an OP_m
        let mut script = Vec::new();
        script.push(0x50 + required as u8); // OP_1 through OP_16
        
        // Add each public key
        for pubkey in pubkeys {
            if pubkey.len() != 33 && pubkey.len() != 65 {
                return Err(ScriptClassifyError::InvalidPublicKey(
                    "Invalid public key length".to_string()
                ));
            }
            
            script.push(pubkey.len() as u8);
            script.extend_from_slice(pubkey);
        }
        
        // Add OP_n and OP_CHECKMULTISIG
        script.push(0x50 + pubkeys.len() as u8); // OP_1 through OP_16
        script.push(0xae); // OP_CHECKMULTISIG
        
        Ok(ScriptBuf::from(script))
    }
    
    /// Check if a script is a multisig script
    pub fn is_multisig(script: &Script) -> bool {
        let script_bytes = script.as_bytes();
        
        // Minimum multisig script is 1-of-1: OP_1 <pubkey> OP_1 OP_CHECKMULTISIG
        if script_bytes.len() < 37 {
            return false;
        }
        
        // Check first and last opcodes
        let first_op = script_bytes[0];
        let last_op = script_bytes[script_bytes.len() - 1];
        
        if first_op < 0x51 || first_op > 0x60 || last_op != 0xae {
            return false;
        }
        
        // Check the second-to-last opcode
        let second_to_last = script_bytes[script_bytes.len() - 2];
        if second_to_last < 0x51 || second_to_last > 0x60 {
            return false;
        }
        
        // Verify n >= m
        let m = first_op - 0x50;
        let n = second_to_last - 0x50;
        
        m <= n
    }
    
    /// Extract public key hash from a P2PKH script
    pub fn extract_p2pkh_hash(script: &Script) -> Option<Vec<u8>> {
        let script_bytes = script.as_bytes();
        
        if Self::classify_script(script) == ScriptType::PubKeyHash {
            // Extract the 20-byte hash from OP_DUP OP_HASH160 <20-byte hash> OP_EQUALVERIFY OP_CHECKSIG
            Some(script_bytes[3..23].to_vec())
        } else {
            None
        }
    }
    
    /// Extract script hash from a P2SH script
    pub fn extract_p2sh_hash(script: &Script) -> Option<Vec<u8>> {
        let script_bytes = script.as_bytes();
        
        if Self::classify_script(script) == ScriptType::ScriptHash {
            // Extract the 20-byte hash from OP_HASH160 <20-byte hash> OP_EQUAL
            Some(script_bytes[2..22].to_vec())
        } else {
            None
        }
    }
    
    /// Extract public key hash from a P2WPKH script
    pub fn extract_p2wpkh_hash(script: &Script) -> Option<Vec<u8>> {
        let script_bytes = script.as_bytes();
        
        if Self::classify_script(script) == ScriptType::WitnessV0PubKeyHash {
            // Extract the 20-byte hash from OP_0 <20-byte hash>
            Some(script_bytes[2..22].to_vec())
        } else {
            None
        }
    }
    
    /// Extract witness script hash from a P2WSH script
    pub fn extract_p2wsh_hash(script: &Script) -> Option<Vec<u8>> {
        let script_bytes = script.as_bytes();
        
        if Self::classify_script(script) == ScriptType::WitnessV0ScriptHash {
            // Extract the 32-byte hash from OP_0 <32-byte hash>
            Some(script_bytes[2..34].to_vec())
        } else {
            None
        }
    }
    
    /// Extract x-only public key from a P2TR script
    pub fn extract_p2tr_pubkey(script: &Script) -> Option<Vec<u8>> {
        let script_bytes = script.as_bytes();
        
        if Self::classify_script(script) == ScriptType::WitnessV1Taproot {
            // Extract the 32-byte x-only pubkey from OP_1 <32-byte pubkey>
            Some(script_bytes[2..34].to_vec())
        } else {
            None
        }
    }
    
    /// Extract data from an OP_RETURN script
    pub fn extract_op_return_data(script: &Script) -> Option<Vec<u8>> {
        let script_bytes = script.as_bytes();
        
        if Self::classify_script(script) != ScriptType::NullData || script_bytes.is_empty() {
            return None;
        }
        
        // Skip OP_RETURN
        let mut pos = 1;
        
        if pos >= script_bytes.len() {
            return Some(vec![]); // Empty OP_RETURN
        }
        
        // Handle different push sizes
        let push_byte = script_bytes[pos];
        pos += 1;
        
        // Direct small push
        if push_byte <= 75 {
            let data_len = push_byte as usize;
            if pos + data_len <= script_bytes.len() {
                return Some(script_bytes[pos..pos + data_len].to_vec());
            }
        } 
        // OP_PUSHDATA1
        else if push_byte == 0x4c && pos < script_bytes.len() {
            let data_len = script_bytes[pos] as usize;
            pos += 1;
            
            if pos + data_len <= script_bytes.len() {
                return Some(script_bytes[pos..pos + data_len].to_vec());
            }
        }
        
        None
    }
    
    /// Check if a script is a standard script
    pub fn is_standard(script: &Script) -> bool {
        match Self::classify_script(script) {
            ScriptType::NonStandard => false,
            _ => true,
        }
    }
    
    /// Get the minimum required signatures for the script type
    pub fn get_sig_op_count(script: &Script) -> usize {
        match Self::classify_script(script) {
            ScriptType::PubKey => 1,
            ScriptType::PubKeyHash => 1,
            ScriptType::MultiSig => {
                // Extract the required signatures (m) from m-of-n
                if let Some(m) = script.as_bytes().first() {
                    // m is encoded as OP_m (0x51 + m - 1)
                    if *m >= 0x51 && *m <= 0x60 {
                        return (*m - 0x50) as usize;
                    }
                }
                0
            },
            ScriptType::WitnessV0PubKeyHash => 1,
            ScriptType::WitnessV1Taproot => 1,
            _ => 0,
        }
    }
}

/// Utility functions for working with Bitcoin witness programs
pub struct WitnessProgram;

impl WitnessProgram {
    /// Extract the witness program version and program from a script
    pub fn extract(script: &Script) -> Option<(u8, Vec<u8>)> {
        let script_bytes = script.as_bytes();
        
        // Must have at least version and length bytes
        if script_bytes.len() < 2 {
            return None;
        }
        
        // First byte is the version opcode
        let version_opcode = script_bytes[0];
        
        // Only support OP_0 through OP_16 for version
        if version_opcode > 0x60 {
            return None;
        }
        
        // Version 0 is encoded as OP_0 (0x00)
        // Version 1-16 is encoded as OP_1 (0x51) through OP_16 (0x60)
        let version = if version_opcode == 0x00 {
            0
        } else if version_opcode >= 0x51 && version_opcode <= 0x60 {
            version_opcode - 0x50
        } else {
            return None;
        };
        
        // Second byte is the length of the program
        let program_len = script_bytes[1] as usize;
        
        // Check if we have enough bytes
        if script_bytes.len() != 2 + program_len {
            return None;
        }
        
        // For version 0, only allow 20-byte and 32-byte programs
        if version == 0 && program_len != 20 && program_len != 32 {
            return None;
        }
        
        // Extract the program
        let program = script_bytes[2..].to_vec();
        
        Some((version, program))
    }
    
    /// Create a witness program script
    pub fn create(version: u8, program: &[u8]) -> Result<ScriptBuf, ScriptClassifyError> {
        if version > 16 {
            return Err(ScriptClassifyError::InvalidFormat(
                "Witness version must be 0-16".to_string()
            ));
        }
        
        // Version 0 only allows 20-byte and 32-byte programs
        if version == 0 && program.len() != 20 && program.len() != 32 {
            return Err(ScriptClassifyError::InvalidFormat(
                "Version 0 witness program must be 20 or 32 bytes".to_string()
            ));
        }
        
        let mut script = Vec::new();
        
        // Version 0 is encoded as OP_0
        if version == 0 {
            script.push(0x00);
        } 
        // Version 1-16 is encoded as OP_1 through OP_16
        else {
            script.push(0x50 + version);
        }
        
        // Push the program with appropriate length
        script.push(program.len() as u8);
        script.extend_from_slice(program);
        
        Ok(ScriptBuf::from(script))
    }
    
    /// Check if a script is a witness program
    pub fn is_witness_program(script: &Script) -> bool {
        Self::extract(script).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_script_classification() {
        // P2PKH test
        let p2pkh_script = StandardScripts::create_p2pkh(&[0; 20]).unwrap();
        assert_eq!(StandardScripts::classify_script(&p2pkh_script), ScriptType::PubKeyHash);
        
        // P2SH test
        let p2sh_script = StandardScripts::create_p2sh(&[0; 20]).unwrap();
        assert_eq!(StandardScripts::classify_script(&p2sh_script), ScriptType::ScriptHash);
        
        // P2WPKH test
        let p2wpkh_script = StandardScripts::create_p2wpkh(&[0; 20]).unwrap();
        assert_eq!(StandardScripts::classify_script(&p2wpkh_script), ScriptType::WitnessV0PubKeyHash);
        
        // P2WSH test
        let p2wsh_script = StandardScripts::create_p2wsh(&[0; 32]).unwrap();
        assert_eq!(StandardScripts::classify_script(&p2wsh_script), ScriptType::WitnessV0ScriptHash);
        
        // P2TR test
        let p2tr_script = StandardScripts::create_p2tr(&[0; 32]).unwrap();
        assert_eq!(StandardScripts::classify_script(&p2tr_script), ScriptType::WitnessV1Taproot);
    }
    
    #[test]
    fn test_witness_program() {
        // P2WPKH
        let p2wpkh_script = StandardScripts::create_p2wpkh(&[0; 20]).unwrap();
        let (version, program) = WitnessProgram::extract(&p2wpkh_script).unwrap();
        assert_eq!(version, 0);
        assert_eq!(program.len(), 20);
        
        // P2TR
        let p2tr_script = StandardScripts::create_p2tr(&[0; 32]).unwrap();
        let (version, program) = WitnessProgram::extract(&p2tr_script).unwrap();
        assert_eq!(version, 1);
        assert_eq!(program.len(), 32);
    }
}
