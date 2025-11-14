//! Performance benchmarks for Analytics and Security ML modules
//!
//! Measures performance improvements of Rust implementations over Python equivalents

use anya_core::{
    analytics::{AnalyticsEngine, AnalyticsConfig, BitcoinMetrics, SystemMetrics},
    security_ml::{SecurityMLEngine, SecurityMLConfig, TransactionData},
};
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Runtime;

fn create_runtime() -> Runtime {
    Runtime::new().unwrap()
}

fn benchmark_metric_recording(c: &mut Criterion) {
    let rt = create_runtime();
    
    c.bench_function("analytics_metric_recording", |b| {
        b.to_async(&rt).iter(|| async {
            let config = AnalyticsConfig::default();
            let engine = AnalyticsEngine::new(config).await.unwrap();
            
            // Record 100 metrics
            for i in 0..100 {
                engine.record_metric(
                    "benchmark_metric", 
                    black_box(i as f64), 
                    None
                ).await.unwrap();
            }
        });
    });
}

fn benchmark_bitcoin_metrics_analysis(c: &mut Criterion) {
    let rt = create_runtime();
    
    let metrics = BitcoinMetrics {
        mempool_depth: 25000,
        avg_fee_rate: 45.5,
        block_propagation_ms: 2500.0,
        peer_count: 12,
        network_hashrate: 300e18,
        difficulty: 50e12,
        unconfirmed_tx_count: 23000,
    };
    
    c.bench_function("bitcoin_metrics_analysis", |b| {
        b.to_async(&rt).iter(|| async {
            let config = AnalyticsConfig::default();
            let engine = AnalyticsEngine::new(config).await.unwrap();
            
            let _patterns = engine.analyze_bitcoin_metrics(black_box(&metrics)).await.unwrap();
        });
    });
}

fn benchmark_system_metrics_analysis(c: &mut Criterion) {
    let rt = create_runtime();
    
    let metrics = SystemMetrics {
        cpu_usage_percent: 45.0,
        memory_usage_percent: 60.0,
        disk_usage_percent: 70.0,
        network_io_bps: 1000000.0,
        active_connections: 50,
        response_times_ms: (150.0, 300.0, 500.0),
    };
    
    c.bench_function("system_metrics_analysis", |b| {
        b.to_async(&rt).iter(|| async {
            let config = AnalyticsConfig::default();
            let engine = AnalyticsEngine::new(config).await.unwrap();
            
            let _patterns = engine.analyze_system_metrics(black_box(&metrics)).await.unwrap();
        });
    });
}

fn benchmark_fraud_detection(c: &mut Criterion) {
    let rt = create_runtime();
    
    c.bench_function("fraud_detection", |b| {
        b.to_async(&rt).iter(|| async {
            let config = SecurityMLConfig::default();
            let engine = SecurityMLEngine::new(config).await.unwrap();
            
            let tx = create_benchmark_transaction();
            let _result = engine.analyze_transaction_fraud(black_box(&tx)).await.unwrap();
        });
    });
}

fn benchmark_fraud_detection_batch(c: &mut Criterion) {
    let rt = create_runtime();
    
    let mut group = c.benchmark_group("fraud_detection_batch");
    
    for batch_size in [10, 50, 100, 500].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(batch_size), batch_size, |b, &size| {
            b.to_async(&rt).iter(|| async {
                let config = SecurityMLConfig::default();
                let engine = SecurityMLEngine::new(config).await.unwrap();
                
                for _ in 0..size {
                    let tx = create_benchmark_transaction();
                    let _result = engine.analyze_transaction_fraud(black_box(&tx)).await.unwrap();
                }
            });
        });
    }
    group.finish();
}

