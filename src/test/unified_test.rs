use log::{info, warn, error};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::process::Command;
use std::path::Path;
use std::fs;
use crate::config;
use crate::testing::UnifiedTester;
use std::fmt;
use crate::dashboard::{Dashboard, DashboardConfig, OperationType};
use crate::network::validation::{NetworkValidator, NetworkValidationConfig, ValidationStatus};

/// Configuration for the unified test runner
pub struct UnifiedTestConfig {
    pub bitcoin_rpc_url: String,
    pub components: Vec<String>,
    pub generate_reports: bool,
    pub report_dir: String,
    pub verbose: bool,
}

impl Default for UnifiedTestConfig {
    fn default() -> Self {
        Self {
            bitcoin_rpc_url: String::new(), // Will be filled from config file
            components: vec![
                "bitcoin".to_string(),
                "dao".to_string(),
                "web5".to_string(),
                "ml".to_string(),
                "system".to_string(),
            ],
            generate_reports: true,
            report_dir: "reports".to_string(),
            verbose: false,
        }
    }
}

/// The main unified test runner
pub struct UnifiedTestRunner {
    config: UnifiedTestConfig,
    tester: Arc<UnifiedTester>,
    results: TestResults,
    network_validation_result: Option<ValidationResult>,
}

/// Structure to hold test results
#[derive(Default)]
pub struct TestResults {
    pub passed: Vec<String>,
    pub failed: Vec<(String, String)>, // (test_name, error)
    pub skipped: Vec<String>,
    pub total_time_ms: u64,
}

/// Status of a system check
#[derive(Debug, PartialEq, Clone)]
pub enum SystemCheckStatus {
    Pass,
    Warning,
    Fail,
    Pending,
}

impl fmt::Display for SystemCheckStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemCheckStatus::Pass => write!(f, "✅ Pass"),
            SystemCheckStatus::Warning => write!(f, "⚠️ Warning"),
            SystemCheckStatus::Fail => write!(f, "❌ Fail"),
            SystemCheckStatus::Pending => write!(f, "⏳ Pending"),
        }
    }
}

/// Report of system checks
pub struct SystemCheckReport {
    pub cpu_check: SystemCheckStatus,
    pub memory_check: SystemCheckStatus,
    pub disk_check: SystemCheckStatus,
    pub network_check: SystemCheckStatus,
    pub bitcoin_node_check: SystemCheckStatus,
    pub recommendations: Vec<String>,
    pub overall_status: SystemCheckStatus,
}

impl UnifiedTestRunner {
    /// Create a new UnifiedTestRunner with the given configuration
    pub fn new(config: UnifiedTestConfig) -> Result<Self, String> {
        // Create reports directory if it doesn't exist
        if config.generate_reports && !Path::new(&config.report_dir).exists() {
            fs::create_dir_all(&config.report_dir)
                .map_err(|e| format!("Failed to create reports directory: {}", e))?;
        }
        
        // Load the Bitcoin RPC URL from configuration if not specified
        let mut config = config;
        if config.bitcoin_rpc_url.is_empty() {
            let app_config = config::load_config("config/anya.conf")
                .map_err(|e| format!("Failed to load configuration: {}", e))?;
            
            config.bitcoin_rpc_url = if !app_config.network.bitcoin_custom_rpc_url.is_empty() {
                app_config.network.bitcoin_custom_rpc_url
            } else if app_config.network.network_type == "mainnet" {
                app_config.network.bitcoin_mainnet_rpc_url
            } else {
                app_config.network.bitcoin_testnet_rpc_url
            };
        }
        
        // Initialize the unified tester
        let tester = Arc::new(UnifiedTester::new());
        
        Ok(Self {
            config,
            tester,
            results: TestResults::default(),
            network_validation_result: None,
        })
    }
    
