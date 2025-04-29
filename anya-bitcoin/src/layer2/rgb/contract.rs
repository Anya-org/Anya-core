//! RGB contract functionality
//!
//! This module provides contract functionality for the RGB protocol.

use std::collections::HashMap;
use bitcoin::Txid;

/// RGB contract type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractType {
    /// Asset issuance contract
    Asset,
    /// Identity contract
    Identity,
    /// Custom contract
    Custom(String),
}

/// RGB contract witness
#[derive(Debug, Clone)]
pub struct Witness {
    /// Witness ID
    pub id: String,
    /// Witness data
    pub data: Vec<u8>,
    /// Witness signature
    pub signature: Vec<u8>,
}

/// RGB contract
#[derive(Debug, Clone)]
pub struct Contract {
    /// Contract ID
    pub id: String,
    /// Contract type
    pub contract_type: ContractType,
    /// Schema ID
    pub schema_id: String,
    /// Contract data
    pub data: HashMap<String, Vec<u8>>,
    /// Contract metadata
    pub metadata: HashMap<String, String>,
    /// Contract issued in transaction
    pub issuance_txid: Txid,
    /// Witnesses
    pub witnesses: Vec<Witness>,
}

/// RGB contract builder
pub struct ContractBuilder {
    contract_type: ContractType,
    schema_id: String,
    data: HashMap<String, Vec<u8>>,
    metadata: HashMap<String, String>,
    witnesses: Vec<Witness>,
}

impl ContractBuilder {
    /// Create a new contract builder
    pub fn new(contract_type: ContractType, schema_id: String) -> Self {
        Self {
            contract_type,
            schema_id,
            data: HashMap::new(),
            metadata: HashMap::new(),
            witnesses: Vec::new(),
        }
    }
    
    /// Add data to the contract
    pub fn with_data(mut self, key: &str, value: Vec<u8>) -> Self {
        self.data.insert(key.to_string(), value);
        self
    }
    
    /// Add metadata to the contract
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Add a witness to the contract
    pub fn with_witness(mut self, witness: Witness) -> Self {
        self.witnesses.push(witness);
        self
    }
    
    /// Build the contract
    pub fn build(self, issuance_txid: Txid) -> Contract {
        Contract {
            id: format!("contract:{}", issuance_txid),
            contract_type: self.contract_type,
            schema_id: self.schema_id,
            data: self.data,
            metadata: self.metadata,
            issuance_txid,
            witnesses: self.witnesses,
        }
    }
} 
