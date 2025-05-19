use std::error::Error;
// Migrated from OPSource to anya-core
// This file was automatically migrated as part of the Rust-only implementation
// Original file: C:\Users\bmokoka\Downloads\OPSource\src\bitcoin\adapter.rs
// src/bitcoin/adapter.rs
//
// This module provides adapter functionality for Bitcoin implementations
// following the hexagonal architecture pattern.

use std::sync::Arc;
use std::error::Error;
use async_trait::async_trait;
use crate::bitcoin::interface::{
    BitcoinInterface, BitcoinError, BitcoinResult, Transaction,
    Address, AddressType, Block, BlockHeader, BitcoinImplementationType
};
use crate::bitcoin::config::BitcoinConfig;

/// Bitcoin adapter for Bitcoin implementation
pub struct BitcoinAdapter {
    /// Configuration
    config: Arc<BitcoinConfig>,
    
    /// Rust implementation
    implementation: Arc<dyn BitcoinInterface>,
}

impl BitcoinAdapter {
    /// Create a new Bitcoin adapter
    /// [BPC-3] Complete real implementation as per BDF v2.5 standards
    pub async fn new(config: BitcoinConfig) -> Result<Self, Box<dyn Error>> {
        // Create a simple Rust implementation
        // Use the rust module implementation to avoid conflicts
        let implementation = Arc::new(crate::bitcoin::rust::RustBitcoinImplementation::new(&config)?) as Arc<dyn BitcoinInterface>;
        
        Ok(Self {
            config: Arc::new(config),
            implementation,
        })
    }
    
    /// Get the Bitcoin implementation
    pub fn get_implementation(&self) -> Arc<dyn BitcoinInterface> {
        self.implementation.clone()
    }
}

// [BPC-3] Implementation of BitcoinInterface following hexagonal architecture pattern
// [AIR-3] Complete real implementation as per BDF v2.5 standards
#[async_trait]
impl BitcoinInterface for BitcoinAdapter {
    async fn get_transaction(&self, txid: &str) -> BitcoinResult<Transaction> {
        self.implementation.get_transaction(txid).await
    }
    
    async fn get_block(&self, hash: &str) -> BitcoinResult<Block> {
        self.implementation.get_block(hash).await
    }
    
    async fn get_block_height(&self) -> BitcoinResult<u32> {
        self.implementation.get_block_height().await
    }
    
    async fn generate_address(&self, address_type: AddressType) -> BitcoinResult<Address> {
        self.implementation.generate_address(address_type).await
    }
    
    async fn create_transaction(
        &self,
        outputs: Vec<(String, u64)>,
        fee_rate: u64,
    ) -> BitcoinResult<Transaction> {
        self.implementation.create_transaction(outputs, fee_rate).await
    }
    
    async fn broadcast_transaction(&self, transaction: &Transaction) -> BitcoinResult<String> {
        self.implementation.broadcast_transaction(transaction).await
    }
    
    async fn get_balance(&self, address: &Address) -> BitcoinResult<u64> {
        self.implementation.get_balance(address).await
    }
    
    async fn estimate_fee(&self, target_blocks: u8) -> BitcoinResult<u64> {
        self.implementation.estimate_fee(target_blocks).await
    }
    
    async fn get_block_header(&self, hash: &str) -> BitcoinResult<BlockHeader> {
        self.implementation.get_block_header(hash).await
    }
    
    async fn verify_merkle_proof(&self, tx_hash: &str, block_header: &BlockHeader) -> BitcoinResult<bool> {
        self.implementation.verify_merkle_proof(tx_hash, block_header).await
    }
    
    async fn send_transaction(&self, tx: &Transaction) -> BitcoinResult<String> {
        self.implementation.send_transaction(tx).await
    }
    
    fn implementation_type(&self) -> BitcoinImplementationType {
        BitcoinImplementationType::Rust
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_adapter_initialization() -> Result<(), Box<dyn Error>> {
        let config = BitcoinConfig::default();
        let adapter = BitcoinAdapter::new(config).await?;
        
        // Check that we can get the implementation
        let implementation = adapter.get_implementation();
        assert_eq!(implementation.implementation_type(), BitcoinImplementationType::Rust);
        
        // Check the default implementation type
        assert_eq!(adapter.implementation_type(), BitcoinImplementationType::Rust);
        
        // [BPC-3] Return success result
        Ok(())
    }
} 

