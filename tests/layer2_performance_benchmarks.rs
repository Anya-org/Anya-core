//! Layer2 Performance Benchmarks
//!
//! This module contains benchmarking tests for both synchronous and asynchronous
//! Layer2 protocol implementations to measure their relative performance.

use std::sync::Arc;
use std::time::{Duration, Instant};
use anya_core::layer2::{
    AssetParams, AssetTransfer, Layer2Protocol, Layer2ProtocolTrait, Layer2ProtocolType,
    TransactionStatus, ProtocolState, Proof, TransferResult, VerificationResult, ValidationResult,
    bob::{BobClient, BobConfig},
    lightning::{LightningNetwork, LightningConfig},
    liquid::{LiquidModule, LiquidConfig},
    rsk::{RskClient, RskConfig},
    stacks::{StacksClient, StacksConfig},
    taproot_assets::{TaprootAssetsProtocol, TaprootAssetsConfig},
    state_channels::{StateChannel, StateChannelConfig, CommitmentType},
    manager::Layer2Manager,
};

/// Performance test configuration
#[derive(Debug, Clone)]
struct PerfTestConfig {
    iterations: u32,
    tx_batch_size: u32,
    verbose: bool,
}

impl Default for PerfTestConfig {
    fn default() -> Self {
        Self {
            iterations: 100,
            tx_batch_size: 10,
            verbose: false,
        }
    }
}

/// Performance test result
#[derive(Debug, Clone)]
struct PerfTestResult {
    name: String,
    avg_time_ms: f64,
    min_time_ms: f64,
    max_time_ms: f64,
    operations_per_sec: f64,
}

/// Benchmark both sync and async implementations and compare their performance
#[tokio::test]
async fn benchmark_sync_vs_async_performance() {
    let config = PerfTestConfig::default();
    println!("Running Layer2 protocol performance benchmarks...");
    
    // Create test data
    let test_tx_data = generate_test_transaction_data(config.tx_batch_size as usize);
    
    // Initialize the clients
    let bob_client = create_bob_client();
    let lightning_network = create_lightning_network();
    let liquid_module = create_liquid_module();
    
    // Benchmark BOB client
    println!("\n--- BOB Client Performance ---");
    let sync_bob_result = benchmark_sync_operations("BOB Sync", &bob_client, &test_tx_data, config.iterations);
    let async_bob_result = benchmark_async_operations("BOB Async", &bob_client, &test_tx_data, config.iterations).await;
    print_comparison(sync_bob_result, async_bob_result);
    
    // Benchmark Lightning Network
    println!("\n--- Lightning Network Performance ---");
    let sync_lightning_result = benchmark_sync_operations("Lightning Sync", &lightning_network, &test_tx_data, config.iterations);
    let async_lightning_result = benchmark_async_operations("Lightning Async", &lightning_network, &test_tx_data, config.iterations).await;
    print_comparison(sync_lightning_result, async_lightning_result);
    
    // Benchmark Liquid Module
    println!("\n--- Liquid Module Performance ---");
    let sync_liquid_result = benchmark_sync_operations("Liquid Sync", &liquid_module, &test_tx_data, config.iterations);
    let async_liquid_result = benchmark_async_operations("Liquid Async", &liquid_module, &test_tx_data, config.iterations).await;
    print_comparison(sync_liquid_result, async_liquid_result);
    
    // Run comprehensive Layer2Manager benchmark
    println!("\n--- Layer2Manager Performance ---");
    benchmark_layer2_manager().await;
}

/// Create a test BOB client
fn create_bob_client() -> BobClient {
    let config = BobConfig {
        rpc_url: "http://localhost:8080".to_string(),
        chain_id: 111,
        timeout_ms: 5000,
        validate_relay: false,
    };
    BobClient::new(config)
}

