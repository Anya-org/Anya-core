use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use toml;
use log::{info, warn, error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BitcoinConfig {
    pub network: String,
    pub rpc_user: String,
    pub rpc_password: String,
    pub rpc_port: u16,
    pub taproot_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DaoConfig {
    pub governance_model: String,
    pub voting_period_days: u8,
    pub proposal_threshold: u32,
    pub execution_delay_hours: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Web5Config {
    pub did_method: String,
    pub dwn_endpoint: String,
    pub storage_location: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MlConfig {
    pub model_path: String,
    pub inference_threads: u8,
    pub telemetry_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemConfig {
    pub log_level: String,
    pub metrics_endpoint: String,
    pub health_check_interval_seconds: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub system: SystemConfig,
    pub bitcoin: Option<BitcoinConfig>,
    pub dao: Option<DaoConfig>,
    pub web5: Option<Web5Config>,
    pub ml: Option<MlConfig>,
}

pub fn load_config(config_path: &str) -> Result<Config, String> {
    if !Path::new(config_path).exists() {
        return Err(format!("Config file not found: {}", config_path));
    }

    let config_content = match fs::read_to_string(config_path) {
        Ok(content) => content,
        Err(e) => return Err(format!("Failed to read config file: {}", e)),
    };

    match toml::from_str(&config_content) {
        Ok(config) => {
            info!("Configuration loaded from {}", config_path);
            Ok(config)
        },
        Err(e) => Err(format!("Failed to parse config file: {}", e)),
    }
}

pub fn generate_deployment_config(config_path: &str) -> Result<(), String> {
    let config = load_config(config_path)?;
    
    // Generate docker-compose.yml
    generate_docker_compose(&config)?;
    
    // Generate .env file
    generate_env_file(&config)?;
    
    info!("Deployment configuration generated successfully");
    Ok(())
}

fn generate_docker_compose(config: &Config) -> Result<(), String> {
    let mut services = String::new();
    
    // Add core service
    services.push_str("  anya-core:\n");
    services.push_str("    image: anya/core:3.1.0\n");
    services.push_str("    restart: unless-stopped\n");
    services.push_str("    env_file: .env\n");
    services.push_str("    volumes:\n");
    services.push_str("      - ./data:/data\n");
    services.push_str("      - ./config:/config\n");
    
    // Add Bitcoin service if enabled
    if config.bitcoin.is_some() {
        services.push_str("  bitcoin:\n");
        services.push_str("    image: bitcoin/core:24.0\n");
        services.push_str("    restart: unless-stopped\n");
        services.push_str("    env_file: .env\n");
        services.push_str("    volumes:\n");
        services.push_str("      - ./bitcoin:/root/.bitcoin\n");
    }
    
    // Add other services based on config...
    
    let docker_compose = format!(
        "version: '3.8'\n\nservices:\n{}\n\nnetworks:\n  default:\n    name: anya-network\n",
        services
    );
    
    match fs::write("docker-compose.yml", docker_compose) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to write docker-compose.yml: {}", e)),
    }
}

fn generate_env_file(config: &Config) -> Result<(), String> {
    let mut env_content = String::new();
    
    // System config
    env_content.push_str(&format!("LOG_LEVEL={}\n", config.system.log_level));
    env_content.push_str(&format!("METRICS_ENDPOINT={}\n", config.system.metrics_endpoint));
    
    // Bitcoin config if enabled
    if let Some(bitcoin) = &config.bitcoin {
        env_content.push_str(&format!("BITCOIN_NETWORK={}\n", bitcoin.network));
        env_content.push_str(&format!("BITCOIN_RPC_USER={}\n", bitcoin.rpc_user));
        env_content.push_str(&format!("BITCOIN_RPC_PASSWORD={}\n", bitcoin.rpc_password));
        env_content.push_str(&format!("BITCOIN_RPC_PORT={}\n", bitcoin.rpc_port));
        env_content.push_str(&format!("BITCOIN_TAPROOT_ENABLED={}\n", bitcoin.taproot_enabled));
    }
    
    // Add other component configs...
    
    match fs::write(".env", env_content) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to write .env file: {}", e)),
    }
} 