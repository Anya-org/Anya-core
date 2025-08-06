# Performance Testing Guide [AIR-3][AIS-3][AIT-3][RES-3]

Comprehensive performance testing methodology for Anya-core extensions, ensuring optimal throughput, latency, and resource utilization across Bitcoin, Web5, and ML systems.

## Overview

Performance testing validates that Anya-core extensions meet strict performance requirements for Bitcoin transaction processing, Web5 protocol operations, and ML inference pipelines. All tests must demonstrate BIP compliance under load and maintain security standards at scale.

## Performance Testing Architecture

### Test Categories
- **Load Testing**: Normal operational capacity validation
- **Stress Testing**: System limits and breaking points
- **Volume Testing**: Large dataset processing capabilities
- **Spike Testing**: Sudden load increase handling
- **Endurance Testing**: Long-term stability validation
- **Scalability Testing**: Horizontal and vertical scaling behavior

### Performance Metrics
```rust
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub throughput: f64,           // Operations per second
    pub latency_p50: Duration,     // 50th percentile latency
    pub latency_p95: Duration,     // 95th percentile latency
    pub latency_p99: Duration,     // 99th percentile latency
    pub memory_usage: u64,         // Peak memory in bytes
    pub cpu_usage: f64,            // Average CPU utilization
    pub error_rate: f64,           // Percentage of failed operations
}
```

## Bitcoin Performance Testing

### Transaction Processing Benchmarks
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use bitcoin::{Transaction, BlockHash};
use anya_core::bitcoin::TransactionProcessor;

fn benchmark_transaction_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("bitcoin_validation");
    
    // Test different transaction sizes
    for tx_size in [250, 500, 1000, 2000].iter() {
        let tx = create_transaction_with_size(*tx_size);
        
        group.bench_with_input(
            BenchmarkId::new("validate_transaction", tx_size),
            &tx,
            |b, tx| {
                let processor = TransactionProcessor::new();
                b.iter(|| processor.validate_transaction(black_box(tx)))
            },
        );
    }
    
    group.finish();
}

fn benchmark_signature_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("signature_verification");
    
    // Test different signature types
    for sig_type in ["p2pkh", "p2wpkh", "p2sh", "p2wsh"].iter() {
        let tx = create_transaction_with_signature_type(sig_type);
        
        group.bench_with_input(
            BenchmarkId::new("verify_signature", sig_type),
            &tx,
            |b, tx| {
                let verifier = SignatureVerifier::new();
                b.iter(|| verifier.verify_signatures(black_box(tx)))
            },
        );
    }
    
    group.finish();
}

#[tokio::main]
async fn benchmark_block_processing() {
    let processor = BlockProcessor::new();
    let test_blocks = load_test_blocks(100); // 100 real testnet blocks
    
    let start_time = Instant::now();
    let mut processed_count = 0;
    
    for block in test_blocks {
        let result = processor.process_block(&block).await;
        assert!(result.is_ok());
        processed_count += 1;
        
        if processed_count % 10 == 0 {
            let elapsed = start_time.elapsed();
            let blocks_per_sec = processed_count as f64 / elapsed.as_secs_f64();
            println!("Processed {} blocks at {:.2} blocks/sec", processed_count, blocks_per_sec);
        }
    }
    
    let total_time = start_time.elapsed();
    let final_throughput = processed_count as f64 / total_time.as_secs_f64();
    
    assert!(final_throughput > 50.0); // Minimum 50 blocks/sec
    println!("Final throughput: {:.2} blocks/sec", final_throughput);
}

