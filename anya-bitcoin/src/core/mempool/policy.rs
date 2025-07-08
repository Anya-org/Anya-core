//! Bitcoin mempool policy implementation

use bitcoin::{Amount, Transaction, TxOut};
use log::debug;
///
/// This module defines the policies for transaction acceptance in the mempool,
/// based on Bitcoin Core principles including security, decentralization, and minimalism.
use std::sync::RwLock;
use thiserror::Error;

/// Default minimum relay fee (satoshis per kilobyte)
pub const DEFAULT_MIN_RELAY_FEE: u64 = 1000; // 1 sat/byte

/// Default maximum transaction size (bytes)
pub const DEFAULT_MAX_TX_SIZE: usize = 100_000;

/// Default minimum transaction output value (satoshis)
pub const DEFAULT_DUST_LIMIT: u64 = 546;

/// Default maximum number of transaction ancestors
pub const DEFAULT_MAX_ANCESTORS: usize = 25;

/// Default maximum number of transaction descendants
pub const DEFAULT_MAX_DESCENDANTS: usize = 25;

/// Default maximum accumulated size of ancestor transactions (bytes)
pub const DEFAULT_MAX_ANCESTOR_SIZE: usize = 101_000;

/// Errors related to mempool policies
#[derive(Debug, Error)]
pub enum PolicyError {
    #[error("Transaction size exceeds maximum")]
    TxTooLarge,

    #[error("Transaction fee too low")]
    FeeTooLow,

    #[error("Transaction has outputs below dust limit")]
    DustOutput,

    #[error("Transaction would exceed ancestor count limit")]
    TooManyAncestors,

    #[error("Transaction would exceed descendant count limit")]
    TooManyDescendants,

    #[error("Transaction fails standardness checks")]
    NonStandard,

    #[error("Transaction contains too many sigops")]
    TooManySigops,

    #[error("Transaction is a recent replacement")]
    RecentReplacement,

    #[error("General policy error: {0}")]
    General(String),
}

/// Mempool acceptance policy
pub struct MempoolPolicy {
    /// Minimum relay fee (satoshis per kilobyte)
    min_relay_fee: RwLock<u64>,

    /// Maximum transaction size (bytes)
    max_tx_size: RwLock<usize>,

    /// Minimum transaction output value (satoshis)
    dust_limit: RwLock<u64>,

    /// Maximum number of transaction ancestors
    max_ancestors: RwLock<usize>,

    /// Maximum number of transaction descendants
    max_descendants: RwLock<usize>,

    /// Maximum accumulated size of ancestor transactions (bytes)
    #[allow(dead_code)]
    // Preserved for compatibility with upstream mempool policy and future upgrades (see docs/INDEX_CORRECTED.md)
    max_ancestor_size: RwLock<usize>,

    /// Whether to allow replacement transactions (RBF)
    allow_replacement: RwLock<bool>,

    /// Whether to accept non-standard transactions
    accept_non_standard: RwLock<bool>,
}

impl MempoolPolicy {
    /// Create a new mempool policy with default settings
    pub fn new() -> Self {
        Self {
            min_relay_fee: RwLock::new(DEFAULT_MIN_RELAY_FEE),
            max_tx_size: RwLock::new(DEFAULT_MAX_TX_SIZE),
            dust_limit: RwLock::new(DEFAULT_DUST_LIMIT),
            max_ancestors: RwLock::new(DEFAULT_MAX_ANCESTORS),
            max_descendants: RwLock::new(DEFAULT_MAX_DESCENDANTS),
            max_ancestor_size: RwLock::new(DEFAULT_MAX_ANCESTOR_SIZE),
            allow_replacement: RwLock::new(true),
            accept_non_standard: RwLock::new(false),
        }
    }

    /// Set the minimum relay fee
    pub fn set_min_relay_fee(&self, fee: u64) {
        let mut guard = self.min_relay_fee.write().unwrap();
        *guard = fee;
    }

    /// Get the minimum relay fee
    pub fn min_relay_fee(&self) -> u64 {
        *self.min_relay_fee.read().unwrap()
    }

    /// Set the maximum transaction size
    pub fn set_max_tx_size(&self, size: usize) {
        let mut guard = self.max_tx_size.write().unwrap();
        *guard = size;
    }

    /// Get the maximum transaction size
    pub fn max_tx_size(&self) -> usize {
        *self.max_tx_size.read().unwrap()
    }

    /// Set the dust limit
    pub fn set_dust_limit(&self, limit: u64) {
        let mut guard = self.dust_limit.write().unwrap();
        *guard = limit;
    }

