use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use log::{debug, error, info, warn};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::core::metrics::PrometheusMetrics;
use crate::monitoring::metrics;

// Global blockchain metrics registry
lazy_static! {
    static ref BLOCKCHAIN_METRICS: Mutex<BlockchainMetrics> = Mutex::new(BlockchainMetrics::new());
}

/// Timestamp-based sample for time series metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSample {
    pub timestamp: u64,
    pub value: f64,
}

/// Blockchain metrics collector for real-time monitoring
#[derive(Debug)]
pub struct BlockchainMetrics {
    /// Last update timestamp
    last_update: Instant,
    
    /// Metrics collector
    metrics: PrometheusMetrics,

    /// Historical segwit usage percentage samples (last 24h)
    segwit_samples: Vec<TimeSample>,
    
    /// Historical taproot usage percentage samples (last 24h)
    taproot_samples: Vec<TimeSample>,
    
    /// Block propagation times (in ms) for the last 100 blocks
    block_propagation_times: Vec<TimeSample>,
    
    /// SegWit percentage in latest block
    segwit_percentage: f64,
    
    /// Taproot percentage in latest block
    taproot_percentage: f64,
    
    /// UTXO set size
    utxo_set_size: u64,
    
    /// Average transaction fee (in sats/vB)
    avg_fee_rate: f64,
    
    /// Error rates by category
    error_rates: HashMap<String, f64>,
    
    /// Mempool size in bytes
    mempool_size: u64,
    
    /// Average block size (last 10 blocks)
    avg_block_size: u64,
    
    /// Block height
    block_height: u64,
    
    /// BIP compliance map
    bip_compliance: HashMap<String, bool>,
    
    /// Network hashrate (EH/s)
    network_hashrate: f64,
    
    /// TPS (Transactions Per Second) metrics
    tps_current: f64,
    tps_samples: Vec<TimeSample>,
    transaction_count_last_block: u64,
    
    /// Block version monitoring
    block_versions: HashMap<u32, u64>, // version -> count
    block_version_samples: Vec<TimeSample>,
    
    /// 51% attack detection
    recent_block_times: Vec<u64>, // timestamps of recent blocks
    hashrate_distribution: HashMap<String, f64>, // pool_id -> hashrate_percentage
    consecutive_blocks_same_miner: u64,
    attack_probability: f64,
}

impl BlockchainMetrics {
    /// Creates a new blockchain metrics collector
    pub fn new() -> Self {
        Self {
            last_update: Instant::now(),
            metrics: PrometheusMetrics::new(),
            segwit_samples: Vec::new(),
            taproot_samples: Vec::new(),
            block_propagation_times: Vec::new(),
            segwit_percentage: 0.0,
            taproot_percentage: 0.0,
            utxo_set_size: 0,
            avg_fee_rate: 0.0,
            error_rates: HashMap::new(),
            mempool_size: 0,
            avg_block_size: 0,
            block_height: 0,
            bip_compliance: HashMap::new(),
            network_hashrate: 0.0,
            tps_current: 0.0,
            tps_samples: Vec::new(),
            transaction_count_last_block: 0,
            block_versions: HashMap::new(),
            block_version_samples: Vec::new(),
            recent_block_times: Vec::new(),
            hashrate_distribution: HashMap::new(),
            consecutive_blocks_same_miner: 0,
            attack_probability: 0.0,
        }
    }
    
    /// Update SegWit adoption percentage
    pub fn update_segwit_percentage(&mut self, percentage: f64) {
        self.segwit_percentage = percentage;
        self.metrics.set_gauge("segwit_adoption_percentage", percentage);
        
        // Add to time series
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.segwit_samples.push(TimeSample { timestamp: now, value: percentage });
        
        // Keep only last 24 hours (assuming samples every 10 minutes = 144 samples)
        if self.segwit_samples.len() > 144 {
            self.segwit_samples.remove(0);
        }
        
        // Also register in the global metrics
        metrics::register_taproot_usage(percentage);
    }
    
    /// Update Taproot adoption percentage
    pub fn update_taproot_percentage(&mut self, percentage: f64) {
        self.taproot_percentage = percentage;
        self.metrics.set_gauge("taproot_adoption_percentage", percentage);
        
        // Add to time series
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.taproot_samples.push(TimeSample { timestamp: now, value: percentage });
        
        // Keep only last 24 hours (assuming samples every 10 minutes = 144 samples)
        if self.taproot_samples.len() > 144 {
            self.taproot_samples.remove(0);
        }
    }
    