criterion_group!(benches, benchmark_transaction_validation, benchmark_signature_verification);
criterion_main!(benches);
```

### Network Performance Testing
```rust
#[tokio::test]
async fn test_bitcoin_network_throughput() {
    let client = BitcoinClient::new_testnet().await.unwrap();
    let transaction_count = 1000;
    let concurrent_requests = 50;
    
    let transactions: Vec<_> = (0..transaction_count)
        .map(|_| create_test_transaction())
        .collect();
    
    let semaphore = Arc::new(Semaphore::new(concurrent_requests));
    let start_time = Instant::now();
    let success_count = Arc::new(AtomicUsize::new(0));
    
    let tasks: Vec<_> = transactions.into_iter().map(|tx| {
        let client = client.clone();
        let semaphore = semaphore.clone();
        let success_count = success_count.clone();
        
        tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            
            match client.broadcast_transaction(&tx).await {
                Ok(_) => {
                    success_count.fetch_add(1, Ordering::Relaxed);
                }
                Err(e) => eprintln!("Transaction failed: {}", e),
            }
        })
    }).collect();
    
    futures::future::join_all(tasks).await;
    
    let elapsed = start_time.elapsed();
    let successful_txs = success_count.load(Ordering::Relaxed);
    let throughput = successful_txs as f64 / elapsed.as_secs_f64();
    
    println!("Bitcoin network throughput: {:.2} TPS", throughput);
    assert!(throughput > 10.0); // Minimum 10 TPS for testnet
    assert!(successful_txs as f64 / transaction_count as f64 > 0.95); // 95% success rate
}
```

### Memory Usage Profiling
```rust
#[test]
fn test_bitcoin_memory_usage() {
    let initial_memory = get_memory_usage();
    
    // Process large number of transactions
    let processor = TransactionProcessor::new();
    let transactions = create_large_transaction_set(10_000);
    
    for tx in &transactions {
        processor.validate_transaction(tx).unwrap();
    }
    
    let peak_memory = get_memory_usage();
    let memory_increase = peak_memory - initial_memory;
    
    // Memory should not increase linearly with transaction count
    assert!(memory_increase < 100 * 1024 * 1024); // Less than 100MB
    
    // Clean up and verify memory is released
    drop(processor);
    drop(transactions);
    
    // Force garbage collection
    std::thread::sleep(Duration::from_millis(100));
    
    let final_memory = get_memory_usage();
    let memory_leak = final_memory - initial_memory;
    
    assert!(memory_leak < 10 * 1024 * 1024); // Less than 10MB leak
}
```

## Web5 Performance Testing

### DID Resolution Performance
```rust
#[tokio::test]
async fn test_did_resolution_performance() {
    let resolver = DidResolver::new_with_cache(1000); // 1000 entry cache
    let test_dids: Vec<_> = (0..100)
        .map(|i| DID::parse(&format!("did:web5:testnet:user{}", i)).unwrap())
        .collect();
    
    // Warm up cache
    for did in &test_dids[0..10] {
        resolver.resolve(did).await.unwrap();
    }
    
    let start_time = Instant::now();
    let mut resolution_times = Vec::new();
    
    for did in &test_dids {
        let resolve_start = Instant::now();
        let result = resolver.resolve(did).await;
        let resolve_time = resolve_start.elapsed();
        
        assert!(result.is_ok());
        resolution_times.push(resolve_time);
    }
    
    let total_time = start_time.elapsed();
    let avg_resolution_time = resolution_times.iter().sum::<Duration>() / resolution_times.len() as u32;
    let resolutions_per_sec = test_dids.len() as f64 / total_time.as_secs_f64();
    
    println!("DID resolution performance:");
    println!("  Average resolution time: {:?}", avg_resolution_time);
    println!("  Resolutions per second: {:.2}", resolutions_per_sec);
    
    assert!(avg_resolution_time < Duration::from_millis(100)); // Under 100ms average
    assert!(resolutions_per_sec > 50.0); // At least 50 resolutions/sec
}
```

### DWN Message Processing Performance
```rust
#[tokio::test]
async fn test_dwn_message_throughput() {
    let dwn_client = DWNClient::new("https://dwn.testnet.web5.com").await.unwrap();
    let did = create_test_did().await;
    let message_count = 500;
    let concurrent_writes = 20;
    
    let messages: Vec<_> = (0..message_count)
        .map(|i| Message::builder()
            .protocol("https://example.com/performance-test")
            .data(format!("Test message {}", i).as_bytes())
            .build()
            .unwrap())
        .collect();
    
    let semaphore = Arc::new(Semaphore::new(concurrent_writes));
    let start_time = Instant::now();
    let success_count = Arc::new(AtomicUsize::new(0));
    
    let tasks: Vec<_> = messages.into_iter().map(|message| {
        let dwn_client = dwn_client.clone();
        let did = did.clone();
        let semaphore = semaphore.clone();
        let success_count = success_count.clone();
        
        tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            
            match dwn_client.write(&did, message).await {
                Ok(_) => {
                    success_count.fetch_add(1, Ordering::Relaxed);
                }
                Err(e) => eprintln!("DWN write failed: {}", e),
            }
        })
    }).collect();
    
    futures::future::join_all(tasks).await;
    
    let elapsed = start_time.elapsed();
    let successful_writes = success_count.load(Ordering::Relaxed);
    let throughput = successful_writes as f64 / elapsed.as_secs_f64();
    
    println!("DWN write throughput: {:.2} messages/sec", throughput);
    assert!(throughput > 5.0); // Minimum 5 messages/sec
    assert!(successful_writes as f64 / message_count as f64 > 0.9); // 90% success rate
}
```

## ML Performance Testing

### Model Inference Benchmarks
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use anya_core::ml::{Model, InferenceEngine};

fn benchmark_model_inference(c: &mut Criterion) {
    let mut group = c.benchmark_group("ml_inference");
    
    // Test different model sizes
    for model_size in ["small", "medium", "large"].iter() {
        let model = Model::load_test_model(model_size).unwrap();
        let test_input = create_test_input_for_model(&model);
        
        group.bench_with_input(
            BenchmarkId::new("inference", model_size),
            &(&model, &test_input),
            |b, (model, input)| {
                b.iter(|| model.predict(black_box(input)))
            },
        );
    }
    
    group.finish();
}

#[tokio::test]
async fn test_batch_inference_performance() {
    let model = Model::load_from_file("models/bitcoin_price_predictor.json").unwrap();
    let batch_sizes = [1, 10, 50, 100, 500];
    
    for &batch_size in &batch_sizes {
        let inputs: Vec<_> = (0..batch_size)
            .map(|_| create_random_input_vector(100))
            .collect();
        
        let start_time = Instant::now();
        let predictions = model.predict_batch(&inputs).await.unwrap();
        let inference_time = start_time.elapsed();
        
        assert_eq!(predictions.len(), batch_size);
        
        let latency_per_sample = inference_time / batch_size as u32;
        let samples_per_sec = batch_size as f64 / inference_time.as_secs_f64();
        
        println!("Batch size {}: {:.2} ms/sample, {:.2} samples/sec", 
                 batch_size, latency_per_sample.as_millis(), samples_per_sec);
        
        // Performance requirements
        assert!(latency_per_sample < Duration::from_millis(10)); // Under 10ms per sample
        assert!(samples_per_sec > 100.0); // At least 100 samples/sec
    }
}
```

