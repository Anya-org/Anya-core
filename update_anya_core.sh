#!/bin/bash
set -e

echo "Updating anya-core for better Layer 4 integration..."

# Update anya-core lib.rs
cat > anya-core/src/lib.rs << 'RUST'
//! Anya Core - Bitcoin Layer 4 Protocol
//! [AIR-3][AIS-3][BPC-3][AIP-3][RES-3]

// Re-export from core
pub use core::l4_protocol;

/// Version of the library
pub const VERSION: &str = "0.1.0";

/// Run the layer 4 protocol with specified endpoint
pub async fn run_l4_protocol(endpoint: Option<&str>, network: Option<&str>) -> Result<(), core::error::Error> {
    // Determine the network
    let bitcoin_network = match network {
        Some("mainnet") => bitcoin::Network::Bitcoin,
        Some("testnet") => bitcoin::Network::Testnet,
        Some("signet") => bitcoin::Network::Signet,
        Some("regtest") => bitcoin::Network::Regtest,
        _ => bitcoin::Network::Testnet, // Default to testnet
    };
    
    // Create the L4 protocol instance
    let l4 = match endpoint {
        Some(ep) => core::l4_protocol::AnyaL4Protocol::with_endpoint(ep),
        None => {
            let mut l4 = core::l4_protocol::AnyaL4Protocol::with_network(bitcoin_network);
            l4.init_hsm("software").map_err(core::error::Error::Protocol)?;
            l4
        }
    };
    
    // Test connection
    l4.test_connection().await.map_err(core::error::Error::Protocol)?;
    
    println!("Layer 4 protocol running successfully on {}!", match bitcoin_network {
        bitcoin::Network::Bitcoin => "mainnet",
        bitcoin::Network::Testnet => "testnet",
        bitcoin::Network::Signet => "signet",
        bitcoin::Network::Regtest => "regtest",
    });
    
    Ok(())
}

/// Create a DLC contract
pub async fn create_dlc_contract(
    endpoint: Option<&str>, 
    oracle_pubkey: &str,
    outcomes: Vec<String>
) -> Result<String, core::error::Error> {
    // Create the L4 protocol instance
    let l4 = match endpoint {
        Some(ep) => core::l4_protocol::AnyaL4Protocol::with_endpoint(ep),
        None => core::l4_protocol::AnyaL4Protocol::new(),
    };
    
    // Parse the oracle public key
    let pubkey = bitcoin::secp256k1::PublicKey::from_str(oracle_pubkey)
        .map_err(|e| core::error::Error::Unknown(e.to_string()))?;
    
    // Create the contract
    let contract = l4.create_dlc_contract(pubkey, outcomes)
        .await
        .map_err(core::error::Error::Protocol)?;
    
    // Return the contract as JSON
    let json = serde_json::to_string_pretty(&contract.to_json())
        .map_err(|e| core::error::Error::Unknown(e.to_string()))?;
    
    Ok(json)
}
RUST

# Update anya-core main.rs to handle arguments
cat > anya-core/src/main.rs << 'RUST'
//! Anya Core CLI - Bitcoin Layer 4 Protocol
//! [AIR-3][AIS-3][BPC-3][AIP-3][RES-3]

use anya_core::{VERSION, run_l4_protocol, create_dlc_contract};
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
            },
            
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
            },
            
            // Endpoint specification
            "--endpoint" => {
                if args.len() < 3 {
                    println!("Usage: anya-core --endpoint <url>");
                    return Ok(());
                }
                
                let endpoint = &args[2];
                println!("Using custom endpoint: {}", endpoint);
                
                run_l4_protocol(Some(endpoint), None).await?;
            },
            
            // Help command
            "help" | "--help" | "-h" => {
                println!("Usage:");
                println!("  anya-core                        Run with default settings");
                println!("  anya-core --network <network>    Run on specified network");
                println!("  anya-core --endpoint <url>       Use custom RPC endpoint");
                println!("  anya-core create-dlc <oracle> <outcomes>  Create DLC contract");
                println!("  anya-core help                   Show this help message");
            },
            
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
RUST

echo "Anya Core updated with improved Layer 4 protocol integration"