    /// Update UTXO set size
    pub fn update_utxo_set_size(&mut self, size: u64) {
        self.utxo_set_size = size;
        self.metrics.set_gauge("utxo_set_size", size as f64);
    }
    
    /// Update average fee rate
    pub fn update_avg_fee_rate(&mut self, sats_per_vb: f64) {
        self.avg_fee_rate = sats_per_vb;
        self.metrics.set_gauge("avg_fee_rate_sats_per_vb", sats_per_vb);
    }
    
    /// Update error rate for a specific category
    pub fn update_error_rate(&mut self, category: &str, rate: f64) {
        self.error_rates.insert(category.to_string(), rate);
        self.metrics.set_gauge(&format!("error_rate_{}", category), rate);
    }
    
    /// Update mempool size
    pub fn update_mempool_size(&mut self, size_bytes: u64) {
        self.mempool_size = size_bytes;
        self.metrics.set_gauge("mempool_size_bytes", size_bytes as f64);
        
        // Also register in the global metrics
        metrics::register_mempool_size((size_bytes / 1024) as usize); // Convert to KB
    }
    
    /// Update average block size
    pub fn update_avg_block_size(&mut self, size_bytes: u64) {
        self.avg_block_size = size_bytes;
        self.metrics.set_gauge("avg_block_size_bytes", size_bytes as f64);
    }
    
    /// Update block height
    pub fn update_block_height(&mut self, height: u64) {
        self.block_height = height;
        self.metrics.set_gauge("block_height", height as f64);
    }
    
    /// Update network hashrate (in EH/s)
    pub fn update_network_hashrate(&mut self, eh_per_second: f64) {
        self.network_hashrate = eh_per_second;
        self.metrics.set_gauge("network_hashrate_eh_per_s", eh_per_second);
    }
    
    /// Update block propagation time
    pub fn update_block_propagation_time(&mut self, block_hash: &str, milliseconds: u64) {
        self.metrics.set_gauge("block_propagation_ms", milliseconds as f64);
        
        // Add to time series with timestamp
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.block_propagation_times.push(TimeSample { timestamp: now, value: milliseconds as f64 });
        
        // Keep only last 100 blocks
        if self.block_propagation_times.len() > 100 {
            self.block_propagation_times.remove(0);
        }
        
        // Also register in the global metrics
        metrics::register_block_propagation_time(milliseconds);
        
        // Increment counter with block hash label
        self.metrics.increment_counter("blocks_received", "hash", block_hash);
    }
    
    /// Set BIP compliance status
    pub fn set_bip_compliance(&mut self, bip_number: &str, compliant: bool) {
        self.bip_compliance.insert(bip_number.to_string(), compliant);
        
        // Also register in the global metrics
        metrics::register_bip_compliance(bip_number, compliant);
    }

    /// Update TPS (Transactions Per Second) metrics
    pub fn update_tps(&mut self, transaction_count: u64, block_time_seconds: u64) {
        if block_time_seconds > 0 {
            self.tps_current = transaction_count as f64 / block_time_seconds as f64;
            self.transaction_count_last_block = transaction_count;
            
            // Update metrics
            self.metrics.set_gauge("tps_current", self.tps_current);
            self.metrics.set_gauge("transactions_last_block", transaction_count as f64);
            
            // Add to time series
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            self.tps_samples.push(TimeSample { timestamp: now, value: self.tps_current });
            
            // Keep only last 144 samples (24 hours assuming 10min blocks)
            if self.tps_samples.len() > 144 {
                self.tps_samples.remove(0);
            }
            
            info!("Updated TPS: {} tx/s ({} transactions in {} seconds)", 
                  self.tps_current, transaction_count, block_time_seconds);
        }
    }
    