    /// Perform system checks before running tests and provide advice
    pub fn check_system(&self) -> Result<SystemCheckReport, String> {
        info!("Running system prerequisite checks...");
        
        let mut report = SystemCheckReport {
            cpu_check: SystemCheckStatus::Pending,
            memory_check: SystemCheckStatus::Pending,
            disk_check: SystemCheckStatus::Pending,
            network_check: SystemCheckStatus::Pending,
            bitcoin_node_check: SystemCheckStatus::Pending,
            recommendations: Vec::new(),
            overall_status: SystemCheckStatus::Pending,
        };
        
        // Check CPU
        let cpu_cores = num_cpus::get();
        if cpu_cores >= 4 {
            report.cpu_check = SystemCheckStatus::Pass;
        } else if cpu_cores >= 2 {
            report.cpu_check = SystemCheckStatus::Warning;
            report.recommendations.push(format!(
                "CPU has only {} cores. For optimal performance, 4+ cores are recommended.",
                cpu_cores
            ));
        } else {
            report.cpu_check = SystemCheckStatus::Fail;
            report.recommendations.push(format!(
                "CPU has only {} core(s). Minimum requirement is 2 cores.",
                cpu_cores
            ));
        }
        
        // Check memory
        let sys_info = sysinfo::System::new_all();
        let total_memory_kb = sys_info.total_memory();
        let total_memory_gb = total_memory_kb as f64 / 1_048_576.0;
        
        if total_memory_gb >= 8.0 {
            report.memory_check = SystemCheckStatus::Pass;
        } else if total_memory_gb >= 4.0 {
            report.memory_check = SystemCheckStatus::Warning;
            report.recommendations.push(format!(
                "System has {:.1}GB of RAM. For optimal performance, 8+ GB are recommended.",
                total_memory_gb
            ));
        } else {
            report.memory_check = SystemCheckStatus::Fail;
            report.recommendations.push(format!(
                "System has only {:.1}GB of RAM. Minimum requirement is 4GB.",
                total_memory_gb
            ));
        }
        
        // Check disk space
        let current_dir = std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;
        let disk_info = fs2::disk_space_available(current_dir).map_err(|e| format!("Failed to get disk space: {}", e))?;
        let disk_space_gb = disk_info as f64 / 1_073_741_824.0;
        
        if disk_space_gb >= 20.0 {
            report.disk_check = SystemCheckStatus::Pass;
        } else if disk_space_gb >= 10.0 {
            report.disk_check = SystemCheckStatus::Warning;
            report.recommendations.push(format!(
                "Only {:.1}GB of disk space available. For optimal performance, 20+ GB are recommended.",
                disk_space_gb
            ));
        } else {
            report.disk_check = SystemCheckStatus::Fail;
            report.recommendations.push(format!(
                "Only {:.1}GB of disk space available. Minimum requirement is 10GB.",
                disk_space_gb
            ));
        }
        
        // Check network connectivity to Bitcoin RPC endpoint
        match self.check_bitcoin_rpc_connectivity() {
            Ok(_) => {
                report.network_check = SystemCheckStatus::Pass;
                report.bitcoin_node_check = SystemCheckStatus::Pass;
            },
            Err(e) => {
                report.network_check = SystemCheckStatus::Fail;
                report.bitcoin_node_check = SystemCheckStatus::Fail;
                report.recommendations.push(format!(
                    "Cannot connect to Bitcoin RPC endpoint ({}): {}",
                    self.config.bitcoin_rpc_url, e
                ));
            }
        }
        
        // Determine overall status
        if report.cpu_check == SystemCheckStatus::Fail || 
           report.memory_check == SystemCheckStatus::Fail || 
           report.disk_check == SystemCheckStatus::Fail ||
           report.network_check == SystemCheckStatus::Fail {
            report.overall_status = SystemCheckStatus::Fail;
        } else if report.cpu_check == SystemCheckStatus::Warning || 
                  report.memory_check == SystemCheckStatus::Warning || 
                  report.disk_check == SystemCheckStatus::Warning {
            report.overall_status = SystemCheckStatus::Warning;
        } else {
            report.overall_status = SystemCheckStatus::Pass;
        }
        
        // Print the report
        self.print_system_check_report(&report);
        
        Ok(report)
    }
    
    /// Check connectivity to Bitcoin RPC endpoint
    fn check_bitcoin_rpc_connectivity(&self) -> Result<(), String> {
        // Parse the URL
        let url = url::Url::parse(&self.config.bitcoin_rpc_url)
            .map_err(|e| format!("Invalid URL: {}", e))?;
        
        // Create a simple JSON-RPC request to getnetworkinfo
        let client = reqwest::blocking::Client::new();
        let request_body = serde_json::json!({
            "jsonrpc": "1.0",
            "id": "anya-core-test",
            "method": "getnetworkinfo",
            "params": []
        });
        
        // Send the request
        let response = client.post(url.clone())
            .header("Content-Type", "application/json")
            .body(request_body.to_string())
            .send()
            .map_err(|e| format!("Failed to connect to Bitcoin RPC endpoint: {}", e))?;
        
        // Check the response
        if !response.status().is_success() {
            return Err(format!("Bitcoin RPC endpoint returned error status: {}", response.status()));
        }
        
        let response_text = response.text()
            .map_err(|e| format!("Failed to read response: {}", e))?;
        
        let response_json: serde_json::Value = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        // Check for error in the response
        if let Some(error) = response_json.get("error") {
            if !error.is_null() {
                return Err(format!("Bitcoin RPC endpoint returned error: {}", error));
            }
        }
        
        Ok(())
    }
    
