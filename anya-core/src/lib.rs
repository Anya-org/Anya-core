#![feature(edition2021)]
use bitcoin::Network;
use core::error::Error;
use core::l4_protocol::{AnyaL4Protocol, PublicRPCAdapter};
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Run the Layer 4 Bitcoin protocol with specified endpoint and network
pub async fn run_l4_protocol(endpoint: Option<&str>, network: Option<&str>) -> Result<AnyaL4Protocol> {
    println!("[AIR-3][AIS-3][BPC-3] Starting Anya Core Layer 4 Protocol");
    
    // Initialize the protocol with specified endpoint or default
    let mut protocol = if let Some(ep) = endpoint {
        AnyaL4Protocol::with_endpoint(ep)
    } else {
        AnyaL4Protocol::new()
    };
    
    // Set the network if specified
    if let Some(net) = network {
        if let Ok(btc_network) = Network::from_str(net) {
            protocol = AnyaL4Protocol::with_network(btc_network);
        } else {
            return Err(format!("Invalid network: {}", net).into());
        }
    }
    
    // Initialize HSM for secure operations
    protocol.init_hsm("tpm").map_err(|e| format!("HSM initialization error: {}", e))?;
    
    println!("Layer 4 Protocol initialized with endpoint: {}", protocol.get_endpoint());
    println!("HSM Initialized: {}", protocol.is_hsm_initialized());
    
    Ok(protocol)
}

/// Create a DLC contract with the given oracle public key and outcomes
pub async fn create_dlc_contract(
    oracle_pubkey: &str,
    outcomes: Vec<String>,
) -> Result<String> {
    let mut protocol = AnyaL4Protocol::new();
    
    // Use the string-based API instead of trying to convert to PublicKey
    let contract_id = protocol.create_dlc_contract(oracle_pubkey, outcomes)?;
    
    println!("DLC Contract created: {}", contract_id);
    Ok(contract_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_run_l4_protocol() {
        let result = run_l4_protocol(Some("https://testnet-rpc.example.com"), Some("testnet")).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_create_dlc_contract() {
        let pubkey = "03a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2";
        let outcomes = vec!["outcome1".to_string(), "outcome2".to_string()];
        
        let result = create_dlc_contract(pubkey, outcomes).await;
        assert!(result.is_ok());
    }
}
