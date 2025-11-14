//! Integration tests for Analytics and Security ML modules
//!
//! Tests the functionality of the new Rust implementations that replace
//! the Python monitoring and ML scripts.

use anya_core::{
    analytics::{AnalyticsEngine, AnalyticsConfig, BitcoinMetrics, SystemMetrics},
    security_ml::{SecurityMLEngine, SecurityMLConfig, TransactionData, BlockData, HashrateData},
};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio;

#[tokio::test]
async fn test_analytics_engine_basic_functionality() {
    let config = AnalyticsConfig::default();
    let engine = AnalyticsEngine::new(config).await.unwrap();

    // Test metric recording
    let result = engine.record_metric("test_cpu_usage", 75.5, None).await;
    assert!(result.is_ok(), "Failed to record metric: {:?}", result);

    // Test metric recording with metadata
    let mut metadata = HashMap::new();
    metadata.insert("source".to_string(), "test".to_string());
    let result = engine.record_metric("test_memory_usage", 60.2, Some(metadata)).await;
    assert!(result.is_ok(), "Failed to record metric with metadata: {:?}", result);
}

#[tokio::test]
async fn test_bitcoin_metrics_analysis() {
    let config = AnalyticsConfig::default();
    let engine = AnalyticsEngine::new(config).await.unwrap();

    let metrics = BitcoinMetrics {
        mempool_depth: 25000,
        avg_fee_rate: 45.5,
        block_propagation_ms: 2500.0,
        peer_count: 12,
        network_hashrate: 300e18,
        difficulty: 50e12,
        unconfirmed_tx_count: 23000,
    };

    let patterns = engine.analyze_bitcoin_metrics(&metrics).await.unwrap();
    
    // Should not trigger any alerts for normal values
    assert!(patterns.is_empty() || patterns.iter().all(|p| p.confidence < 0.9));
}

#[tokio::test]
async fn test_bitcoin_metrics_high_fee_detection() {
    let config = AnalyticsConfig::default();
    let engine = AnalyticsEngine::new(config).await.unwrap();

    let metrics = BitcoinMetrics {
        mempool_depth: 150000, // High mempool
        avg_fee_rate: 250.0,   // Very high fee rate
        block_propagation_ms: 2500.0,
        peer_count: 12,
        network_hashrate: 300e18,
        difficulty: 50e12,
        unconfirmed_tx_count: 145000,
    };

    let patterns = engine.analyze_bitcoin_metrics(&metrics).await.unwrap();
    
    // Should detect fee spike and mempool congestion
    assert!(!patterns.is_empty(), "Should detect high fees and congestion");
    assert!(patterns.iter().any(|p| p.pattern_type == "fee_spike"));
    assert!(patterns.iter().any(|p| p.pattern_type == "mempool_congestion"));
}

#[tokio::test]
async fn test_system_metrics_analysis() {
    let config = AnalyticsConfig::default();
    let engine = AnalyticsEngine::new(config).await.unwrap();

    let metrics = SystemMetrics {
        cpu_usage_percent: 45.0,
        memory_usage_percent: 60.0,
        disk_usage_percent: 70.0,
        network_io_bps: 1000000.0,
        active_connections: 50,
        response_times_ms: (150.0, 300.0, 500.0),
    };

    let patterns = engine.analyze_system_metrics(&metrics).await.unwrap();
    
    // Normal values should not trigger alerts
    assert!(patterns.is_empty() || patterns.iter().all(|p| p.confidence < 0.9));
}

#[tokio::test]
async fn test_system_metrics_high_usage_detection() {
    let config = AnalyticsConfig::default();
    let engine = AnalyticsEngine::new(config).await.unwrap();

    let metrics = SystemMetrics {
        cpu_usage_percent: 95.0,  // Very high CPU
        memory_usage_percent: 92.0, // Very high memory
        disk_usage_percent: 70.0,
        network_io_bps: 1000000.0,
        active_connections: 50,
        response_times_ms: (150.0, 800.0, 2500.0), // High response times
    };

    let patterns = engine.analyze_system_metrics(&metrics).await.unwrap();
    
    // Should detect high resource usage
    assert!(!patterns.is_empty(), "Should detect high resource usage");
    assert!(patterns.iter().any(|p| p.pattern_type == "high_cpu_usage"));
    assert!(patterns.iter().any(|p| p.pattern_type == "high_memory_usage"));
    assert!(patterns.iter().any(|p| p.pattern_type == "high_response_times"));
}