    /// Get the dust limit
    pub fn dust_limit(&self) -> u64 {
        *self.dust_limit.read().unwrap()
    }

    /// Set whether to allow replacement transactions
    pub fn set_allow_replacement(&self, allow: bool) {
        let mut guard = self.allow_replacement.write().unwrap();
        *guard = allow;
    }

    /// Get whether replacement transactions are allowed
    pub fn allow_replacement(&self) -> bool {
        *self.allow_replacement.read().unwrap()
    }

    /// Check if a transaction passes all policy checks
    pub fn check_transaction(&self, tx: &Transaction) -> Result<(), PolicyError> {
        // Check transaction size
        self.check_tx_size(tx)?;

        // Check outputs for dust
        self.check_dust_outputs(tx)?;

        // Check standardness
        self.check_standardness(tx)?;

        // In a real implementation, we would also check:
        // - Fee rate
        // - Ancestor/descendant limits
        // - Sigop count
        // - RBF rules

        Ok(())
    }

    /// Check if transaction size is within limits
    fn check_tx_size(&self, tx: &Transaction) -> Result<(), PolicyError> {
        let tx_size = u64::from(tx.weight()) as usize / 4; // Convert to vsize
        let max_size = self.max_tx_size();

        if tx_size > max_size {
            debug!("Transaction too large: {} > {}", tx_size, max_size);
            return Err(PolicyError::TxTooLarge);
        }

        Ok(())
    }

    /// Check if all outputs are above the dust limit
    fn check_dust_outputs(&self, tx: &Transaction) -> Result<(), PolicyError> {
        let dust_limit = self.dust_limit();

        for output in &tx.output {
            if Self::is_dust_output(output, dust_limit) {
                debug!(
                    "Transaction has dust output: {} < {}",
                    output.value, dust_limit
                );
                return Err(PolicyError::DustOutput);
            }
        }

        Ok(())
    }

    /// Check if a transaction follows standard format rules
    fn check_standardness(&self, tx: &Transaction) -> Result<(), PolicyError> {
        // In a full implementation, this would check:
        // - Script types (P2PKH, P2SH, etc.)
        // - Input/output counts
        // - Non-final transactions

        if !self.accept_non_standard() && !Self::is_standard_transaction(tx) {
            return Err(PolicyError::NonStandard);
        }

        Ok(())
    }

    /// Get whether to accept non-standard transactions
    pub fn accept_non_standard(&self) -> bool {
        *self.accept_non_standard.read().unwrap()
    }

    /// Set whether to accept non-standard transactions
    pub fn set_accept_non_standard(&self, accept: bool) {
        let mut guard = self.accept_non_standard.write().unwrap();
        *guard = accept;
    }

    /// Calculate the minimum fee for a transaction
    pub fn calculate_min_fee(&self, tx: &Transaction) -> u64 {
        let min_relay_fee = self.min_relay_fee();
        let tx_size = u64::from(tx.weight()) / 4; // Convert to vsize

        // Calculate minimum fee: min_relay_fee * tx_size / 1000
        (min_relay_fee * tx_size + 999) / 1000
    }

    /// Check if an output is considered dust
    fn is_dust_output(output: &TxOut, dust_limit: u64) -> bool {
        // For simplicity, we'll just compare against the dust limit
        // In reality, this would take the script type into account
        output.value < Amount::from_sat(dust_limit)
    }

    /// Check if a transaction follows standard format
    fn is_standard_transaction(_tx: &Transaction) -> bool {
        // For simplicity, we'll always return true in this placeholder
        // In a real implementation, this would check script types, etc.
        true
    }

    /// Set the maximum ancestor count
    pub fn set_max_ancestors(&self, count: usize) {
        let mut guard = self.max_ancestors.write().unwrap();
        *guard = count;
    }

    /// Get the maximum ancestor count
    pub fn max_ancestors(&self) -> usize {
        *self.max_ancestors.read().unwrap()
    }

    /// Set the maximum descendant count
    pub fn set_max_descendants(&self, count: usize) {
        let mut guard = self.max_descendants.write().unwrap();
        *guard = count;
    }

    /// Get the maximum descendant count
    pub fn max_descendants(&self) -> usize {
        *self.max_descendants.read().unwrap()
    }
}

impl Default for MempoolPolicy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_dust_detection() {
        // This would test detection of dust outputs
    }

    #[test]
    fn test_tx_size_limits() {
        // This would test the transaction size limits
    }
}
