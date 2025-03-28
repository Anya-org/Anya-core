#![feature(edition2021)]
// Stacks Smart Contract Integration for OPSource
// Enables Clarity smart contracts anchored to Bitcoin

use anyhow::{anyhow, Result};
use bitcoin::Network;
use clarity_repl::clarity::functions::define::DefineFunction;
use clarity_repl::clarity::types::{PrincipalData, QualifiedContractIdentifier, Value as ClarityValue};
use clarity_repl::clarity::ClarityConnection;
use clarity_repl::repl::{Session, SessionSettings};
use reqwest::{Client, Url};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

mod sip009;
mod sip010;
mod contract_call;
mod post_conditions;

pub use sip009::Sip009NftManager;
pub use sip010::Sip010TokenManager;
pub use contract_call::ContractCallBuilder;
pub use post_conditions::{PostCondition, PostConditionMode};

/// Stacks network type
#[derive(Debug, Clone, PartialEq)]
pub enum StacksNetwork {
    Mainnet,
    Testnet,
    Mocknet,
    Devnet,
}

/// Stacks manager for interacting with Stacks blockchain
pub struct StacksManager {
    bitcoin_network: Network,
    stacks_network: StacksNetwork,
    api_url: Url,
    client: Client,
    data_dir: PathBuf,
    session: Option<Session>,
    // Add SIP token managers
    sip010_manager: Option<Sip010TokenManager>,
    sip009_manager: Option<Sip009NftManager>,
}

/// Contract deployment result
#[derive(Debug, Clone)]
pub struct ContractDeployment {
    pub tx_id: String,
    pub contract_id: String,
    pub sender: String,
    pub status: String,
}

/// Contract call result
#[derive(Debug, Clone)]
pub struct ContractCallResult {
    pub tx_id: Option<String>,
    pub result: Value,
    pub sender: String,
}

impl StacksManager {
    /// Create a new Stacks manager
    pub fn new(
        bitcoin_network: Network,
        data_dir: &PathBuf,
        api_url: Option<&str>,
    ) -> Result<Self> {
        // Determine Stacks network based on Bitcoin network
        let stacks_network = match bitcoin_network {
            Network::Bitcoin => StacksNetwork::Mainnet,
            Network::Testnet => StacksNetwork::Testnet,
            Network::Regtest => StacksNetwork::Devnet,
            _ => return Err(anyhow!("Unsupported network for Stacks: {:?}", bitcoin_network)),
        };
        
        // Set default API URL based on network
        let api_url_str = match api_url {
            Some(url) => url.to_string(),
            None => match stacks_network {
                StacksNetwork::Mainnet => "https://stacks-node-api.mainnet.stacks.co".to_string(),
                StacksNetwork::Testnet => "https://stacks-node-api.testnet.stacks.co".to_string(),
                StacksNetwork::Mocknet => "http://localhost:3999".to_string(),
                StacksNetwork::Devnet => "http://localhost:3999".to_string(),
            },
        };
        
        // Parse URL
        let api_url = Url::parse(&api_url_str)?;
        
        // Create HTTP client
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;
        
        // Create Stacks data directory
        let stacks_data_dir = data_dir.join("stacks").join(match stacks_network {
            StacksNetwork::Mainnet => "mainnet",
            StacksNetwork::Testnet => "testnet",
            StacksNetwork::Mocknet => "mocknet",
            StacksNetwork::Devnet => "devnet",
        });
        std::fs::create_dir_all(&stacks_data_dir)?;
        
        Ok(Self {
            bitcoin_network,
            stacks_network,
            api_url,
            client,
            data_dir: stacks_data_dir,
            session: None,
            sip010_manager: None,
            sip009_manager: None,
        })
    }
    
    /// Initialize SIP-010 token manager
    pub fn init_sip010_manager(&mut self) -> Result<&mut Sip010TokenManager> {
        if self.sip010_manager.is_none() {
            let manager = sip010::Sip010TokenManager::new(
                self.stacks_network.clone(),
                self.api_url.clone(),
                self.client.clone(),
            )?;
            self.sip010_manager = Some(manager);
        }
        
        Ok(self.sip010_manager.as_mut().unwrap())
    }
    
    /// Initialize SIP-009 NFT manager
    pub fn init_sip009_manager(&mut self) -> Result<&mut Sip009NftManager> {
        if self.sip009_manager.is_none() {
            let manager = sip009::Sip009NftManager::new(
                self.stacks_network.clone(),
                self.api_url.clone(),
                self.client.clone(),
            )?;
            self.sip009_manager = Some(manager);
        }
        
        Ok(self.sip009_manager.as_mut().unwrap())
    }
    