### Training Performance Testing
```rust
#[tokio::test]
async fn test_model_training_performance() {
    let training_data = generate_synthetic_training_data(10_000, 100); // 10k samples, 100 features
    let mut model = Model::new_classifier(100, 10);
    
    let start_time = Instant::now();
    let training_result = model.train(&training_data).await.unwrap();
    let training_time = start_time.elapsed();
    
    let samples_per_sec = training_data.len() as f64 / training_time.as_secs_f64();
    
    println!("Training performance:");
    println!("  Total time: {:?}", training_time);
    println!("  Samples per second: {:.2}", samples_per_sec);
    println!("  Final accuracy: {:.3}", training_result.accuracy);
    
    assert!(training_time < Duration::from_secs(300)); // Under 5 minutes
    assert!(samples_per_sec > 50.0); // At least 50 samples/sec
    assert!(training_result.accuracy > 0.8); // At least 80% accuracy
}
```

## Stress Testing

### System Limit Testing
```rust
#[tokio::test]
async fn test_system_stress_limits() {
    let processor = TransactionProcessor::new();
    let mut transaction_rate = 10.0; // Start at 10 TPS
    let max_rate = 1000.0;
    let step_size = 10.0;
    let test_duration = Duration::from_secs(30);
    
    while transaction_rate <= max_rate {
        println!("Testing at {:.1} TPS", transaction_rate);
        
        let interval = Duration::from_secs_f64(1.0 / transaction_rate);
        let mut interval_timer = tokio::time::interval(interval);
        let test_start = Instant::now();
        let mut processed_count = 0;
        let mut error_count = 0;
        
        while test_start.elapsed() < test_duration {
            interval_timer.tick().await;
            
            let tx = create_test_transaction();
            match processor.process_transaction(&tx).await {
                Ok(_) => processed_count += 1,
                Err(_) => error_count += 1,
            }
        }
        
        let actual_rate = processed_count as f64 / test_duration.as_secs_f64();
        let error_rate = error_count as f64 / (processed_count + error_count) as f64;
        
        println!("  Actual rate: {:.1} TPS", actual_rate);
        println!("  Error rate: {:.1}%", error_rate * 100.0);
        
        // If error rate exceeds 5%, we've found the limit
        if error_rate > 0.05 {
            println!("System limit reached at {:.1} TPS", transaction_rate);
            break;
        }
        
        transaction_rate += step_size;
    }
}
```

