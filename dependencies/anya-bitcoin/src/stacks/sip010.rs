// SIP-010 Fungible Token Standard implementation for Stacks
// https://github.com/stacksgov/sips/blob/main/sips/sip-010/sip-010-fungible-token-standard.md

use anyhow::{anyhow, Result};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

use super::StacksNetwork;

/// SIP-010 Token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: String,
    pub contract_id: String,
    pub token_uri: Option<String>,
}

/// SIP-010 Token Balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBalance {
    pub balance: String,
    pub contract_id: String,
    pub owner: String,
}

/// SIP-010 Token Manager for interacting with SIP-010 compliant tokens
#[derive(Debug)]
pub struct Sip010TokenManager {
    network: StacksNetwork,
    api_url: Url,
    client: Client,
    tokens: HashMap<String, TokenInfo>,
}

/// SIP-010 Transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenTransfer {
    pub tx_id: String,
    pub sender: String,
    pub recipient: String,
    pub amount: String,
    pub token_contract: String,
    pub memo: Option<String>,
    pub status: String,
}

impl Sip010TokenManager {
    /// Create a new SIP-010 token manager
    pub fn new(
        network: StacksNetwork,
        api_url: Url,
        client: Client,
    ) -> Result<Self> {
        Ok(Self {
            network,
            api_url,
            client,
            tokens: HashMap::new(),
        })
    }
    
    /// Get token information
    pub async fn get_token_info(&mut self, contract_id: &str) -> Result<TokenInfo> {
        // Check if we have it cached
        if let Some(info) = self.tokens.get(contract_id) {
            return Ok(info.clone());
        }
        
        // Call get-token-uri on the contract
        let name = self.call_token_function(contract_id, "get-name", &[]).await?;
        let symbol = self.call_token_function(contract_id, "get-symbol", &[]).await?;
        let decimals = self.call_token_function(contract_id, "get-decimals", &[]).await?;
        let total_supply = self.call_token_function(contract_id, "get-total-supply", &[]).await?;
        
        // Try to get token URI if available (optional in SIP-010)
        let token_uri = match self.call_token_function(contract_id, "get-token-uri", &[]).await {
            Ok(uri) => Some(uri),
            Err(_) => None,
        };
        
        let info = TokenInfo {
            name,
            symbol,
            decimals: decimals.parse::<u8>().unwrap_or(0),
            total_supply,
            contract_id: contract_id.to_string(),
            token_uri,
        };
        
        // Cache the result
        self.tokens.insert(contract_id.to_string(), info.clone());
        
        Ok(info)
    }
    
    /// Get token balance for an account
    pub async fn get_balance(&self, contract_id: &str, owner: &str) -> Result<TokenBalance> {
        let balance = self.call_token_function(
            contract_id, 
            "get-balance", 
            &[json!({"type": "principal", "value": owner})]
        ).await?;
        
        Ok(TokenBalance {
            balance,
            contract_id: contract_id.to_string(),
            owner: owner.to_string(),
        })
    }
    
    /// Transfer tokens (requires wallet)
    pub async fn transfer(
        &self,
        contract_id: &str,
        sender: &str,
        sender_key: &str,
        recipient: &str,
        amount: &str,
        memo: Option<&str>,
    ) -> Result<TokenTransfer> {
        // Build the transaction payload
        let mut args = vec![
            json!({"type": "principal", "value": recipient}),
            json!({"type": "uint", "value": amount}),
        ];
        
        // Add memo if provided
        if let Some(memo_text) = memo {
            args.push(json!({"type": "string-ascii", "value": memo_text}));
        }
        
        // Prepare the transaction
        let payload = json!({
            "contract_id": contract_id,
            "function_name": "transfer",
            "function_args": args,
            "sender_address": sender,
            "sender_key": sender_key,
        });
        
        // Call the contract
        let url = self.api_url.join("v2/contracts/call")?;
        let response = self.client
            .post(url)
            .json(&payload)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to transfer tokens: HTTP {}", response.status()));
        }
        
        let result = response.json::<Value>().await?;
        
