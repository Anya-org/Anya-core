//! Integration tests for the performance testing framework

#[cfg(test)]
mod tests {
    use anya_core::testing::performance::{
        PerformanceTestRunner, TestConfig
    };
    use anya_core::testing::performance::cache::{
        CachePerformanceTest, CacheConfig, CacheAlgorithm, AccessPattern
    };
    use std::collections::HashMap;

    #[test]
    fn test_cache_performance_integration() {
        // Create a small cache test that completes quickly
        let config = CacheConfig {
            size: 10,
            algorithm: CacheAlgorithm::LRU,
            access_pattern: AccessPattern::Uniform,
            key_space_size: 20,
            zipf_param: 1.0,
            repeated_set_size: 5,
        };
        
        let test = CachePerformanceTest::new(config);
        
        let mut runner = PerformanceTestRunner::new();
        
        // Add test configuration
        let test_config = TestConfig {
            name: "cache_test".to_string(),
            iterations: 100,
            warmup_iterations: 10,
            duration_limit_secs: 10,
            parameters: HashMap::new(),
        };
        runner.add_config(test_config);
        
        // Add the test component
        runner.add_component(Box::new(test));
        
        // Run the test
        let result = runner.run_test("cache_test");
        assert!(result.is_ok(), "Integration test should run successfully");
        
        // Verify results exist
        let results = runner.get_results();
        assert!(!results.is_empty(), "Should have test results");
        
        // Verify report generation
        let report = runner.generate_report_markdown();
        assert!(!report.is_empty(), "Report should not be empty");
        assert!(report.contains("Performance Test Results"), "Report should have title");
    }
} 