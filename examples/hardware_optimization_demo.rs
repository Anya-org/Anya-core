//! Hardware Optimization Framework Demo
//!
//! This example demonstrates the Universal Adaptive Hardware Optimization Framework
//! in action, automatically detecting and leveraging available hardware accelerators
//! including CPUs, GPUs, and NPUs while maintaining Bitcoin protocol compliance.
//!
//! TEMPORARILY DISABLED - hardware_optimization module needs to be properly integrated

/*
use rand::{thread_rng, Rng};
use std::error::Error;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

use anya_core::hardware_optimization::benchmark::{BenchmarkSettings, BenchmarkSuite};
use anya_core::hardware_optimization::integration::create_accelerated_optimizer;
use anya_core::hardware_optimization::{
    self, ExecutionError, HardwareOptimizationManager, MemoryTarget, Operation, OptimizationError,
    PowerTarget, Priority, WorkloadProfile,
};

/// Sample size for benchmarking (in bytes)
const SAMPLE_SIZE: usize = 1024 * 1024; // 1 MB

/// Number of iterations for performance testing
const ITERATIONS: usize = 1000;

/// Number of signatures to verify in batch operations
const BATCH_SIZE: usize = 1000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Bitcoin Anya Core - Universal Adaptive Hardware Optimization Framework Demo");
    println!("=====================================================================");

    // Create accelerated optimizer (auto-detects GPU/NPU)
    println!("\n🔍 Detecting optimal hardware accelerator...");
    let manager = match create_accelerated_optimizer().await {
        Ok(m) => {
            println!("✅ Hardware accelerator initialized successfully");
            m
        }
        Err(e) => {
            println!("⚠️ Error detecting hardware: {}", e);
            println!("⚠️ Falling back to generic implementation");
            HardwareOptimizationManager::new().await?
        }
    };

    // Display detected hardware capabilities
    let capabilities = manager.get_capabilities().await;
    println!("\n🖥️ Hardware Capabilities:");
    println!("  Architecture: {}", capabilities.architecture);
    println!("  Vendor: {}", capabilities.vendor);
    println!("  Model: {}", capabilities.model);
    println!(
        "  Cores: {}, Threads: {}",
        capabilities.core_count, capabilities.thread_count
    );
    println!("  Vector Extensions: {:?}", capabilities.vector_extensions);
    println!("  Crypto Extensions: {:?}", capabilities.crypto_extensions);

    // Display GPU/NPU capabilities if available
    if let Some(gpu_caps) = &capabilities.gpu_capabilities {
        if gpu_caps.gpu_available {
            println!("\n🎮 GPU Capabilities:");
            println!("  GPU: {} from {:?}", gpu_caps.model, gpu_caps.vendor);
            println!("  Memory: {} MB", gpu_caps.memory_mb);
            println!("  Compute Units: {}", gpu_caps.compute_units);
            println!("  Backends: {:?}", gpu_caps.backends);

            if let Some(cc) = gpu_caps.cuda_compute_capability {
                println!("  CUDA Compute Capability: {}.{}", cc.0, cc.1);
            }
        }

        if gpu_caps.npu_available {
            println!("\n🧠 NPU Capabilities:");
            println!("  NPU Type: {:?}", gpu_caps.npu_type);
        }
    }

    // Get active optimizations
    let status = manager.get_status().await;
    println!("\n⚡ Active Optimizations:");
    for opt in &status.active_optimizations {
        println!("  - {}", opt);
    }

    // Generate test data
    println!("\n📊 Generating test data...");
    let mut rng = thread_rng();
    let random_data: Vec<u8> = (0..SAMPLE_SIZE).map(|_| rng.gen()).collect();

    // Generate batch verification data
    let mut batch_data = Vec::with_capacity(BATCH_SIZE * 64);
    for i in 0..BATCH_SIZE {
        // First byte determines if signature is "valid" (for testing)
        batch_data.push(if i % 10 == 0 { 0 } else { 1 });

        // Remaining 63 bytes are random
        batch_data.extend((0..63).map(|_| rng.gen::<u8>()));
    }

    // Tune for high-performance workload
    let high_perf_workload = WorkloadProfile {
        transaction_volume: 5000,
        block_validation_priority: Priority::High,
        memory_target: MemoryTarget::Performance,
        power_target: PowerTarget::Performance,
        custom_parameters: std::collections::HashMap::new(),
    };

    println!("\n🔧 Tuning for high-performance workload...");
    manager.update_workload(high_perf_workload).await?;

    // Basic operations benchmark
    println!("\n🧪 Testing basic operations:");

    // SHA-256 benchmark
    let start = Instant::now();
    let sha256_path = manager.optimize_operation(Operation::SHA256).await?;
    for _ in 0..ITERATIONS {
        let _ = sha256_path.execute(&random_data[..1024]).await?;
    }
    let elapsed = start.elapsed();
    println!(
        "  SHA-256 (1KB): {} iterations in {:.2?} ({:.2} hashes/sec)",
        ITERATIONS,
        elapsed,
        ITERATIONS as f64 / elapsed.as_secs_f64()
    );

    // Schnorr signature verification benchmark
    let valid_sig = vec![1u8; 128]; // Our test convention: first byte 1 = valid
    let start = Instant::now();
    let schnorr_path = manager
        .optimize_operation(Operation::SchnorrVerification)
        .await?;
    for _ in 0..ITERATIONS {
        let _ = schnorr_path.execute(&valid_sig).await?;
    }
    let elapsed = start.elapsed();
    println!(
        "  Schnorr Verification: {} iterations in {:.2?} ({:.2} verifications/sec)",
        ITERATIONS,
        elapsed,
        ITERATIONS as f64 / elapsed.as_secs_f64()
    );

    // Batch verification benchmark (most likely to benefit from GPU/NPU)
    let start = Instant::now();
    let batch_path = manager
        .optimize_operation(Operation::BatchVerification)
        .await?;
    let result = batch_path.execute(&batch_data).await?;
    let elapsed = start.elapsed();

    let valid_count = result.iter().filter(|&&b| b == 1).count();
    let invalid_count = result.len() - valid_count;

    println!(
        "  Batch Verification: {} signatures in {:.2?} ({:.2} sigs/sec)",
        BATCH_SIZE,
        elapsed,
        BATCH_SIZE as f64 / elapsed.as_secs_f64()
    );
    println!("    Valid: {}, Invalid: {}", valid_count, invalid_count);

    // Run comprehensive benchmark suite
    println!("\n📈 Running comprehensive benchmark suite...");
    let benchmark_suite = BenchmarkSuite::new(Arc::new(manager.clone()))
        .await
        .with_settings(BenchmarkSettings {
            warmup_iterations: 3,
            iterations: 100,
            data_sizes: vec![64, 256, 1024, 4096],
            operations: vec![
                Operation::SchnorrVerification,
                Operation::SHA256,
                Operation::BatchVerification,
            ],
            multi_threaded: true,
            verify_correctness: true,
            ..Default::default()
        });

    // Run benchmark and generate report
    let benchmark_report = benchmark_suite.run_benchmark_suite().await?;
    let report_text = benchmark_suite.generate_report(&benchmark_report);

    println!("\n📋 Benchmark Results:");
    println!("{}", report_text);

    // Save benchmark report
    let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
    let report_path = format!("hardware_benchmark_{}.md", timestamp);
    benchmark_suite
        .save_report(&benchmark_report, &report_path)
        .await?;
    println!("\n💾 Benchmark report saved to: {}", report_path);

    // Verify correctness of all optimizations
    println!("\n✅ Verifying all optimizations for consensus compliance...");
    manager.verify_correctness().await?;
    println!("All optimizations verified - maintaining Bitcoin protocol compliance");

    println!("\n🎉 Hardware optimization framework demo completed successfully!");

    Ok(())
}

/// Generate a vector of random signatures for testing batch verification
fn generate_test_signatures(count: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    let mut data = Vec::with_capacity(count * 64);

    for i in 0..count {
        // First byte of each signature determines if it's "valid" in our test
        data.push(if i % 10 == 0 { 0 } else { 1 });

        // Rest is random data
        for _ in 1..64 {
            data.push(rng.gen::<u8>());
        }
    }

    data
}

/// Print performance comparison between different hardware implementations
async fn print_performance_comparison(
    manager: &HardwareOptimizationManager,
) -> Result<(), OptimizationError> {
    println!("\n📊 Performance Comparison:");

    // Create generic fallback optimizer for baseline
    let generic_capabilities = manager.get_capabilities().await;
    let generic_optimizer =
        hardware_optimization::fallback::GenericOptimizer::new(&generic_capabilities).await?;

    // Operations to compare
    let operations = vec![
        Operation::SHA256,
        Operation::SchnorrVerification,
        Operation::BatchVerification,
    ];

    // Test sample sizes
    let sample_sizes = vec![64, 1024, 16384];

    for &operation in &operations {
        println!("  Operation: {:?}", operation);

        for &size in &sample_sizes {
            // Generate test data
            let mut rng = thread_rng();
            let test_data: Vec<u8> = (0..size).map(|_| rng.gen()).collect();

            // Get optimized path
            let optimized_path = manager.optimize_operation(operation).await?;

            // Get generic fallback path
            let generic_path = generic_optimizer.optimize_operation(operation).await;

            // Benchmark optimized implementation
            let start = Instant::now();
            for _ in 0..100 {
                let _ = optimized_path.execute(&test_data).await?;
            }
            let optimized_time = start.elapsed();

            // Benchmark generic implementation
            let start = Instant::now();
            for _ in 0..100 {
                let _ = generic_path.execute(&test_data).await?;
            }
            let generic_time = start.elapsed();

            // Calculate speedup
            let speedup = generic_time.as_secs_f64() / optimized_time.as_secs_f64();

            println!("    Size {} bytes: {:.2}x speedup", size, speedup);
        }
    }
*/

fn main() {
    println!("Hardware optimization demo temporarily disabled - module needs integration");
}
