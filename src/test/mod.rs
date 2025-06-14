//! Test module for Anya Core
//! 
//! This module provides comprehensive testing capabilities for all components
//! of the Anya Core system including Bitcoin integration, ML components, 
//! Web5 protocols, DAO functionality, and system integration tests.

#![allow(dead_code)] // Allow unused test utilities

pub mod bitcoin_tests;
pub mod dao_tests;
pub mod web5_tests;
pub mod ml_tests;
pub mod system_tests;
pub mod unified_test;

use std::error::Error;
use std::fmt;

/// Test result status
#[derive(Debug, Clone, PartialEq)]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
    Error,
}

/// Individual test result
#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: String,
    pub status: TestStatus,
    pub duration_ms: u64,
    pub message: Option<String>,
    pub error: Option<String>,
}

/// Test suite results
#[derive(Debug, Default)]
pub struct TestSuiteResults {
    pub suite_name: String,
    pub tests: Vec<TestResult>,
    pub total_duration_ms: u64,
}

impl TestSuiteResults {
    pub fn new(suite_name: String) -> Self {
        Self {
            suite_name,
            tests: Vec::new(),
            total_duration_ms: 0,
        }
    }

    pub fn add_test(&mut self, result: TestResult) {
        self.total_duration_ms += result.duration_ms;
        self.tests.push(result);
    }

    pub fn passed_count(&self) -> usize {
        self.tests.iter().filter(|t| t.status == TestStatus::Passed).count()
    }

    pub fn failed_count(&self) -> usize {
        self.tests.iter().filter(|t| t.status == TestStatus::Failed).count()
    }

    pub fn error_count(&self) -> usize {
        self.tests.iter().filter(|t| t.status == TestStatus::Error).count()
    }

    pub fn skipped_count(&self) -> usize {
        self.tests.iter().filter(|t| t.status == TestStatus::Skipped).count()
    }

    pub fn total_count(&self) -> usize {
        self.tests.len()
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_count() == 0 {
            return 0.0;
        }
        self.passed_count() as f64 / self.total_count() as f64 * 100.0
    }
}

impl fmt::Display for TestSuiteResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Test Suite: {}", self.suite_name)?;
        writeln!(f, "Total Tests: {}", self.total_count())?;
        writeln!(f, "Passed: {}", self.passed_count())?;
        writeln!(f, "Failed: {}", self.failed_count())?;
        writeln!(f, "Errors: {}", self.error_count())?;
        writeln!(f, "Skipped: {}", self.skipped_count())?;
        writeln!(f, "Success Rate: {:.2}%", self.success_rate())?;
        writeln!(f, "Duration: {}ms", self.total_duration_ms)?;
        
        for test in &self.tests {
            let status_char = match test.status {
                TestStatus::Passed => "✅",
                TestStatus::Failed => "❌",
                TestStatus::Error => "💥",
                TestStatus::Skipped => "⏭️",
            };
            writeln!(f, "  {} {} ({}ms)", status_char, test.name, test.duration_ms)?;
            if let Some(ref msg) = test.message {
                writeln!(f, "    Message: {}", msg)?;
            }
            if let Some(ref err) = test.error {
                writeln!(f, "    Error: {}", err)?;
            }
        }
        Ok(())
    }
}

/// Trait for test runners
pub trait TestRunner {
    fn run_tests(&mut self) -> Result<TestSuiteResults, Box<dyn Error>>;
    fn setup(&mut self) -> Result<(), Box<dyn Error>> { Ok(()) }
    fn teardown(&mut self) -> Result<(), Box<dyn Error>> { Ok(()) }
}

/// Macro for creating test cases
#[macro_export]
macro_rules! test_case {
    ($name:expr, $test_fn:expr) => {
        {
            let start = std::time::Instant::now();
            let name = $name.to_string();
            
            match std::panic::catch_unwind(|| $test_fn()) {
                Ok(Ok(())) => TestResult {
                    name,
                    status: TestStatus::Passed,
                    duration_ms: start.elapsed().as_millis() as u64,
                    message: None,
                    error: None,
                },
                Ok(Err(e)) => TestResult {
                    name,
                    status: TestStatus::Failed,
                    duration_ms: start.elapsed().as_millis() as u64,
                    message: None,
                    error: Some(e.to_string()),
                },
                Err(panic) => TestResult {
                    name,
                    status: TestStatus::Error,
                    duration_ms: start.elapsed().as_millis() as u64,
                    message: None,
                    error: Some(format!("Panic: {:?}", panic)),
                },
            }
        }
    };
}

/// Test configuration
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub bitcoin_rpc_url: Option<String>,
    pub verbose: bool,
    pub generate_reports: bool,
    pub report_dir: String,
    pub timeout_ms: u64,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            bitcoin_rpc_url: None,
            verbose: false,
            generate_reports: true,
            report_dir: "test-reports".to_string(),
            timeout_ms: 30000, // 30 seconds
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suite_results_calculations() {
        let mut suite = TestSuiteResults::new("test_suite".to_string());
        
        suite.add_test(TestResult {
            name: "test1".to_string(),
            status: TestStatus::Passed,
            duration_ms: 100,
            message: None,
            error: None,
        });
        
        suite.add_test(TestResult {
            name: "test2".to_string(),
            status: TestStatus::Failed,
            duration_ms: 200,
            message: None,
            error: Some("Test failed".to_string()),
        });

        assert_eq!(suite.total_count(), 2);
        assert_eq!(suite.passed_count(), 1);
        assert_eq!(suite.failed_count(), 1);
        assert_eq!(suite.total_duration_ms, 300);
        assert_eq!(suite.success_rate(), 50.0);
    }

    #[test]
    fn test_result_status_display() {
        let result = TestResult {
            name: "sample_test".to_string(),
            status: TestStatus::Passed,
            duration_ms: 150,
            message: Some("Test completed successfully".to_string()),
            error: None,
        };

        assert_eq!(result.status, TestStatus::Passed);
        assert_eq!(result.duration_ms, 150);
    }
}
