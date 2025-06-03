use std::error::Error;
// [AIR-3][AIS-3][BPC-3][RES-3] Bitcoin adapters module implementation
// This follows official Bitcoin Improvement Proposals (BIPs) standards for hexagonal architecture
use std::sync::Arc;
// [AIR-3][AIS-3][BPC-3][RES-3] Removed unused import: async_trait::async_trait

// [AIR-3][AIS-3][BPC-3][RES-3] Import Bitcoin interface types
// This follows official Bitcoin Improvement Proposals (BIPs) standards for type consistency
use crate::bitcoin::interface::{
// [AIR-3][AIS-3][BPC-3][RES-3] Removed unused import: BitcoinError
    BitcoinInterface, BitcoinResult, Transaction,
    Address, AddressType, Block, BlockHeader, BitcoinImplementationType
};
use crate::bitcoin::config::BitcoinConfig;

/// [AIR-3][AIS-3][BPC-3][RES-3] Bitcoin adapter for Bitcoin implementation
pub struct BitcoinAdapter {
    /// Configuration
    config: Arc<BitcoinConfig>,
    
    /// Implementation
    implementation: Arc<dyn BitcoinInterface>,
}

impl BitcoinAdapter {
    /// Create a new Bitcoin adapter
    pub async fn new(config: BitcoinConfig) -> Result<Self, Box<dyn Error>> {
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

/// Implementation of BitcoinInterface following hexagonal architecture pattern
/// [AIR-3][AIS-3][BPC-3][RES-3] Using async_trait for async interface implementation
#[async_trait::async_trait]
impl BitcoinInterface for BitcoinAdapter {
    /// [AIR-3][AIS-3][BPC-3][RES-3] Get transaction by ID
    async fn get_transaction(&self, txid: &str) -> BitcoinResult<Transaction> {
        self.implementation.get_transaction(txid).await
    }
    
    /// [AIR-3][AIS-3][BPC-3][RES-3] Get block by hash
    async fn get_block(&self, hash: &str) -> BitcoinResult<Block> {
        self.implementation.get_block(hash).await
    }
    
    /// [AIR-3][AIS-3][BPC-3][RES-3] Get current block height
    async fn get_block_height(&self) -> BitcoinResult<u32> {
        self.implementation.get_block_height().await
    }
    
    /// [AIR-3][AIS-3][BPC-3][RES-3] Generate address of specified type
    async fn generate_address(&self, address_type: AddressType) -> BitcoinResult<Address> {
        self.implementation.generate_address(address_type).await
    }
    
    /// [AIR-3][AIS-3][BPC-3][RES-3] Create transaction with outputs and fee rate
    async fn create_transaction(
        &self,
        outputs: Vec<(String, u64)>,
        fee_rate: u64,
    ) -> BitcoinResult<Transaction> {
        self.implementation.create_transaction(outputs, fee_rate).await
    }
    
    /// [AIR-3][AIS-3][BPC-3][RES-3] Broadcast transaction to network
    async fn broadcast_transaction(&self, transaction: &Transaction) -> BitcoinResult<String> {
        self.implementation.broadcast_transaction(transaction).await
    }
    
    /// [AIR-3][AIS-3][BPC-3][RES-3] Get balance for address
    async fn get_balance(&self, address: &Address) -> BitcoinResult<u64> {
        self.implementation.get_balance(address).await
    }
    
    /// [AIR-3][AIS-3][BPC-3][RES-3] Estimate fee for target confirmation blocks
    async fn estimate_fee(&self, target_blocks: u8) -> BitcoinResult<u64> {
        self.implementation.estimate_fee(target_blocks).await
    }
    
    /// [AIR-3][AIS-3][BPC-3][RES-3] Get block header by hash
    async fn get_block_header(&self, hash: &str) -> BitcoinResult<BlockHeader> {
        self.implementation.get_block_header(hash).await
    }
    
    /// [AIR-3][AIS-3][BPC-3][RES-3] Verify merkle proof for transaction
    async fn verify_merkle_proof(&self, tx_hash: &str, block_header: &BlockHeader) -> BitcoinResult<bool> {
        self.implementation.verify_merkle_proof(tx_hash, block_header).await
    }
    
    /// [AIR-3][AIS-3][BPC-3][RES-3] Send transaction to network
    async fn send_transaction(&self, tx: &Transaction) -> BitcoinResult<String> {
        self.implementation.send_transaction(tx).await
    }
    
    fn implementation_type(&self) -> BitcoinImplementationType {
        self.implementation.implementation_type()
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