/// Create a test Lightning Network
fn create_lightning_network() -> LightningNetwork {
    let config = LightningConfig {
        network: "testnet".to_string(),
        node_url: "http://localhost:9735".to_string(),
        // auth_token: None, // Field removed
        // auto_pilot: false, // Field removed
        // watchtower_enabled: true, // Field removed
        // min_channel_capacity: 20000, // Field removed
        // fee_rate: 1, // Field removed
    };
    LightningNetwork::new(config)
}

/// Create a test Liquid Module
fn create_liquid_module() -> LiquidModule {
    let config = LiquidConfig {
        network: "testnet".to_string(),
        rpc_url: "http://localhost:7041".to_string(),
        confidential: true,
        timeout_ms: 5000,
        federation_pubkeys: vec![
            "02142b5513b2bb94c35310618b6e7c80b08c04b0e3c26ba7e1b306b7f3fecefbfb".to_string(),
        ],
        required_signatures: 2,
        elementsd_path: "/usr/local/bin/elementsd".to_string(),
    };
    LiquidModule::new(config)
}

/// Generate test transaction data
fn generate_test_transaction_data(count: usize) -> Vec<Vec<u8>> {
    let mut data = Vec::with_capacity(count);
    for i in 0..count {
        // Create a unique transaction with deterministic content for testing
        let mut tx_data = Vec::with_capacity(100);
        // Add some header data
        tx_data.extend_from_slice(&[0xF9, 0x02, 0xA2]);
        // Add a "nonce" value to make each transaction unique
        tx_data.extend_from_slice(&(i as u32).to_le_bytes());
        // Add some fake tx fields (recipient, amount, etc)
        tx_data.extend_from_slice(b"0xReceiver1234567890");
        tx_data.extend_from_slice(&(1000_u64 * (i as u64 + 1)).to_le_bytes());
        // Add some padding to reach ~100 bytes
        while tx_data.len() < 100 {
            tx_data.push(0xAB);
        }
        data.push(tx_data);
    }
    data
}

/// Benchmark synchronous operations
fn benchmark_sync_operations<T>(
    name: &str, 
    client: &T, 
    test_data: &[Vec<u8>], 
    iterations: u32
) -> PerfTestResult 
where 
    T: Layer2ProtocolTrait + ?Sized,
{
    let mut timings: Vec<Duration> = Vec::with_capacity(iterations as usize);
    
    // Warmup
    let _ = client.submit_transaction(&test_data[0]);
    
    // Actual benchmarking
    for i in 0..iterations {
        let tx_data = &test_data[i as usize % test_data.len()];
        
        let start = Instant::now();
        let tx_id = client.submit_transaction(tx_data).unwrap();
        let _ = client.check_transaction_status(&tx_id).unwrap();
        let duration = start.elapsed();
        
        timings.push(duration);
    }
    
    // Calculate statistics
    let total_time: Duration = timings.iter().sum();
    let avg_time = total_time.as_secs_f64() / iterations as f64;
    let min_time = timings.iter().min().unwrap().as_secs_f64();
    let max_time = timings.iter().max().unwrap().as_secs_f64();
    let ops_per_sec = 1.0 / avg_time;
    
    PerfTestResult {
        name: name.to_string(),
        avg_time_ms: avg_time * 1000.0,
        min_time_ms: min_time * 1000.0,
        max_time_ms: max_time * 1000.0,
        operations_per_sec: ops_per_sec,
    }
}

