use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;
/// Performance Testing Framework [BPC-3]
///
/// This module provides tools for performance testing of the Bitcoin
/// implementation, including transaction throughput benchmarking,
/// database access pattern analysis, and cache optimization.
use thiserror::Error;

// Module exports
pub mod cache;
pub mod database;
pub mod runner;
pub mod transaction;

/// Error types for performance testing
#[derive(Debug, Error)]
pub enum PerfTestError {
    #[error("Test error: {0}")]
    TestError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Measurement error: {0}")]
    MeasurementError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Bitcoin error: {0}")]
    BitcoinError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type for performance operations
pub type Result<T> = std::result::Result<T, PerfTestError>;

/// Performance metric type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MetricType {
    /// Transactions per second
    TPS,

    /// Milliseconds per operation
    LatencyMs,

    /// Memory usage in megabytes
    MemoryMB,

    /// CPU usage percentage
    CpuPercent,

    /// Database operations per second
    DbOpsPerSecond,

    /// Cache hit rate percentage
    CacheHitRate,
}

/// Performance test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    /// Test name
    pub name: String,

    /// Test timestamp
    pub timestamp: String,

    /// Duration in milliseconds
    pub duration_ms: u64,

    /// Metrics collected
    pub metrics: HashMap<String, f64>,

    /// Metric types
    pub metric_types: HashMap<String, MetricType>,

    /// Configuration parameters
    pub parameters: HashMap<String, String>,
}

/// Performance test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    /// Test name
    pub name: String,

    /// Number of iterations
    pub iterations: usize,

    /// Warmup iterations
    pub warmup_iterations: usize,

    /// Test duration limit in seconds
    pub duration_limit_secs: u64,

    /// Configuration parameters
    pub parameters: HashMap<String, String>,
}

/// Trait for performance testable components
pub trait PerformanceTestable {
    /// Run a performance test
    fn run_test(&self, config: &TestConfig) -> Result<TestResult>;

    /// Get the name of the component
    fn name(&self) -> &str;
}

/// Performance test runner
pub struct PerformanceTestRunner {
    /// Test configurations
    configs: Vec<TestConfig>,

    /// Test components
    components: Vec<Box<dyn PerformanceTestable>>,

    /// Results
    results: Vec<TestResult>,
}

impl Default for PerformanceTestRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceTestRunner {
    /// Create a new performance test runner
    pub fn new() -> Self {
        Self {
            configs: Vec::new(),
            components: Vec::new(),
            results: Vec::new(),
        }
    }

    /// Add a test configuration
    pub fn add_config(&mut self, config: TestConfig) {
        self.configs.push(config);
    }

    /// Add a testable component
    pub fn add_component(&mut self, component: Box<dyn PerformanceTestable>) {
        self.components.push(component);
    }

    /// Run all tests
    pub fn run_all_tests(&mut self) -> Result<()> {
        for config in &self.configs {
            for component in &self.components {
                if component.name() == config.name || config.name == "all" {
                    println!(
                        "Running test: {} on component: {}",
                        config.name,
                        component.name()
                    );
                    let result = component.run_test(config)?;
                    self.results.push(result);
                }
            }
        }

        Ok(())
    }

    /// Run a specific test
    pub fn run_test(&mut self, test_name: &str) -> Result<()> {
        let config = self
            .configs
            .iter()
            .find(|c| c.name == test_name)
            .ok_or_else(|| {
                PerfTestError::ConfigurationError(format!(
                    "Test configuration not found: {}",
                    test_name
                ))
            })?;

        for component in &self.components {
            if component.name() == config.name || config.name == "all" {
                println!(
                    "Running test: {} on component: {}",
                    config.name,
                    component.name()
                );
                let result = component.run_test(config)?;
                self.results.push(result);
            }
        }

        Ok(())
    }

    /// Get all results
    pub fn get_results(&self) -> &[TestResult] {
        &self.results
    }

    /// Generate a report as markdown
    pub fn generate_report_markdown(&self) -> String {
        let mut markdown = String::new();

        // Title
        markdown.push_str("# Performance Test Results\n\n");

        // Metadata
        markdown.push_str(&format!(
            "- **Date:** {}\n",
            chrono::Utc::now().to_rfc3339()
        ));
        markdown.push_str(&format!("- **Total Tests:** {}\n\n", self.results.len()));

        // Results
        for result in &self.results {
            markdown.push_str(&format!("## Test: {}\n\n", result.name));
            markdown.push_str(&format!("- **Duration:** {} ms\n", result.duration_ms));
            markdown.push_str("- **Metrics:**\n");

            for (name, value) in &result.metrics {
                let type_str = match result.metric_types.get(name) {
                    Some(MetricType::TPS) => "TPS",
                    Some(MetricType::LatencyMs) => "ms",
                    Some(MetricType::MemoryMB) => "MB",
                    Some(MetricType::CpuPercent) => "%",
                    Some(MetricType::DbOpsPerSecond) => "ops/s",
                    Some(MetricType::CacheHitRate) => "%",
                    None => "",
                };

                markdown.push_str(&format!("  - **{}:** {:.2} {}\n", name, value, type_str));
            }

            markdown.push_str("- **Parameters:**\n");
            for (name, value) in &result.parameters {
                markdown.push_str(&format!("  - **{}:** {}\n", name, value));
            }

            markdown.push('\n');
        }

        markdown
    }
}

/// Basic timer utility for measuring performance
pub struct Timer {
    /// Start time
    start: Option<Instant>,

    /// End time
    end: Option<Instant>,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    /// Create a new timer
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    /// Start the timer
    pub fn start(&mut self) {
        self.start = Some(Instant::now());
        self.end = None;
    }

    /// Stop the timer
    pub fn stop(&mut self) {
        self.end = Some(Instant::now());
    }

    /// Get the elapsed time in milliseconds
    pub fn elapsed_ms(&self) -> Result<u64> {
        match (self.start, self.end) {
            (Some(start), Some(end)) => Ok(end.duration_since(start).as_millis() as u64),
            (Some(start), None) => Ok(Instant::now().duration_since(start).as_millis() as u64),
            _ => Err(PerfTestError::MeasurementError(
                "Timer not started".to_string(),
            )),
        }
    }

    /// Get the elapsed time in seconds
    pub fn elapsed_secs(&self) -> Result<f64> {
        Ok(self.elapsed_ms()? as f64 / 1000.0)
    }
}