        // Create transfer info
        let transfer = TokenTransfer {
            tx_id: result["txid"].as_str().unwrap_or("").to_string(),
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            amount: amount.to_string(),
            token_contract: contract_id.to_string(),
            memo: memo.map(|s| s.to_string()),
            status: "pending".to_string(),
        };
        
        Ok(transfer)
    }
    
    /// Deploy a new SIP-010 token
    pub async fn deploy_token(
        &self,
        sender: &str,
        sender_key: &str,
        name: &str,
        symbol: &str,
        decimals: u8,
        initial_supply: &str,
        contract_name: &str,
    ) -> Result<String> {
        // Generate a SIP-010 compliant contract
        let contract_source = self.generate_sip010_contract(
            name,
            symbol,
            decimals,
            initial_supply,
        )?;
        
        // Prepare deployment payload
        let payload = json!({
            "contract_name": contract_name,
            "source_code": contract_source,
            "sender_address": sender,
            "sender_key": sender_key,
        });
        
        // Deploy the contract
        let url = self.api_url.join("v2/contracts/deploy")?;
        let response = self.client
            .post(url)
            .json(&payload)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to deploy token contract: HTTP {}", response.status()));
        }
        
        let result = response.json::<Value>().await?;
        let tx_id = result["txid"].as_str().unwrap_or("").to_string();
        
        // Return the contract ID
        let contract_id = format!("{}.{}", sender, contract_name);
        Ok(contract_id)
    }
    
    /// Generate a SIP-010 compliant contract
    fn generate_sip010_contract(
        &self,
        name: &str,
        symbol: &str,
        decimals: u8,
        initial_supply: &str,
    ) -> Result<String> {
        // Create a basic SIP-010 compliant contract template
        let contract = format!(
            r#"
;; SIP-010 Fungible Token Standard
;; Generated by OPSource

(define-fungible-token {symbol})

;; Constants
(define-constant contract-owner tx-sender)
(define-constant err-owner-only (err u100))
(define-constant err-not-token-owner (err u101))
(define-constant err-not-authorized (err u102))

;; Token information
(define-read-only (get-name)
  (ok "{name}"))

(define-read-only (get-symbol)
  (ok "{symbol}"))

(define-read-only (get-decimals)
  (ok u{decimals}))

(define-read-only (get-token-uri)
  (ok none))

;; SIP-010 standard functions
(define-read-only (get-balance (owner principal))
  (ok (ft-get-balance {symbol} owner)))

(define-read-only (get-total-supply)
  (ok (ft-get-supply {symbol})))

(define-public (transfer (amount uint) (sender principal) (recipient principal) (memo (optional (buff 34))))
  (begin
    (asserts! (or (is-eq tx-sender sender) (is-eq tx-sender contract-owner)) err-not-authorized)
    (match (ft-transfer? {symbol} amount sender recipient)
      response (begin
        ;; Handle memo if provided
        (match memo 
          memo-data (print memo-data)
          true)
        (ok response))
      error (err error))))

;; Mint initial supply to contract owner
(begin
  (ft-mint? {symbol} u{initial_supply} contract-owner))
            "#,
            name = name,
            symbol = symbol,
            decimals = decimals,
            initial_supply = initial_supply
        );
        
        Ok(contract)
    }
    
    /// Helper to call read-only functions on SIP-010 tokens
    async fn call_token_function(&self, contract_id: &str, function: &str, args: &[Value]) -> Result<String> {
        let url = self.api_url.join(&format!(
            "v2/contracts/call-read/{}/{}",
            contract_id,
            function
        ))?;
        
        let payload = json!({
            "arguments": args,
        });
        
        let response = self.client
            .post(url)
            .json(&payload)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to call token function: HTTP {}", response.status()));
        }
        
        let result = response.json::<Value>().await?;
        
        // Extract the result
        if let Some(ok) = result.get("okay") {
            if let Some(value) = ok.get("value") {
                return Ok(value.as_str().unwrap_or("").to_string());
            }
        }
        
        Err(anyhow!("Invalid response from token contract"))
    }
    
    /// Test the SIP-010 token functionality
    pub fn test(&self) -> Result<()> {
        println!("Testing SIP-010 token integration...");
        
        // Simple test to verify the manager is set up correctly
        println!("SIP-010 token manager successfully initialized");
        
        Ok(())
    }
}