    /// Initialize local Clarity session for testing and simulation
    pub fn init_clarity_session(&mut self) -> Result<()> {
        let settings = SessionSettings::default();
        let session = Session::new(settings);
        
        self.session = Some(session);
        Ok(())
    }
    
    /// Get Stacks node info
    pub async fn get_node_info(&self) -> Result<Value> {
        let url = self.api_url.join("v2/info")?;
        
        let response = self.client
            .get(url)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to get node info: HTTP {}", response.status()));
        }
        
        let info = response.json::<Value>().await?;
        Ok(info)
    }
    
    /// Get account info
    pub async fn get_account_info(&self, address: &str) -> Result<Value> {
        let url = self.api_url.join(&format!("v2/accounts/{}", address))?;
        
        let response = self.client
            .get(url)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to get account info: HTTP {}", response.status()));
        }
        
        let info = response.json::<Value>().await?;
        Ok(info)
    }
    
    /// Deploy a Clarity contract (API call for broadcasting)
    pub async fn deploy_contract(
        &self,
        contract_name: &str,
        contract_source: &str,
        sender_address: &str,
        sender_private_key: &str,
    ) -> Result<ContractDeployment> {
        // For a complete implementation, this would:
        // 1. Generate and sign the transaction using the sender's private key
        // 2. Broadcast the transaction to the Stacks network
        // 3. Return the deployment info
        
        // This is a simplified implementation
        let payload = json!({
            "contract_name": contract_name,
            "source_code": contract_source,
            "sender_address": sender_address,
            // Normally we wouldn't send the private key to the API, this is just for demonstration
            "sender_key": sender_private_key
        });
        
        // For demonstration, we're showing what the API call might look like
        // In a real implementation, this would be replaced with proper transaction creation
        let url = self.api_url.join("v2/contracts/deploy")?;
        
        let response = self.client
            .post(url)
            .json(&payload)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to deploy contract: HTTP {}", response.status()));
        }
        
        let result = response.json::<Value>().await?;
        
        // Create deployment info
        let deployment = ContractDeployment {
            tx_id: result["txid"].as_str().unwrap_or("").to_string(),
            contract_id: format!("{}.{}", sender_address, contract_name),
            sender: sender_address.to_string(),
            status: "pending".to_string(),
        };
        
        Ok(deployment)
    }
    
    /// Read from a contract (non-mutating call)
    pub async fn call_read_only(
        &self,
        contract_id: &str,
        function_name: &str,
        function_args: &[Value],
        sender_address: Option<&str>,
    ) -> Result<Value> {
        let mut url = self.api_url.join(&format!(
            "v2/contracts/call-read/{}/{}",
            contract_id, function_name
        ))?;
        
        // Convert function args to Clarity format
        let args: Vec<Value> = function_args
            .iter()
            .map(|arg| json!({"type": arg["type"].as_str().unwrap_or(""), "value": arg["value"].clone()}))
            .collect();
        
        let mut payload = json!({
            "arguments": args,
        });
        
        if let Some(sender) = sender_address {
            payload["sender"] = json!(sender);
        }
        
        let response = self.client
            .post(url)
            .json(&payload)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to call contract: HTTP {}", response.status()));
        }
        
        let result = response.json::<Value>().await?;
        
        Ok(result)
    }
    
    /// Call a contract function (mutating)
    pub async fn call_contract(
        &self,
        contract_id: &str,
        function_name: &str,
        function_args: &[Value],
        sender_address: &str,
        sender_private_key: &str,
    ) -> Result<ContractCallResult> {
        // For a complete implementation, this would:
        // 1. Generate and sign the transaction using the sender's private key
        // 2. Broadcast the transaction to the Stacks network
        // 3. Return the call result
        
        // This is a simplified implementation
        let payload = json!({
            "contract_id": contract_id,
            "function_name": function_name,
            "function_args": function_args,
            "sender_address": sender_address,
            // Normally we wouldn't send the private key to the API, this is just for demonstration
            "sender_key": sender_private_key
        });
        
        // For demonstration, we're showing what the API call might look like
        // In a real implementation, this would be replaced with proper transaction creation
        let url = self.api_url.join("v2/contracts/call")?;
        
        let response = self.client
            .post(url)
            .json(&payload)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to call contract: HTTP {}", response.status()));
        }
        
        let result = response.json::<Value>().await?;
        
        // Create call result
        let call_result = ContractCallResult {
            tx_id: result["txid"].as_str().map(|s| s.to_string()),
            result: result,
            sender: sender_address.to_string(),
        };
        
        Ok(call_result)
    }
    
    /// Simulate a contract deployment using local Clarity session
    pub fn simulate_contract_deploy(
        &mut self,
        contract_name: &str,
        contract_source: &str,
        sender_address: &str,
    ) -> Result<()> {
        let session = self.session.as_mut()
            .ok_or_else(|| anyhow!("Clarity session not initialized. Call init_clarity_session() first."))?;
        
        // Parse the sender address into a PrincipalData
        let sender = PrincipalData::from_str(sender_address)
            .map_err(|_| anyhow!("Invalid sender address"))?;
        
        // Create a QualifiedContractIdentifier from the sender and contract name
        let contract_id = QualifiedContractIdentifier::new(
            sender.clone().into(),
            contract_name.to_string(),
        );
        
        // Deploy the contract in the session
        session.deploy_contract(&contract_id, contract_source)
            .map_err(|e| anyhow!("Failed to deploy contract: {}", e))?;
        
        Ok(())
    }
    
    /// Simulate a contract call using local Clarity session
    pub fn simulate_contract_call(
        &mut self,
        contract_name: &str,
        function_name: &str,
        function_args: &[ClarityValue],
        sender_address: &str,
    ) -> Result<ClarityValue> {
        let session = self.session.as_mut()
            .ok_or_else(|| anyhow!("Clarity session not initialized. Call init_clarity_session() first."))?;
        
        // Parse the sender address into a PrincipalData
        let sender = PrincipalData::from_str(sender_address)
            .map_err(|_| anyhow!("Invalid sender address"))?;
        
        // Create a QualifiedContractIdentifier from the sender and contract name
        let contract_id = QualifiedContractIdentifier::new(
            sender.clone().into(),
            contract_name.to_string(),
        );
        
        // Call the function in the session
        let result = session.call_contract_func(
            &contract_id,
            function_name,
            function_args,
        ).map_err(|e| anyhow!("Failed to call contract function: {}", e))?;
        
        Ok(result)
    }
    
    /// Create a new contract call builder for this Stacks manager
    pub fn create_contract_call_builder(&self, contract_address: &str, contract_name: &str) -> contract_call::ContractCallBuilder {
        contract_call::ContractCallBuilder::new(
            self.stacks_network.clone(),
            self.api_url.clone(),
            self.client.clone(),
            contract_address,
            contract_name,
        )
    }
    
    /// Test the Stacks integration
    pub fn test(&mut self) -> Result<()> {
        println!("Testing Stacks integration...");
        
        // Initialize Clarity session for local testing
        if self.session.is_none() {
            self.init_clarity_session()?;
        }
        
        // Try getting node info
        match self.get_node_info() {
            Ok(info) => println!("Connected to Stacks node: {:?}", info),
            Err(e) => println!("Could not connect to Stacks node: {}", e),
        }
        
        // Test a simple contract deployment (local)
        let contract_name = "counter";
        let contract_source = r#"
            ;; Simple counter contract
            (define-data-var counter uint u0)
            
            (define-read-only (get-counter)
              (var-get counter))
            
            (define-public (increment)
              (begin
                (var-set counter (+ (var-get counter) u1))
                (ok (var-get counter))))
        "#;
        
        // Test deploying the contract locally
        self.simulate_contract_deploy(
            contract_name,
            contract_source,
            "ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM",
        )?;
        
        println!("Successfully deployed counter contract in simulation");
        
        // Test calling the contract locally
        let result = self.simulate_contract_call(
            contract_name,
            "get-counter",
            &[],
            "ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM",
        )?;
        
        println!("Counter value: {:?}", result);
        
        // Test SIP-010 token functionality
        let sip010 = self.init_sip010_manager()?;
        sip010.test()?;
        
        // Test SIP-009 NFT functionality
        let sip009 = self.init_sip009_manager()?;
        sip009.test()?;
        
        // Test contract call builder and post conditions
        let builder = self.create_contract_call_builder(
            "ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM", 
            contract_name
        );
        println!("Created contract call builder for contract: {}.{}", 
            "ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM", 
            contract_name
        );
        
        // Test post conditions
        let post_condition = post_conditions::PostCondition::stx(
            post_conditions::PrincipalType::Origin,
            post_conditions::ConditionCode::SentLe,
            1000,
        );
        println!("Created post condition for transaction: {:?}", post_condition);
        
        println!("Stacks integration tests completed successfully!");
        Ok(())
    }
}
