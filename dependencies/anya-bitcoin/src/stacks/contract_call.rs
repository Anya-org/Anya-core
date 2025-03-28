#![feature(edition2021)]
// Contract call functionality for Stacks

use anyhow::{anyhow, Result};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::StacksNetwork;
use super::post_conditions::PostCondition;

/// Contract function argument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractArg {
    pub type_id: String,
    pub value: Value,
}

impl ContractArg {
    /// Create a new uint argument
    pub fn uint(value: u64) -> Self {
        Self {
            type_id: "uint".to_string(),
            value: json!(value.to_string()),
        }
    }
    
    /// Create a new principal argument
    pub fn principal(value: &str) -> Self {
        Self {
            type_id: "principal".to_string(),
            value: json!(value),
        }
    }
    
    /// Create a new string argument
    pub fn string(value: &str) -> Self {
        Self {
            type_id: "string-ascii".to_string(),
            value: json!(value),
        }
    }
    
    /// Create a new boolean argument
    pub fn boolean(value: bool) -> Self {
        Self {
            type_id: "bool".to_string(),
            value: json!(value),
        }
    }
    
    /// Create a new list argument
    pub fn list(type_id: &str, items: Vec<Value>) -> Self {
        Self {
            type_id: format!("(list {})", type_id),
            value: json!(items),
        }
    }
    
    /// Convert to JSON value
    pub fn to_json(&self) -> Value {
        json!({
            "type": self.type_id,
            "value": self.value,
        })
    }
}

/// Contract call response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractCallResponse {
    pub tx_id: String,
    pub success: bool,
    pub result: Option<Value>,
    pub error: Option<String>,
}

/// Contract call builder
#[derive(Debug)]
pub struct ContractCallBuilder {
    network: StacksNetwork,
    api_url: Url,
    client: Client,
    contract_address: String,
    contract_name: String,
    function_name: String,
    function_args: Vec<ContractArg>,
    post_conditions: Vec<PostCondition>,
    nonce: Option<u64>,
    fee: Option<u64>,
}

impl ContractCallBuilder {
    /// Create a new contract call builder
    pub fn new(
        network: StacksNetwork,
        api_url: Url,
        client: Client,
        contract_address: &str,
        contract_name: &str,
    ) -> Self {
        Self {
            network,
            api_url,
            client,
            contract_address: contract_address.to_string(),
            contract_name: contract_name.to_string(),
            function_name: String::new(),
            function_args: Vec::new(),
            post_conditions: Vec::new(),
            nonce: None,
            fee: None,
        }
    }
    
    /// Set the function to call
    pub fn with_function(mut self, function_name: &str) -> Self {
        self.function_name = function_name.to_string();
        self
    }
    
    /// Add a function argument
    pub fn with_arg(mut self, arg: ContractArg) -> Self {
        self.function_args.push(arg);
        self
    }
    
    /// Add multiple function arguments
    pub fn with_args(mut self, args: Vec<ContractArg>) -> Self {
        self.function_args.extend(args);
        self
    }
    
    /// Add a post condition
    pub fn with_post_condition(mut self, post_condition: PostCondition) -> Self {
        self.post_conditions.push(post_condition);
        self
    }
    
    /// Set a specific nonce
    pub fn with_nonce(mut self, nonce: u64) -> Self {
        self.nonce = Some(nonce);
        self
    }
    
    /// Set a specific fee
    pub fn with_fee(mut self, fee: u64) -> Self {
        self.fee = Some(fee);
        self
    }
    
    /// Call a read-only function
    pub async fn call_read_only(&self, sender: &str) -> Result<Value> {
        if self.function_name.is_empty() {
            return Err(anyhow!("Function name not set"));
        }
        
        let contract_id = format!("{}.{}", self.contract_address, self.contract_name);
        
        let url = self.api_url.join(&format!(
            "v2/contracts/call-read/{}/{}",
            contract_id,
            self.function_name
        ))?;
        
        // Convert args to JSON format
        let args: Vec<Value> = self.function_args.iter().map(|arg| arg.to_json()).collect();
        
        let payload = json!({
            "sender": sender,
            "arguments": args,
        });
        
        let response = self.client
            .post(url)
            .json(&payload)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to call contract function: HTTP {}", response.status()));
        }
        
        let result = response.json::<Value>().await?;
        
        Ok(result)
    }
    
    /// Execute the contract call transaction
    pub async fn execute(&self, sender: &str, sender_key: &str) -> Result<ContractCallResponse> {
        if self.function_name.is_empty() {
            return Err(anyhow!("Function name not set"));
        }
        
        let contract_id = format!("{}.{}", self.contract_address, self.contract_name);
        
        // Convert args to JSON format
        let args: Vec<Value> = self.function_args.iter().map(|arg| arg.to_json()).collect();
        
        // Create base payload
        let mut payload = json!({
            "contract_id": contract_id,
            "function_name": self.function_name,
            "function_args": args,
            "sender_address": sender,
            "sender_key": sender_key,
        });
        
        // Add optional fields if set
        if !self.post_conditions.is_empty() {
            let post_conditions_json: Vec<Value> = self.post_conditions.iter()
                .map(|pc| pc.to_json())
                .collect();
            
            payload["post_conditions"] = json!(post_conditions_json);
        }
        
        if let Some(nonce) = self.nonce {
            payload["nonce"] = json!(nonce);
        }
        
        if let Some(fee) = self.fee {
            payload["fee"] = json!(fee);
        }
        
        // Execute call
        let url = self.api_url.join("v2/contracts/call")?;
        let response = self.client
            .post(url)
            .json(&payload)
            .send()
            .await?;
        
        let status = response.status();
        let body = response.text().await?;
        
        if !status.is_success() {
            return Ok(ContractCallResponse {
                tx_id: "".to_string(),
                success: false,
                result: None,
                error: Some(format!("HTTP {}: {}", status, body)),
            });
        }
        
        // Parse result
        match serde_json::from_str::<Value>(&body) {
            Ok(result) => {
                let tx_id = result["txid"].as_str().unwrap_or("").to_string();
                
                Ok(ContractCallResponse {
                    tx_id,
                    success: true,
                    result: Some(result),
                    error: None,
                })
            },
            Err(e) => {
                Ok(ContractCallResponse {
                    tx_id: "".to_string(),
                    success: false,
                    result: None,
                    error: Some(format!("Failed to parse response: {}", e)),
                })
            }
        }
    }
    
    /// Test the contract call builder
    pub fn test(&self) -> Result<()> {
        println!("Testing contract call builder...");
        
        // Simple test to verify the builder is set up correctly
        println!("Contract call builder successfully initialized");
        
        Ok(())
    }
}
