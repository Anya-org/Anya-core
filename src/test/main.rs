mod bitcoin_tests;
mod dao_tests;
mod web5_tests;
mod ml_tests;
mod system_tests;
mod compliance;
mod unified_test;
mod performance;

use clap::{App, Arg, SubCommand};
use log::{info, error};
use unified_test::{UnifiedTestRunner, UnifiedTestConfig};
use performance::PerformanceTestRunner;

fn main() {
    // Initialize testing environment
    let matches = App::new("Anya-Core Tester")
        .version("3.1.0")
        .author("Anya-Core Team")
        .about("BPC-3 compliant unified testing framework")
        .arg(Arg::with_name("rpc-endpoint")
            .long("rpc-endpoint")
            .value_name("URL")
            .help("Bitcoin RPC endpoint URL")
            .takes_value(true))
        .arg(Arg::with_name("report-dir")
            .long("report-dir")
            .value_name("DIR")
            .help("Directory for test reports")
            .default_value("reports")
            .takes_value(true))
        .arg(Arg::with_name("no-reports")
            .long("no-reports")
            .help("Disable report generation"))
        .arg(Arg::with_name("verbose")
            .short('v')
            .long("verbose")
            .help("Enable verbose output"))
        .subcommand(SubCommand::with_name("component")
            .about("Test specific components")
            .arg(Arg::with_name("component")
                .required(true)
                .help("Component to test: bitcoin, dao, web5, ml")))
        .subcommand(SubCommand::with_name("system")
            .about("Run full system test"))
        .subcommand(SubCommand::with_name("compliance")
            .about("Verify compliance with standards")
            .arg(Arg::with_name("standard")
                .help("Standard to verify: BPC-3, DAO-4, AIS-3, etc.")))
        .subcommand(SubCommand::with_name("performance")
            .about("Run performance tests")
            .arg(Arg::with_name("test-type")
                .long("test-type")
                .value_name("TYPE")
                .help("Type of performance test: throughput, database, cache, all")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("unified")
            .about("Run unified test suite")
            .arg(Arg::with_name("components")
                .long("components")
                .value_name("COMPONENTS")
                .help("Comma-separated list of components to test")
                .takes_value(true)))
        .get_matches();

    // Configure the test runner
    let mut config = UnifiedTestConfig {
        bitcoin_rpc_url: matches.value_of("rpc-endpoint").unwrap_or("").to_string(),
        generate_reports: !matches.is_present("no-reports"),
        report_dir: matches.value_of("report-dir").unwrap_or("reports").to_string(),
        verbose: matches.is_present("verbose"),
        ..Default::default()
    };

    match matches.subcommand() {
        ("component", Some(matches)) => {
            let component = matches.value_of("component").unwrap();
            config.components = vec![component.to_string()];
            run_unified_tests(config);
        }
        ("system", _) => {
            config.components = vec!["system".to_string()];
            run_unified_tests(config);
        }
        ("compliance", Some(matches)) => {
            if let Some(standard) = matches.value_of("standard") {
                config.components = match standard {
        run_unified_tests(config);
    } else if let Some(matches) = matches.subcommand_matches("unified") {
        if let Some(components) = matches.value_of("components") {
            config.components = components.split(',').map(String::from).collect();
        }
        run_unified_tests(config);
    } else {
        // Default behavior: run all tests using unified test runner
        run_unified_tests(config);
    }
}

fn run_unified_tests(config: UnifiedTestConfig) {
    info!("Running unified tests with components: {}", config.components.join(", "));
    
    match UnifiedTestRunner::new(config) {
        Ok(mut runner) => {
            match runner.run_all_tests() {
                Ok(results) => {
                    // Prompt for configuration update if tests passed
                    if results.failed.is_empty() {
                        println!("Would you like to update your Anya-Core configuration based on test results? [y/N]");
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).ok();
                        if input.trim().to_lowercase() == "y" {
                            match runner.update_config() {
                                Ok(_) => println!("✅ Configuration updated successfully."),
                                Err(e) => {
                                    error!("Failed to update configuration: {}", e);
                                }
                            }
                        }
                    }
                    
                    if !results.failed.is_empty() {
                        std::process::exit(1);
                    }
                },
                Err(e) => {
                    error!("Failed to run tests: {}", e);
                    std::process::exit(1);
                }
            }
        },
        Err(e) => {
            error!("Failed to initialize test runner: {}", e);
            std::process::exit(1);
        }
    }
} 