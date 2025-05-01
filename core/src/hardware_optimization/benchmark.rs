//! Benchmarking module for hardware optimization framework
//!
//! This module provides comprehensive benchmarking capabilities for the hardware
//! optimization framework, allowing performance measurement across different
//! architectures while ensuring Bitcoin protocol compliance.

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

use super::{
    Architecture, HardwareCapabilities, Operation, OptimizationError,
    ExecutionError, WorkloadProfile, PerformanceMetrics,
    HardwareOptimization, ExecutionPath, HardwareOptimizationManager,
    Priority, MemoryTarget, PowerTarget,
};

use crate::metrics::MetricsProvider;
use crate::system::SystemComponent;

/// Benchmark result for a single operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Operation that was benchmarked
    pub operation: Operation,
    
    /// Architecture used for the benchmark
    pub architecture: Architecture,
    
    /// Hardware capabilities during benchmark
    pub capabilities: HardwareCapabilities,
    
    /// Number of iterations performed
    pub iterations: usize,
    
    /// Average execution time per operation in microseconds
    pub avg_execution_time_us: f64,
    
    /// Operations per second
    pub operations_per_second: f64,
    
    /// Performance improvement percentage over baseline
    pub improvement_percentage: Option<f64>,
    
    /// Detailed metrics
    pub metrics: PerformanceMetrics,
    
    /// Timestamp when benchmark was run
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Comprehensive benchmark suite
pub struct BenchmarkSuite {
    /// Hardware optimization manager
    manager: Arc<HardwareOptimizationManager>,
    
    /// Baseline metrics (generic implementation)
    baseline_metrics: Arc<RwLock<std::collections::HashMap<Operation, BenchmarkResult>>>,
    
    /// Optimized metrics
    optimized_metrics: Arc<RwLock<std::collections::HashMap<Operation, BenchmarkResult>>>,
    
    /// Benchmark settings
    settings: BenchmarkSettings,
}

/// Benchmark settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSettings {
    /// Number of warmup iterations
    pub warmup_iterations: usize,
    
    /// Number of benchmark iterations
    pub iterations: usize,
    
    /// Input data sizes to test (in bytes)
    pub data_sizes: Vec<usize>,
    
    /// Operations to benchmark
    pub operations: Vec<Operation>,
    
    /// Workload profile to use
    pub workload: WorkloadProfile,
    
    /// Whether to benchmark across all available CPU cores
    pub multi_threaded: bool,
    
    /// Whether to verify correctness during benchmarking
    pub verify_correctness: bool,
}

impl Default for BenchmarkSettings {
    fn default() -> Self {
        Self {
            warmup_iterations: 10,
            iterations: 1000,
            data_sizes: vec![64, 256, 1024, 4096, 16384],
            operations: vec![
                Operation::SchnorrVerification,
                Operation::ECDSAVerification,
                Operation::SHA256,
                Operation::SHA512,
                Operation::BatchVerification,
                Operation::ScriptExecution,
                Operation::MerkleVerification,
                Operation::TaprootVerification,
                Operation::TapscriptExecution,
            ],
            workload: WorkloadProfile {
                transaction_volume: 1000,
                block_validation_priority: Priority::High,
                memory_target: MemoryTarget::Performance,
                power_target: PowerTarget::Performance,
                custom_parameters: std::collections::HashMap::new(),
            },
            multi_threaded: true,
            verify_correctness: true,
        }
    }
}

impl BenchmarkSuite {
    /// Create a new benchmark suite
    pub async fn new(manager: Arc<HardwareOptimizationManager>) -> Self {
        Self {
            manager,
            baseline_metrics: Arc::new(RwLock::new(std::collections::HashMap::new())),
            optimized_metrics: Arc::new(RwLock::new(std::collections::HashMap::new())),
            settings: BenchmarkSettings::default(),
        }
    }
    
    /// Configure benchmark settings
    pub fn with_settings(mut self, settings: BenchmarkSettings) -> Self {
        self.settings = settings;
        self
    }
    
