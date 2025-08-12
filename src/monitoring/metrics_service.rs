use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use log::{debug, error, info, warn};

use crate::monitoring::blockchain_metrics;
use crate::monitoring::blockchain_alerts;
use crate::monitoring::system_metrics::SystemMetricsCollector;

// Default interval for metrics collection (in milliseconds)
const DEFAULT_METRICS_INTERVAL_MS: u64 = 10000; // 10 seconds

/// Metrics collection and monitoring service
pub struct MetricsService {
    /// Collection interval in milliseconds
    interval_ms: u64,
    
    /// Whether the service is running
    running: Arc<Mutex<bool>>,
    
    /// Last collection time
    last_collection: Arc<Mutex<Instant>>,
    
    /// System metrics collector
    system_collector: Arc<Mutex<SystemMetricsCollector>>,
}

impl MetricsService {
    /// Create a new metrics service
    pub fn new(interval_ms: Option<u64>) -> Self {
        Self {
            interval_ms: interval_ms.unwrap_or(DEFAULT_METRICS_INTERVAL_MS),
            running: Arc::new(Mutex::new(false)),
            last_collection: Arc::new(Mutex::new(Instant::now())),
            system_collector: Arc::new(Mutex::new(SystemMetricsCollector::new())),
        }
    }
    
    /// Start the metrics collection service
    pub fn start(&self) {
        let mut running = self.running.lock().unwrap();
        
        if *running {
            warn!("Metrics service is already running");
            return;
        }
        
        *running = true;
        info!("Starting blockchain metrics service with interval of {}ms", self.interval_ms);
        
        // Clone the Arc references for the thread
        let running_clone = Arc::clone(&self.running);
        let last_collection_clone = Arc::clone(&self.last_collection);
        let system_collector_clone = Arc::clone(&self.system_collector);
        let interval_ms = self.interval_ms;
        
        // Spawn collection thread
        thread::spawn(move || {
            while *running_clone.lock().unwrap() {
                // Collect metrics
                Self::collect_metrics(&system_collector_clone);
                
                // Check alerts
                blockchain_alerts::check_alerts();
                
                // Update last collection time
                *last_collection_clone.lock().unwrap() = Instant::now();
                
                // Sleep for the configured interval
                thread::sleep(Duration::from_millis(interval_ms));
            }
            
            info!("Blockchain metrics service stopped");
        });
    }
    
    /// Stop the metrics collection service
    pub fn stop(&self) {
        let mut running = self.running.lock().unwrap();
        *running = false;
        info!("Stopping blockchain metrics service");
    }
    
    /// Collect all blockchain metrics
    fn collect_metrics(system_collector: &Arc<Mutex<SystemMetricsCollector>>) {
        debug!("Collecting blockchain and system metrics...");
        
        // Collect real system metrics using sysinfo
        if let Ok(mut collector) = system_collector.lock() {
            collector.collect_system_metrics();
        } else {
            error!("Failed to acquire lock on system metrics collector");
        }
        
        // Keep Bitcoin-specific metrics simulation for now
        // These should eventually be replaced with real Bitcoin Core RPC calls
        Self::collect_simulated_bitcoin_metrics();
    }
    
    /// Collect simulated Bitcoin blockchain metrics for demonstration
    /// TODO: Replace these with real Bitcoin Core RPC calls in a future update
    fn collect_simulated_bitcoin_metrics() {
        info!("Collecting simulated Bitcoin blockchain metrics (not system metrics)");
        
        // NOTE: These are Bitcoin blockchain metrics that require a Bitcoin Core node connection
        // They are kept simulated until proper Bitcoin Core RPC integration is implemented
        
        // Simulated SegWit adoption percentage (random variation between 82-87%)
        let segwit_pct = 85.0 + (rand::random::<f64>() - 0.5) * 5.0;
        blockchain_metrics::update_segwit_percentage(segwit_pct);
        
        // Simulated Taproot adoption percentage (random variation between 11-14%)
        let taproot_pct = 12.5 + (rand::random::<f64>() - 0.5) * 3.0;
        blockchain_metrics::update_taproot_percentage(taproot_pct);
        
        // Simulated UTXO set size (random variation around 82.5M)
        let utxo_size = 82_500_000 + (rand::random::<f64>() * 100_000.0) as u64;
        blockchain_metrics::update_utxo_set_size(utxo_size);
        
        // Simulated average fee rate (sats/vB) with some randomness
        let fee_rate = 20.0 + (rand::random::<f64>() - 0.5) * 10.0;
        blockchain_metrics::update_avg_fee_rate(fee_rate);
        
        // Simulated connection error rates (usually low, but occasionally spikes)
        let conn_error_rate = if rand::random::<f64>() < 0.95 {
            // Normal case: 0-2% error rate
            rand::random::<f64>() * 0.02
        } else {
            // Occasional spike: 5-10% error rate
            0.05 + rand::random::<f64>() * 0.05
        };
        blockchain_metrics::update_error_rate("connection_failure", conn_error_rate);
        
        // Simulated mempool size
        let mempool_size = 15_000_000 + (rand::random::<f64>() * 10_000_000.0) as u64;
        blockchain_metrics::update_mempool_size(mempool_size);
        
        // Simulated average block size
        let avg_block_size = 1_200_000 + (rand::random::<f64>() * 200_000.0) as u64;
        blockchain_metrics::update_avg_block_size(avg_block_size);
        
        // Block height increases occasionally
        static mut LAST_BLOCK_HEIGHT: u64 = 750_432;
        let new_block = rand::random::<f64>() < 0.1; // 10% chance of new block
        
        unsafe {
            if new_block {
                LAST_BLOCK_HEIGHT += 1;
                
                // Also update propagation time for new blocks
                let block_hash = format!("000000000000000000{:x}", rand::random::<u32>());
                let propagation_time = 200 + (rand::random::<f64>() * 300.0) as u64;
                blockchain_metrics::update_block_propagation_time(&block_hash, propagation_time);
            }
            
            blockchain_metrics::update_block_height(LAST_BLOCK_HEIGHT);
        }
        
        // Network hashrate
        let hashrate = 300.0 + (rand::random::<f64>() - 0.5) * 15.0;
        blockchain_metrics::update_network_hashrate(hashrate);
        
        // BIP compliance (mostly static)
        blockchain_metrics::set_bip_compliance("341", true); // Taproot
        blockchain_metrics::set_bip_compliance("342", true); // Taproot validation
        blockchain_metrics::set_bip_compliance("174", true); // SegWit
    }
}

/// Gets metrics collection interval from environment or uses default
pub fn get_metrics_interval() -> u64 {
    std::env::var("ANYA_METRICS_COLLECTION_INTERVAL_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_METRICS_INTERVAL_MS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_service() {
        // Create service with a short interval
        let service = MetricsService::new(Some(100));
        
        // Start service
        service.start();
        
        // Let it run for a moment
        thread::sleep(Duration::from_millis(300));
        
        // Stop service
        service.stop();
        
        // Verify metrics were collected at least once
        let metrics = blockchain_metrics::get_metrics_json();
        assert!(metrics["segwit_percentage"].as_f64().unwrap() > 0.0);
    }
}
