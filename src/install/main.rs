mod config;
mod components;
mod validation;
mod bitcoin_compliance;
mod telemetry;

use clap::{App, Arg};
use log::{info, warn, error};
use std::path::Path;

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
    // 1. Verify system requirements
    validation::check_system_requirements()?;
    
    // 2. Install core components based on detected requirements
    let components = if let Some(comp_list) = matches.value_of("components") {
        comp_list.split(',').collect()
    } else {
        // Default full installation
        vec!["core", "bitcoin", "dao", "web5", "ml"]
    };
    
    // 3. Install selected components
    for component in components {
        install_component(component, config_path)?;
    }
    
    // 4. Verify installation
    if matches.is_present("verify") {
        validation::verify_installation()?;
    }
    
    // 5. Generate deployment configuration
    generate_deployment_config(config_path)?;
    
    Ok(())
}

fn install_component(component: &str, config_path: &str) -> Result<(), String> {
    match component {
        "core" => components::install_core(config_path),
        "bitcoin" => components::install_bitcoin(config_path),
        "dao" => components::install_dao(config_path),
        "web5" => components::install_web5(config_path),
        "ml" => components::install_ml(config_path),
        _ => Err(format!("Unknown component: {}", component))
    }
}

fn generate_deployment_config(config_path: &str) -> Result<(), String> {
    // Generate configuration based on installed components
    // This follows your hexagonal architecture requirements
    config::generate_deployment_config(config_path)
} 