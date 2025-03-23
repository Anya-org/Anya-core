//! Bitcoin Development Framework CLI v2.5
//! Compliant with BIP 174/341/342

use anya_core::{bitcoin, secp256k1};
use clap::{Parser, Subcommand};
use console::style;
use anyhow::Context;

#[derive(Parser)]
#[command(name = "anya-cli")]
#[command(version = "2.5.0")]
#[command(about = "Bitcoin Development Framework CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate BIP-341 Taproot key pair
    Keygen {
        /// Network type (mainnet|testnet|signet|regtest)
        network: String
    },
    
    /// Sign PSBT transaction (BIP-174)
    Sign {
        /// Input PSBT file path
        psbt_file: String,
        
        /// Output PSBT file path
        output: String
    },
    
    /// Verify transaction against Bitcoin consensus rules
    Verify {
        /// Transaction hex or file path
        transaction: String
    },
    
    /// Display network status metrics
    Status {
        /// Prometheus metrics endpoint port
        #[arg(short, long, default_value_t = 9183)]
        port: u16
    },
    
    /// Generate HSM-backed Taproot key (BIP-341)
    HsmKeygen {
        /// HSM connection URL
        #[arg(long, default_value = "mock://localhost")]
        hsm_url: String,
        /// Key identifier
        #[arg(short, long)]
        key_id: String
    },
    
    /// Generate compliance badge
    ComplianceBadge {
        /// Output format
        #[arg(short, long, default_value = "svg")]
        format: String
    },
    
    /// Start chaos visualization server
    ChaosViz {
        /// Web server port
        #[arg(short, long, default_value_t = 8080)]
        port: u16
    },
    
    /// Verify compliance badge authenticity
    VerifyBadge {
        /// Badge file path
        #[arg(short, long)]
        file: String
    }
}

fn main() {
    println!("Anya CLI v2.5 - Bitcoin Development Framework");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Initialize BDF-compliant logging
    anya_core::init_logging()?;

    // Add hardware capability check
    if let Commands::HsmKeygen { .. } | Commands::ChaosViz { .. } = &cli.command {
        anya_core::validate_hardware_capabilities()?;
    }

    match cli.command {
        Commands::Keygen { network } => {
            let (sk, pubkey) = anya_core::generate_taproot_key(network)?;
            println!("{} Key pair generated:", style("✓").green());
            println!("Private Key: {}", sk);
            println!("Public Key:  {}", pubkey);
        }
        
        Commands::Sign { psbt_file, output } => {
            let signed_psbt = anya_core::sign_psbt(&psbt_file)?;
            std::fs::write(&output, &signed_psbt)?;
            println!("{} PSBT signed and saved to {}", style("✓").green(), output);
        }
        
        Commands::Verify { transaction } => {
            let verification = anya_core::verify_transaction(&transaction).await?;
            println!("{} Transaction verification:", style("✓").green());
            println!("{}", serde_json::to_string_pretty(&verification)?);
        }
        
        Commands::Status { port } => {
            println!("Starting metrics server on port {}...", port);
            anya_core::serve_metrics(port).await?;
        }
        
        Commands::HsmKeygen { hsm_url, key_id } => {
            // Add PSBT version validation
            if !anya_core::psbt::validate_version(2) {
                anyhow::bail!("PSBT v2 required for HSM operations");
            }
            
            // Enhanced HSM connection security
            let hsm_conn = anya_core::hsm::secure_connect(&hsm_url)
                .await
                .context("Failed to establish secure HSM connection")?;
                
            let (pubkey, key_handle) = anya_core::hsm::generate_hsm_key(hsm_conn, &key_id)
                .await
                .context("HSM key generation failed")?;
            
            // Audit trail integration
            anya_core::audit::log_keygen(&pubkey, "hsm")?;
            
            println!("{} HSM-backed key generated:", style("✓").green());
            println!("Public Key: {}", pubkey);
            println!("Key Handle: {}", key_handle);
        }
        
        Commands::ComplianceBadge { format } => {
            // Add real-time validation
            let report = anya_core::compliance::generate_report()?;
            if report.overall_score < 0.95 {
                anyhow::bail!("Compliance score too low for badge generation");
            }
            
            let badge_url = anya_core::compliance::generate_badge(&format)?;
            println!("Compliance badge URL: {}", badge_url);
            
            // Update BIP-341/342 status in docs
            anya_core::docs::update_compliance_status(&report)?;
        }
        
        Commands::ChaosViz { port } => {
            // Add network security validation
            if !anya_core::network::validate_chaos_permissions() {
                anyhow::bail!("Insufficient permissions for chaos visualization");
            }
            
            println!("Starting chaos visualization on port {}...", port);
            anya_core::chaos::visualization::run_server(port)
                .await
                .context("Chaos visualization server failed")?;
        }
        
        Commands::VerifyBadge { file } => {
            let valid = anya_core::compliance::verify_badge(&file)?;
            println!("Badge verification: {}", 
                style(if valid { "VALID" } else { "INVALID" })
                    .color(if valid { Color::Green } else { Color::Red }));
        }
    }

    Ok(())
} 