    /// Print the system check report
    fn print_system_check_report(&self, report: &SystemCheckReport) {
        info!("========= System Check Report =========");
        info!("CPU: {}", report.cpu_check);
        info!("Memory: {}", report.memory_check);
        info!("Disk Space: {}", report.disk_check);
        info!("Network Connectivity: {}", report.network_check);
        info!("Bitcoin Node Connection: {}", report.bitcoin_node_check);
        info!("Overall Status: {}", report.overall_status);
        
        if !report.recommendations.is_empty() {
            info!("Recommendations:");
            for (i, recommendation) in report.recommendations.iter().enumerate() {
                info!("  {}. {}", i + 1, recommendation);
            }
        }
        
        match report.overall_status {
            SystemCheckStatus::Pass => info!("✅ System meets all requirements for testing."),
            SystemCheckStatus::Warning => info!("⚠️ System meets minimum requirements, but some optimizations are recommended."),
            SystemCheckStatus::Fail => error!("❌ System does not meet minimum requirements for testing."),
            SystemCheckStatus::Pending => error!("⚠️ System check incomplete."),
        }
        info!("=======================================");
    }
    
    /// Start the dashboard
    fn start_dashboard(&self) -> Dashboard {
        let config = DashboardConfig {
            title: format!("Anya-Core Unified Test Suite - {}", 
                           if self.config.bitcoin_rpc_url.contains("testnet") {
                               "Testnet"
                           } else {
                               "Mainnet"
                           }),
            ..Default::default()
        };
        
        let mut dashboard = Dashboard::new(config);
        dashboard.start();
        
        // Initial state
        dashboard.set_operation("Initializing test suite...", OperationType::Info);
        dashboard.set_progress(0, self.config.components.len() * 5); // Approximation
        
        dashboard
    }
    
    /// Run network validation as a prerequisite before running tests
    pub async fn validate_network(&self, dashboard: &Dashboard) -> Result<(), String> {
        dashboard.set_operation("Running network validation...", OperationType::Info);
        
        // Create network validator with our Bitcoin RPC endpoint
        let mut config = NetworkValidationConfig::default();
        
        // Add our specific endpoints for validation
        if !self.config.bitcoin_rpc_url.is_empty() {
            config.endpoints.push(self.config.bitcoin_rpc_url.clone());
        }
        
        let validator = NetworkValidator::new(config);
        let validation_result = validator.validate_network().await;
        
        // Update dashboard with network validation results
        dashboard.set_operation(
            &format!("Network validation completed: {}", match validation_result.overall_status {
                ValidationStatus::Pass => "PASSED",
                ValidationStatus::Warning => "WARNING",
                ValidationStatus::Fail => "FAILED",
                ValidationStatus::Skipped => "SKIPPED",
            }),
            match validation_result.overall_status {
                ValidationStatus::Pass => OperationType::Success,
                ValidationStatus::Warning => OperationType::Warning,
                ValidationStatus::Fail => OperationType::Error,
                ValidationStatus::Skipped => OperationType::Info,
            }
        );
        
        // Display recommendations in the dashboard
        for (i, recommendation) in validation_result.recommendations.iter().enumerate() {
            dashboard.add_detail(&format!("Recommendation {}: {}", i+1, recommendation));
        }
        
        // If the validation failed, return an error
        if validation_result.overall_status == ValidationStatus::Fail {
            return Err("Network validation failed. Please check recommendations and try again.".to_string());
        }
        
        // Store validation results for later use
        self.network_validation_result = Some(validation_result);
        
        Ok(())
    }
    
