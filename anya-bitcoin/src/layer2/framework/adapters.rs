// Layer 2 Protocol Adapters
// This file contains adapter implementations for the Layer 2 protocols

use crate::core::error::AnyaResult;
use crate::layer2::framework::TransactionStatus;
use async_trait::async_trait;
use std::fmt;
use std::sync::Arc;

/// Protocol adapter trait for bridging between Bitcoin and Layer 2 protocols
#[async_trait]
pub trait ProtocolAdapter: Send + Sync + fmt::Debug {
    /// Get the protocol name
    fn protocol_name(&self) -> &str;
    
    /// Submit a transaction to the protocol
    async fn submit_transaction(&self, tx_data: &[u8]) -> AnyaResult<String>;
    
    /// Get the status of a transaction
    async fn get_transaction_status(&self, tx_id: &str) -> AnyaResult<TransactionStatus>;
    
    /// Verify a transaction
    async fn verify_transaction(&self, tx_id: &str) -> AnyaResult<bool>;
}

/// Factory for creating protocol adapters
pub struct AdapterFactory;

impl AdapterFactory {
    /// Create a new adapter factory
    pub fn new() -> Self {
        Self
    }
    
    /// Create an adapter for the given protocol
    pub fn create_adapter(&self, protocol_name: &str) -> AnyaResult<Arc<dyn ProtocolAdapter>> {
        match protocol_name {
            // No concrete implementations yet, just return NoopAdapter for all protocols
            _ => Ok(Arc::new(NoopAdapter::new(protocol_name))),
        }
    }
}

/// No-op implementation of the protocol adapter for testing
#[derive(Debug)]
pub struct NoopAdapter {
    protocol_name: String,
}

impl NoopAdapter {
    /// Create a new no-op adapter
    pub fn new(protocol_name: &str) -> Self {
        Self {
            protocol_name: protocol_name.to_string(),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for NoopAdapter {
    fn protocol_name(&self) -> &str {
        &self.protocol_name
    }
    
    async fn submit_transaction(&self, _tx_data: &[u8]) -> AnyaResult<String> {
        Ok("mock_tx_id".to_string())
    }
    
    async fn get_transaction_status(&self, _tx_id: &str) -> AnyaResult<TransactionStatus> {
        Ok(TransactionStatus::Confirmed)
    }
    
    async fn verify_transaction(&self, _tx_id: &str) -> AnyaResult<bool> {
        Ok(true)
    }
} 