    /// Run comprehensive benchmark suite
    pub async fn run_benchmark_suite(&self) -> Result<BenchmarkReport, OptimizationError> {
        let start_time = Instant::now();
        let mut report = BenchmarkReport {
            duration: Duration::default(),
            system_info: self.manager.get_capabilities().await,
            baseline_results: Vec::new(),
            optimized_results: Vec::new(),
            summary: BenchmarkSummary {
                average_improvement: 0.0,
                operations_tested: 0,
                successful_operations: 0,
                failed_operations: 0,
                timestamp: chrono::Utc::now(),
            },
        };
        
        // Get hardware capabilities
        let capabilities = self.manager.get_capabilities().await;
        
        // For each operation and data size
        for &operation in &self.settings.operations {
            for &data_size in &self.settings.data_sizes {
                // Generate test data
                let test_data = generate_test_data(operation, data_size);
                
                // Run baseline benchmark
                let baseline = self.benchmark_baseline(operation, &test_data).await?;
                report.baseline_results.push(baseline.clone());
                
                // Run optimized benchmark
                let optimized = self.benchmark_optimized(operation, &test_data).await?;
                report.optimized_results.push(optimized.clone());
                
                // Update metrics maps
                {
                    let mut baseline_metrics = self.baseline_metrics.write().await;
                    baseline_metrics.insert(operation, baseline);
                    
                    let mut optimized_metrics = self.optimized_metrics.write().await;
                    optimized_metrics.insert(operation, optimized);
                }
            }
        }
        
        // Calculate summary statistics
        let mut total_improvement = 0.0;
        let mut successful_operations = 0;
        let mut failed_operations = 0;
        
        for optimized in &report.optimized_results {
            if let Some(improvement) = optimized.improvement_percentage {
                total_improvement += improvement;
                successful_operations += 1;
            } else {
                failed_operations += 1;
            }
        }
        
        // Update summary
        report.summary.average_improvement = if successful_operations > 0 {
            total_improvement / successful_operations as f64
        } else {
            0.0
        };
        report.summary.operations_tested = report.optimized_results.len();
        report.summary.successful_operations = successful_operations;
        report.summary.failed_operations = failed_operations;
        report.summary.timestamp = chrono::Utc::now();
        
        // Set duration
        report.duration = start_time.elapsed();
        
        Ok(report)
    }
    
    /// Benchmark baseline (generic) implementation
    async fn benchmark_baseline(&self, operation: Operation, data: &[u8]) -> Result<BenchmarkResult, OptimizationError> {
        // Get generic optimizer
        let capabilities = self.manager.get_capabilities().await;
        let generic_optimizer = super::fallback::GenericOptimizer::new(&capabilities).await.unwrap();
        
        // Get execution path
        let path = generic_optimizer.optimize_operation(operation).await;
        
        // Warmup
        for _ in 0..self.settings.warmup_iterations {
            let _ = path.execute(data).await?;
        }
        
        // Benchmark
        let start = Instant::now();
        for _ in 0..self.settings.iterations {
            let _ = path.execute(data).await?;
        }
        let elapsed = start.elapsed();
        
        // Calculate metrics
        let avg_execution_time_us = elapsed.as_micros() as f64 / self.settings.iterations as f64;
        let operations_per_second = 1_000_000.0 / avg_execution_time_us;
        
        // Get performance metrics
        let metrics = path.benchmark(100).await
            .map_err(|e| OptimizationError::BenchmarkError(e.to_string()))?;
        
        Ok(BenchmarkResult {
            operation,
            architecture: Architecture::Generic,
            capabilities: capabilities.clone(),
            iterations: self.settings.iterations,
            avg_execution_time_us,
            operations_per_second,
            improvement_percentage: None,
            metrics,
            timestamp: chrono::Utc::now(),
        })
    }
    
