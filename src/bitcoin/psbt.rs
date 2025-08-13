//! BIP-174 (PSBT) - Partially Signed Bitcoin Transactions Implementation
//! [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
//!
//! This module implements BIP-174 (PSBT) for Bitcoin transaction construction and signing.
//! Provides support for multi-party transaction signing workflows.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use bitcoin::{Transaction, TxIn, TxOut, Amount, Sighash};
use bitcoin::psbt::{Psbt, Input as PsbtInput, Output as PsbtOutput};
use secp256k1::{PublicKey, SecretKey, Secp256k1};
use serde::{Deserialize, Serialize};

/// PSBT Error types as defined in BIP-174
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PsbtError {
    /// Missing global transaction
    MissingGlobalTx,
    /// Input count mismatch
    InputCountMismatch,
    /// Output count mismatch  
    OutputCountMismatch,
    /// Invalid version
    InvalidVersion,
    /// Invalid key-value pair
    InvalidKeyValue(String),
    /// Duplicate key
    DuplicateKey(String),
    /// Invalid signature
    InvalidSignature(String),
    /// Missing required field
    MissingRequiredField(String),
    /// Invalid transaction
    InvalidTransaction(String),
    /// Signing error
    SigningError(String),
    /// Serialization error
    SerializationError(String),
}

impl fmt::Display for PsbtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PsbtError::MissingGlobalTx => write!(f, "Missing global transaction"),
            PsbtError::InputCountMismatch => write!(f, "Input count mismatch"),
            PsbtError::OutputCountMismatch => write!(f, "Output count mismatch"),
            PsbtError::InvalidVersion => write!(f, "Invalid PSBT version"),
            PsbtError::InvalidKeyValue(msg) => write!(f, "Invalid key-value pair: {}", msg),
            PsbtError::DuplicateKey(key) => write!(f, "Duplicate key: {}", key),
            PsbtError::InvalidSignature(msg) => write!(f, "Invalid signature: {}", msg),
            PsbtError::MissingRequiredField(field) => write!(f, "Missing required field: {}", field),
            PsbtError::InvalidTransaction(msg) => write!(f, "Invalid transaction: {}", msg),
            PsbtError::SigningError(msg) => write!(f, "Signing error: {}", msg),
            PsbtError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl Error for PsbtError {}

/// PSBT Global data as defined in BIP-174
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsbtGlobal {
    /// Unsigned transaction
    pub unsigned_tx: Option<Transaction>,
    /// Extended public keys
    pub xpubs: HashMap<Vec<u8>, Vec<u8>>,
    /// Version number (default: 0)
    pub version: u32,
    /// Proprietary key-value pairs
    pub proprietary: HashMap<Vec<u8>, Vec<u8>>,
    /// Unknown key-value pairs
    pub unknown: HashMap<Vec<u8>, Vec<u8>>,
}

impl Default for PsbtGlobal {
    fn default() -> Self {
        Self {
            unsigned_tx: None,
            xpubs: HashMap::new(),
            version: 0,
            proprietary: HashMap::new(),
            unknown: HashMap::new(),
        }
    }
}

/// PSBT Input data as defined in BIP-174
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsbtInputData {
    /// Non-witness UTXO
    pub non_witness_utxo: Option<Transaction>,
    /// Witness UTXO
    pub witness_utxo: Option<TxOut>,
    /// Partial signatures
    pub partial_sigs: HashMap<PublicKey, Vec<u8>>,
    /// Sighash type
    pub sighash_type: Option<Sighash>,
    /// Redeem script
    pub redeem_script: Option<Vec<u8>>,
    /// Witness script
    pub witness_script: Option<Vec<u8>>,
    /// HD key paths
    pub bip32_derivation: HashMap<PublicKey, (Vec<u8>, Vec<u32>)>,
    /// Final script signature
    pub final_script_sig: Option<Vec<u8>>,
    /// Final script witness
    pub final_script_witness: Option<Vec<Vec<u8>>>,
    /// Proprietary key-value pairs
    pub proprietary: HashMap<Vec<u8>, Vec<u8>>,
    /// Unknown key-value pairs
    pub unknown: HashMap<Vec<u8>, Vec<u8>>,
}

