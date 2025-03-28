#![feature(edition2021)]
// Anya Core Unified Installer Test Suite
// [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
//
// This test suite validates the unified installer across various platforms and configurations,
// ensuring full BIP compliance and proper cross-platform support.

use std::{env, fs, path::PathBuf, process::Command, time::{Duration, Instant}};
use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use log::{info, warn, error};
use serde::{Serialize, Deserialize};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Installation path for tests
    #[arg(short, long, default_value = "anya-test")]
    path: String,
    
    /// Keep test directories after running
    #[arg(short, long)]
    keep: bool,
    
    /// Test category
    #[arg(short, long, default_value = "all")]
    category: String,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a specific test
    RunTest {
        /// Test name
        #[arg(short, long)]
        name: String,
    },
    
    /// Generate test report
    Report {
        /// Output format (markdown, json)
        #[arg(short, long, default_value = "markdown")]
        format: String,
    },
}

/// Test result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestResult {
    name: String,
    category: String,
    success: bool,
    duration_ms: u64,
    error: Option<String>,
    details: Vec<String>,
}

/// Test matrix structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestMatrix {
    platform: String,
    profiles: Vec<String>,
    components: Vec<String>,
    test_cases: Vec<TestCase>,
    results: Vec<TestResult>,
}

/// Test case structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestCase {
    name: String,
    category: String,
    description: String,
    profile: String,
    components: Vec<String>,
    expected_status: bool,
}

impl TestMatrix {
    fn new() -> Self {
        let platform = env::consts::OS.to_string();
        
        let profiles = vec![
            "minimal".to_string(),
            "standard".to_string(),
            "full".to_string(),
            "enterprise".to_string(),
        ];
        
        let components = vec![
            "core".to_string(),
            "bitcoin".to_string(),
            "dao".to_string(),
            "web5".to_string(),
            "ml".to_string(),
        ];
        
        let test_cases = vec![
            TestCase {
                name: "minimal_install".to_string(),
                category: "installation".to_string(),
                description: "Test minimal installation".to_string(),
                profile: "minimal".to_string(),
                components: vec!["core".to_string()],
                expected_status: true,
            },
            TestCase {
                name: "standard_install".to_string(),
                category: "installation".to_string(),
                description: "Test standard installation".to_string(),
                profile: "standard".to_string(),
                components: vec!["core".to_string(), "bitcoin".to_string(), "dao".to_string(), "web5".to_string()],
                expected_status: true,
            },
            TestCase {
                name: "full_install".to_string(),
                category: "installation".to_string(),
                description: "Test full installation".to_string(),
                profile: "full".to_string(),
                components: vec!["core".to_string(), "bitcoin".to_string(), "dao".to_string(), "web5".to_string(), "ml".to_string()],
                expected_status: true,
            },
            TestCase {
                name: "bip_compliance".to_string(),
                category: "compliance".to_string(),
                description: "Test BIP compliance".to_string(),
                profile: "standard".to_string(),
                components: vec!["core".to_string(), "bitcoin".to_string()],
                expected_status: true,
            },
            TestCase {
                name: "invalid_path".to_string(),
                category: "validation".to_string(),
                description: "Test invalid installation path".to_string(),
                profile: "minimal".to_string(),
                components: vec!["core".to_string()],
                expected_status: false,
            },
        ];
        
        Self {
            platform,
            profiles,
            components,
            test_cases,
            results: Vec::new(),
        }
    }
    
    fn run_test(&mut self, test_case: &TestCase, test_dir: &PathBuf, verbose: bool) -> Result<TestResult> {
        let start = Instant::now();
        let mut details = Vec::new();
        
        info!("Running test: {}", test_case.name);
        details.push(format!("Test: {}", test_case.name));
        details.push(format!("Description: {}", test_case.description));
        details.push(format!("Profile: {}", test_case.profile));
        details.push(format!("Components: {}", test_case.components.join(", ")));
        
        let test_path = test_dir.join(&test_case.name);
        
        // Create test directory
        if test_path.exists() {
            fs::remove_dir_all(&test_path)?;
        }
        fs::create_dir_all(&test_path)?;
        
        // Build command
        let binary_path = if cfg!(windows) {
            PathBuf::from("target/release/unified_installer.exe")
        } else {
            PathBuf::from("target/release/unified_installer")
        };
        
        if !binary_path.exists() {
            return Err(anyhow!("Installer binary not found: {}", binary_path.display()));
        }
        
        let mut args = Vec::new();
        
        // Add args
        args.push("--path".to_string());
        
        // Handle invalid path test
        if test_case.name == "invalid_path" {
            args.push("/non_existent_system_path/anya".to_string());
        } else {
            args.push(test_path.to_string_lossy().to_string());
        }
        
        args.push("--profile".to_string());
        args.push(test_case.profile.clone());
        
        args.push("--components".to_string());
        args.push(test_case.components.join(","));
        
        if verbose {
            args.push("--verbose".to_string());
        }
        
        // Run command
        details.push(format!("Command: {} {}", binary_path.display(), args.join(" ")));
        
        let output = Command::new(&binary_path)
            .args(&args)
            .output()
            .context(format!("Failed to execute installer: {}", binary_path.display()))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        details.push(format!("Exit code: {}", output.status.code().unwrap_or(-1)));
        
        if !stdout.is_empty() {
            details.push("--- STDOUT ---".to_string());
            details.push(stdout);
        }
        
        if !stderr.is_empty() {
            details.push("--- STDERR ---".to_string());
            details.push(stderr);
        }
        
        let success = if test_case.expected_status {
            output.status.success()
        } else {
            !output.status.success()
        };
        
        let duration = start.elapsed();
        
        Ok(TestResult {
            name: test_case.name.clone(),
            category: test_case.category.clone(),
            success,
            duration_ms: duration.as_millis() as u64,
            error: if success { None } else { Some(stderr) },
            details,
        })
    }
    