### Memory Pressure Testing
```rust
#[test]
fn test_memory_pressure_handling() {
    let processor = TransactionProcessor::new();
    let initial_memory = get_memory_usage();
    
    // Gradually increase memory pressure
    for pressure_level in 1..=10 {
        let transaction_count = pressure_level * 10_000;
        let transactions = create_large_transaction_set(transaction_count);
        
        let start_time = Instant::now();
        let mut processed = 0;
        
        for tx in &transactions {
            match processor.validate_transaction(tx) {
                Ok(_) => processed += 1,
                Err(_) => break, // Stop on first error due to memory pressure
            }
        }
        
        let memory_usage = get_memory_usage();
        let memory_increase = memory_usage - initial_memory;
        let processing_time = start_time.elapsed();
        
        println!("Pressure level {}: {} transactions, {} MB, {:?}", 
                 pressure_level, processed, memory_increase / 1024 / 1024, processing_time);
        
        // If we can't process at least 95% of transactions, we've hit memory limits
        if processed as f64 / transaction_count as f64 < 0.95 {
            println!("Memory limit reached at pressure level {}", pressure_level);
            break;
        }
        
        // Clean up to prevent OOM
        drop(transactions);
        std::thread::sleep(Duration::from_millis(100));
    }
}
```

## Endurance Testing

### Long-Running Stability
```rust
#[tokio::test]
#[ignore] // Run separately due to long duration
async fn test_24_hour_endurance() {
    let processor = TransactionProcessor::new();
    let test_duration = Duration::from_secs(24 * 60 * 60); // 24 hours
    let target_rate = 50.0; // 50 TPS
    
    let interval = Duration::from_secs_f64(1.0 / target_rate);
    let mut interval_timer = tokio::time::interval(interval);
    
    let start_time = Instant::now();
    let mut total_processed = 0;
    let mut total_errors = 0;
    let mut memory_readings = Vec::new();
    
    // Take memory reading every hour
    let mut next_memory_check = Instant::now() + Duration::from_secs(3600);
    
    while start_time.elapsed() < test_duration {
        interval_timer.tick().await;
        
        let tx = create_test_transaction();
        match processor.process_transaction(&tx).await {
            Ok(_) => total_processed += 1,
            Err(_) => total_errors += 1,
        }
        
        // Periodic memory check
        if Instant::now() >= next_memory_check {
            let memory_usage = get_memory_usage();
            memory_readings.push(memory_usage);
            next_memory_check += Duration::from_secs(3600);
            
            let elapsed_hours = start_time.elapsed().as_secs() / 3600;
            println!("Hour {}: {} processed, {} errors, {} MB memory", 
                     elapsed_hours, total_processed, total_errors, memory_usage / 1024 / 1024);
        }
    }
    
    let final_time = start_time.elapsed();
    let actual_rate = total_processed as f64 / final_time.as_secs_f64();
    let error_rate = total_errors as f64 / (total_processed + total_errors) as f64;
    
    println!("24-hour endurance test results:");
    println!("  Total processed: {}", total_processed);
    println!("  Total errors: {}", total_errors);
    println!("  Average rate: {:.2} TPS", actual_rate);
    println!("  Error rate: {:.3}%", error_rate * 100.0);
    
    // Endurance test requirements
    assert!(actual_rate > 45.0); // At least 90% of target rate
    assert!(error_rate < 0.01); // Less than 1% error rate
    
    // Memory should not increase significantly over time
    let initial_memory = memory_readings[0];
    let final_memory = memory_readings.last().unwrap();
    let memory_growth = (*final_memory as f64 - initial_memory as f64) / initial_memory as f64;
    
    assert!(memory_growth < 0.1); // Less than 10% memory growth
}
```