    /// Benchmark optimized implementation
    async fn benchmark_optimized(&self, operation: Operation, data: &[u8]) -> Result<BenchmarkResult, OptimizationError> {
        // Get optimized execution path
        let path = self.manager.optimize_operation(operation).await?;
        
        // Warmup
        for _ in 0..self.settings.warmup_iterations {
            let _ = path.execute(data).await?;
        }
        
        // Benchmark
        let start = Instant::now();
        for _ in 0..self.settings.iterations {
            let _ = path.execute(data).await?;
        }
        let elapsed = start.elapsed();
        
        // Calculate metrics
        let avg_execution_time_us = elapsed.as_micros() as f64 / self.settings.iterations as f64;
        let operations_per_second = 1_000_000.0 / avg_execution_time_us;
        
        // Get baseline metrics
        let baseline_metrics = self.baseline_metrics.read().await;
        let baseline = baseline_metrics.get(&operation);
        
        // Calculate improvement percentage
        let improvement_percentage = baseline.map(|b| {
            ((operations_per_second / b.operations_per_second) - 1.0) * 100.0
        });
        
        // Get performance metrics
        let metrics = path.benchmark(100).await
            .map_err(|e| OptimizationError::BenchmarkError(e.to_string()))?;
        
        // Get capabilities
        let capabilities = self.manager.get_capabilities().await;
        
        Ok(BenchmarkResult {
            operation,
            architecture: capabilities.architecture,
            capabilities: capabilities.clone(),
            iterations: self.settings.iterations,
            avg_execution_time_us,
            operations_per_second,
            improvement_percentage,
            metrics,
            timestamp: chrono::Utc::now(),
        })
    }
    
    /// Run parallel benchmark
    pub async fn run_parallel_benchmark(&self) -> Result<BenchmarkReport, OptimizationError> {
        // Similar to run_benchmark_suite but using tokio::spawn for parallelism
        // This is more complex and would involve managing multiple tasks and aggregating results
        // For brevity, not fully implemented here
        
        let report = BenchmarkReport {
            duration: Duration::default(),
            system_info: self.manager.get_capabilities().await,
            baseline_results: Vec::new(),
            optimized_results: Vec::new(),
            summary: BenchmarkSummary {
                average_improvement: 0.0,
                operations_tested: 0,
                successful_operations: 0,
                failed_operations: 0,
                timestamp: chrono::Utc::now(),
            },
        };
        
        // Implementation would spawn tasks for each operation/data size
        // and collect results
        
        Ok(report)
    }
    
    /// Generate a human-readable benchmark report
    pub fn generate_report(&self, report: &BenchmarkReport) -> String {
        let mut output = String::new();
        
        // Add header
        output.push_str("# Bitcoin Anya Hardware Optimization Benchmark Report\n\n");
        output.push_str(&format!("Generated: {}\n", report.summary.timestamp));
        output.push_str(&format!("Total benchmark duration: {:?}\n\n", report.duration));
        
        // Add system information
        output.push_str("## System Information\n\n");
        output.push_str(&format!("Architecture: {}\n", report.system_info.architecture));
        output.push_str(&format!("Vendor: {}\n", report.system_info.vendor));
        output.push_str(&format!("Model: {}\n", report.system_info.model));
        output.push_str(&format!("Cores: {}, Threads: {}\n", report.system_info.core_count, report.system_info.thread_count));
        output.push_str(&format!("Vector extensions: {:?}\n", report.system_info.vector_extensions));
        output.push_str(&format!("Crypto extensions: {:?}\n\n", report.system_info.crypto_extensions));
        
        // Add summary
        output.push_str("## Performance Summary\n\n");
        output.push_str(&format!("Operations tested: {}\n", report.summary.operations_tested));
        output.push_str(&format!("Successful operations: {}\n", report.summary.successful_operations));
        output.push_str(&format!("Failed operations: {}\n", report.summary.failed_operations));
        output.push_str(&format!("Average improvement: {:.2}%\n\n", report.summary.average_improvement));
        
        // Add detailed results
        output.push_str("## Detailed Results\n\n");
        output.push_str("| Operation | Data Size | Baseline Ops/s | Optimized Ops/s | Improvement |\n");
        output.push_str("|-----------|-----------|----------------|-----------------|-------------|\n");
        
        // Group results by operation
        let mut grouped_results: std::collections::HashMap<Operation, Vec<(&BenchmarkResult, &BenchmarkResult)>> = std::collections::HashMap::new();
        
        for (baseline, optimized) in report.baseline_results.iter().zip(report.optimized_results.iter()) {
            if baseline.operation == optimized.operation {
                grouped_results.entry(baseline.operation)
                    .or_insert_with(Vec::new)
                    .push((baseline, optimized));
            }
        }
        
        // Add rows for each operation and data size
        for (operation, results) in grouped_results {
            for (baseline, optimized) in results {
                let data_size = std::cmp::min(baseline.capabilities.core_count, optimized.capabilities.core_count);
                
                output.push_str(&format!(
                    "| {:?} | {} bytes | {:.2} | {:.2} | {:.2}% |\n",
                    operation,
                    data_size,
                    baseline.operations_per_second,
                    optimized.operations_per_second,
                    optimized.improvement_percentage.unwrap_or(0.0)
                ));
            }
        }
        
        output
    }
    