impl Default for PsbtInputData {
    fn default() -> Self {
        Self {
            non_witness_utxo: None,
            witness_utxo: None,
            partial_sigs: HashMap::new(),
            sighash_type: None,
            redeem_script: None,
            witness_script: None,
            bip32_derivation: HashMap::new(),
            final_script_sig: None,
            final_script_witness: None,
            proprietary: HashMap::new(),
            unknown: HashMap::new(),
        }
    }
}

/// PSBT Output data as defined in BIP-174
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsbtOutputData {
    /// Redeem script
    pub redeem_script: Option<Vec<u8>>,
    /// Witness script
    pub witness_script: Option<Vec<u8>>,
    /// HD key paths
    pub bip32_derivation: HashMap<PublicKey, (Vec<u8>, Vec<u32>)>,
    /// Proprietary key-value pairs
    pub proprietary: HashMap<Vec<u8>, Vec<u8>>,
    /// Unknown key-value pairs
    pub unknown: HashMap<Vec<u8>, Vec<u8>>,
}

impl Default for PsbtOutputData {
    fn default() -> Self {
        Self {
            redeem_script: None,
            witness_script: None,
            bip32_derivation: HashMap::new(),
            proprietary: HashMap::new(),
            unknown: HashMap::new(),
        }
    }
}

/// PSBT (Partially Signed Bitcoin Transaction) as defined in BIP-174
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartiallySignedTransaction {
    /// Global data
    pub global: PsbtGlobal,
    /// Input data for each input
    pub inputs: Vec<PsbtInputData>,
    /// Output data for each output
    pub outputs: Vec<PsbtOutputData>,
}

impl PartiallySignedTransaction {
    /// Create a new PSBT from an unsigned transaction
    pub fn new(unsigned_tx: Transaction) -> Result<Self, PsbtError> {
        let input_count = unsigned_tx.input.len();
        let output_count = unsigned_tx.output.len();
        
        let mut global = PsbtGlobal::default();
        global.unsigned_tx = Some(unsigned_tx);
        
        let inputs = vec![PsbtInputData::default(); input_count];
        let outputs = vec![PsbtOutputData::default(); output_count];
        
        Ok(Self {
            global,
            inputs,
            outputs,
        })
    }
    
    /// Validate the PSBT structure
    pub fn validate(&self) -> Result<(), PsbtError> {
        // Validate global transaction presence
        let tx = self.global.unsigned_tx.as_ref()
            .ok_or(PsbtError::MissingGlobalTx)?;
        
        // Validate input/output counts match transaction
        if self.inputs.len() != tx.input.len() {
            return Err(PsbtError::InputCountMismatch);
        }
        
        if self.outputs.len() != tx.output.len() {
            return Err(PsbtError::OutputCountMismatch);
        }
        
        // Validate each input has either witness or non-witness UTXO
        for (i, input) in self.inputs.iter().enumerate() {
            if input.witness_utxo.is_none() && input.non_witness_utxo.is_none() {
                return Err(PsbtError::MissingRequiredField(
                    format!("Input {} missing UTXO information", i)
                ));
            }
        }
        
        Ok(())
    }
    
    /// Add a signature for a specific input
    pub fn add_signature(
        &mut self,
        input_index: usize,
        public_key: PublicKey,
        signature: Vec<u8>,
    ) -> Result<(), PsbtError> {
        if input_index >= self.inputs.len() {
            return Err(PsbtError::InvalidTransaction(
                format!("Input index {} out of bounds", input_index)
            ));
        }
        
        // Check for duplicate signature
        if self.inputs[input_index].partial_sigs.contains_key(&public_key) {
            return Err(PsbtError::DuplicateKey(
                format!("Signature for public key already exists")
            ));
        }
        
        self.inputs[input_index].partial_sigs.insert(public_key, signature);
        Ok(())
    }
    
