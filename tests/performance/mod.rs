use std::path::Path;
use std::time::Instant;
use serde_json::Value;
use std::fs;

mod runner;
mod transaction;
mod database;
mod cache;

pub use runner::PerformanceTestRunner;
use crate::shared::test_utils;

pub struct TestConfig {
    pub name: String,
    pub iterations: usize,
    pub warmup_iterations: usize,
    pub duration_limit_secs: u64,
    pub parameters: Value,
}

pub struct TestResult {
    pub name: String,
    pub duration: f64,
    pub operations_per_second: f64,
    pub metrics: Value,
}

impl TestResult {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            duration: 0.0,
            operations_per_second: 0.0,
            metrics: Value::Null,
        }
    }
}

pub trait PerformanceTestable {
    fn name(&self) -> &str;
    fn run_test(&self, config: &TestConfig) -> Result<TestResult, String>;
    fn warmup(&self, iterations: usize) -> Result<(), String> {
        Ok(()) // Default no-op implementation
    }
}

pub fn load_test_config() -> Result<Value, String> {
    let config_path = Path::new("tests/test_config.json");
    let config_str = fs::read_to_string(config_path)
        .map_err(|e| format!("Failed to read test config: {}", e))?;
    
    serde_json::from_str(&config_str)
        .map_err(|e| format!("Failed to parse test config: {}", e))
}

pub fn run_performance_suite(report_dir: &Path) -> Result<(), String> {
    let config = load_test_config()?;
    let perf_config = &config["performance_tests"];
    
    let mut runner = runner::PerformanceTestRunner::new();
    
    // Configure transaction tests
    if let Some(tx_config) = perf_config["transaction_throughput"].as_object() {
        let iterations = tx_config["iterations"].as_u64().unwrap_or(1000) as usize;
        runner.add_transaction_tests(iterations);
    }
    
    // Configure database tests
    if let Some(db_config) = perf_config["database_access"].as_object() {
        let batch_sizes: Vec<usize> = db_config["batch_sizes"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_u64().map(|n| n as usize)).collect())
            .unwrap_or_else(|| vec![100, 1000, 10000]);
        
        runner.add_database_tests(&batch_sizes);
    }
    
    // Configure cache tests
    if let Some(cache_config) = perf_config["cache_performance"].as_object() {
        let sizes: Vec<usize> = cache_config["sizes"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_u64().map(|n| n as usize)).collect())
            .unwrap_or_else(|| vec![1000, 10000, 100000]);
        
        runner.add_cache_tests(&sizes);
    }
    
    // Run all tests and generate report
    runner.run_all_tests()?;
    runner.generate_report(report_dir)?;
    
    Ok(())
}