//! Anya Core CLI - Bitcoin Layer 4 Protocol
//! [AIR-3][AIS-3][BPC-3][AIP-3][RES-3]

use anya_core::{create_dlc_contract, run_l4_protocol, VERSION};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    println!("Anya Core v{} - Bitcoin Layer 4 Protocol", VERSION);

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            // Create DLC contract command
            "create-dlc" => {
                if args.len() < 4 {
                    println!("Usage: anya-core create-dlc <oracle_pubkey> <outcome1,outcome2,...>");
                    return Ok(());
                }

                let oracle_pubkey = &args[2];
                let outcomes: Vec<String> = args[3].split(',').map(String::from).collect();

                println!("Creating DLC contract with oracle: {}", oracle_pubkey);
                println!("Outcomes: {:?}", outcomes);

                let contract_json = create_dlc_contract(None, oracle_pubkey, outcomes).await?;
                println!("Contract created successfully:");
                println!("{}", contract_json);
            }

            // Network specification
            "--network" => {
                if args.len() < 3 {
                    println!("Usage: anya-core --network <mainnet|testnet|signet|regtest>");
                    return Ok(());
                }

                let network = &args[2];
                println!("Running on {} network", network);

                // Get endpoint if specified
                let endpoint = if args.len() > 4 && args[3] == "--endpoint" {
                    Some(args[4].as_str())
                } else {
                    None
                };

                run_l4_protocol(endpoint, Some(network)).await?;
            }

            // Endpoint specification
            "--endpoint" => {
                if args.len() < 3 {
                    println!("Usage: anya-core --endpoint <url>");
                    return Ok(());
                }

                let endpoint = &args[2];
                println!("Using custom endpoint: {}", endpoint);

                run_l4_protocol(Some(endpoint), None).await?;
            }

            // Help command
            "help" | "--help" | "-h" => {
                println!("Usage:");
                println!("  anya-core                        Run with default settings");
                println!("  anya-core --network <network>    Run on specified network");
                println!("  anya-core --endpoint <url>       Use custom RPC endpoint");
                println!("  anya-core create-dlc <oracle> <outcomes>  Create DLC contract");
                println!("  anya-core help                   Show this help message");
            }

            // Unknown command
            _ => {
                println!("Unknown command: {}", args[1]);
                println!("Run 'anya-core help' for usage information.");
            }
        }
    } else {
        // Default behavior: run with default settings
        println!("Running with default configuration (testnet)");
        run_l4_protocol(None, None).await?;
    }

    Ok(())
}
