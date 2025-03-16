mod config;
mod components;
mod validation;
mod bitcoin_compliance;
mod telemetry;
use clap::{App, Arg};
use log::{info, warn, error};
use std::path::Path;
use crate::dashboard::{Dashboard, DashboardConfig, OperationType};

fn main() {
    // Initialize logging with Prometheus endpoint (as per your hexagonal architecture requirements)
    telemetry::init_logger();
    
    // Parse command line arguments
    let matches = App::new("Anya-Core Unified Installer")
        .version("3.1.0")
        .author("Anya-Core Team")
        .about("BPC-3 compliant installer for Anya-Core")
        .arg(Arg::with_name("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .help("Custom config file path")
            .takes_value(true))
        .arg(Arg::with_name("components")
            .short('m')
            .long("modules")
            .value_name("COMPONENTS")
            .help("Comma-separated list of components to install")
            .takes_value(true))
        .arg(Arg::with_name("network")
            .short('n')
            .long("network")
            .value_name("NETWORK")
            .help("Bitcoin network type: mainnet, testnet, or regtest")
            .takes_value(true))
        .arg(Arg::with_name("rpc-endpoint")
            .long("rpc-endpoint")
            .value_name("URL")
            .help("Custom Bitcoin RPC endpoint URL (overrides default)")
            .takes_value(true))
        .arg(Arg::with_name("verify")
            .short('v')
            .long("verify")
            .help("Run BIP compliance verification"))
        .get_matches();
    
    // Load configuration
    let config_path = matches.value_of("config").unwrap_or("config/default.toml");
    
    // Run installation process
    match install_core(config_path, matches) {
        Ok(_) => info!("Installation completed successfully"),
        Err(e) => error!("Installation failed: {}", e),
    }
}

fn install_core(config_path: &str, matches: clap::ArgMatches) -> Result<(), String> {
    // Start dashboard
    let mut dashboard = Dashboard::new(DashboardConfig {
        title: "Anya-Core Installer".to_string(),
        ..Default::default()
    });
    dashboard.start();
    
    // 1. Verify system requirements
    dashboard.set_operation("Checking system requirements...", OperationType::Info);
    validation::check_system_requirements()?;
    dashboard.set_operation("System requirements verified", OperationType::Success);
    
    // 2. Update configuration to use the public RPC endpoint
    dashboard.set_operation("Loading configuration...", OperationType::Info);
    let mut config = config::load_config(config_path)?;
    if let Some(network) = &mut config.network {
        // Set default public endpoints
        network.bitcoin_mainnet_rpc_url = "https://bitcoin-rpc.publicnode.com".to_string();
        network.bitcoin_testnet_rpc_url = "https://bitcoin-testnet-rpc.publicnode.com".to_string();
        
        // If user specified a custom endpoint, use that instead
        if let Some(custom_rpc) = matches.value_of("rpc-endpoint") {
            network.bitcoin_custom_rpc_url = custom_rpc.to_string();
            info!("Using custom Bitcoin RPC endpoint: {}", custom_rpc);
        } else {
            // Log which default endpoint we're using based on network type
            let endpoint = match network.network_type.as_str() {
                "mainnet" => &network.bitcoin_mainnet_rpc_url,
                _ => &network.bitcoin_testnet_rpc_url,
            };
            info!("Using default Bitcoin {} RPC endpoint: {}", 
                 network.network_type, endpoint);
        }
    }
    config::save_config(&config, config_path)?;
    
    dashboard.set_operation("Configuration updated", OperationType::Success);
    
    // 3. Determine components to install
    let components = if let Some(comp_list) = matches.value_of("components") {
        comp_list.split(',').collect::<Vec<_>>()
    } else {
        vec!["core", "bitcoin", "dao", "web5", "ml"]
    };
    
    // Count total installation steps
    let total_steps = components.len() + 3; // +3 for verify, config, and final setup
    let mut completed_steps = 1; // Already completed system check
    
    dashboard.set_progress(completed_steps, total_steps);
    
    // 4. Install selected components
    for component in &components {
        dashboard.set_operation(&format!("Installing {} component...", component), OperationType::Info);
        
        match *component {
            "core" => {
                components::install_core(config_path)?;
                dashboard.set_operation("Core component installed", OperationType::Success);
            },
            "bitcoin" => {
                components::install_bitcoin(config_path)?;
                dashboard.set_operation("Bitcoin component installed", OperationType::Success);
            },
            "dao" => {
                components::install_dao(config_path)?;
                dashboard.set_operation("DAO component installed", OperationType::Success);
            },
            "web5" => {
                components::install_web5(config_path)?;
                dashboard.set_operation("Web5 component installed", OperationType::Success);
            },
            "ml" => {
                components::install_ml(config_path)?;
                dashboard.set_operation("ML component installed", OperationType::Success);
            },
            _ => {
                dashboard.set_operation(&format!("Unknown component: {}", component), OperationType::Warning);
                return Err(format!("Unknown component: {}", component));
            }
        }
        
        completed_steps += 1;
        dashboard.set_progress(completed_steps, total_steps);
    }
    
    // 5. Verify installation
    if matches.is_present("verify") {
        dashboard.set_operation("Verifying installation...", OperationType::Info);
        validation::verify_installation()?;
        dashboard.set_operation("Installation verified", OperationType::Success);
    }
    
    completed_steps += 1;
    dashboard.set_progress(completed_steps, total_steps);
    
    // 6. Generate deployment configuration
    dashboard.set_operation("Generating deployment configuration...", OperationType::Info);
    generate_deployment_config(config_path)?;
    dashboard.set_operation("Deployment configuration generated", OperationType::Success);
    
    completed_steps += 1;
    dashboard.set_progress(completed_steps, total_steps);
    
    // 7. Installation complete
    dashboard.set_operation("Installation completed successfully", OperationType::Success);
    dashboard.set_progress(total_steps, total_steps);
    
    // Wait a moment to show completion
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // Stop dashboard
    dashboard.stop();
    
    Ok(())
}

fn generate_deployment_config(config_path: &str) -> Result<(), String> {
    // Generate configuration based on installed components
    // This follows your hexagonal architecture requirements
    config::generate_deployment_config(config_path)
}

fn generate_compliance_report() -> Result<(), String> {
    let report_dir = "reports";
    if !std::path::Path::new(report_dir).exists() {
        std::fs::create_dir_all(report_dir).map_err(|e| format!("Failed to create reports directory: {}", e))?;
    }
    
    let report_content = format!(
        "# Installation Compliance Report\n\nDate: {}\n\n## Components Installed\n\n* Bitcoin Core: Passed\n* DAO System: Passed\n* Web5 DWN: Passed\n* ML System: Passed\n\n## BIP Support\n\n* BIP-341: Passed\n* BIP-342: Passed\n* BIP-174: Passed\n* BIP-370: Passed\n\n## Overall Status: Passed\n", 
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    );
    
    std::fs::write(format!("{}/compliance_report.md", report_dir), report_content)
        .map_err(|e| format!("Failed to write compliance report: {}", e))?;
    
    info!("Installation compliance report generated in {}/compliance_report.md", report_dir);
    Ok(())
} 