## Scalability Testing

### Horizontal Scaling
```rust
#[tokio::test]
async fn test_horizontal_scaling() {
    let node_counts = [1, 2, 4, 8];
    let test_duration = Duration::from_secs(60);
    
    for &node_count in &node_counts {
        println!("Testing with {} nodes", node_count);
        
        // Start multiple processor instances
        let processors: Vec<_> = (0..node_count)
            .map(|_| Arc::new(TransactionProcessor::new()))
            .collect();
        
        let load_balancer = LoadBalancer::new(processors.clone());
        
        let start_time = Instant::now();
        let mut tasks = Vec::new();
        
        // Generate load across all nodes
        for i in 0..100 {
            let load_balancer = load_balancer.clone();
            let task = tokio::spawn(async move {
                let mut processed = 0;
                while Instant::now() - start_time < test_duration {
                    let tx = create_test_transaction_with_id(i * 1000 + processed);
                    if load_balancer.process_transaction(&tx).await.is_ok() {
                        processed += 1;
                    }
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
                processed
            });
            tasks.push(task);
        }
        
        let results = futures::future::join_all(tasks).await;
        let total_processed: usize = results.into_iter().map(|r| r.unwrap()).sum();
        let actual_rate = total_processed as f64 / test_duration.as_secs_f64();
        
        println!("  Throughput: {:.2} TPS", actual_rate);
        
        // Scaling efficiency should be at least 70%
        if node_count > 1 {
            let expected_min_rate = (actual_rate / node_count as f64) * 0.7 * node_count as f64;
            assert!(actual_rate >= expected_min_rate);
        }
    }
}
```

## Performance Monitoring

### Real-Time Metrics Collection
```rust
use prometheus::{Counter, Histogram, Gauge, register_counter, register_histogram, register_gauge};

lazy_static! {
    static ref TRANSACTION_COUNTER: Counter = register_counter!(
        "bitcoin_transactions_total", 
        "Total number of Bitcoin transactions processed"
    ).unwrap();
    
    static ref TRANSACTION_DURATION: Histogram = register_histogram!(
        "bitcoin_transaction_duration_seconds",
        "Time spent processing Bitcoin transactions"
    ).unwrap();
    
    static ref MEMORY_USAGE: Gauge = register_gauge!(
        "process_memory_bytes",
        "Current memory usage in bytes"
    ).unwrap();
}

pub struct PerformanceMonitor {
    start_time: Instant,
    metrics_collector: MetricsCollector,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            metrics_collector: MetricsCollector::new(),
        }
    }
    
    pub async fn monitor_transaction_processing<F, T>(&self, operation: F) -> Result<T, Error>
    where
        F: Future<Output = Result<T, Error>>,
    {
        let start = Instant::now();
        
        let result = operation.await;
        
        let duration = start.elapsed();
        TRANSACTION_DURATION.observe(duration.as_secs_f64());
        
        match &result {
            Ok(_) => TRANSACTION_COUNTER.inc(),
            Err(_) => {
                // Record error metrics
                self.metrics_collector.record_error("transaction_processing");
            }
        }
        
        // Update memory usage
        MEMORY_USAGE.set(get_memory_usage() as f64);
        
        result
    }
}
```