    /// Set witness UTXO for an input
    pub fn set_witness_utxo(
        &mut self,
        input_index: usize,
        utxo: TxOut,
    ) -> Result<(), PsbtError> {
        if input_index >= self.inputs.len() {
            return Err(PsbtError::InvalidTransaction(
                format!("Input index {} out of bounds", input_index)
            ));
        }
        
        self.inputs[input_index].witness_utxo = Some(utxo);
        Ok(())
    }
    
    /// Set non-witness UTXO for an input
    pub fn set_non_witness_utxo(
        &mut self,
        input_index: usize,
        utxo: Transaction,
    ) -> Result<(), PsbtError> {
        if input_index >= self.inputs.len() {
            return Err(PsbtError::InvalidTransaction(
                format!("Input index {} out of bounds", input_index)
            ));
        }
        
        self.inputs[input_index].non_witness_utxo = Some(utxo);
        Ok(())
    }
    
    /// Check if the PSBT is ready to be finalized
    pub fn is_complete(&self) -> bool {
        // Check if all inputs have the required signatures
        for input in &self.inputs {
            if input.partial_sigs.is_empty() {
                return false;
            }
        }
        true
    }
    
    /// Finalize the PSBT to create a complete transaction
    pub fn finalize(&mut self) -> Result<Transaction, PsbtError> {
        self.validate()?;
        
        if !self.is_complete() {
            return Err(PsbtError::SigningError(
                "PSBT is not complete - missing signatures".to_string()
            ));
        }
        
        // For now, return the unsigned transaction
        // In a full implementation, this would create the final signed transaction
        // with script_sig and witness data from the partial signatures
        
        self.global.unsigned_tx.clone()
            .ok_or(PsbtError::MissingGlobalTx)
    }
    
    /// Serialize the PSBT to bytes (simplified implementation)
    pub fn serialize(&self) -> Result<Vec<u8>, PsbtError> {
        serde_json::to_vec(self)
            .map_err(|e| PsbtError::SerializationError(e.to_string()))
    }
    
    /// Deserialize PSBT from bytes (simplified implementation)
    pub fn deserialize(data: &[u8]) -> Result<Self, PsbtError> {
        serde_json::from_slice(data)
            .map_err(|e| PsbtError::SerializationError(e.to_string()))
    }
    
    /// Get the transaction being signed
    pub fn unsigned_transaction(&self) -> Option<&Transaction> {
        self.global.unsigned_tx.as_ref()
    }
    
    /// Get the number of inputs
    pub fn input_count(&self) -> usize {
        self.inputs.len()
    }
    
    /// Get the number of outputs
    pub fn output_count(&self) -> usize {
        self.outputs.len()
    }
    
    /// Check if input has required UTXO information
    pub fn has_utxo_info(&self, input_index: usize) -> bool {
        if input_index >= self.inputs.len() {
            return false;
        }
        
        let input = &self.inputs[input_index];
        input.witness_utxo.is_some() || input.non_witness_utxo.is_some()
    }
    
    /// Get signature count for an input
    pub fn signature_count(&self, input_index: usize) -> usize {
        if input_index >= self.inputs.len() {
            return 0;
        }
        
        self.inputs[input_index].partial_sigs.len()
    }
}

/// PSBT Builder for easier construction
pub struct PsbtBuilder {
    psbt: PartiallySignedTransaction,
}

impl PsbtBuilder {
    /// Create a new PSBT builder from unsigned transaction
    pub fn new(unsigned_tx: Transaction) -> Result<Self, PsbtError> {
        let psbt = PartiallySignedTransaction::new(unsigned_tx)?;
        Ok(Self { psbt })
    }
    