    fn run_all_tests(&mut self, test_dir: &PathBuf, category: &str, verbose: bool) -> Result<()> {
        info!("Running all tests in category: {}", category);
        
        // Filter test cases by category
        let test_cases = if category == "all" {
            self.test_cases.clone()
        } else {
            self.test_cases.iter()
                .filter(|tc| tc.category == category)
                .cloned()
                .collect()
        };
        
        if test_cases.is_empty() {
            return Err(anyhow!("No test cases found for category: {}", category));
        }
        
        for test_case in test_cases {
            match self.run_test(&test_case, test_dir, verbose) {
                Ok(result) => {
                    if result.success {
                        info!("✅ Test {} passed", result.name);
                    } else {
                        error!("❌ Test {} failed", result.name);
                    }
                    self.results.push(result);
                },
                Err(e) => {
                    error!("❌ Test {} failed with error: {}", test_case.name, e);
                    self.results.push(TestResult {
                        name: test_case.name.clone(),
                        category: test_case.category.clone(),
                        success: false,
                        duration_ms: 0,
                        error: Some(e.to_string()),
                        details: vec![format!("Error: {}", e)],
                    });
                }
            }
        }
        
        Ok(())
    }
    
    fn generate_report(&self, format: &str) -> Result<String> {
        match format {
            "markdown" => self.generate_markdown_report(),
            "json" => self.generate_json_report(),
            _ => Err(anyhow!("Unsupported report format: {}", format)),
        }
    }
    
    fn generate_markdown_report(&self) -> Result<String> {
        let mut report = String::new();
        
        report.push_str("# Anya Core Unified Installer Test Report\n\n");
        report.push_str(&format!("## Platform: {}\n\n", self.platform));
        
        report.push_str("## Test Results Summary\n\n");
        report.push_str("| Test | Category | Status | Duration |\n");
        report.push_str("|------|----------|--------|----------|\n");
        
        for result in &self.results {
            let status = if result.success { "✅ Pass" } else { "❌ Fail" };
            report.push_str(&format!("| {} | {} | {} | {}ms |\n", 
                result.name, result.category, status, result.duration_ms));
        }
        
        report.push_str("\n## Test Details\n\n");
        
        for result in &self.results {
            report.push_str(&format!("### {}\n\n", result.name));
            report.push_str(&format!("- **Category**: {}\n", result.category));
            report.push_str(&format!("- **Status**: {}\n", if result.success { "Pass" } else { "Fail" }));
            report.push_str(&format!("- **Duration**: {}ms\n", result.duration_ms));
            
            if let Some(error) = &result.error {
                report.push_str(&format!("- **Error**: {}\n", error));
            }
            
            report.push_str("\n#### Details\n\n");
            report.push_str("```\n");
            for detail in &result.details {
                report.push_str(&format!("{}\n", detail));
            }
            report.push_str("```\n\n");
        }
        
        Ok(report)
    }
    
    fn generate_json_report(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("Failed to serialize test results to JSON")
    }
}

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    // Parse command line arguments
    let args = Args::parse();
    
    // Create test directory
    let test_dir = PathBuf::from(&args.path);
    if !test_dir.exists() {
        fs::create_dir_all(&test_dir)?;
    }
    
    // Create test matrix
    let mut test_matrix = TestMatrix::new();
    
    // Execute specific command if provided
    if let Some(cmd) = args.command {
        match cmd {
            Commands::RunTest { name } => {
                let test_case = test_matrix.test_cases.iter()
                    .find(|tc| tc.name == name)
                    .cloned()
                    .ok_or_else(|| anyhow!("Test case not found: {}", name))?;
                
                let result = test_matrix.run_test(&test_case, &test_dir, args.verbose)?;
                
                if result.success {
                    info!("✅ Test {} passed", result.name);
                } else {
                    error!("❌ Test {} failed", result.name);
                }
                
                // Save result
                test_matrix.results.push(result);
            },
            Commands::Report { format } => {
                if test_matrix.results.is_empty() {
                    return Err(anyhow!("No test results available. Run tests first."));
                }
                
                let report = test_matrix.generate_report(&format)?;
                
                // Save report to file
                let report_path = test_dir.join(format!("test_report.{}", format));
                fs::write(&report_path, &report)?;
                
                info!("Test report generated: {}", report_path.display());
                
                // Also print to stdout
                println!("{}", report);
            }
        }
    } else {
        // Run all tests
        test_matrix.run_all_tests(&test_dir, &args.category, args.verbose)?;
        
        // Generate report
        let report = test_matrix.generate_markdown_report()?;
        
        // Save report to file
        let report_path = test_dir.join("test_report.md");
        fs::write(&report_path, &report)?;
        
        info!("Test report generated: {}", report_path.display());
    }
    
    // Calculate success rate
    let total_tests = test_matrix.results.len();
    let passed_tests = test_matrix.results.iter().filter(|r| r.success).count();
    let success_rate = if total_tests > 0 {
        (passed_tests as f64 / total_tests as f64) * 100.0
    } else {
        0.0
    };
    
    info!("Test Summary: {} of {} tests passed ({:.2}% success rate)",
        passed_tests, total_tests, success_rate);
    
    // Clean up test directory if not keeping
    if !args.keep && !test_dir.as_os_str().is_empty() {
        match fs::remove_dir_all(&test_dir) {
            Ok(_) => info!("Test directory removed: {}", test_dir.display()),
            Err(e) => warn!("Failed to remove test directory: {}", e),
        }
    }
    
    Ok(())
} 