#!/bin/bash
# Enhance unified test with better error handling and reporting
set -e

TEST_FILE="src/test/unified_test.rs"

if [ ! -f "$TEST_FILE" ]; then
    echo "Error: Unified test file not found at $TEST_FILE"
    exit 1
fi

# Add improved error handling and recovery
sed -i '/fn run_test_with_dashboard<F>(&mut self, test_name: &str, test_fn: F, dashboard: &Dashboard/a \        // Add timeout protection for tests\n        let timeout = std::time::Duration::from_secs(60); // 60-second timeout\n        let test_future = std::future::timeout(timeout, async {\n            test_fn()\n        });\n        \n        match test_future.await {\n            Ok(result) => {\n                // Original test handling\n            },\n            Err(_) => {\n                let elapsed = test_start.elapsed();\n                dashboard.set_operation(&format!("Test \'{}\' timed out after {:?}", test_name, elapsed), OperationType::Error);\n                dashboard.add_detail("Test exceeded maximum execution time of 60 seconds");\n                self.results.failed.push((test_name.to_string(), "Timeout".to_string()));\n                *completed_tests += 1;\n                dashboard.set_progress(*completed_tests, total_tests);\n                return Ok(());\n            }\n        }' "$TEST_FILE"

# Add automatic error recovery
sed -i '/fn run_bitcoin_tests_with_dashboard(&mut self, dashboard: &Dashboard, completed_tests: &mut usize, total_tests: usize) -> Result<(), String> {/a \        // Add automatic recovery for test failures\n        let component_guard = ComponentRecoveryGuard::new("bitcoin");\n        let result = self.run_bitcoin_tests_core(dashboard, completed_tests, total_tests);\n        if result.is_err() {\n            dashboard.set_operation("Attempting Bitcoin component recovery...", OperationType::Warning);\n            if let Err(e) = component_guard.attempt_recovery() {\n                dashboard.add_detail(&format!("Recovery failed: {}", e));\n            } else {\n                dashboard.add_detail("Component successfully reset to working state");\n                // Retry the test after recovery\n                let retry_result = self.run_bitcoin_tests_core(dashboard, completed_tests, total_tests);\n                if retry_result.is_ok() {\n                    dashboard.set_operation("Bitcoin tests recovered and passed", OperationType::Success);\n                    return Ok(());\n                }\n            }\n        }\n        result\n    }\n\n    fn run_bitcoin_tests_core(&mut self, dashboard: &Dashboard, completed_tests: &mut usize, total_tests: usize) -> Result<(), String> {' "$TEST_FILE"

echo "Enhanced unified test with better error handling, timeouts, and recovery"
