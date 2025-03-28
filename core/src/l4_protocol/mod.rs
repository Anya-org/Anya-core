#![feature(edition2021)]
mod rpc_adapter;
pub use rpc_adapter::PublicRPCAdapter;

use bitcoin::Network;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct ContractMetadata {
    pub oracle: String,
    pub outcomes: Vec<String>,
    pub timestamp: i64,
    pub status: String,
}

/// Layer 4 Protocol Core Implementation
pub struct AnyaL4Protocol {
    pub network: Network,
    pub rpc_adapter: PublicRPCAdapter,
    hsm_initialized: bool,
    contracts: HashMap<String, ContractMetadata>,
}

impl AnyaL4Protocol {
    /// Initialize with public RPC endpoints
    pub fn new() -> Self {
        Self {
            network: Network::Testnet,
            rpc_adapter: PublicRPCAdapter::new(),
            hsm_initialized: false,
            contracts: HashMap::new(),
        }
    }
    
    /// Initialize with specific network
    pub fn with_network(network: Network) -> Self {
        Self {
            network,
            rpc_adapter: PublicRPCAdapter::new(),
            hsm_initialized: false,
            contracts: HashMap::new(),
        }
    }
    
    /// Initialize with custom endpoint
    pub fn with_endpoint(endpoint: &str) -> Self {
        Self {
            network: Network::Testnet,
            rpc_adapter: PublicRPCAdapter::with_endpoint(endpoint),
            hsm_initialized: false,
            contracts: HashMap::new(),
        }
    }

    /// Initialize HSM for secure operations
    pub fn init_hsm(&mut self, hsm_type: &str) -> Result<(), crate::error::Error> {
        println!("Initializing HSM of type: {}", hsm_type);
        self.hsm_initialized = true;
        Ok(())
    }
    
    /// Generate a unique ID for contracts
    fn generate_unique_id(&self) -> String {
        let random_bytes: [u8; 8] = rand::random();
        hex::encode(random_bytes)
    }
    
    /// Create a DLC contract using string-based pubkey (for testing)
    pub fn create_dlc_contract(&mut self, oracle_pubkey: &str, outcome_values: Vec<String>) -> Result<String, Box<dyn Error>> {
        // Generate a unique contract ID
        let contract_id = format!("dlc-{}", self.generate_unique_id());
        
        // Store contract metadata
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
            
        self.contracts.insert(contract_id.clone(), ContractMetadata {
            oracle: oracle_pubkey.to_string(),
            outcomes: outcome_values,
            timestamp,
            status: "active".to_string(),
        });
        
        Ok(contract_id)
    }
    
    // Getter for the endpoint for testing
    pub fn get_endpoint(&self) -> String {
        if self.rpc_adapter.endpoints.is_empty() {
            String::new()
        } else {
            self.rpc_adapter.endpoints[0].clone()
        }
    }
    
    // Getter for HSM initialization status
    pub fn is_hsm_initialized(&self) -> bool {
        self.hsm_initialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dlc_contract() {
        let mut protocol = AnyaL4Protocol::new();
        let oracle_pubkey = "03a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2";
        let outcomes = vec!["outcome1".to_string(), "outcome2".to_string()];
        
        let result = protocol.create_dlc_contract(oracle_pubkey, outcomes);
        assert!(result.is_ok());
    }
}
