//! Bitcoin mempool implementation

use async_trait::async_trait;
use bitcoin::{Transaction, Txid};
use log::{debug, info};
///
/// This module contains the implementation of the mempool for Bitcoin transactions,
/// following Bitcoin Core principles including decentralization and security.
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

use super::{fees::FeeEstimator, policy::MempoolPolicy, AcceptanceResult};
use crate::core::error::AnyaResult;

/// Default maximum size of the mempool in bytes
pub const DEFAULT_MEMPOOL_SIZE: usize = 300_000_000; // 300 MB

/// Mempool implementation for storing unconfirmed transactions
pub struct MempoolImpl {
    /// Transactions indexed by txid
    transactions: Arc<RwLock<HashMap<Txid, Transaction>>>,
    /// Transaction acceptance policy
    policy: Arc<MempoolPolicy>,
    /// Fee estimator
    fee_estimator: Arc<FeeEstimator>,
    /// Maximum size of the mempool in bytes
    max_size: usize,
    /// Current size of the mempool in bytes
    size: Arc<RwLock<usize>>,
    /// Set of recently rejected transactions
    recently_rejected: Arc<RwLock<HashSet<Txid>>>,
}

impl MempoolImpl {
    /// Create a new mempool with default settings
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_MEMPOOL_SIZE)
    }

    /// Create a new mempool with the specified capacity
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            transactions: Arc::new(RwLock::new(HashMap::new())),
            policy: Arc::new(MempoolPolicy::new()),
            fee_estimator: Arc::new(FeeEstimator::new()),
            max_size,
            size: Arc::new(RwLock::new(0)),
            recently_rejected: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    /// Create a new mempool with custom policy and fee estimator
    pub fn with_policy_and_estimator(
        policy: Arc<MempoolPolicy>,
        fee_estimator: Arc<FeeEstimator>,
        max_size: usize,
    ) -> Self {
        Self {
            transactions: Arc::new(RwLock::new(HashMap::new())),
            policy,
            fee_estimator,
            max_size,
            size: Arc::new(RwLock::new(0)),
            recently_rejected: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    /// Get the mempool policy
    pub fn policy(&self) -> Arc<MempoolPolicy> {
        self.policy.clone()
    }

    /// Get the fee estimator
    pub fn fee_estimator(&self) -> Arc<FeeEstimator> {
        self.fee_estimator.clone()
    }

    /// Update fee estimates based on current mempool contents
    pub fn update_fee_estimates(&self) -> AnyaResult<()> {
        let txs = {
            let txs_guard = self.transactions.read().unwrap();
            txs_guard.values().cloned().collect::<Vec<_>>()
        };

        self.fee_estimator.update_estimates(&txs)?;
        Ok(())
    }

    /// Check if the mempool is full
    fn is_full(&self) -> bool {
        let size = *self.size.read().unwrap();
        size >= self.max_size
    }

    /// Calculate transaction fee
    fn calculate_fee(&self, tx: &Transaction) -> u64 {
        // In a real implementation, this would calculate the fee
        // by comparing inputs and outputs
        // For simplicity, we'll return a placeholder value
        let weight = u64::from(tx.weight());
        weight * 10 // Simple fee calculation
    }

    /// Calculate fee rate (satoshis per byte)
    fn calculate_fee_rate(&self, tx: &Transaction, fee: u64) -> f64 {
        let weight = u64::from(tx.weight()) as f64;
        let vsize = weight / 4.0; // Convert weight units to virtual size
        if vsize > 0.0 {
            fee as f64 / vsize
        } else {
            0.0
        }
    }

    /// Add transaction to recently rejected set
    fn add_to_rejected(&self, txid: Txid, reason: &str) {
        let mut rejected = self.recently_rejected.write().unwrap();
        rejected.insert(txid);
        debug!("Transaction {} rejected: {}", txid, reason);
    }

    /// Clean up transactions with low fee when mempool is full
    fn clean_low_fee_transactions(&self) -> AnyaResult<usize> {
        if !self.is_full() {
            return Ok(0);
        }

        let mut txs_to_remove = Vec::new();

        // Calculate fees for all transactions
        {
            let txs_guard = self.transactions.read().unwrap();
            let mut tx_fees: Vec<_> = txs_guard
                .iter()
                .map(|(txid, tx)| {
                    let fee = self.calculate_fee(tx);
                    let fee_rate = self.calculate_fee_rate(tx, fee);
                    (txid, fee_rate)
                })
                .collect();

            // Sort by fee rate (ascending)
            tx_fees.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            // Take lowest fee transactions until we've freed up enough space
            let size_to_free = (*self.size.read().unwrap() / 4) as usize; // Free 25%
            let mut freed = 0;

            for (txid, _) in tx_fees {
                if freed >= size_to_free {
                    break;
                }

                txs_to_remove.push(*txid);

                if let Some(tx) = txs_guard.get(txid) {
                    freed += u64::from(tx.weight()) as usize / 4; // Convert to vsize
                }
            }
        }

        // Remove the transactions
        let removed = txs_to_remove.len();
        {
            let mut txs_guard = self.transactions.write().unwrap();
            let mut size_guard = self.size.write().unwrap();

            for txid in txs_to_remove {
                if let Some(tx) = txs_guard.remove(&txid) {
                    *size_guard -= u64::from(tx.weight()) as usize / 4;
                }
            }
        }

        info!("Removed {} low-fee transactions from mempool", removed);
        Ok(removed)
    }
}

#[async_trait]
impl super::Mempool for MempoolImpl {
    async fn add_transaction(&self, tx: &Transaction) -> AnyaResult<AcceptanceResult> {
        let txid = tx.compute_txid();

        // Check if transaction is already in mempool
        {
            let txs_guard = self.transactions.read().unwrap();
            if txs_guard.contains_key(&txid) {
                return Ok(AcceptanceResult {
                    accepted: true,
                    reason: None,
                    txid,
                    fee: self.calculate_fee(tx),
                    fee_rate: self.calculate_fee_rate(tx, self.calculate_fee(tx)),
                });
            }
        }

        // Check if recently rejected
        {
            let rejected_guard = self.recently_rejected.read().unwrap();
            if rejected_guard.contains(&txid) {
                return Ok(AcceptanceResult {
                    accepted: false,
                    reason: Some("Transaction was recently rejected".to_string()),
                    txid,
                    fee: 0,
                    fee_rate: 0.0,
                });
            }
        }

        // Check policy
        if let Err(e) = self.policy.check_transaction(tx) {
            let reason = e.to_string();
            self.add_to_rejected(txid, &reason);
            return Ok(AcceptanceResult {
                accepted: false,
                reason: Some(reason),
                txid,
                fee: 0,
                fee_rate: 0.0,
            });
        }

        // Check if mempool is full and clean if necessary
        if self.is_full() {
            self.clean_low_fee_transactions()?;

            // If still full after cleaning, reject
            if self.is_full() {
                let reason = "Mempool is full".to_string();
                self.add_to_rejected(txid, &reason);
                return Ok(AcceptanceResult {
                    accepted: false,
                    reason: Some(reason),
                    txid,
                    fee: 0,
                    fee_rate: 0.0,
                });
            }
        }

        // Calculate fee and fee rate
        let fee = self.calculate_fee(tx);
        let fee_rate = self.calculate_fee_rate(tx, fee);

        // Add to mempool
        {
            let tx_size = u64::from(tx.weight()) as usize / 4; // vsize
            let mut txs_guard = self.transactions.write().unwrap();
            let mut size_guard = self.size.write().unwrap();

            txs_guard.insert(txid, tx.clone());
            *size_guard += tx_size;
        }

        // Update fee estimator
        self.fee_estimator.add_transaction(tx, fee_rate)?;

        info!(
            "Added transaction {} to mempool (fee rate: {:.2} sat/vB)",
            txid, fee_rate
        );

        Ok(AcceptanceResult {
            accepted: true,
            reason: None,
            txid,
            fee,
            fee_rate,
        })
    }

    async fn remove_transaction(&self, txid: &Txid) -> AnyaResult<bool> {
        let mut removed = false;

        {
            let mut txs_guard = self.transactions.write().unwrap();
            let mut size_guard = self.size.write().unwrap();

            if let Some(tx) = txs_guard.remove(txid) {
                let tx_size = u64::from(tx.weight()) as usize / 4; // vsize
                *size_guard -= tx_size;
                removed = true;

                debug!("Removed transaction {} from mempool", txid);
            }
        }

        Ok(removed)
    }

    async fn has_transaction(&self, txid: &Txid) -> AnyaResult<bool> {
        let txs_guard = self.transactions.read().unwrap();
        Ok(txs_guard.contains_key(txid))
    }

    async fn get_transaction(&self, txid: &Txid) -> AnyaResult<Option<Transaction>> {
        let txs_guard = self.transactions.read().unwrap();
        Ok(txs_guard.get(txid).cloned())
    }

    async fn get_all_transactions(&self) -> AnyaResult<Vec<Transaction>> {
        let txs_guard = self.transactions.read().unwrap();
        Ok(txs_guard.values().cloned().collect())
    }

    async fn get_transaction_count(&self) -> AnyaResult<usize> {
        let txs_guard = self.transactions.read().unwrap();
        Ok(txs_guard.len())
    }

    async fn clear(&self) -> AnyaResult<()> {
        {
            let mut txs_guard = self.transactions.write().unwrap();
            let mut size_guard = self.size.write().unwrap();

            txs_guard.clear();
            *size_guard = 0;
        }

        {
            let mut rejected_guard = self.recently_rejected.write().unwrap();
            rejected_guard.clear();
        }

        info!("Mempool cleared");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_transaction() {
        // This would test adding valid and invalid transactions to the mempool
    }

    #[tokio::test]
    async fn test_mempool_full() {
        // This would test the behavior when the mempool is full
    }
}