    /// Monitor block versions according to BIP standards
    pub fn update_block_version(&mut self, version: u32) {
        // Update version counts
        *self.block_versions.entry(version).or_insert(0) += 1;
        
        // Update metrics
        self.metrics.set_gauge("current_block_version", version as f64);
        
        // Add to time series
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.block_version_samples.push(TimeSample { timestamp: now, value: version as f64 });
        
        // Keep only last 2016 blocks (difficulty adjustment period)
        if self.block_version_samples.len() > 2016 {
            self.block_version_samples.remove(0);
        }
        
        // Check for version adoption
        let total_blocks = self.block_versions.values().sum::<u64>();
        if total_blocks > 0 {
            for (ver, count) in &self.block_versions {
                let percentage = (*count as f64 / total_blocks as f64) * 100.0;
                self.metrics.set_gauge(&format!("block_version_{}_percentage", ver), percentage);
                
                // Log significant version changes
                if percentage > 5.0 {
                    debug!("Block version {}: {:.1}% ({} of {} blocks)", 
                           ver, percentage, count, total_blocks);
                }
            }
        }
        
        info!("Updated block version: {} (total tracked: {})", version, total_blocks);
    }
    
    /// Update hashrate distribution for 51% attack detection
    pub fn update_hashrate_distribution(&mut self, miner_id: &str, hashrate_percentage: f64) {
        self.hashrate_distribution.insert(miner_id.to_string(), hashrate_percentage);
        
        // Calculate 51% attack probability
        self.calculate_attack_probability();
        
        // Update metrics
        self.metrics.set_gauge(&format!("hashrate_{}", miner_id), hashrate_percentage);
        self.metrics.set_gauge("attack_probability", self.attack_probability);
        
        // Alert if any miner approaches 51%
        if hashrate_percentage > 45.0 {
            warn!("SECURITY ALERT: Miner {} has {:.1}% hashrate - approaching 51% threshold", 
                  miner_id, hashrate_percentage);
        }
    }
    
    /// Record a new block for 51% attack monitoring
    pub fn record_block(&mut self, block_timestamp: u64, miner_id: &str) {
        // Track recent block times
        self.recent_block_times.push(block_timestamp);
        
        // Keep only last 100 blocks for timing analysis
        if self.recent_block_times.len() > 100 {
            self.recent_block_times.remove(0);
        }
        
        // Check for consecutive blocks from same miner
        // In a real implementation, this would track the actual miner IDs
        let _ = miner_id; // Suppress unused warning for now
        
        // Detect rapid block mining (potential attack indicator)
        if self.recent_block_times.len() >= 2 {
            let time_diff = block_timestamp.saturating_sub(
                self.recent_block_times[self.recent_block_times.len() - 2]
            );
            
            // Alert if blocks are being mined too quickly (< 5 minutes average)
            if time_diff < 300 && self.recent_block_times.len() > 10 {
                let avg_time = self.calculate_average_block_time();
                if avg_time < 300.0 {
                    warn!("SECURITY ALERT: Rapid block mining detected - avg time: {:.1}s (expected: 600s)", 
                          avg_time);
                }
            }
        }
        
        self.calculate_attack_probability();
    }
    
    /// Calculate the probability of a 51% attack based on current metrics
    fn calculate_attack_probability(&mut self) {
        let mut max_hashrate = 0.0;
        let mut total_hashrate = 0.0;
        
        for hashrate in self.hashrate_distribution.values() {
            total_hashrate += hashrate;
            if *hashrate > max_hashrate {
                max_hashrate = *hashrate;
            }
        }
        
        // Base probability on maximum hashrate concentration
        self.attack_probability = if max_hashrate > 51.0 {
            0.9 // Very high probability if someone already has >51%
        } else if max_hashrate > 45.0 {
            0.7 // High probability if approaching 51%
        } else if max_hashrate > 40.0 {
            0.4 // Moderate probability 
        } else if max_hashrate > 35.0 {
            0.2 // Low probability
        } else {
            0.05 // Very low probability
        };
        
        // Adjust based on block timing patterns
        if self.recent_block_times.len() > 10 {
            let avg_time = self.calculate_average_block_time();
            if avg_time < 300.0 { // Less than 5 minutes average
                self.attack_probability += 0.1; // Increase probability
            }
        }
        
        // Cap at 1.0
        self.attack_probability = self.attack_probability.min(1.0);
        
        // Log high attack probability
        if self.attack_probability > 0.5 {
            error!("HIGH SECURITY RISK: 51% attack probability: {:.1}% (max hashrate: {:.1}%)", 
                   self.attack_probability * 100.0, max_hashrate);
        }
    }
    
