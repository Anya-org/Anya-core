//! Bitcoin fee estimation

use bitcoin::{Transaction, Txid};
use log::{debug, info};
///
/// This module provides fee estimation for Bitcoin transactions based on
/// mempool state and recent block history, following Bitcoin Core principles.
use std::collections::HashMap;
use std::sync::RwLock;

use crate::core::error::AnyaResult;

/// Default number of blocks to target for fee estimation
pub const DEFAULT_TARGET_BLOCKS: usize = 6;

/// Default number of fee buckets for fee estimation
pub const DEFAULT_FEE_BUCKETS: usize = 10;

/// Fee bucket for tracking transactions with similar fee rates
struct FeeBucket {
    /// Fee rate (satoshis per virtual byte)
    fee_rate: f64,
    /// Number of transactions in this bucket
    count: usize,
    /// Total fee amount in this bucket
    total_fee: u64,
}

/// Fee estimator for Bitcoin transactions
pub struct FeeEstimator {
    /// Fee rate buckets indexed by confirmation target
    buckets: RwLock<HashMap<usize, Vec<FeeBucket>>>,
    /// Tracked transactions with their fee rates
    tracked_txs: RwLock<HashMap<Txid, f64>>,
    /// Minimum fee rate to track
    min_fee_rate: RwLock<f64>,
    /// Whether to use smart fee estimation (use historical data)
    use_smart_fee: RwLock<bool>,
    /// Default confirmation target
    default_target: RwLock<usize>,
}

impl FeeEstimator {
    /// Create a new fee estimator with default settings
    pub fn new() -> Self {
        let mut buckets = HashMap::new();

        // Initialize buckets for common confirmation targets
        for target in &[1, 2, 3, 6, 12, 24, 48, 144, 504, 1008] {
            buckets.insert(*target, Self::initialize_fee_buckets(DEFAULT_FEE_BUCKETS));
        }

        Self {
            buckets: RwLock::new(buckets),
            tracked_txs: RwLock::new(HashMap::new()),
            min_fee_rate: RwLock::new(1.0), // 1 sat/vB
            use_smart_fee: RwLock::new(true),
            default_target: RwLock::new(DEFAULT_TARGET_BLOCKS),
        }
    }

    /// Initialize fee buckets with exponentially increasing fee rates
    fn initialize_fee_buckets(count: usize) -> Vec<FeeBucket> {
        let mut buckets = Vec::with_capacity(count);

        // Start at 1 sat/vB and increase exponentially
        let mut fee_rate = 1.0;
        for _ in 0..count {
            buckets.push(FeeBucket {
                fee_rate,
                count: 0,
                total_fee: 0,
            });

            // Increase by approximately 50% each step
            fee_rate *= 1.5;
        }

        buckets
    }

    /// Add a transaction to the fee estimator
    pub fn add_transaction(&self, tx: &Transaction, fee_rate: f64) -> AnyaResult<()> {
        let txid = tx.compute_txid();

        // Skip transactions with fee rate below minimum
        if fee_rate < self.min_fee_rate() {
            return Ok(());
        }

        // Track this transaction
        {
            let mut tracked = self.tracked_txs.write().unwrap();
            tracked.insert(txid, fee_rate);
        }

        // Add to appropriate buckets
        self.add_to_buckets(fee_rate, 1, fee_rate as u64 * (u64::from(tx.weight()) / 4))?;

        debug!(
            "Added transaction {} to fee estimator (fee rate: {:.2} sat/vB)",
            txid, fee_rate
        );
        Ok(())
    }

    /// Add a fee rate to appropriate buckets
    fn add_to_buckets(&self, fee_rate: f64, count: usize, total_fee: u64) -> AnyaResult<()> {
        let mut buckets_guard = self.buckets.write().unwrap();

        for (_, bucket_list) in buckets_guard.iter_mut() {
            Self::add_to_bucket_list(bucket_list, fee_rate, count, total_fee);
        }

        Ok(())
    }

    /// Add a fee rate to a specific bucket list
    fn add_to_bucket_list(
        buckets: &mut Vec<FeeBucket>,
        fee_rate: f64,
        count: usize,
        total_fee: u64,
    ) {
        // Find the appropriate bucket
        for bucket in buckets.iter_mut() {
            if fee_rate <= bucket.fee_rate {
                bucket.count += count;
                bucket.total_fee += total_fee;
                return;
            }
        }

        // If we get here, fee rate is higher than all buckets
        // Add to the highest bucket
        if let Some(bucket) = buckets.last_mut() {
            bucket.count += count;
            bucket.total_fee += total_fee;
        }
    }

    /// Estimate the fee rate for a given confirmation target
    pub fn estimate_fee_rate(&self, target_blocks: usize) -> f64 {
        let target = if target_blocks == 0 {
            *self.default_target.read().unwrap()
        } else {
            target_blocks
        };

        // If not using smart fee, use a simple estimate based on mempool
        if !*self.use_smart_fee.read().unwrap() {
            return self.simple_estimate(target);
        }

        // Otherwise, use historical data for the target
        let buckets_guard = self.buckets.read().unwrap();

        // Find the closest target we have data for
        let available_targets: Vec<_> = buckets_guard.keys().cloned().collect();
        let actual_target = Self::closest_target(&available_targets, target);

        if let Some(bucket_list) = buckets_guard.get(&actual_target) {
            Self::estimate_from_buckets(bucket_list, 0.95) // 95% success rate
        } else {
            // Fallback to simple estimate
            self.simple_estimate(target)
        }
    }

