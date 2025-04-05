//! Tests for the performance testing framework

#[cfg(test)]
mod tests {
    use crate::testing::performance::{
        PerformanceTestRunner, TestConfig, Timer, MetricType
    };
    use crate::testing::performance::cache::{
        CachePerformanceTest, CacheConfig, CacheAlgorithm, AccessPattern
    };
    use std::collections::HashMap;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_timer() {
        let mut timer = Timer::new();
        
        // Test elapsed time without starting
        let result = timer.elapsed_ms();
        assert!(result.is_err(), "Timer should error if not started");
        
        // Test elapsed time with starting
        timer.start();
        thread::sleep(Duration::from_millis(10));
        let result = timer.elapsed_ms();
        assert!(result.is_ok(), "Timer should work after starting");
        
        let elapsed = result.unwrap();
        assert!(elapsed >= 10, "Timer should measure at least 10ms");
        
        // Test with stop
        timer.stop();
        let elapsed_after_stop = timer.elapsed_ms().unwrap();
        assert_eq!(elapsed_after_stop, elapsed, "Elapsed time should not change after stop");
    }
    
    #[test]
    fn test_cache_performance_test() {
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
        
        let test_config = TestConfig {
            name: "cache_performance".to_string(),
            iterations: 100,
            warmup_iterations: 10,
            duration_limit_secs: 10,
            parameters: HashMap::new(),
        };
        
        let result = test.run_test(&test_config);
        assert!(result.is_ok(), "Cache test should complete successfully");
        
        let test_result = result.unwrap();
        assert_eq!(test_result.name, "cache_performance_LRU_Uniform");
        assert!(test_result.metrics.contains_key("cache_hit_rate"));
        assert!(test_result.metrics.contains_key("operations_per_second"));
        
        let hit_rate = test_result.metrics.get("cache_hit_rate").unwrap();
        assert!(*hit_rate >= 0.0 && *hit_rate <= 100.0, "Hit rate should be between 0-100%");
    }
    
    #[test]
    fn test_performance_test_runner() {
        let mut runner = PerformanceTestRunner::new();
        
        // Add a test configuration
        let config = TestConfig {
            name: "test".to_string(),
            iterations: 10,
            warmup_iterations: 2,
            duration_limit_secs: 5,
            parameters: HashMap::new(),
        };
        runner.add_config(config);
        
        // Create a dummy test implementation
        struct DummyTest;
        
        impl crate::testing::performance::PerformanceTestable for DummyTest {
            fn run_test(&self, config: &TestConfig) -> crate::testing::performance::Result<crate::testing::performance::TestResult> {
                let mut metrics = HashMap::new();
                metrics.insert("dummy_metric".to_string(), 42.0);
                
                let mut metric_types = HashMap::new();
                metric_types.insert("dummy_metric".to_string(), MetricType::TPS);
                
                Ok(crate::testing::performance::TestResult {
                    name: "dummy_test".to_string(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    duration_ms: 123,
                    metrics,
                    metric_types,
                    parameters: HashMap::new(),
                })
            }
            
            fn name(&self) -> &str {
                "test"
            }
        }
        
        // Add the test component
        runner.add_component(Box::new(DummyTest));
        
        // Run the test
        let result = runner.run_test("test");
        assert!(result.is_ok(), "Test should run successfully");
        
        // Check the results
        let results = runner.get_results();
        assert_eq!(results.len(), 1, "Should have one test result");
        assert_eq!(results[0].name, "dummy_test");
        
        // Check the report generation
        let report = runner.generate_report_markdown();
        assert!(report.contains("dummy_test"), "Report should contain test name");
        assert!(report.contains("42.00 TPS"), "Report should contain formatted metric");
    }
} 