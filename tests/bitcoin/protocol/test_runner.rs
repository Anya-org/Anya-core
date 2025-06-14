// Bitcoin Protocol Test Runner
// [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
//
// Test runner for Bitcoin protocol implementations according to
// Bitcoin Development Framework v2.5 requirements

use anyhow::{Context, Result};
use colored::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Test result with metadata
#[derive(Debug)]
struct TestResult {
    name: String,
    success: bool,
    duration: Duration,
    error: Option<String>,
}

/// Test suite for running Bitcoin protocol tests
pub struct ProtocolTestSuite {
    results: Vec<TestResult>,
}

impl ProtocolTestSuite {
    /// Create a new test suite
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Run a test function and record the result
    pub fn run_test<F>(&mut self, name: &str, test_fn: F)
    where
        F: FnOnce() -> Result<()>,
    {
        println!("Running test: {}", name.blue());

        let start = Instant::now();
        let result = test_fn();
        let duration = start.elapsed();

        let success = result.is_ok();
        let error = result.err().map(|e| format!("{:#}", e));

        let test_result = TestResult {
            name: name.to_string(),
            success,
            duration,
            error,
        };

        self.results.push(test_result);

        if success {
            println!("  {} ({:.2?})", "SUCCESS".green(), duration);
        } else {
            println!("  {} ({:.2?})", "FAILED".red(), duration);
            if let Some(err) = &self.results.last().unwrap().error {
                println!("  Error: {}", err.red());
            }
        }
    }

    /// Print summary of test results
    pub fn print_summary(&self) {
        let total = self.results.len();
        let passed = self.results.iter().filter(|r| r.success).count();
        let failed = total - passed;

        println!("\n{}", "=".repeat(50));
        println!("Test Summary:");
        println!("  Total: {}", total);
        println!("  Passed: {}", passed.to_string().green());

        if failed > 0 {
            println!("  Failed: {}", failed.to_string().red());

            println!("\nFailed tests:");
            for result in &self.results {
                if !result.success {
                    println!("  - {} ({:.2?})", result.name.red(), result.duration);
                    if let Some(err) = &result.error {
                        println!("    Error: {}", err);
                    }
                }
            }
        } else {
            println!("  Failed: {}", "0".green());
        }
        println!("{}", "=".repeat(50));
    }

    /// Get the overall success status
    pub fn is_success(&self) -> bool {
        self.results.iter().all(|r| r.success)
    }

    /// Get a map of test names to success status
    pub fn results_map(&self) -> HashMap<String, bool> {
        self.results
            .iter()
            .map(|r| (r.name.clone(), r.success))
            .collect()
    }

    /// Get the total number of tests run
    pub fn total_tests(&self) -> usize {
        self.results.len()
    }

    /// Get the number of successful tests
    pub fn successful_tests(&self) -> usize {
        self.results.iter().filter(|r| r.success).count()
    }
}

/// Run all Bitcoin protocol tests
pub fn run_all_tests() -> Result<HashMap<String, bool>> {
    let mut suite = ProtocolTestSuite::new();

    // Import test modules
    use super::bip341_compliance::*;

    // Run BIP-341 tests
    suite.run_test("BIP-341: Key Path Spending", || {
        test_taproot_key_path_spending()
    });
    suite.run_test("BIP-341: Script Path Spending", || {
        test_taproot_script_path_spending()
    });
    suite.run_test("BIP-341: Multisig with Schnorr", || {
        test_taproot_multisig_schnorr()
    });
    suite.run_test("BIP-341: Edge Cases", || test_taproot_edge_cases());
    suite.run_test("BIP-341: Compliance Vectors", || {
        test_taproot_compliance_vectors()
    });

    // Add more tests here

    // Print summary
    suite.print_summary();

    Ok(suite.results_map())
}

/// Main entry point for running tests
pub fn main() -> Result<()> {
    println!("{}", "Running Bitcoin Protocol Tests".yellow().bold());
    println!("{}", "=".repeat(50));

    let results = run_all_tests()?;

    let all_passed = results.values().all(|&success| success);
    if all_passed {
        println!("\n{}", "All tests passed!".green().bold());
    } else {
        println!("\n{}", "Some tests failed!".red().bold());
        std::process::exit(1);
    }

    Ok(())
}
