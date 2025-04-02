#![forbid(unsafe_code)]
//! Anya Core Binary Entry Point
//! 
//! Main executable for Anya Core with BIP-342 support.

use std::path::PathBuf;
use std::sync::Arc;
use std::net::SocketAddr;
use anyhow::{Result, Context};
use clap::{Parser, Subcommand};
use tokio::sync::RwLock;
use log::{info, warn, error, debug};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

// Import internal crates
use anya_core_core::{AnyaCore, AnyaCoreConfig};
use anya_core_core::l4_protocol::AnyaL4Protocol;
use anya_core_core::security::enforcement;
use anya_core_protocol_adapters::{
    ProtocolAdapter,
    create_bitcoin_adapter,
    create_legacy_bitcoin_adapter,
};
use anya_core_mcp_interface::{McpServer, McpServerConfig};
use anya_core_bitcoin_network::{BitcoinNetwork, BitcoinNetworkConfig};
use anya_core_metrics::{MetricsService, MetricsConfig};

/// Anya Core - Bitcoin Development Framework with BIP-342 support
#[derive(Parser)]
#[clap(name = "anya-core", about, version, author)]
struct Cli {
    /// Sets the configuration file
    #[clap(short, long, value_name = "FILE", default_value = "config.toml")]
    config: PathBuf,
    
    /// Sets the data directory
    #[clap(short, long, value_name = "DIR", default_value = ".anya")]
    datadir: PathBuf,
    
    /// Enables verbose output
    #[clap(short, long)]
    verbose: bool,
    
    /// Disable BIP-342 support (legacy mode)
    #[clap(long)]
    no_bip342: bool,
    
    /// Subcommands
    #[clap(subcommand)]
    command: Commands,
}

/// Anya Core subcommands
#[derive(Subcommand)]
enum Commands {
    /// Start the Anya Core server
    Start {
        /// MCP server address
        #[clap(long, default_value = "0.0.0.0:8080")]
        mcp_address: SocketAddr,
        
        /// Metrics server address
        #[clap(long, default_value = "127.0.0.1:9000")]
        metrics_address: SocketAddr,
    },
    
    /// Check BIP-342 compatibility
    CheckBip342 {
        /// Path to script file to check
        script_file: PathBuf,
    },
    
    /// Initialize Hardware Security Module
    InitHsm {
        /// HSM type
        #[clap(long, default_value = "softkey")]
        hsm_type: String,
        
        /// HSM endpoint
        #[clap(long)]
        hsm_endpoint: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Set up logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    
    // Initialize tracing
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new(format!("anya_core={},warn", log_level)))
        .init();
    
    info!("Starting Anya Core v{}", env!("CARGO_PKG_VERSION"));
    
    // Check for BIP-342 support
    if cli.no_bip342 {
        warn!("BIP-342 support disabled (legacy mode)");
    } else {
        info!("BIP-342 support enabled");
    }
    
    // Run security enforcement checks
    enforcement::run_environment_checks().context("Security checks failed")?;
    
    // Match command
    match cli.command {
        Commands::Start { mcp_address, metrics_address } => {
            // Create metrics service
            let metrics_config = MetricsConfig {
                enabled: true,
                address: metrics_address,
                collection_interval: 15,
                prefix: "anya_core".to_string(),
            };
            
            let mut metrics_service = MetricsService::new(metrics_config)
                .context("Failed to create metrics service")?;
            
            // Start metrics service
            metrics_service.start().context("Failed to start metrics service")?;
            
            // Create Bitcoin protocol adapter
            let bitcoin_adapter = if cli.no_bip342 {
                create_legacy_bitcoin_adapter()
            } else {
                create_bitcoin_adapter()
            }.context("Failed to create Bitcoin protocol adapter")?;
            
            // Create Layer 4 Protocol
            let l4_protocol = AnyaL4Protocol::new(
                "https://localhost:8080".to_string(),
                bitcoin::Network::Testnet,
            );
            
            // Create network configuration
            let network_config = BitcoinNetworkConfig {
                network: bitcoin::Network::Testnet,
                datadir: cli.datadir.join("bitcoin"),
                ..Default::default()
            };
            
            // Create Bitcoin network service
            let bitcoin_network = BitcoinNetwork::new(network_config)
                .await
                .context("Failed to create Bitcoin network service")?;
            
            // Create MCP server configuration
            let mcp_config = McpServerConfig {
                addr: mcp_address,
                workers: 4,
                ..Default::default()
            };
            
            // Create MCP server
            let mcp_server = McpServer::new(mcp_config)
                .context("Failed to create MCP server")?;
            
            // Start MCP server
            mcp_server.start()
                .await
                .context("Failed to start MCP server")?;
            
            // Create Anya Core configuration
            let core_config = AnyaCoreConfig {
                datadir: cli.datadir.clone(),
                network: bitcoin::Network::Testnet,
                bip342_enabled: !cli.no_bip342,
            };
            
            // Create Anya Core
            let anya_core = AnyaCore::new(core_config)
                .context("Failed to create Anya Core")?;
            
            // Start Anya Core
            anya_core.start()
                .await
                .context("Failed to start Anya Core")?;
            
            info!("Anya Core started successfully");
            info!("MCP server listening on {}", mcp_address);
            info!("Metrics server listening on {}", metrics_address);
            
            // Wait for Ctrl+C signal
            tokio::signal::ctrl_c().await?;
            
            info!("Shutting down Anya Core...");
            
            // Stop in reverse order
            anya_core.stop().await.context("Failed to stop Anya Core")?;
            mcp_server.stop().await.context("Failed to stop MCP server")?;
            metrics_service.stop().await.context("Failed to stop metrics service")?;
            
            info!("Anya Core stopped successfully");
        },
        
        Commands::CheckBip342 { script_file } => {
            info!("Checking BIP-342 compatibility for script: {:?}", script_file);
            
            // Read script file
            let script_content = std::fs::read_to_string(&script_file)
                .context("Failed to read script file")?;
            
            // Create Bitcoin adapter
            let bitcoin_adapter = create_bitcoin_adapter()
                .context("Failed to create Bitcoin protocol adapter")?;
            
            // Verify BIP-342 compatibility
            if bitcoin_adapter.supports_feature("BIP-342") {
                info!("The script is compatible with BIP-342");
            } else {
                warn!("The script is not compatible with BIP-342");
            }
        },
        
        Commands::InitHsm { hsm_type, hsm_endpoint } => {
            info!("Initializing HSM: {}", hsm_type);
            
            let endpoint = hsm_endpoint.unwrap_or_else(|| {
                let default_endpoint = "localhost:9876".to_string();
                info!("No HSM endpoint specified, using default: {}", default_endpoint);
                default_endpoint
            });
            
            // Create Layer 4 Protocol
            let mut l4_protocol = AnyaL4Protocol::new(
                endpoint,
                bitcoin::Network::Testnet,
            );
            
            // Initialize HSM
            l4_protocol.initialize_hsm(&hsm_type)
                .await
                .context("Failed to initialize HSM")?;
            
            info!("HSM initialized successfully");
        },
    }
    
    Ok(())
}