fn benchmark_fee_spike_analysis(c: &mut Criterion) {
    let rt = create_runtime();
    
    let historical_rates = vec![20.0, 22.0, 24.0, 26.0, 23.0, 25.0, 21.0, 19.0, 27.0, 23.5];
    
    c.bench_function("fee_spike_analysis", |b| {
        b.to_async(&rt).iter(|| async {
            let config = SecurityMLConfig::default();
            let engine = SecurityMLEngine::new(config).await.unwrap();
            
            let _result = engine.analyze_fee_spike(
                black_box(250.0), 
                black_box(&historical_rates)
            ).await.unwrap();
        });
    });
}

fn benchmark_anomaly_detection(c: &mut Criterion) {
    let rt = create_runtime();
    
    c.bench_function("anomaly_detection", |b| {
        b.to_async(&rt).iter(|| async {
            let config = AnalyticsConfig::default();
            let engine = AnalyticsEngine::new(config).await.unwrap();
            
            // Pre-populate with some data
            for i in 0..50 {
                engine.record_metric("test_metric", i as f64, None).await.unwrap();
            }
            
            let _anomalies = engine.detect_anomalies(black_box("test_metric")).await.unwrap();
        });
    });
}

fn benchmark_report_generation(c: &mut Criterion) {
    let rt = create_runtime();
    
    c.bench_function("analytics_report_generation", |b| {
        b.to_async(&rt).iter(|| async {
            let config = AnalyticsConfig::default();
            let engine = AnalyticsEngine::new(config).await.unwrap();
            
            // Pre-populate with test data
            for i in 0..100 {
                let mut metadata = HashMap::new();
                metadata.insert("test_run".to_string(), i.to_string());
                
                engine.record_metric("cpu_usage", (i % 100) as f64, Some(metadata.clone())).await.unwrap();
                engine.record_metric("memory_usage", ((i * 2) % 100) as f64, Some(metadata.clone())).await.unwrap();
                engine.record_metric("disk_usage", ((i * 3) % 100) as f64, Some(metadata)).await.unwrap();
            }
            
            let _report = engine.generate_report(black_box(1)).await.unwrap();
        });
    });
}

fn benchmark_concurrent_operations(c: &mut Criterion) {
    let rt = create_runtime();
    
    c.bench_function("concurrent_metric_recording", |b| {
        b.to_async(&rt).iter(|| async {
            let config = AnalyticsConfig::default();
            let engine = AnalyticsEngine::new(config).await.unwrap();
            
            // Simulate concurrent metric recording from multiple sources
            let mut handles = Vec::new();
            
            for thread_id in 0..10 {
                let engine_clone = std::sync::Arc::new(engine);
                let handle = tokio::spawn(async move {
                    for i in 0..10 {
                        let metric_name = format!("thread_{}_metric", thread_id);
                        engine_clone.record_metric(&metric_name, i as f64, None).await.unwrap();
                    }
                });
                handles.push(handle);
            }
            
            // Wait for all tasks to complete
            for handle in handles {
                handle.await.unwrap();
            }
        });
    });
}

fn benchmark_memory_usage(c: &mut Criterion) {
    let rt = create_runtime();
    
    c.bench_function("memory_efficiency", |b| {
        b.to_async(&rt).iter(|| async {
            let config = AnalyticsConfig {
                anomaly_detection_enabled: true,
                anomaly_threshold: 0.8,
                time_window_seconds: 300,
                max_data_points: 1000, // Limit data points for memory test
                real_time_analysis: false, // Disable to focus on storage
            };
            let engine = AnalyticsEngine::new(config).await.unwrap();
            
            // Fill up to max capacity
            for i in 0..1000 {
                engine.record_metric("memory_test", i as f64, None).await.unwrap();
            }
            
            // Test that it handles overflow correctly
            for i in 1000..1100 {
                engine.record_metric("memory_test", i as f64, None).await.unwrap();
            }
        });
    });
}