    /// Calculate average block time from recent blocks
    fn calculate_average_block_time(&self) -> f64 {
        if self.recent_block_times.len() < 2 {
            return 600.0; // Default to 10 minutes
        }
        
        let total_time = self.recent_block_times.last().unwrap() - 
                        self.recent_block_times.first().unwrap();
        let block_count = (self.recent_block_times.len() - 1) as f64;
        
        total_time as f64 / block_count
    }
    
    /// Get attack detection summary
    pub fn get_attack_detection_summary(&self) -> HashMap<String, f64> {
        let mut summary = HashMap::new();
        
        summary.insert("attack_probability".to_string(), self.attack_probability);
        summary.insert("max_hashrate_percentage".to_string(), 
                      self.hashrate_distribution.values().fold(0.0, |a, &b| a.max(b)));
        summary.insert("active_miners".to_string(), self.hashrate_distribution.len() as f64);
        summary.insert("avg_block_time_seconds".to_string(), self.calculate_average_block_time());
        summary.insert("recent_blocks_tracked".to_string(), self.recent_block_times.len() as f64);
        
        summary
    }

    /// Get the current metrics as JSON
    pub fn as_json(&self) -> serde_json::Value {
        let mut result = serde_json::json!({
            "last_update_seconds_ago": self.last_update.elapsed().as_secs(),
            "segwit_percentage": self.segwit_percentage,
            "taproot_percentage": self.taproot_percentage,
            "utxo_set_size": self.utxo_set_size,
            "avg_fee_rate": self.avg_fee_rate,
            "mempool_size": self.mempool_size,
            "avg_block_size": self.avg_block_size,
            "block_height": self.block_height,
            "network_hashrate": self.network_hashrate,
            "tps_current": self.tps_current,
            "transaction_count_last_block": self.transaction_count_last_block,
            "attack_probability": self.attack_probability,
        });
        
        // Add error rates
        let mut error_rates = serde_json::Map::new();
        for (category, rate) in &self.error_rates {
            error_rates.insert(category.clone(), serde_json::json!(rate));
        }
        result["error_rates"] = serde_json::json!(error_rates);
        
        // Add BIP compliance
        let mut bip_compliance = serde_json::Map::new();
        for (bip, compliant) in &self.bip_compliance {
            bip_compliance.insert(bip.clone(), serde_json::json!(compliant));
        }
        result["bip_compliance"] = serde_json::json!(bip_compliance);
        
        // Add block versions
        let mut block_versions = serde_json::Map::new();
        for (version, count) in &self.block_versions {
            block_versions.insert(version.to_string(), serde_json::json!(count));
        }
        result["block_versions"] = serde_json::json!(block_versions);
        
        // Add hashrate distribution
        let mut hashrate_dist = serde_json::Map::new();
        for (miner, percentage) in &self.hashrate_distribution {
            hashrate_dist.insert(miner.clone(), serde_json::json!(percentage));
        }
        result["hashrate_distribution"] = serde_json::json!(hashrate_dist);
        
        // Add attack detection summary
        result["attack_detection"] = serde_json::json!(self.get_attack_detection_summary());
        
        result
    }
    
    /// Get historical data for a specific metric
    pub fn get_historical_data(&self, metric_name: &str) -> Option<Vec<TimeSample>> {
        match metric_name {
            "segwit_percentage" => Some(self.segwit_samples.clone()),
            "taproot_percentage" => Some(self.taproot_samples.clone()),
            "block_propagation_times" => Some(self.block_propagation_times.clone()),
            "tps" => Some(self.tps_samples.clone()),
            "block_versions" => Some(self.block_version_samples.clone()),
            _ => None,
        }
    }
}

/// Update SegWit adoption percentage
pub fn update_segwit_percentage(percentage: f64) {
    let mut metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.update_segwit_percentage(percentage);
    debug!("Updated SegWit adoption percentage to {:.2}%", percentage);
}

/// Update Taproot adoption percentage
pub fn update_taproot_percentage(percentage: f64) {
    let mut metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.update_taproot_percentage(percentage);
    debug!("Updated Taproot adoption percentage to {:.2}%", percentage);
}

/// Update UTXO set size
pub fn update_utxo_set_size(size: u64) {
    let mut metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.update_utxo_set_size(size);
    debug!("Updated UTXO set size to {}", size);
}