    /// Save benchmark report to file
    pub async fn save_report(&self, report: &BenchmarkReport, path: &str) -> Result<(), std::io::Error> {
        // Generate report
        let report_text = self.generate_report(report);
        
        // Save to file
        tokio::fs::write(path, report_text).await
    }
    
    /// Save benchmark report as JSON
    pub async fn save_report_json(&self, report: &BenchmarkReport, path: &str) -> Result<(), std::io::Error> {
        // Serialize to JSON
        let json = serde_json::to_string_pretty(report)?;
        
        // Save to file
        tokio::fs::write(path, json).await
    }
}

/// Benchmark report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkReport {
    /// Total duration of benchmark
    pub duration: Duration,
    
    /// System information
    pub system_info: HardwareCapabilities,
    
    /// Baseline results
    pub baseline_results: Vec<BenchmarkResult>,
    
    /// Optimized results
    pub optimized_results: Vec<BenchmarkResult>,
    
    /// Summary statistics
    pub summary: BenchmarkSummary,
}

/// Benchmark summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    /// Average improvement percentage
    pub average_improvement: f64,
    
    /// Number of operations tested
    pub operations_tested: usize,
    
    /// Number of successful operations
    pub successful_operations: usize,
    
    /// Number of failed operations
    pub failed_operations: usize,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Generate test data for benchmark
fn generate_test_data(operation: Operation, size: usize) -> Vec<u8> {
    match operation {
        Operation::SchnorrVerification => {
            // For Schnorr verification, we need a specific format
            // This is a placeholder, real implementation would use actual signature format
            let mut data = vec![0; size];
            // Set first byte to 1 to indicate "valid" signature for tests
            if !data.is_empty() {
                data[0] = 1;
            }
            data
        },
        Operation::BatchVerification => {
            // For batch verification, create array of "valid" signatures
            let mut data = vec![0; size];
            // Format as multiple signatures
            for i in 0..size / 64 {
                if i * 64 < size {
                    data[i * 64] = 1; // Mark as "valid"
                }
            }
            data
        },
        _ => {
            // For most operations, random data is fine
            let mut data = vec![0; size];
            let mut rng = rand::thread_rng();
            rand::Rng::fill(&mut rng, &mut data[..]);
            data
        }
    }
}

/// Benchmark command-line interface
pub struct BenchmarkCli {
    /// Benchmark suite
    suite: BenchmarkSuite,
}

impl BenchmarkCli {
    /// Create a new benchmark CLI
    pub async fn new() -> Result<Self, OptimizationError> {
        // Create manager
        let manager = Arc::new(HardwareOptimizationManager::new().await?);
        
        // Create benchmark suite
        let suite = BenchmarkSuite::new(manager).await;
        
        Ok(Self { suite })
    }
    