// Comparison benchmark simulating Python equivalent operations
fn benchmark_python_equivalent_operations(c: &mut Criterion) {
    let rt = create_runtime();
    
    c.bench_function("python_equivalent_monitoring", |b| {
        b.to_async(&rt).iter(|| async {
            // This simulates the operations that would be done in Python
            // Based on the monitor.py script we analyzed
            
            let config = AnalyticsConfig::default();
            let engine = AnalyticsEngine::new(config).await.unwrap();
            
            // Simulate system metrics collection (like psutil calls)
            let system_metrics = SystemMetrics {
                cpu_usage_percent: black_box({
                    use rand::Rng;
                    rand::thread_rng().gen_range(0.0..100.0)
                }),
                memory_usage_percent: black_box({
                    use rand::Rng;
                    rand::thread_rng().gen_range(0.0..100.0)
                }),
                disk_usage_percent: black_box({
                    use rand::Rng;
                    rand::thread_rng().gen_range(0.0..100.0)
                }),
                network_io_bps: black_box({
                    use rand::Rng;
                    rand::thread_rng().gen_range(0.0..1000000.0)
                }),
                active_connections: black_box({
                    use rand::Rng;
                    rand::thread_rng().gen_range(0..1000)
                }),
                response_times_ms: (50.0, 150.0, 300.0),
            };
            
            // Simulate Bitcoin metrics collection (like RPC calls)
            let bitcoin_metrics = BitcoinMetrics {
                mempool_depth: black_box({
                    use rand::Rng;
                    rand::thread_rng().gen_range(0..100000)
                }),
                avg_fee_rate: black_box({
                    use rand::Rng;
                    rand::thread_rng().gen_range(0.0..200.0)
                }),
                block_propagation_ms: black_box({
                    use rand::Rng;
                    rand::thread_rng().gen_range(0.0..5000.0)
                }),
                peer_count: black_box({
                    use rand::Rng;
                    8 + rand::thread_rng().gen_range(0..50)
                }),
                network_hashrate: 300e18,
                difficulty: 50e12,
                unconfirmed_tx_count: black_box({
                    use rand::Rng;
                    rand::thread_rng().gen_range(0..50000)
                }),
            };
            
            // Record metrics (equivalent to Python metric storage)
            engine.record_metric("system.cpu_usage", system_metrics.cpu_usage_percent, None).await.unwrap();
            engine.record_metric("system.memory_usage", system_metrics.memory_usage_percent, None).await.unwrap();
            engine.record_metric("bitcoin.mempool_depth", bitcoin_metrics.mempool_depth as f64, None).await.unwrap();
            engine.record_metric("bitcoin.avg_fee_rate", bitcoin_metrics.avg_fee_rate, None).await.unwrap();
            
            // Analyze patterns (equivalent to Python analysis)
            let _sys_patterns = engine.analyze_system_metrics(&system_metrics).await.unwrap();
            let _btc_patterns = engine.analyze_bitcoin_metrics(&bitcoin_metrics).await.unwrap();
        });
    });
}

// Helper function to create test transaction
fn create_benchmark_transaction() -> TransactionData {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    TransactionData {
        txid: format!("benchmark_tx_{}", rng.gen::<u64>()),
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        input_count: 1 + rng.gen_range(0..5),
        output_count: 1 + rng.gen_range(0..3),
        fee_rate: 10.0 + rng.gen_range(0.0..40.0),
        total_value: 50_000_000 + rng.gen_range(0..200_000_000),
        is_rbf: rng.gen::<bool>(),
        has_witness: true,
        size_bytes: 200 + rng.gen_range(0..300),
        confirmations: 0,
        input_addresses: vec!["benchmark_addr_1".to_string()],
        output_addresses: vec!["benchmark_addr_2".to_string()],
    }
}

criterion_group!(
    benches,
    benchmark_metric_recording,
    benchmark_bitcoin_metrics_analysis,
    benchmark_system_metrics_analysis,
    benchmark_fraud_detection,
    benchmark_fraud_detection_batch,
    benchmark_fee_spike_analysis,
    benchmark_anomaly_detection,
    benchmark_report_generation,
    benchmark_concurrent_operations,
    benchmark_memory_usage,
    benchmark_python_equivalent_operations
);

criterion_main!(benches)