#[tokio::test]
async fn test_anomaly_detection() {
    let config = AnalyticsConfig::default();
    let engine = AnalyticsEngine::new(config).await.unwrap();

    // Record some normal baseline data
    for i in 0..20 {
        let value = 50.0 + (i as f64 * 0.5); // Gradually increasing from 50 to 59.5
        engine.record_metric("cpu_usage", value, None).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }

    // Wait for data to be processed
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Now try to detect anomalies (this will likely find few since we need more data)
    let anomalies = engine.detect_anomalies("cpu_usage").await.unwrap();
    
    // The test is mainly to ensure the function doesn't crash
    // Real anomaly detection would need more sophisticated test data
    assert!(anomalies.len() <= 20, "Should not have more anomalies than data points");
}

#[tokio::test]
async fn test_analytics_report_generation() {
    let config = AnalyticsConfig::default();
    let engine = AnalyticsEngine::new(config).await.unwrap();

    // Record some test data
    engine.record_metric("test_metric_1", 100.0, None).await.unwrap();
    engine.record_metric("test_metric_2", 200.0, None).await.unwrap();
    engine.record_metric("test_metric_1", 105.0, None).await.unwrap();

    let report = engine.generate_report(1).await.unwrap(); // 1 hour window
    
    assert_eq!(report.time_range_hours, 1);
    assert!(!report.metric_summary.is_empty());
    assert!(report.generated_at > 0);
}

#[tokio::test]
async fn test_security_ml_engine_creation() {
    let config = SecurityMLConfig::default();
    let engine = SecurityMLEngine::new(config).await;
    assert!(engine.is_ok(), "Failed to create SecurityMLEngine: {:?}", engine);
}

#[tokio::test]
async fn test_fraud_detection_normal_transaction() {
    let config = SecurityMLConfig::default();
    let engine = SecurityMLEngine::new(config).await.unwrap();

    let tx = create_normal_transaction();
    let result = engine.analyze_transaction_fraud(&tx).await.unwrap();

    assert_eq!(result.transaction_id, tx.txid);
    assert!(!result.is_suspicious, "Normal transaction should not be flagged as suspicious");
    assert!(result.fraud_probability < 0.5, "Fraud probability should be low for normal transaction");
}

#[tokio::test]
async fn test_fraud_detection_suspicious_transaction() {
    let config = SecurityMLConfig::default();
    let engine = SecurityMLEngine::new(config).await.unwrap();

    let tx = create_suspicious_transaction();
    let result = engine.analyze_transaction_fraud(&tx).await.unwrap();

    assert_eq!(result.transaction_id, tx.txid);
    assert!(!result.risk_factors.is_empty(), "Suspicious transaction should have risk factors");
    // Note: is_suspicious depends on ML model output, so we don't assert on it
}

#[tokio::test]
async fn test_51_percent_attack_monitoring() {
    let config = SecurityMLConfig::default();
    let engine = SecurityMLEngine::new(config).await.unwrap();

    let blocks = create_normal_blocks();
    let hashrate = create_normal_hashrate_data();

    let result = engine.monitor_51_percent_attack(&blocks, &hashrate).await.unwrap();

    assert!(result.probability >= 0.0 && result.probability <= 1.0);
    assert!(!result.estimated_impact.is_empty());
    assert!(!result.mitigation_suggestions.is_empty());
}

#[tokio::test]
async fn test_fee_spike_analysis_normal_fees() {
    let config = SecurityMLConfig::default();
    let engine = SecurityMLEngine::new(config).await.unwrap();

    let current_fee_rate = 25.0;
    let historical_rates = vec![20.0, 22.0, 24.0, 26.0, 23.0, 25.0, 21.0];

    let result = engine.analyze_fee_spike(current_fee_rate, &historical_rates).await.unwrap();

    assert_eq!(result.current_fee_rate, current_fee_rate);
    assert!(!result.is_spike, "Normal fee should not be detected as spike");
    assert!(result.spike_magnitude < 1.0, "Spike magnitude should be low for normal fees");
}