    /// Run all specified tests
    pub async fn run_all_tests(&mut self) -> Result<&TestResults, String> {
        // Run system check first
        let system_report = self.check_system()?;
        
        // If system check fails, return error
        if system_report.overall_status == SystemCheckStatus::Fail {
            return Err("System does not meet minimum requirements for testing.".to_string());
        }
        
        // Start the dashboard
        let dashboard = self.start_dashboard();
        
        // Run network validation
        if let Err(e) = self.validate_network(&dashboard).await {
            dashboard.set_operation(&format!("Network validation failed: {}", e), OperationType::Error);
            return Err(e);
        }
        
        let start_time = Instant::now();
        dashboard.set_operation(&format!("Starting unified test suite with Bitcoin RPC endpoint: {}", 
                                       self.config.bitcoin_rpc_url), 
                              OperationType::Info);
        
        // Run component tests
        let total_tests = self.count_total_tests();
        let mut completed_tests = 0;
        
        for component in &self.config.components {
            dashboard.set_operation(&format!("Running {} tests...", component), OperationType::Info);
            
            match component.as_str() {
                "bitcoin" => {
                    let result = self.run_bitcoin_tests_with_dashboard(&dashboard, &mut completed_tests, total_tests);
                    if let Err(e) = result {
                        dashboard.set_operation(&format!("Bitcoin tests failed: {}", e), OperationType::Error);
                        return Err(e);
                    }
                },
                "dao" => {
                    let result = self.run_dao_tests_with_dashboard(&dashboard, &mut completed_tests, total_tests);
                    if let Err(e) = result {
                        dashboard.set_operation(&format!("DAO tests failed: {}", e), OperationType::Error);
                        return Err(e);
                    }
                },
                "web5" => {
                    let result = self.run_web5_tests_with_dashboard(&dashboard, &mut completed_tests, total_tests);
                    if let Err(e) = result {
                        dashboard.set_operation(&format!("Web5 tests failed: {}", e), OperationType::Error);
                        return Err(e);
                    }
                },
                "ml" => {
                    let result = self.run_ml_tests_with_dashboard(&dashboard, &mut completed_tests, total_tests);
                    if let Err(e) = result {
                        dashboard.set_operation(&format!("ML tests failed: {}", e), OperationType::Error);
                        return Err(e);
                    }
                },
                "system" => {
                    let result = self.run_system_tests_with_dashboard(&dashboard, &mut completed_tests, total_tests);
                    if let Err(e) = result {
                        dashboard.set_operation(&format!("System tests failed: {}", e), OperationType::Error);
                        return Err(e);
                    }
                },
                "compliance" => {
                    let result = self.run_compliance_tests_with_dashboard(&dashboard, &mut completed_tests, total_tests);
                    if let Err(e) = result {
                        dashboard.set_operation(&format!("Compliance tests failed: {}", e), OperationType::Error);
                        return Err(e);
                    }
                },
                _ => {
                    warn!("Skipping unknown component: {}", component);
                    self.results.skipped.push(format!("Unknown component: {}", component));
                }
            }
        }
        
        // Calculate total time
        self.results.total_time_ms = start_time.elapsed().as_millis() as u64;
        
        // Generate final report
        if self.config.generate_reports {
            dashboard.set_operation("Generating unified test report...", OperationType::Info);
            self.generate_unified_report()?;
        }
        
        // Log summary
        dashboard.set_operation("Test suite completed", OperationType::Success);
        self.log_summary();
        
        // Stop the dashboard
        dashboard.stop();
        
        Ok(&self.results)
    }
    
    /// Count the total number of tests across all components
    fn count_total_tests(&self) -> usize {
        let mut count = 0;
        for component in &self.config.components {
            count += match component.as_str() {
                "bitcoin" => 4, // Number of Bitcoin tests
                "dao" => 4,     // Number of DAO tests
                "web5" => 4,     // Number of Web5 tests
                "ml" => 3,       // Number of ML tests
                "system" => 6,   // Number of system tests
                "compliance" => 3, // Number of compliance tests
                _ => 0,
            };
        }
        count
    }
    
    /// Run Bitcoin tests with dashboard updates
    fn run_bitcoin_tests_with_dashboard(&mut self, dashboard: &Dashboard, completed_tests: &mut usize, total_tests: usize) -> Result<(), String> {
        dashboard.set_operation("Running Bitcoin tests...", OperationType::Info);
        let component_start = Instant::now();
        
        // Test Bitcoin Core connection
        dashboard.set_operation("Testing Bitcoin Core connection...", OperationType::Info);
        self.run_test_with_dashboard("bitcoin_connection", || {
            crate::test::bitcoin_tests::test_bitcoin_core_connection(&self.tester)
        }, dashboard, completed_tests, total_tests)?;
        
        // Test Taproot support
        dashboard.set_operation("Testing Taproot support...", OperationType::Info);
        self.run_test_with_dashboard("bitcoin_taproot", || {
            // Pass our configured RPC URL
            self.tester.bitcoin_validator.verify_taproot_support_with_endpoint(&self.config.bitcoin_rpc_url)
        }, dashboard, completed_tests, total_tests)?;
        
        // Test transaction validation
        dashboard.set_operation("Testing transaction validation...", OperationType::Info);
        self.run_test_with_dashboard("bitcoin_transaction", || {
            crate::test::bitcoin_tests::test_transaction_validation(&self.tester)
        }, dashboard, completed_tests, total_tests)?;
        
        // Test PSBT handling
        dashboard.set_operation("Testing PSBT handling...", OperationType::Info);
        self.run_test_with_dashboard("bitcoin_psbt", || {
            crate::test::bitcoin_tests::test_psbt_handling(&self.tester)
        }, dashboard, completed_tests, total_tests)?;
        
        dashboard.set_operation(&format!("Bitcoin tests completed in {:?}", component_start.elapsed()), OperationType::Success);
        Ok(())
    }
    
