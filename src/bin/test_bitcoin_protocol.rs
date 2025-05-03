// Bitcoin Protocol Test Runner Binary
// [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
//
// Command-line tool for running Bitcoin protocol tests according to
// Bitcoin Development Framework v2.5 requirements

use anyhow::{Result, Context};
use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run all Bitcoin protocol tests
    RunAll {
        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
        
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Run specific Bitcoin protocol tests
    RunSpecific {
        /// Test name pattern to match
        #[arg(required = true)]
        test_pattern: String,
        
        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
        
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

/// Main entry point
fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::RunAll { format, verbose } => {
            run_all_tests(*verbose, format)
        },
        Commands::RunSpecific { test_pattern, format, verbose } => {
            run_specific_tests(test_pattern, *verbose, format)
        },
    }
}

/// Run all Bitcoin protocol tests
fn run_all_tests(verbose: bool, output_format: &str) -> Result<()> {
    println!("{}", "Running All Bitcoin Protocol Tests".yellow().bold());
    
    if verbose {
        println!("Output Format: {}", output_format);
        println!("Verbose Mode: Enabled");
        println!("{}", "=".repeat(50));
    }
    
    // Import test modules
    let results = anya_core::tests::bitcoin::protocol::run_all_tests()?;
    
    // Check overall success
    let success = results.values().all(|&success| success);
    
    if output_format == "json" {
        // Output JSON format
        let json = serde_json::to_string_pretty(&results)?;
        println!("{}", json);
    }
    
    // Return success status
    if !success {
        std::process::exit(1);
    }
    
    Ok(())
}

/// Run specific Bitcoin protocol tests based on pattern
fn run_specific_tests(pattern: &str, verbose: bool, output_format: &str) -> Result<()> {
    println!("{}", format!("Running Bitcoin Protocol Tests matching: {}", pattern).yellow().bold());
    
    if verbose {
        println!("Test Pattern: {}", pattern);
        println!("Output Format: {}", output_format);
        println!("Verbose Mode: Enabled");
        println!("{}", "=".repeat(50));
    }
    
    // Import test modules
    let all_results = anya_core::tests::bitcoin::protocol::run_all_tests()?;
    
    // Filter tests based on pattern
    let filtered_results: std::collections::HashMap<String, bool> = all_results
        .into_iter()
        .filter(|(name, _)| name.contains(pattern))
        .collect();
    
    if filtered_results.is_empty() {
        println!("No tests found matching pattern: {}", pattern);
        return Ok(());
    }
    
    // Output filtered results
    if output_format == "json" {
        // Output JSON format
        let json = serde_json::to_string_pretty(&filtered_results)?;
        println!("{}", json);
    } else {
        // Text format already output by the test runner
        println!("\nFiltered Test Results:");
        for (name, success) in &filtered_results {
            if *success {
                println!("  {} {}", "✓".green(), name);
            } else {
                println!("  {} {}", "✗".red(), name);
            }
        }
    }
    
    // Check overall success
    let success = filtered_results.values().all(|&success| success);
    if !success {
        std::process::exit(1);
    }
    
    Ok(())
} 