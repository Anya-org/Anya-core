use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Performance test runner integration

use crate::testing::performance::{
    PerformanceTestRunner, TestConfig, Result
};

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
    runner.add_config(create_standard_test_config("performance_test", 10000));
    runner.add_config(create_standard_test_config("all", 1000));
    
    // Note: Add performance testable components here when available
    
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
        "performance_test" => {
            // Note: Add specific performance test components here
            println!("Running performance test: {}", test_name);
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
