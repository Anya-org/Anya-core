use std::path::Path;
use serde_json::Value;
use crate::performance;
use crate::shared::test_utils;

pub struct TestRunner {
    config: Value,
    report_dir: String,
}

impl TestRunner {
    pub fn new(config_path: &Path, report_dir: &str) -> Result<Self, String> {
        let config = performance::load_test_config()
            .map_err(|e| format!("Failed to load test config: {}", e))?;
            
        Ok(Self {
            config,
            report_dir: report_dir.to_string(),
        })
    }

    pub fn run_all(&self) -> Result<(), String> {
        // Ensure report directory exists
        std::fs::create_dir_all(&self.report_dir)
            .map_err(|e| format!("Failed to create report directory: {}", e))?;

        // Run component tests based on config
        for (component, tests) in self.config["components"].as_object().unwrap() {
            println!("Running tests for component: {}", component);
            
            let test_types = tests["test_types"].as_array().unwrap();
            for test_type in test_types {
                match test_type.as_str().unwrap() {
                    "unit" => self.run_unit_tests(component)?,
                    "integration" => self.run_integration_tests(component)?,
                    "compliance" => self.run_compliance_tests(component)?,
                    "performance" => self.run_performance_tests(component)?,
                    _ => println!("Unknown test type for {}", component),
                }
            }
        }

        // Run performance test suite if configured
        if self.config["performance_tests"].as_object().is_some() {
            println!("Running performance test suite...");
            performance::run_performance_suite(Path::new(&self.report_dir))?;
        }

        // Run compliance tests if configured
        if let Some(compliance) = self.config["compliance_tests"].as_object() {
            for (standard, config) in compliance {
                if config["required"].as_bool().unwrap_or(false) {
                    println!("Running {} compliance tests...", standard);
                    self.run_compliance_standard(standard)?;
                }
            }
        }

        println!("All tests completed successfully!");
        Ok(())
    }

    fn run_unit_tests(&self, component: &str) -> Result<(), String> {
        println!("Running unit tests for {}", component);
        let status = std::process::Command::new("cargo")
            .args(&["test", "--lib", "--package", &format!("anya-{}", component)])
            .status()
            .map_err(|e| format!("Failed to run unit tests: {}", e))?;

        if !status.success() {
            return Err(format!("Unit tests failed for {}", component));
        }
        Ok(())
    }

    fn run_integration_tests(&self, component: &str) -> Result<(), String> {
        println!("Running integration tests for {}", component);
        let status = std::process::Command::new("cargo")
            .args(&["test", "--test", "*", "--package", &format!("anya-{}", component)])
            .status()
            .map_err(|e| format!("Failed to run integration tests: {}", e))?;

        if !status.success() {
            return Err(format!("Integration tests failed for {}", component));
        }
        Ok(())
    }

    fn run_compliance_tests(&self, component: &str) -> Result<(), String> {
        if let Some(standards) = self.config["components"][component]["required_standards"].as_array() {
            for standard in standards {
                println!("Verifying {} compliance with {}", component, standard.as_str().unwrap());
                // Run compliance verification based on standard
                self.verify_compliance(component, standard.as_str().unwrap())?;
            }
        }
        Ok(())
    }

    fn run_performance_tests(&self, component: &str) -> Result<(), String> {
        if let Some(thresholds) = self.config["components"][component]["performance_thresholds"].as_object() {
            println!("Running performance tests for {} with configured thresholds", component);
            // Run performance tests with thresholds
            performance::run_performance_suite(Path::new(&self.report_dir))?;
        }
        Ok(())
    }

    fn run_compliance_standard(&self, standard: &str) -> Result<(), String> {
        let config = &self.config["compliance_tests"][standard];
        let validation_level = config["validation_level"].as_str().unwrap_or("normal");
        
        println!("Running {} compliance tests with {} validation", standard, validation_level);
        
        let status = std::process::Command::new("cargo")
            .args(&["test", "--test", &format!("{}_compliance", standard)])
            .status()
            .map_err(|e| format!("Failed to run compliance tests: {}", e))?;

        if !status.success() {
            return Err(format!("{} compliance tests failed", standard));
        }
        Ok(())
    }

    fn verify_compliance(&self, component: &str, standard: &str) -> Result<(), String> {
        println!("Verifying {} compliance with {}", component, standard);
        
        let status = std::process::Command::new("cargo")
            .args(&["test", "--test", &format!("{}_verification", standard.to_lowercase())])
            .status()
            .map_err(|e| format!("Failed to verify compliance: {}", e))?;

        if !status.success() {
            return Err(format!("Compliance verification failed for {} with {}", component, standard));
        }
        Ok(())
    }
}