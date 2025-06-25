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
        
        result
    }
    
    /// Get historical data for a specific metric
    pub fn get_historical_data(&self, metric_name: &str) -> Option<Vec<TimeSample>> {
        match metric_name {
            "segwit_percentage" => Some(self.segwit_samples.clone()),
            "taproot_percentage" => Some(self.taproot_samples.clone()),
            "block_propagation_times" => Some(self.block_propagation_times.clone()),
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
