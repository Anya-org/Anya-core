use std::error::Error;
// Migrated from OPSource to anya-core
// This file was automatically migrated as part of the Rust-only implementation
// Original file: C:\Users\bmokoka\Downloads\OPSource\src\bitcoin\adapter.rs
// src/bitcoin/adapter.rs
//
// This module provides adapter functionality for Bitcoin implementations
// following the hexagonal architecture pattern.

use std::sync::Arc;
use crate::bitcoin::interface::{
    BitcoinInterface, BitcoinError, BitcoinResult, BitcoinTransaction,
    BitcoinAddress, AddressType, TransactionInput, TransactionOutput,
    BlockHeader, BitcoinImplementationType
};
use crate::bitcoin::config::Config;
use crate::bitcoin::rust::RustBitcoinImplementation;

/// Bitcoin adapter for Bitcoin implementation
pub struct BitcoinAdapter {
    /// Configuration
    config: Arc<Config>,
    
    /// Rust implementation
    implementation: Arc<dyn BitcoinInterface>,
}

impl BitcoinAdapter {
    /// Create a new Bitcoin adapter
    pub fn new(config: Arc<Config>) -> Self  -> Result<(), Box<dyn Error>> {
        // Create the Rust implementation
        let implementation = Arc::new(RustBitcoinImplementation::new(&config)) as Arc<dyn BitcoinInterface>;
        
        Self {
            config,
            implementation,
        }
    }
    
    /// Get the Bitcoin implementation
    pub fn get_implementation(&self) -> Arc<dyn BitcoinInterface>  -> Result<(), Box<dyn Error>> {
        self.implementation.clone()
    }
}

impl BitcoinInterface for BitcoinAdapter {
    fn get_transaction(&self, txid: &str) -> BitcoinResult<BitcoinTransaction>  -> Result<(), Box<dyn Error>> {
        self.implementation.get_transaction(txid)
    }
    
    fn get_block(&self, hash: &str) -> BitcoinResult<Vec<BitcoinTransaction>>  -> Result<(), Box<dyn Error>> {
        self.implementation.get_block(hash)
    }
    
    fn get_block_height(&self) -> BitcoinResult<u32>  -> Result<(), Box<dyn Error>> {
        self.implementation.get_block_height()
    }
    
    fn generate_address(&self, address_type: AddressType) -> BitcoinResult<BitcoinAddress>  -> Result<(), Box<dyn Error>> {
        self.implementation.generate_address(address_type)
    }
    
    fn create_transaction(
        &self,
        outputs: Vec<(String, u64)>,
        fee_rate: u64,
    ) -> BitcoinResult<BitcoinTransaction>  -> Result<(), Box<dyn Error>> {
        self.implementation.create_transaction(outputs, fee_rate)
    }
    
    fn broadcast_transaction(&self, transaction: &BitcoinTransaction) -> BitcoinResult<String>  -> Result<(), Box<dyn Error>> {
        self.implementation.broadcast_transaction(transaction)
    }
    
    fn get_balance(&self) -> BitcoinResult<u64>  -> Result<(), Box<dyn Error>> {
        self.implementation.get_balance()
    }
    
    fn estimate_fee(&self, target_blocks: u8) -> BitcoinResult<u64>  -> Result<(), Box<dyn Error>> {
        self.implementation.estimate_fee(target_blocks)
    }
    
    fn implementation_type(&self) -> BitcoinImplementationType  -> Result<(), Box<dyn Error>> {
        BitcoinImplementationType::Rust
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_adapter_initialization()  -> Result<(), Box<dyn Error>> {
        let config = Arc::new(Config::default());
        let adapter = BitcoinAdapter::new(config);
        
        // Check that we can get the implementation
        let implementation = adapter.get_implementation();
        assert_eq!(implementation.implementation_type(), BitcoinImplementationType::Rust);
        
        // Check the default implementation type
        assert_eq!(adapter.implementation_type(), BitcoinImplementationType::Rust);
    }
} 

