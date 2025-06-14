use anya_core::core::system_awareness::{
    AlertSeverity, BlockInfo, BlockVersionMetrics, FeeAnalysis, FeeStats, MempoolInfo,
    MempoolStatus, NetworkClient, NetworkStateMonitor, Result, SecurityAlert, SecurityAlertType,
    SecurityMonitor,
};
use std::collections::HashMap;

// Mock network client for testing
struct MockNetworkClient {
    mempool_size: usize,
    tx_count: usize,
    fee_rates: FeeStats,
    blocks: Vec<BlockInfo>,
}

impl NetworkClient for MockNetworkClient {
    fn get_mempool_info(&self) -> Result<MempoolInfo> {
        Ok(MempoolInfo {
            size_bytes: self.mempool_size,
            tx_count: self.tx_count,
            fee_stats: self.fee_rates.clone(),
        })
    }

    fn get_recent_blocks(&self, count: usize) -> Result<Vec<BlockInfo>> {
        Ok(self.blocks.iter().take(count).cloned().collect())
    }
}

impl MockNetworkClient {
    fn new() -> Self {
        Self {
            mempool_size: 150 * 1024, // 150KB
            tx_count: 1000,
            fee_rates: FeeStats {
                min_fee_rate: 1.0,
                median_fee_rate: 3.0,
                max_fee_rate: 10.0,
            },
            blocks: vec![
                BlockInfo {
                    hash: "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
                        .to_string(),
                    height: 0,
                    version: 1,
                    miner: Some("Satoshi".to_string()),
                },
                BlockInfo {
                    hash: "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26e"
                        .to_string(),
                    height: 1,
                    version: 1,
                    miner: Some("Satoshi".to_string()),
                },
                BlockInfo {
                    hash: "0000000069e244f73d78e8fd29ba2fd2ed618bd6fa2ee92559f542fdb26e7c1d"
                        .to_string(),
                    height: 2,
                    version: 2,
                    miner: Some("Unknown".to_string()),
                },
            ],
        }
    }
}

#[test]
fn test_mempool_monitoring() {
    let mut monitor = NetworkStateMonitor::new(100, 200.0); // 100KB threshold, 200% fee spike
    let client = Box::new(MockNetworkClient::new());
    monitor.set_client(client);

    let result = monitor.monitor_mempool_depth();
    assert!(result.is_ok(), "Mempool monitoring failed");

    let status = result.unwrap();
    assert_eq!(status.size_bytes, 150 * 1024);
    assert_eq!(status.tx_count, 1000);
    assert!(status.alert, "Alert should be true when over threshold");
}

#[test]
fn test_block_version_tracking() {
    let mut monitor = NetworkStateMonitor::new(100, 200.0);
    let client = Box::new(MockNetworkClient::new());
    monitor.set_client(client);

    let result = monitor.track_block_version();
    assert!(result.is_ok(), "Block version tracking failed");

    let metrics = result.unwrap();
    assert_eq!(metrics.current_version, 1); // Version 1 appears twice in mock data
    assert_eq!(metrics.version_distribution.len(), 2);
    assert_eq!(*metrics.version_distribution.get(&1).unwrap(), 2);
    assert_eq!(*metrics.version_distribution.get(&2).unwrap(), 1);
    assert!((metrics.current_version_percentage - 66.67).abs() < 0.1);
}

#[test]
fn test_fee_spike_detection() {
    let mut monitor = NetworkStateMonitor::new(100, 200.0);
    let client = Box::new(MockNetworkClient::new());
    monitor.set_client(client);

    // Update historical fees
    monitor.update_historical_fees(1.0);

    let security_monitor = SecurityMonitor::new(monitor, 60.0); // 60% attack threshold

    let result = security_monitor.analyze_fee_spike();
    assert!(result.is_ok(), "Fee spike analysis failed");

    let analysis = result.unwrap();
    assert_eq!(analysis.current_median_fee, 3.0);
    assert_eq!(analysis.historical_median_fee, 1.0);
    assert_eq!(analysis.percentage_increase, 200.0);
    assert!(analysis.is_spike, "Should detect fee spike");
}

#[test]
fn test_51_percent_attack_detection() {
    let mut monitor = NetworkStateMonitor::new(100, 200.0);
    let client = Box::new(MockNetworkClient::new());
    monitor.set_client(client);

    let security_monitor = SecurityMonitor::new(monitor, 60.0); // 60% attack threshold

    let result = security_monitor.detect_51_percent_attack();
    assert!(result.is_ok(), "Attack detection failed");

    let alert = result.unwrap();
    assert_eq!(alert.alert_type, SecurityAlertType::AttackPotential51);

    // Since our mock data has 66.67% blocks with version 1, and our threshold is 60%,
    // this should trigger a critical alert
    assert_eq!(alert.severity, AlertSeverity::Critical);
}