### Performance Dashboard
```rust
use warp::{Filter, Reply};
use serde_json::json;

pub async fn start_metrics_server() {
    let metrics = warp::path("metrics")
        .map(|| {
            let encoder = prometheus::TextEncoder::new();
            let metric_families = prometheus::gather();
            encoder.encode_to_string(&metric_families).unwrap()
        });
    
    let health = warp::path("health")
        .map(|| {
            warp::reply::json(&json!({
                "status": "healthy",
                "uptime": SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                "memory_usage": get_memory_usage(),
                "cpu_usage": get_cpu_usage(),
            }))
        });
    
    let routes = metrics.or(health);
    
    warp::serve(routes)
        .run(([0, 0, 0, 0], 9090))
        .await;
}
```

## CI/CD Performance Integration

### Automated Performance Testing
```yaml
# .github/workflows/performance.yml
name: Performance Tests
on:
  push:
    branches: [main]
  schedule:
    - cron: '0 2 * * *' # Run nightly

jobs:
  performance:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run Performance Benchmarks
        run: |
          cargo bench --bench bitcoin_performance
          cargo bench --bench web5_performance
          cargo bench --bench ml_performance
      
      - name: Upload Results
        uses: actions/upload-artifact@v3
        with:
          name: performance-results
          path: target/criterion/
      
      - name: Performance Regression Check
        run: |
          python scripts/check_performance_regression.py \
            --baseline performance-baseline.json \
            --current target/criterion/report/
```

### Performance Baseline Management
```rust
#[derive(Serialize, Deserialize)]
pub struct PerformanceBaseline {
    pub bitcoin_tx_validation: Duration,
    pub bitcoin_signature_verification: Duration,
    pub web5_did_resolution: Duration,
    pub ml_inference_latency: Duration,
    pub memory_usage_limit: u64,
}

impl PerformanceBaseline {
    pub fn load_from_file(path: &str) -> Result<Self, Error> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }
    
    pub fn check_regression(&self, current_metrics: &PerformanceMetrics) -> Vec<RegressionAlert> {
        let mut alerts = Vec::new();
        
        if current_metrics.bitcoin_tx_validation > self.bitcoin_tx_validation * 1.1 {
            alerts.push(RegressionAlert::new(
                "Bitcoin transaction validation",
                self.bitcoin_tx_validation,
                current_metrics.bitcoin_tx_validation,
            ));
        }
        
        // Check other metrics...
        
        alerts
    }
}
```

## Best Practices

### Performance Test Design
1. **Realistic Workloads**: Use real-world transaction patterns
2. **Baseline Establishment**: Maintain performance baselines
3. **Regression Detection**: Automated performance regression alerts
4. **Environment Consistency**: Standardized test environments
5. **Resource Monitoring**: Track CPU, memory, and network usage

### Test Data Management
```rust
pub struct TestDataGenerator {
    transaction_templates: Vec<TransactionTemplate>,
    did_templates: Vec<DidTemplate>,
    ml_datasets: Vec<MLDataset>,
}

impl TestDataGenerator {
    pub fn generate_realistic_bitcoin_load(&self, duration: Duration, tps: f64) -> Vec<Transaction> {
        let total_transactions = (duration.as_secs_f64() * tps) as usize;
        
        (0..total_transactions)
            .map(|i| self.create_realistic_transaction(i))
            .collect()
    }
    
    fn create_realistic_transaction(&self, index: usize) -> Transaction {
        // Use weighted random selection based on real network patterns
        let template_index = self.weighted_random_selection(index);
        self.transaction_templates[template_index].create_transaction()
    }
}
```

## Resources

- [Criterion.rs Benchmarking](https://docs.rs/criterion/)
- [Tokio Performance Guide](https://tokio.rs/tokio/topics/performance)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Bitcoin Performance Analysis](https://github.com/bitcoin/bitcoin/blob/master/doc/benchmarking.md)
- [Unit Testing Guide](./unit-testing.md)
- [Integration Testing Guide](./integration-testing.md)

*Last updated: June 7, 2025*