    /// Run DAO-specific tests
    fn run_dao_tests_with_dashboard(&mut self, dashboard: &Dashboard, completed_tests: &mut usize, total_tests: usize) -> Result<(), String> {
        dashboard.set_operation("Running DAO tests...", OperationType::Info);
        let component_start = Instant::now();
        
        // Add calls to DAO tests here, similar to Bitcoin tests
        self.run_test_with_dashboard("dao_contracts", || {
            crate::test::dao_tests::test_dao_contracts()
        }, dashboard, completed_tests, total_tests)?;
        
        self.run_test_with_dashboard("dao_governance", || {
            crate::test::dao_tests::test_governance()
        }, dashboard, completed_tests, total_tests)?;
        
        self.run_test_with_dashboard("dao_voting", || {
            crate::test::dao_tests::test_voting()
        }, dashboard, completed_tests, total_tests)?;
        
        self.run_test_with_dashboard("dao_proposal_execution", || {
            crate::test::dao_tests::test_proposal_execution()
        }, dashboard, completed_tests, total_tests)?;
        
        dashboard.set_operation(&format!("DAO tests completed in {:?}", component_start.elapsed()), OperationType::Success);
        Ok(())
    }
    
    /// Run Web5-specific tests
    fn run_web5_tests_with_dashboard(&mut self, dashboard: &Dashboard, completed_tests: &mut usize, total_tests: usize) -> Result<(), String> {
        dashboard.set_operation("Running Web5 tests...", OperationType::Info);
        let component_start = Instant::now();
        
        // Add calls to Web5 tests here
        self.run_test_with_dashboard("web5_dwn_connection", || {
            crate::test::web5_tests::test_dwn_connection()
        }, dashboard, completed_tests, total_tests)?;
        
        self.run_test_with_dashboard("web5_did_operations", || {
            crate::test::web5_tests::test_did_operations()
        }, dashboard, completed_tests, total_tests)?;
        
        self.run_test_with_dashboard("web5_data_operations", || {
            crate::test::web5_tests::test_data_operations()
        }, dashboard, completed_tests, total_tests)?;
        
        self.run_test_with_dashboard("web5_protocols", || {
            crate::test::web5_tests::test_protocols()
        }, dashboard, completed_tests, total_tests)?;
        
        dashboard.set_operation(&format!("Web5 tests completed in {:?}", component_start.elapsed()), OperationType::Success);
        Ok(())
    }
    
    /// Run ML-specific tests
    fn run_ml_tests_with_dashboard(&mut self, dashboard: &Dashboard, completed_tests: &mut usize, total_tests: usize) -> Result<(), String> {
        dashboard.set_operation("Running ML tests...", OperationType::Info);
        let component_start = Instant::now();
        
        // Add calls to ML tests here
        self.run_test_with_dashboard("ml_model_loading", || {
            crate::test::ml_tests::test_model_loading()
        }, dashboard, completed_tests, total_tests)?;
        
        self.run_test_with_dashboard("ml_inference", || {
            crate::test::ml_tests::test_inference()
        }, dashboard, completed_tests, total_tests)?;
        
        self.run_test_with_dashboard("ml_telemetry", || {
            crate::test::ml_tests::test_telemetry()
        }, dashboard, completed_tests, total_tests)?;
        
        dashboard.set_operation(&format!("ML tests completed in {:?}", component_start.elapsed()), OperationType::Success);
        Ok(())
    }
    
