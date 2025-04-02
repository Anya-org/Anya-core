//! Transaction mempool implementation
//! This module manages the in-memory pool of unconfirmed transactions

use log::info;
use std::collections::HashMap;
use bitcoin::Txid;

/// A simple mempool implementation for Bitcoin transactions
pub struct Mempool {
    /// Transactions in the mempool
    transactions: HashMap<Txid, Vec<u8>>,
}

impl Mempool {
    /// Create a new empty mempool
    pub fn new() -> Self {
        info!("Creating new mempool");
        Self {
            transactions: HashMap::new(),
        }
    }
    
    /// Get the number of transactions in the mempool
    pub fn size(&self) -> usize {
        self.transactions.len()
    }
}