#[tokio::test]
async fn test_fee_spike_analysis_actual_spike() {
    let config = SecurityMLConfig::default();
    let engine = SecurityMLEngine::new(config).await.unwrap();

    let current_fee_rate = 250.0; // Very high fee
    let historical_rates = vec![20.0, 22.0, 24.0, 26.0, 23.0, 25.0, 21.0];

    let result = engine.analyze_fee_spike(current_fee_rate, &historical_rates).await.unwrap();

    assert_eq!(result.current_fee_rate, current_fee_rate);
    assert!(result.is_spike, "High fee should be detected as spike");
    assert!(result.spike_magnitude > 2.0, "Spike magnitude should be high");
    assert!(!result.cause_analysis.is_empty(), "Should provide cause analysis");
}

#[tokio::test]
async fn test_security_alerts_retrieval() {
    let config = SecurityMLConfig::default();
    let engine = SecurityMLEngine::new(config).await.unwrap();

    // Process some transactions to potentially generate alerts
    let tx1 = create_normal_transaction();
    let tx2 = create_suspicious_transaction();

    let _result1 = engine.analyze_transaction_fraud(&tx1).await.unwrap();
    let _result2 = engine.analyze_transaction_fraud(&tx2).await.unwrap();

    let alerts = engine.get_recent_alerts(Some(10)).await;
    
    // Alerts depend on ML model outputs, so we just check the structure
    assert!(alerts.len() <= 10, "Should respect limit parameter");
}

#[tokio::test]
async fn test_performance_benchmarks() {
    // Test performance of analytics operations
    let config = AnalyticsConfig::default();
    let engine = AnalyticsEngine::new(config).await.unwrap();

    let start = std::time::Instant::now();
    
    // Record 1000 metrics
    for i in 0..1000 {
        engine.record_metric("performance_test", i as f64, None).await.unwrap();
    }
    
    let elapsed = start.elapsed();
    println!("Recorded 1000 metrics in {:?}", elapsed);
    
    // Should be able to record at least 100 metrics per second
    assert!(elapsed.as_secs_f64() < 10.0, "Recording 1000 metrics took too long: {:?}", elapsed);
}

// Helper functions to create test data

fn create_normal_transaction() -> TransactionData {
    TransactionData {
        txid: "normal_tx_123".to_string(),
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        input_count: 2,
        output_count: 2,
        fee_rate: 25.0,
        total_value: 100_000_000, // 1 BTC
        is_rbf: false,
        has_witness: true,
        size_bytes: 250,
        confirmations: 0,
        input_addresses: vec!["1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string()],
        output_addresses: vec![
            "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2".to_string(),
            "1HLoD9E4SDFFPDiYfNYnkBLQ85Y51J3Zb1".to_string(),
        ],
    }
}

fn create_suspicious_transaction() -> TransactionData {
    TransactionData {
        txid: "suspicious_tx_456".to_string(),
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        input_count: 50, // Unusually high input count
        output_count: 1,
        fee_rate: 500.0, // Very high fee rate
        total_value: 2_000_000_000, // 20 BTC - high value
        is_rbf: true,
        has_witness: true,
        size_bytes: 5000, // Large transaction
        confirmations: 0,
        input_addresses: (0..50).map(|i| format!("addr_input_{}", i)).collect(),
        output_addresses: vec!["output_addr_1".to_string()],
    }
}

fn create_normal_blocks() -> Vec<BlockData> {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    (0..10).map(|i| BlockData {
        height: 800000 + i,
        hash: format!("block_hash_{}", i),
        timestamp: current_time - (10 - i) * 600, // 10 minute intervals
        size_bytes: 1_000_000 + i * 50000,
        tx_count: 2000 + i * 100,
        total_fees: 50_000_000 + i * 5_000_000,
        miner_address: Some(format!("miner_{}", i % 3)), // 3 different miners
        difficulty: 50e12,
        previous_hash: format!("prev_block_hash_{}", i.saturating_sub(1)),
        merkle_root: format!("merkle_root_{}", i),
    }).collect()
}

fn create_normal_hashrate_data() -> Vec<HashrateData> {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    (0..10).map(|i| HashrateData {
        timestamp: current_time - (10 - i) * 600,
        estimated_hashrate: 300e18 + (i as f64 * 5e18), // Gradually increasing hashrate
        difficulty: 50e12,
        block_interval_seconds: 600.0 + (i as f64 * 10.0),
        network_blocks_per_hour: 6.0,
    }).collect()
}