// [AIR-3][AIS-3][BPC-3][AIT-3] BIP Health CLI Tool
// Command-line tool for checking BIP implementation health

use chrono::Utc;
use clap::{Parser, Subcommand};
use std::error::Error;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

use anya_core::bip::validation::BitcoinConfig;
use anya_core::bip::{Bip353, Bip353Config, Bip353Status, BipHealthChecker};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn on verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Output file path (defaults to reports/bip-health-{timestamp}.md)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Check interval in seconds (default: 3600)
    #[arg(short, long, default_value = "3600")]
    interval: u64,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check and print BIP health to console
    Check {
        /// Output format (text, json, markdown)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Generate a health report and save to file
    Report {
        /// Output format (markdown, json)
        #[arg(short, long, default_value = "markdown")]
        format: String,
    },

    /// Start monitoring BIP health (generates reports periodically)
    Monitor {
        /// Check interval in seconds
        #[arg(short, long, default_value = "3600")]
        interval: u64,

        /// Output directory for reports
        #[arg(short, long, default_value = "reports")]
        output_dir: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Initialize logging
    let level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Create default output path if not specified
    let default_output = format!(
        "reports/bip-health-{}.md",
        Utc::now().format("%Y%m%d%H%M%S")
    );

    let output_path = cli.output.unwrap_or_else(|| PathBuf::from(default_output));

    // Initialize BIP health checker with all supported BIPs enabled
    let config = BitcoinConfig {
        taproot_enabled: true,
        tapscript_enabled: true,
        psbt_version: 2,
        bip353_enabled: true,
        bip353_status: Bip353Status::Beta,
    };

    // Create BIP353 instance for detailed health checking
    let bip353_config = Bip353Config {
        status: Bip353Status::Beta,
        default_resolver: "1.1.1.1".to_string(),
        cache_duration: 3600,
        validate_dnssec: true,
        beta_features: Default::default(),
    };

    let bip353 = match Bip353::new(bip353_config) {
        Ok(bip353) => Some(Arc::new(Mutex::new(bip353))),
        Err(e) => {
            warn!("Failed to initialize BIP353 instance: {}", e);
            None
        }
    };

    // Create health checker
    let mut health_checker = BipHealthChecker::new(config, bip353);
    health_checker.set_check_interval(cli.interval);

    // Process command
    match cli.command.unwrap_or(Commands::Check {
        format: "text".to_string(),
    }) {
        Commands::Check { format } => {
            let report = health_checker.check_health()?;

            match format.to_lowercase().as_str() {
                "json" => {
                    println!("{}", report.to_json()?);
                }
                "markdown" => {
                    println!("{}", report.to_markdown());
                }
                _ => {
                    // Simple text output
                    println!("BIP System Health Report");
                    println!("=======================");
                    println!("Generated: {}", report.timestamp);
                    println!();
                    println!(
                        "Overall Health: {}",
                        if report.healthy {
                            "HEALTHY"
                        } else {
                            "NEEDS ATTENTION"
                        }
                    );
                    println!("Total BIPs: {}", report.total_supported);
                    println!("Compliant: {}", report.compliant_count);
                    println!("Partial: {}", report.partial_count);
                    println!("Beta Features: {}", report.beta_count);
                    println!("Missing: {}", report.missing_count);
                    println!();
                    println!("BIP Details:");

                    // Sort BIPs by number
                    let mut bips: Vec<&String> = report.bips.keys().collect();
                    bips.sort();

                    for bip_key in bips {
                        if let Some(bip) = report.bips.get(bip_key) {
                            println!("- {}: {} ({})", bip.bip, bip.name, bip.status);
                        }
                    }
                }
            }
        }
        Commands::Report { format } => {
            // Make sure the output directory exists
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // Generate report
            health_checker.generate_report_file(output_path.to_str().unwrap())?;

            info!("BIP health report generated: {:?}", output_path);

            // If JSON format is requested, point to the JSON file
            if format.to_lowercase() == "json" {
                let json_path = output_path.with_extension("json");
                info!("JSON report available at: {:?}", json_path);
            }
        }
        Commands::Monitor {
            interval,
            output_dir,
        } => {
            // Create output directory if it doesn't exist
            std::fs::create_dir_all(&output_dir)?;

            // Set check interval
            health_checker.set_check_interval(interval);

            info!(
                "Starting BIP health monitoring (interval: {} seconds)",
                interval
            );
            info!("Reports will be saved to: {:?}", output_dir);

            // Run initial check
            let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
            let output_file = output_dir.join(format!("bip-health-{}.md", timestamp));

            health_checker.generate_report_file(output_file.to_str().unwrap())?;

            // In a real application, this would run in a loop or background task
            info!("Initial BIP health report generated");
            info!("In a production environment, this would continue running at the specified interval");
        }
    }

    Ok(())
}