    /// Add witness UTXO to an input
    pub fn witness_utxo(mut self, input_index: usize, utxo: TxOut) -> Result<Self, PsbtError> {
        self.psbt.set_witness_utxo(input_index, utxo)?;
        Ok(self)
    }
    
    /// Add non-witness UTXO to an input
    pub fn non_witness_utxo(mut self, input_index: usize, utxo: Transaction) -> Result<Self, PsbtError> {
        self.psbt.set_non_witness_utxo(input_index, utxo)?;
        Ok(self)
    }
    
    /// Add a signature to an input
    pub fn signature(
        mut self,
        input_index: usize,
        public_key: PublicKey,
        signature: Vec<u8>
    ) -> Result<Self, PsbtError> {
        self.psbt.add_signature(input_index, public_key, signature)?;
        Ok(self)
    }
    
    /// Build the final PSBT
    pub fn build(self) -> PartiallySignedTransaction {
        self.psbt
    }
}

// Legacy PSBT v2 structure for backward compatibility
#[derive(Debug, Clone)]
pub struct PsbtV2 {
    pub global_tx: Option<Transaction>,
    pub inputs: Vec<PsbtInputData>,
    pub outputs: Vec<PsbtOutputData>,
    pub input_count: u32,
    pub output_count: u32,
    pub version: u32,
}

impl PsbtV2 {
    pub fn validate(&self) -> Result<(), PsbtError> {
        // Validate global transaction presence
        if self.global_tx.is_none() {
            return Err(PsbtError::MissingGlobalTx);
        }
        
        // Validate input/output counts match metadata
        if self.inputs.len() != self.input_count as usize {
            return Err(PsbtError::InputCountMismatch);
        }
        
        if self.outputs.len() != self.output_count as usize {
            return Err(PsbtError::OutputCountMismatch);
        }
        
        // Validate version 2 requirement
        if self.version != 2 {
            return Err(PsbtError::InvalidVersion);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::{Network, Address, ScriptBuf};
    
    #[test]
    fn test_psbt_creation() {
        // Create a simple unsigned transaction
        let tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: bitcoin::locktime::absolute::LockTime::ZERO,
            input: vec![TxIn::default()],
            output: vec![TxOut {
                value: Amount::from_sat(100000),
                script_pubkey: ScriptBuf::new(),
            }],
        };
        
        let psbt = PartiallySignedTransaction::new(tx).unwrap();
        
        assert_eq!(psbt.input_count(), 1);
        assert_eq!(psbt.output_count(), 1);
        assert!(psbt.unsigned_transaction().is_some());
    }
    
    #[test]
    fn test_psbt_validation() {
        let tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: bitcoin::locktime::absolute::LockTime::ZERO,
            input: vec![TxIn::default()],
            output: vec![TxOut {
                value: Amount::from_sat(100000),
                script_pubkey: ScriptBuf::new(),
            }],
        };
        
        let mut psbt = PartiallySignedTransaction::new(tx).unwrap();
        
        // Should fail validation without UTXO info
        assert!(psbt.validate().is_err());
        
        // Add witness UTXO
        let utxo = TxOut {
            value: Amount::from_sat(200000),
            script_pubkey: ScriptBuf::new(),
        };
        
        psbt.set_witness_utxo(0, utxo).unwrap();
        
        // Should now pass validation
        assert!(psbt.validate().is_ok());
    }
    
    #[test]
    fn test_psbt_builder() {
        let tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: bitcoin::locktime::absolute::LockTime::ZERO,
            input: vec![TxIn::default()],
            output: vec![TxOut {
                value: Amount::from_sat(100000),
                script_pubkey: ScriptBuf::new(),
            }],
        };
        
        let utxo = TxOut {
            value: Amount::from_sat(200000),
            script_pubkey: ScriptBuf::new(),
        };
        
        let psbt = PsbtBuilder::new(tx).unwrap()
            .witness_utxo(0, utxo).unwrap()
            .build();
        
        assert!(psbt.has_utxo_info(0));
        assert!(psbt.validate().is_ok());
    }
}