    /// Run benchmark from command line
    pub async fn run(&self, args: Vec<String>) -> Result<(), OptimizationError> {
        // Parse arguments
        let settings = self.parse_args(args)?;
        
        // Configure suite
        let suite = BenchmarkSuite::new(self.suite.manager.clone()).await
            .with_settings(settings);
        
        // Run benchmark
        let report = suite.run_benchmark_suite().await?;
        
        // Generate and print report
        let report_text = suite.generate_report(&report);
        println!("{}", report_text);
        
        // Save report
        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
        let report_path = format!("benchmark_report_{}.md", timestamp);
        suite.save_report(&report, &report_path).await?;
        
        // Save JSON
        let json_path = format!("benchmark_report_{}.json", timestamp);
        suite.save_report_json(&report, &json_path).await?;
        
        println!("Benchmark report saved to: {}", report_path);
        println!("JSON report saved to: {}", json_path);
        
        Ok(())
    }
    
    /// Parse command line arguments
    fn parse_args(&self, args: Vec<String>) -> Result<BenchmarkSettings, OptimizationError> {
        // Default settings
        let mut settings = BenchmarkSettings::default();
        
        // Parse args
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--iterations" | "-i" => {
                    if i + 1 < args.len() {
                        settings.iterations = args[i + 1].parse()
                            .map_err(|_| OptimizationError::InvalidParameter("Invalid iterations value".to_string()))?;
                        i += 1;
                    }
                },
                "--warmup" | "-w" => {
                    if i + 1 < args.len() {
                        settings.warmup_iterations = args[i + 1].parse()
                            .map_err(|_| OptimizationError::InvalidParameter("Invalid warmup value".to_string()))?;
                        i += 1;
                    }
                },
                "--data-sizes" | "-d" => {
                    if i + 1 < args.len() {
                        let sizes: Result<Vec<usize>, _> = args[i + 1].split(',')
                            .map(|s| s.parse())
                            .collect();
                        settings.data_sizes = sizes
                            .map_err(|_| OptimizationError::InvalidParameter("Invalid data sizes".to_string()))?;
                        i += 1;
                    }
                },
                "--operations" | "-o" => {
                    if i + 1 < args.len() {
                        // Parse operations list
                        // This is simplified, real implementation would map strings to Operation enum
                        i += 1;
                    }
                },
                "--multi-threaded" | "-m" => {
                    settings.multi_threaded = true;
                },
                "--single-threaded" | "-s" => {
                    settings.multi_threaded = false;
                },
                "--verify" | "-v" => {
                    settings.verify_correctness = true;
                },
                "--no-verify" | "-n" => {
                    settings.verify_correctness = false;
                },
                _ => {
                    // Unknown argument
                }
            }
            i += 1;
        }
        
        Ok(settings)
    }
}

/// Quick benchmark for a specific operation
pub async fn quick_benchmark(operation: Operation, iterations: usize) -> Result<BenchmarkResult, OptimizationError> {
    // Create manager
    let manager = HardwareOptimizationManager::new().await?;
    
    // Get hardware capabilities
    let capabilities = manager.get_capabilities().await;
    
    // Get optimized path
    let path = manager.optimize_operation(operation).await?;
    
    // Generate test data
    let data = generate_test_data(operation, 1024);
    
    // Warmup
    for _ in 0..10 {
        let _ = path.execute(&data).await?;
    }
    
    // Benchmark
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = path.execute(&data).await?;
    }
    let elapsed = start.elapsed();
    
    // Calculate metrics
    let avg_execution_time_us = elapsed.as_micros() as f64 / iterations as f64;
    let operations_per_second = 1_000_000.0 / avg_execution_time_us;
    
    // Get metrics from execution path
    let metrics = path.benchmark(100).await
        .map_err(|e| OptimizationError::BenchmarkError(e.to_string()))?;
    
    Ok(BenchmarkResult {
        operation,
        architecture: capabilities.architecture,
        capabilities,
        iterations,
        avg_execution_time_us,
        operations_per_second,
        improvement_percentage: None,
        metrics,
        timestamp: chrono::Utc::now(),
    })
}