/// Benchmark asynchronous operations
async fn benchmark_async_operations<T>(
    name: &str, 
    client: &T, 
    test_data: &[Vec<u8>], 
    iterations: u32
) -> PerfTestResult 
where 
    T: Layer2Protocol + ?Sized,
{
    let mut timings: Vec<Duration> = Vec::with_capacity(iterations as usize);
    
    // Warmup
    let _ = client.submit_transaction(&test_data[0]).await;
    
    // Actual benchmarking
    for i in 0..iterations {
        let tx_data = &test_data[i as usize % test_data.len()];
        
        let start = Instant::now();
        let tx_id = client.submit_transaction(tx_data).await.unwrap();
        let _ = client.check_transaction_status(&tx_id).await.unwrap();
        let duration = start.elapsed();
        
        timings.push(duration);
    }
    
    // Calculate statistics
    let total_time: Duration = timings.iter().sum();
    let avg_time = total_time.as_secs_f64() / iterations as f64;
    let min_time = timings.iter().min().unwrap().as_secs_f64();
    let max_time = timings.iter().max().unwrap().as_secs_f64();
    let ops_per_sec = 1.0 / avg_time;
    
    PerfTestResult {
        name: name.to_string(),
        avg_time_ms: avg_time * 1000.0,
        min_time_ms: min_time * 1000.0,
        max_time_ms: max_time * 1000.0,
        operations_per_sec: ops_per_sec,
    }
}

/// Print comparison between sync and async results
fn print_comparison(sync_result: PerfTestResult, async_result: PerfTestResult) {
    let improvement = (sync_result.avg_time_ms - async_result.avg_time_ms) / sync_result.avg_time_ms * 100.0;
    
    println!("Sync:  {:.2} ms avg (min: {:.2} ms, max: {:.2} ms, {:.2} ops/sec)",
        sync_result.avg_time_ms, 
        sync_result.min_time_ms,
        sync_result.max_time_ms,
        sync_result.operations_per_sec);
        
    println!("Async: {:.2} ms avg (min: {:.2} ms, max: {:.2} ms, {:.2} ops/sec)",
        async_result.avg_time_ms,
        async_result.min_time_ms,
        async_result.max_time_ms,
        async_result.operations_per_sec);
        
    println!("Performance improvement with async: {:.2}%", improvement);
}

/// Benchmark Layer2Manager performance with multiple protocols
async fn benchmark_layer2_manager() {
    let mut manager = Layer2Manager::new();
    
    // Initialize with some protocols
    // manager.bob_client = Some(create_bob_client()); // Private field
    // manager.lightning_network = Some(create_lightning_network()); // Private field
    // manager.liquid_module = Some(create_liquid_module()); // Private field
    
    // Generate test data for cross-layer operations
    let asset_ids = vec!["asset1", "asset2", "asset3"];
    let amounts = vec![1000, 5000, 10000];
    
    // Measure sync cross-layer operations
    let sync_start = Instant::now();
    for i in 0..10 {
        let from_protocol = Layer2ProtocolType::BOB;
        let to_protocol = Layer2ProtocolType::Lightning;
        let asset_id = asset_ids[i % asset_ids.len()];
        let amount = amounts[i % amounts.len()];
        
        let result = manager.cross_layer_transfer(
            from_protocol,
            to_protocol,
            asset_id,
            amount as u64,
        ).unwrap();
        
        assert!(!result.is_empty());
    }
    let sync_duration = sync_start.elapsed();
    
    // Measure async cross-layer operations
    let async_start = Instant::now();
    for i in 0..10 {
        let from_protocol = Layer2ProtocolType::BOB;
        let to_protocol = Layer2ProtocolType::Lightning;
        let asset_id = asset_ids[i % asset_ids.len()];
        let amount = amounts[i % amounts.len()];
        
        let result = manager.cross_layer_transfer_async(
            from_protocol,
            to_protocol,
            asset_id,
            amount as u64,
        ).await.unwrap();
        
        assert!(!result.is_empty());
    }
    let async_duration = async_start.elapsed();
    
    // Print results
    println!("Layer2Manager sync cross-layer operations:  {:.2} ms", 
        sync_duration.as_secs_f64() * 1000.0);
    println!("Layer2Manager async cross-layer operations: {:.2} ms", 
        async_duration.as_secs_f64() * 1000.0);
    
    let improvement = (sync_duration.as_secs_f64() - async_duration.as_secs_f64()) / 
        sync_duration.as_secs_f64() * 100.0;
    println!("Cross-layer performance improvement with async: {:.2}%", improvement);
}
