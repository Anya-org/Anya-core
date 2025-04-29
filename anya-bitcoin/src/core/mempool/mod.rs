pub mod pool;
pub mod policy;
pub mod fees;

// Re-export commonly used items
pub use pool::Mempool;
pub use policy::MempoolPolicy;
pub use fees::FeeEstimator;

//! Bitcoin mempool implementation
//!
//! This module contains the memory pool implementation for Bitcoin transactions.

use std::sync::Arc;
use async_trait::async_trait;
use bitcoin::{Transaction, Txid};
use crate::core::error::AnyaResult;

/// Transaction acceptance result
#[derive(Debug, Clone)]
pub struct AcceptanceResult {
    /// Whether the transaction was accepted
    pub accepted: bool,
    /// Rejection reason if not accepted
    pub reason: Option<String>,
    /// Transaction ID
    pub txid: Txid,
    /// Fee paid by the transaction (in satoshis)
    pub fee: u64,
    /// Fee rate (satoshis per byte)
    pub fee_rate: f64,
}

/// Mempool interface
#[async_trait]
pub trait Mempool: Send + Sync {
    /// Add a transaction to the mempool
    async fn add_transaction(&self, tx: &Transaction) -> AnyaResult<AcceptanceResult>;
    
    /// Remove a transaction from the mempool
    async fn remove_transaction(&self, txid: &Txid) -> AnyaResult<bool>;
    
    /// Check if a transaction is in the mempool
    async fn has_transaction(&self, txid: &Txid) -> AnyaResult<bool>;
    
    /// Get a transaction from the mempool
    async fn get_transaction(&self, txid: &Txid) -> AnyaResult<Option<Transaction>>;
    
    /// Get all transactions in the mempool
    async fn get_all_transactions(&self) -> AnyaResult<Vec<Transaction>>;
    
    /// Get the number of transactions in the mempool
    async fn get_transaction_count(&self) -> AnyaResult<usize>;
    
    /// Clear the mempool
    async fn clear(&self) -> AnyaResult<()>;
}

/// No-op mempool implementation for testing
pub struct NoopMempool;

#[async_trait]
impl Mempool for NoopMempool {
    async fn add_transaction(&self, tx: &Transaction) -> AnyaResult<AcceptanceResult> {
        Ok(AcceptanceResult {
            accepted: true,
            reason: None,
            txid: tx.txid(),
            fee: 0,
            fee_rate: 0.0,
        })
    }
    
    async fn remove_transaction(&self, _txid: &Txid) -> AnyaResult<bool> {
        Ok(true)
    }
    
    async fn has_transaction(&self, _txid: &Txid) -> AnyaResult<bool> {
        Ok(false)
    }
    
    async fn get_transaction(&self, _txid: &Txid) -> AnyaResult<Option<Transaction>> {
        Ok(None)
    }
    
    async fn get_all_transactions(&self) -> AnyaResult<Vec<Transaction>> {
        Ok(Vec::new())
    }
    
    async fn get_transaction_count(&self) -> AnyaResult<usize> {
        Ok(0)
    }
    
    async fn clear(&self) -> AnyaResult<()> {
        Ok(())
    }
} 