    /// Run system-level integration tests
    fn run_system_tests_with_dashboard(&mut self, dashboard: &Dashboard, completed_tests: &mut usize, total_tests: usize) -> Result<(), String> {
        dashboard.set_operation("Running system integration tests...", OperationType::Info);
        let component_start = Instant::now();
        
        // Test component dependencies
        self.run_test_with_dashboard("system_dependencies", || {
            crate::test::system_tests::test_component_dependencies()
        }, dashboard, completed_tests, total_tests)?;
        
        // Test system health
        self.run_test_with_dashboard("system_health", || {
            crate::test::system_tests::test_system_health()
        }, dashboard, completed_tests, total_tests)?;
        
        // Test Bitcoin-DAO integration with our configured RPC URL
        self.run_test_with_dashboard("bitcoin_dao_integration", || {
            // Create a custom test with our configured RPC URL
            let proposal_data = format!(r#"{{
                "title": "Test Bitcoin Integration",
                "description": "This is a test proposal with Bitcoin integration",
                "action": {{
                    "type": "bitcoin_transaction",
                    "network": "{}",
                    "endpoint": "{}",
                    "recipient": "{}",
                    "amount": 0.001
                }}
            }}"#, 
            if self.config.bitcoin_rpc_url.contains("testnet") { "testnet" } else { "mainnet" },
            self.config.bitcoin_rpc_url,
            if self.config.bitcoin_rpc_url.contains("testnet") { 
                "tb1q6rhpng9evdsfnn8kz0rk6e9vlsq8we5utg3447" 
            } else { 
                "bc1q6rhpng9evdsfnn8kz0rk6e9vlsq8we5utg3447" 
            });
            
            // Custom implementation of the Bitcoin-DAO integration test
            // that passes our specific configuration
            let proposal_file = "test_proposal.json";
            match fs::write(proposal_file, &proposal_data) {
                Ok(_) => (),
                Err(e) => return Err(format!("Failed to create test proposal file: {}", e)),
            }
            
            // Rest of the test implementation...
            // This is a placeholder - in a real implementation, you would call methods from system_tests
            // with the appropriate parameters
            fs::remove_file(proposal_file).ok(); // Clean up
            
            // For now, just call the original function
            crate::test::system_tests::test_bitcoin_dao_integration()
        }, dashboard, completed_tests, total_tests)?;
        
        // Test Web5-ML integration
        self.run_test_with_dashboard("web5_ml_integration", || {
            crate::test::system_tests::test_web5_ml_integration()
        }, dashboard, completed_tests, total_tests)?;
        
        // Test performance
        self.run_test_with_dashboard("performance", || {
            crate::test::system_tests::test_performance()
        }, dashboard, completed_tests, total_tests)?;
        
        // Test BIP compliance with our configured RPC URL
        self.run_test_with_dashboard("bip_compliance", || {
            // Call the original function which already uses the configuration
            crate::test::system_tests::verify_bip_compliance()
        }, dashboard, completed_tests, total_tests)?;
        
        dashboard.set_operation(&format!("System integration tests completed in {:?}", component_start.elapsed()), OperationType::Success);
        Ok(())
    }
    
    /// Run compliance verification tests
    fn run_compliance_tests_with_dashboard(&mut self, dashboard: &Dashboard, completed_tests: &mut usize, total_tests: usize) -> Result<(), String> {
        dashboard.set_operation("Running compliance verification tests...", OperationType::Info);
        let component_start = Instant::now();
        
        // Verify BPC-3 compliance
        self.run_test_with_dashboard("bpc3_compliance", || {
            let result = crate::compliance::verify_bpc3();
            // Convert the void result to our Result type
            if let Err(e) = result {
                Err(e.to_string())
            } else {
                Ok(())
            }
        }, dashboard, completed_tests, total_tests)?;
        
        // Verify DAO-4 compliance
        self.run_test_with_dashboard("dao4_compliance", || {
            let result = crate::compliance::verify_dao4();
            // Convert the void result to our Result type
            if let Err(e) = result {
                Err(e.to_string())
            } else {
                Ok(())
            }
        }, dashboard, completed_tests, total_tests)?;
        
        // Verify AIS-3 compliance
        self.run_test_with_dashboard("ais3_compliance", || {
            let result = crate::compliance::verify_ais3();
            // Convert the void result to our Result type
            if let Err(e) = result {
                Err(e.to_string())
            } else {
                Ok(())
            }
        }, dashboard, completed_tests, total_tests)?;
        
        dashboard.set_operation(&format!("Compliance verification tests completed in {:?}", component_start.elapsed()), OperationType::Success);
        Ok(())
    }
    
    /// Run a single test with dashboard updates
    fn run_test_with_dashboard<F>(&mut self, test_name: &str, test_fn: F, dashboard: &Dashboard, 
                                completed_tests: &mut usize, total_tests: usize) -> Result<(), String> 
    where
        F: FnOnce() -> Result<(), String>
    {
        dashboard.set_operation(&format!("Running test: {}", test_name), OperationType::Info);
        let test_start = Instant::now();
        
        match test_fn() {
            Ok(_) => {
                let elapsed = test_start.elapsed();
                dashboard.set_operation(&format!("Test '{}' passed in {:?}", test_name, elapsed), OperationType::Success);
                self.results.passed.push(test_name.to_string());
                *completed_tests += 1;
                dashboard.set_progress(*completed_tests, total_tests);
                Ok(())
            },
            Err(e) => {
                let elapsed = test_start.elapsed();
                dashboard.set_operation(&format!("Test '{}' failed in {:?}", test_name, elapsed), OperationType::Error);
                dashboard.add_detail(&format!("Error: {}", e));
                self.results.failed.push((test_name.to_string(), e));
                *completed_tests += 1;
                dashboard.set_progress(*completed_tests, total_tests);
                
                // Continue with other tests even if one fails
                Ok(())
            }
        }
    }
    
    /// Generate a unified test report
    fn generate_unified_report(&self) -> Result<(), String> {
        let report_path = std::path::Path::new(self.config.report_dir).join("unified_test_report.md").to_string_lossy();
        
        let mut report_content = format!(
            "# Anya-Core Unified Test Report\n\n" +
            "Date: {}\n\n" +
            "## Configuration\n\n" +
            "* Bitcoin RPC Endpoint: {}\n" +
            "* Tested Components: {}\n\n" +
            "## Summary\n\n" +
            "* Total Tests: {}\n" +
            "* Passed: {}\n" +
            "* Failed: {}\n" +
            "* Skipped: {}\n" +
            "* Total Time: {}ms\n\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            self.config.bitcoin_rpc_url,
            self.config.components.join(", "),
            self.results.passed.len() + self.results.failed.len(),
            self.results.passed.len(),
            self.results.failed.len(),
            self.results.skipped.len(),
            self.results.total_time_ms
        );
        
        // Add passed tests
        if !self.results.passed.is_empty() {
            report_content.push_str("## Passed Tests\n\n");
            for test in &self.results.passed {
                report_content.push_str(&format!("* ✅ {}\n", test));
            }
            report_content.push_str("\n");
        }
        
        // Add failed tests with errors
        if !self.results.failed.is_empty() {
            report_content.push_str("## Failed Tests\n\n");
            for (test, error) in &self.results.failed {
                report_content.push_str(&format!("### ❌ {}\n\n```\n{}\n```\n\n", test, error));
            }
        }
        
        // Add skipped tests
        if !self.results.skipped.is_empty() {
            report_content.push_str("## Skipped Tests\n\n");
            for test in &self.results.skipped {
                report_content.push_str(&format!("* ⚠️ {}\n", test));
            }
            report_content.push_str("\n");
        }
        
        // Add compliance section
        report_content.push_str("## Compliance Status\n\n");
        report_content.push_str("| Standard | Status |\n");
        report_content.push_str("|----------|--------|\n");
        
        let bpc3_status = if self.results.failed.iter().any(|(name, _)| name.contains("bpc3")) {
            "❌ Failed"
        } else if self.results.passed.iter().any(|name| name.contains("bpc3")) {
            "✅ Passed"
        } else {
            "⚠️ Not Tested"
        };
        
        let dao4_status = if self.results.failed.iter().any(|(name, _)| name.contains("dao4")) {
            "❌ Failed"
        } else if self.results.passed.iter().any(|name| name.contains("dao4")) {
            "✅ Passed"
        } else {
            "⚠️ Not Tested"
        };
        
        let ais3_status = if self.results.failed.iter().any(|(name, _)| name.contains("ais3")) {
            "❌ Failed"
        } else if self.results.passed.iter().any(|name| name.contains("ais3")) {
            "✅ Passed"
        } else {
            "⚠️ Not Tested"
        };
        
        report_content.push_str(&format!("| BPC-3 | {} |\n", bpc3_status));
        report_content.push_str(&format!("| DAO-4 | {} |\n", dao4_status));
        report_content.push_str(&format!("| AIS-3 | {} |\n", ais3_status));
        
        // Write the report
        fs::write(&report_path, report_content)
            .map_err(|e| format!("Failed to write unified test report: {}", e))?;
        
        info!("Unified test report generated at {}", report_path);
        Ok(())
    }
    
    /// Log a summary of the test results
    fn log_summary(&self) {
        info!("========= Unified Test Summary =========");
        info!("Total Tests: {}", self.results.passed.len() + self.results.failed.len());
        info!("Passed: {}", self.results.passed.len());
        info!("Failed: {}", self.results.failed.len());
        info!("Skipped: {}", self.results.skipped.len());
        info!("Total Time: {}ms", self.results.total_time_ms);
        
        if !self.results.failed.is_empty() {
            info!("Failed Tests:");
            for (test, _) in &self.results.failed {
                info!("  - {}", test);
            }
        }
        
        if self.results.failed.is_empty() {
            info!("🎉 All tests passed!");
        } else {
            error!("❌ Some tests failed. See the report for details.");
        }
        
        // Add prompt for configuration updates
        if self.results.failed.is_empty() {
            info!("Would you like to update your configuration based on test results? [y/N]");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();
            if input.trim().to_lowercase() == "y" {
                match self.update_config() {
                    Ok(_) => info!("✅ Configuration updated successfully."),
                    Err(e) => error!("❌ Failed to update configuration: {}", e),
                }
            }
        } else {
            info!("Some tests failed. Fix the issues before updating configuration.");
        }
        info!("========================================");
    }
    
    /// Update the anya-config based on test results
    pub fn update_config(&self) -> Result<(), String> {
        info!("Updating Anya-Core configuration based on test results...");
        
        // Load the current configuration
        let mut config = config::load_config("config/anya.conf")
            .map_err(|e| format!("Failed to load configuration: {}", e))?;
        
        // Update configuration based on test results
        let mut updates = Vec::new();
        
        // 1. Update Bitcoin RPC URL if the tests were successful
        if self.results.failed.iter().all(|(name, _)| !name.contains("bitcoin")) {
            // All Bitcoin tests passed, set the RPC URL in the config
            if let Some(network) = &mut config.network {
                // Check if we're using testnet or mainnet based on the URL
                if self.config.bitcoin_rpc_url.contains("testnet") {
                    network.network_type = "testnet".to_string();
                    network.bitcoin_testnet_rpc_url = self.config.bitcoin_rpc_url.clone();
                    updates.push("Updated Bitcoin testnet RPC URL based on successful tests".to_string());
                } else {
                    network.network_type = "mainnet".to_string();
                    network.bitcoin_mainnet_rpc_url = self.config.bitcoin_rpc_url.clone();
                    updates.push("Updated Bitcoin mainnet RPC URL based on successful tests".to_string());
                }
            }
        }
        
        // 2. Adjust performance settings based on system capabilities and test results
        let sys_info = sysinfo::System::new_all();
        let total_memory_kb = sys_info.total_memory();
        let total_memory_gb = total_memory_kb as f64 / 1_048_576.0;
        
        if let Some(performance) = &mut config.performance {
            // Adjust cache size based on available memory
            if total_memory_gb >= 16.0 {
                performance.cache_size_mb = 200;
                updates.push("Set cache size to 200MB based on high system memory".to_string());
            } else if total_memory_gb >= 8.0 {
                performance.cache_size_mb = 100;
                updates.push("Set cache size to 100MB based on medium system memory".to_string());
            } else {
                performance.cache_size_mb = 50;
                updates.push("Set cache size to 50MB based on limited system memory".to_string());
            }
            
            // Adjust batch size based on performance test results
            if self.results.failed.iter().any(|(name, _)| name == "performance") {
                // Performance test failed, reduce batch size
                performance.batch_size = performance.batch_size.saturating_sub(50);
                updates.push(format!("Reduced batch size to {} due to performance test failure", performance.batch_size));
            }
        }
        
        // 3. Adjust DAO settings based on test results
        if let Some(dao) = &mut config.dao {
            // If DAO tests passed, keep current settings, otherwise adjust
            if self.results.failed.iter().any(|(name, _)| name.contains("dao")) {
                // Some DAO tests failed, adjust settings for better stability
                dao.voting_period_days = dao.voting_period_days.max(3); // Ensure at least 3 days
                updates.push(format!("Adjusted DAO voting period to {} days for stability", dao.voting_period_days));
            }
        }
        
        // 4. Save the updated configuration
        config::save_config(&config, "config/anya.conf")
            .map_err(|e| format!("Failed to save updated configuration: {}", e))?;
        
        // Log the updates
        if !updates.is_empty() {
            info!("Configuration updates based on test results:");
            for (i, update) in updates.iter().enumerate() {
                info!("  {}. {}", i + 1, update);
            }
        } else {
            info!("No configuration updates required based on test results.");
        }
        
        Ok(())
    }
} 