    /// Find the closest target from the available options
    fn closest_target(available: &[usize], target: usize) -> usize {
        let mut closest = *available.first().unwrap_or(&DEFAULT_TARGET_BLOCKS);
        let mut min_diff = (closest as isize - target as isize).abs();

        for &t in available {
            let diff = (t as isize - target as isize).abs();
            if diff < min_diff {
                min_diff = diff;
                closest = t;
            }
        }

        closest
    }

    /// Estimate fee rate from bucket data
    fn estimate_from_buckets(buckets: &[FeeBucket], success_rate: f64) -> f64 {
        let total_txs: usize = buckets.iter().map(|b| b.count).sum();
        if total_txs == 0 {
            return 1.0; // Default to 1 sat/vB if no data
        }

        let target_count = (total_txs as f64 * success_rate) as usize;
        let mut cumulative = 0;

        // Start from the highest fee bucket
        for bucket in buckets.iter().rev() {
            cumulative += bucket.count;
            if cumulative >= target_count {
                return bucket.fee_rate;
            }
        }

        // If we don't have enough data, return the lowest bucket
        buckets.first().map(|b| b.fee_rate).unwrap_or(1.0)
    }

    /// Simple fee estimation based on mempool state
    fn simple_estimate(&self, target_blocks: usize) -> f64 {
        // For a simple estimate, we'll return higher fees for faster confirmation
        // This is a simplistic model; a real implementation would be more sophisticated
        match target_blocks {
            0..=1 => 50.0, // Very fast: 50 sat/vB
            2..=3 => 20.0, // Fast: 20 sat/vB
            4..=6 => 10.0, // Medium: 10 sat/vB
            7..=24 => 5.0, // Slow: 5 sat/vB
            _ => 1.0,      // Very slow: 1 sat/vB
        }
    }

    /// Update fee estimates based on confirmed transactions
    pub fn update_estimates(&self, recent_txs: &[Transaction]) -> AnyaResult<()> {
        if recent_txs.is_empty() {
            return Ok(());
        }

        // Track how many transactions were confirmed
        let mut confirmed_count = 0;

        {
            let mut tracked = self.tracked_txs.write().unwrap();

            // Remove confirmed transactions from tracking
            for tx in recent_txs {
                let txid = tx.compute_txid();
                if tracked.remove(&txid).is_some() {
                    confirmed_count += 1;
                }
            }
        }

        info!(
            "Updated fee estimates: {} transactions confirmed",
            confirmed_count
        );
        Ok(())
    }

    /// Set the default confirmation target
    pub fn set_default_target(&self, blocks: usize) {
        let mut target = self.default_target.write().unwrap();
        *target = blocks;
    }

    /// Get the default confirmation target
    pub fn default_target(&self) -> usize {
        *self.default_target.read().unwrap()
    }

    /// Set the minimum fee rate to track
    pub fn set_min_fee_rate(&self, rate: f64) {
        let mut min_rate = self.min_fee_rate.write().unwrap();
        *min_rate = rate;
    }

    /// Get the minimum fee rate
    pub fn min_fee_rate(&self) -> f64 {
        *self.min_fee_rate.read().unwrap()
    }

    /// Enable or disable smart fee estimation
    pub fn set_use_smart_fee(&self, use_smart: bool) {
        let mut smart = self.use_smart_fee.write().unwrap();
        *smart = use_smart;
    }

    /// Get whether smart fee estimation is enabled
    pub fn use_smart_fee(&self) -> bool {
        *self.use_smart_fee.read().unwrap()
    }

    /// Get the current fee statistics for all targets
    pub fn get_fee_stats(&self) -> HashMap<usize, f64> {
        let mut stats = HashMap::new();
        let buckets_guard = self.buckets.read().unwrap();

        for (&target, bucket_list) in buckets_guard.iter() {
            stats.insert(target, Self::estimate_from_buckets(bucket_list, 0.8));
        }

        stats
    }

    /// Clear all fee estimation data
    pub fn clear(&self) -> AnyaResult<()> {
        {
            let mut buckets_guard = self.buckets.write().unwrap();

            // Reset all buckets
            for (_, bucket_list) in buckets_guard.iter_mut() {
                for bucket in bucket_list.iter_mut() {
                    bucket.count = 0;
                    bucket.total_fee = 0;
                }
            }
        }

        {
            let mut tracked = self.tracked_txs.write().unwrap();
            tracked.clear();
        }

        info!("Cleared fee estimation data");
        Ok(())
    }
}

impl Default for FeeEstimator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fee_estimation() {
        // This would test fee estimation with various inputs
    }

    #[test]
    fn test_bucket_selection() {
        // This would test the selection of appropriate fee buckets
    }
}
