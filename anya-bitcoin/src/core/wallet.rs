//! Bitcoin wallet functionality
//!
//! This module contains wallet-related functionality for managing
//! Bitcoin transactions and UTXOs.

use bitcoin::{Address, Txid};
use std::collections::HashMap;

/// Options for transaction creation
#[derive(Debug, Clone)]
pub struct TxOptions {
    /// Fee rate in satoshis per byte
    pub fee_rate: f64,
    
    /// Replace-by-fee flag
    pub rbf: bool,
    
    /// Transaction locktime
    pub locktime: Option<u32>,
    
    /// Custom transaction inputs
    pub inputs: Option<Vec<Txid>>,
    
    /// Custom change address
    pub change_address: Option<Address>,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl Default for TxOptions {
    fn default() -> Self {
        Self {
            fee_rate: 1.0,
            rbf: true,
            locktime: None,
            inputs: None,
            change_address: None,
            metadata: HashMap::new(),
        }
    }
} 