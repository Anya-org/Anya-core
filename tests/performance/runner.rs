//! Performance test runner integration

use crate::testing::performance::{
    PerformanceTestRunner, TestConfig, PerformanceTestable, Result
};
use crate::testing::performance::transaction::{
    TransactionThroughputTest, TxGenConfig
};
use crate::testing::performance::database::{
    DatabaseAccessTest, DbConfig, DbOperation
};
use crate::testing::performance::cache::{
    CachePerformanceTest, CacheConfig, create_standard_cache_tests
};
use crate::bitcoin::validation::TransactionValidator;
use bitcoin::Network;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Create a standard test configuration
pub fn create_standard_test_config(name: &str, iterations: usize) -> TestConfig {
    TestConfig {
        name: name.to_string(),
        iterations,
        warmup_iterations: iterations / 10,
        duration_limit_secs: 3600, // 1 hour max
        parameters: HashMap::new(),
    }
}

/// Create and run a comprehensive performance test suite
pub fn run_comprehensive_test_suite(output_dir: &Path) -> Result<()> {
    let mut runner = PerformanceTestRunner::new();
    
    // Add test configurations
    runner.add_config(create_standard_test_config("transaction_throughput", 1000));
    runner.add_config(create_standard_test_config("database_access", 10000));
    runner.add_config(create_standard_test_config("cache_performance", 100000));
    runner.add_config(create_standard_test_config("all", 1000));
    
    // Add transaction throughput tests
    let validator = TransactionValidator::new();
    
    // Single-threaded transaction test
    let tx_test = TransactionThroughputTest::new(
        validator.clone(),
        TxGenConfig {
            multithreaded: false,
            ..TxGenConfig::default()
        }
    );
    runner.add_component(Box::new(tx_test));
    
    // Multi-threaded transaction test
    let tx_test_multi = TransactionThroughputTest::new(
        validator,
        TxGenConfig {
            multithreaded: true,
            thread_count: 4,
            ..TxGenConfig::default()
        }
    );
    runner.add_component(Box::new(tx_test_multi));
    
    // Add database access tests
    let db_test = DatabaseAccessTest::new(
        DbConfig::default(),
        vec![
            DbOperation::Read,
            DbOperation::Write,
            DbOperation::Update,
            DbOperation::Delete,
        ],
        10000, // key space size
        100,   // value size in bytes
    );
    runner.add_component(Box::new(db_test));
    
    // Add cache performance tests
    for test in create_standard_cache_tests() {
        runner.add_component(test);
    }
    
    // Run all tests
    runner.run_all_tests()?;
    
    // Generate report
    let report = runner.generate_report_markdown();
    
    // Ensure the directory exists
    fs::create_dir_all(output_dir)?;
    
    // Write the report to a file
    let report_path = output_dir.join("performance_report.md");
    fs::write(&report_path, report).map_err(|e| 
        crate::testing::performance::PerfTestError::TestError(
            format!("Failed to write report: {}", e)
        )
    )?;
    
    println!("Performance report written to: {}", report_path.display());
    
    Ok(())
}

/// Run a targeted performance test
pub fn run_targeted_test(test_name: &str, iterations: usize, output_dir: &Path) -> Result<()> {
    let mut runner = PerformanceTestRunner::new();
    
    // Add test configuration
    runner.add_config(create_standard_test_config(test_name, iterations));
    
    match test_name {
        "transaction_throughput" => {
            // Add transaction tests
            let validator = TransactionValidator::new();
            
            // Single-threaded
            let tx_test = TransactionThroughputTest::new(
                validator.clone(),
                TxGenConfig {
                    multithreaded: false,
                    ..TxGenConfig::default()
                }
            );
            runner.add_component(Box::new(tx_test));
            
            // Multi-threaded
            let tx_test_multi = TransactionThroughputTest::new(
                validator,
                TxGenConfig {
                    multithreaded: true,
                    thread_count: 4,
                    ..TxGenConfig::default()
                }
            );
            runner.add_component(Box::new(tx_test_multi));
        }
        "database_access" => {
            // Add database test
            let db_test = DatabaseAccessTest::new(
                DbConfig::default(),
                vec![
                    DbOperation::Read,
                    DbOperation::Write,
                    DbOperation::Update,
                    DbOperation::Delete,
                ],
                10000, // key space size
                100,   // value size in bytes
            );
            runner.add_component(Box::new(db_test));
        }
        "cache_performance" => {
            // Add cache tests
            for test in create_standard_cache_tests() {
                runner.add_component(test);
            }
        }
        _ => {
            return Err(crate::testing::performance::PerfTestError::ConfigurationError(
                format!("Unknown test name: {}", test_name)
            ));
        }
    }
    
    // Run the test
    runner.run_test(test_name)?;
    
    // Generate report
    let report = runner.generate_report_markdown();
    
    // Ensure the directory exists
    fs::create_dir_all(output_dir)?;
    
    // Write the report to a file
    let report_path = output_dir.join(format!("{}_report.md", test_name));
    fs::write(&report_path, report).map_err(|e| 
        crate::testing::performance::PerfTestError::TestError(
            format!("Failed to write report: {}", e)
        )
    )?;
    
    println!("Performance report written to: {}", report_path.display());
    
    Ok(())
} 