/// Update average fee rate
pub fn update_avg_fee_rate(sats_per_vb: f64) {
    let mut metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.update_avg_fee_rate(sats_per_vb);
    debug!("Updated average fee rate to {:.2} sats/vB", sats_per_vb);
}

/// Update error rate for a specific category
pub fn update_error_rate(category: &str, rate: f64) {
    let mut metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.update_error_rate(category, rate);
    
    // Log warning if error rate is high
    if rate > 0.05 { // 5% error rate threshold
        warn!("High error rate in {}: {:.2}%", category, rate * 100.0);
    }
}

/// Update mempool size
pub fn update_mempool_size(size_bytes: u64) {
    let mut metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.update_mempool_size(size_bytes);
    debug!("Updated mempool size to {} bytes", size_bytes);
}

/// Update average block size
pub fn update_avg_block_size(size_bytes: u64) {
    let mut metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.update_avg_block_size(size_bytes);
    debug!("Updated average block size to {} bytes", size_bytes);
}

/// Update block height
pub fn update_block_height(height: u64) {
    let mut metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.update_block_height(height);
    debug!("Updated block height to {}", height);
}

/// Update network hashrate
pub fn update_network_hashrate(eh_per_second: f64) {
    let mut metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.update_network_hashrate(eh_per_second);
    debug!("Updated network hashrate to {:.2} EH/s", eh_per_second);
}

/// Update block propagation time
pub fn update_block_propagation_time(block_hash: &str, milliseconds: u64) {
    let mut metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.update_block_propagation_time(block_hash, milliseconds);
    debug!("Block {} propagation time: {}ms", block_hash, milliseconds);
}

/// Set BIP compliance status
pub fn set_bip_compliance(bip_number: &str, compliant: bool) {
    let mut metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.set_bip_compliance(bip_number, compliant);
    debug!("BIP-{} compliance status: {}", bip_number, compliant);
}

/// Update TPS (Transactions Per Second) metrics
pub fn update_tps(transaction_count: u64, block_time_seconds: u64) {
    let mut metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.update_tps(transaction_count, block_time_seconds);
}

/// Update block version monitoring
pub fn update_block_version(version: u32) {
    let mut metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.update_block_version(version);
}

/// Update hashrate distribution for 51% attack detection
pub fn update_hashrate_distribution(miner_id: &str, hashrate_percentage: f64) {
    let mut metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.update_hashrate_distribution(miner_id, hashrate_percentage);
}

/// Record a new block for attack detection monitoring
pub fn record_block(block_timestamp: u64, miner_id: &str) {
    let mut metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.record_block(block_timestamp, miner_id);
}

/// Get attack detection summary
pub fn get_attack_detection_summary() -> HashMap<String, f64> {
    let metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.get_attack_detection_summary()
}

/// Get the current blockchain metrics as JSON
pub fn get_metrics_json() -> serde_json::Value {
    let metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.as_json()
}

/// Get historical data for a specific metric
pub fn get_historical_data(metric_name: &str) -> Option<Vec<TimeSample>> {
    let metrics = BLOCKCHAIN_METRICS.lock().unwrap();
    metrics.get_historical_data(metric_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_metrics() {
        // Update some metrics
        update_segwit_percentage(85.2);
        update_taproot_percentage(12.8);
        update_utxo_set_size(82_564_432);
        update_avg_fee_rate(25.5);
        update_error_rate("connection_failure", 0.02);
        update_mempool_size(15_000_000);
        update_avg_block_size(1_250_000);
        update_block_height(750_432);
        update_network_hashrate(305.7);
        update_block_propagation_time("000000000000000000035c5f8c0294672f8456bc", 235);
        set_bip_compliance("341", true);
        
        // Export and verify
        let json = get_metrics_json();
        
        assert_eq!(json["segwit_percentage"], 85.2);
        assert_eq!(json["taproot_percentage"], 12.8);
        assert_eq!(json["utxo_set_size"], 82_564_432);
        assert_eq!(json["avg_fee_rate"], 25.5);
        assert_eq!(json["error_rates"]["connection_failure"], 0.02);
        assert_eq!(json["mempool_size"], 15_000_000);
        assert_eq!(json["avg_block_size"], 1_250_000);
        assert_eq!(json["block_height"], 750_432);
        assert_eq!(json["network_hashrate"], 305.7);
        assert_eq!(json["bip_compliance"]["341"], true);
    }
}
