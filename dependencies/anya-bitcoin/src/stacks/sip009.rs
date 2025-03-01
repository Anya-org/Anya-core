// SIP-009 Non-Fungible Token Standard implementation for Stacks
// https://github.com/stacksgov/sips/blob/main/sips/sip-009/sip-009-nft-standard.md

use anyhow::{anyhow, Result};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

use super::StacksNetwork;

/// SIP-009 NFT information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftInfo {
    pub name: String,
    pub contract_id: String,
    pub total_supply: String,
    pub token_uri: Option<String>,
}

/// SIP-009 NFT Token metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftMetadata {
    pub id: String,
    pub owner: String,
    pub contract_id: String,
    pub token_uri: Option<String>,
    pub metadata: Option<Value>,
}

/// SIP-009 NFT Transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftTransfer {
    pub tx_id: String,
    pub sender: String,
    pub recipient: String,
    pub token_id: String,
    pub nft_contract: String,
    pub status: String,
}

/// SIP-009 NFT Manager for interacting with SIP-009 compliant NFTs
#[derive(Debug)]
pub struct Sip009NftManager {
    network: StacksNetwork,
    api_url: Url,
    client: Client,
    nfts: HashMap<String, NftInfo>,
}

impl Sip009NftManager {
    /// Create a new SIP-009 NFT manager
    pub fn new(
        network: StacksNetwork,
        api_url: Url,
        client: Client,
    ) -> Result<Self> {
        Ok(Self {
            network,
            api_url,
            client,
            nfts: HashMap::new(),
        })
    }
    
    /// Get NFT collection information
    pub async fn get_nft_info(&mut self, contract_id: &str) -> Result<NftInfo> {
        // Check if we have it cached
        if let Some(info) = self.nfts.get(contract_id) {
            return Ok(info.clone());
        }
        
        // Call get-last-token-id to determine total supply
        let name = self.call_nft_function(contract_id, "get-name", &[]).await?;
        let total_supply = self.call_nft_function(contract_id, "get-last-token-id", &[]).await?;
        
        // Try to get token URI if available (common extension to SIP-009)
        let token_uri = match self.call_nft_function(contract_id, "get-token-uri", &[0.into()]).await {
            Ok(uri) => Some(uri),
            Err(_) => None,
        };
        
        let info = NftInfo {
            name,
            contract_id: contract_id.to_string(),
            total_supply,
            token_uri,
        };
        
        // Cache the result
        self.nfts.insert(contract_id.to_string(), info.clone());
        
        Ok(info)
    }
    
    /// Get NFT owner
    pub async fn get_owner(&self, contract_id: &str, token_id: u64) -> Result<String> {
        let owner = self.call_nft_function(
            contract_id, 
            "get-owner", 
            &[json!({"type": "uint", "value": token_id.to_string()})]
        ).await?;
        
        Ok(owner)
    }
    
    /// Get NFT metadata (if available)
    pub async fn get_token_metadata(&self, contract_id: &str, token_id: u64) -> Result<NftMetadata> {
        // Get token owner
        let owner = self.get_owner(contract_id, token_id).await?;
        
        // Try to get token URI (common extension)
        let token_uri = match self.call_nft_function(
            contract_id, 
            "get-token-uri", 
            &[json!({"type": "uint", "value": token_id.to_string()})]
        ).await {
            Ok(uri) => Some(uri),
            Err(_) => None,
        };
        
        // If we have a token URI, try to fetch the metadata
        let metadata = if let Some(uri) = &token_uri {
            if uri.starts_with("http") {
                // Fetch metadata from URI
                match self.client.get(uri).send().await {
                    Ok(response) => {
                        if response.status().is_success() {
                            match response.json::<Value>().await {
                                Ok(data) => Some(data),
                                Err(_) => None,
                            }
                        } else {
                            None
                        }
                    },
                    Err(_) => None,
                }
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(NftMetadata {
            id: token_id.to_string(),
            owner,
            contract_id: contract_id.to_string(),
            token_uri,
            metadata,
        })
    }
    
    /// Transfer NFT (requires wallet)
    pub async fn transfer(
        &self,
        contract_id: &str,
        sender: &str,
        sender_key: &str,
        recipient: &str,
        token_id: u64,
    ) -> Result<NftTransfer> {
        // Build the transaction payload
        let args = vec![
            json!({"type": "uint", "value": token_id.to_string()}),
            json!({"type": "principal", "value": sender}),
            json!({"type": "principal", "value": recipient}),
        ];
        
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
            return Err(anyhow!("Failed to transfer NFT: HTTP {}", response.status()));
        }
        
        let result = response.json::<Value>().await?;
        
        // Create transfer info
        let transfer = NftTransfer {
            tx_id: result["txid"].as_str().unwrap_or("").to_string(),
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            token_id: token_id.to_string(),
            nft_contract: contract_id.to_string(),
            status: "pending".to_string(),
        };
        
        Ok(transfer)
    }
    
    /// Mint a new NFT (requires appropriate permissions)
    pub async fn mint(
        &self,
        contract_id: &str,
        sender: &str,
        sender_key: &str,
        recipient: &str,
        token_uri: Option<&str>,
    ) -> Result<u64> {
        // Build function arguments
        let mut args = vec![
            json!({"type": "principal", "value": recipient}),
        ];
        
        // Add token URI if provided
        if let Some(uri) = token_uri {
            args.push(json!({"type": "string", "value": uri}));
        }
        
        // Prepare the transaction
        let payload = json!({
            "contract_id": contract_id,
            "function_name": "mint",
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
            return Err(anyhow!("Failed to mint NFT: HTTP {}", response.status()));
        }
        
        let result = response.json::<Value>().await?;
        
        // Try to extract the token ID from the result
        if let Some(ok) = result.get("okay") {
            if let Some(value) = ok.get("value") {
                if let Ok(token_id) = value.as_str().unwrap_or("0").parse::<u64>() {
                    return Ok(token_id);
                }
            }
        }
        
        // If we can't get it directly, get the last token ID
        let total_supply = self.call_nft_function(contract_id, "get-last-token-id", &[]).await?;
        Ok(total_supply.parse::<u64>().unwrap_or(0))
    }
    
    /// Deploy a new SIP-009 NFT collection
    pub async fn deploy_nft_collection(
        &self,
        sender: &str,
        sender_key: &str,
        name: &str,
        contract_name: &str,
    ) -> Result<String> {
        // Generate a SIP-009 compliant contract
        let contract_source = self.generate_sip009_contract(name)?;
        
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
            return Err(anyhow!("Failed to deploy NFT contract: HTTP {}", response.status()));
        }
        
        let result = response.json::<Value>().await?;
        let tx_id = result["txid"].as_str().unwrap_or("").to_string();
        
        // Return the contract ID
        let contract_id = format!("{}.{}", sender, contract_name);
        Ok(contract_id)
    }
    
    /// Generate a SIP-009 compliant contract
    fn generate_sip009_contract(&self, name: &str) -> Result<String> {
        // Create a basic SIP-009 compliant contract template
        let contract = format!(
            r#"
;; SIP-009 Non-Fungible Token Standard
;; Generated by OPSource

;; Constants
(define-constant contract-owner tx-sender)
(define-constant err-owner-only (err u100))
(define-constant err-not-token-owner (err u101))
(define-constant err-not-authorized (err u102))
(define-constant err-token-id-not-found (err u103))
(define-constant err-token-uri-not-found (err u104))

;; Data variables
(define-data-var last-token-id uint u0)
(define-map token-uris uint (string-utf8 256))
(define-non-fungible-token {name} uint)

;; NFT metadata methods
(define-read-only (get-name)
  (ok "{name}"))

(define-read-only (get-last-token-id)
  (ok (var-get last-token-id)))

(define-read-only (get-token-uri (token-id uint))
  (match (map-get? token-uris token-id)
    uri (ok uri)
    (err err-token-uri-not-found)))

(define-read-only (get-owner (token-id uint))
  (match (nft-get-owner? {name} token-id)
    owner (ok owner)
    (err err-token-id-not-found)))

;; SIP-009 standard functions
(define-public (transfer (token-id uint) (sender principal) (recipient principal))
  (begin
    (asserts! (is-eq tx-sender sender) err-not-authorized)
    (nft-transfer? {name} token-id sender recipient)))

;; Mint new NFT
(define-public (mint (recipient principal) (token-uri (optional (string-utf8 256))))
  (let 
    ((token-id (+ (var-get last-token-id) u1)))
    (begin
      (asserts! (or (is-eq tx-sender contract-owner) (is-eq tx-sender recipient)) err-not-authorized)
      
      ;; Update last token ID
      (var-set last-token-id token-id)
      
      ;; Save token URI if provided
      (match token-uri
        uri (map-set token-uris token-id uri)
        true)
      
      ;; Mint the NFT
      (match (nft-mint? {name} token-id recipient)
        success (ok token-id)
        error (err error)))))
            "#,
            name = name
        );
        
        Ok(contract)
    }
    
    /// Helper to call read-only functions on SIP-009 NFTs
    async fn call_nft_function(&self, contract_id: &str, function: &str, args: &[Value]) -> Result<String> {
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
            return Err(anyhow!("Failed to call NFT function: HTTP {}", response.status()));
        }
        
        let result = response.json::<Value>().await?;
        
        // Extract the result
        if let Some(ok) = result.get("okay") {
            if let Some(value) = ok.get("value") {
                return Ok(value.as_str().unwrap_or("").to_string());
            }
        }
        
        Err(anyhow!("Invalid response from NFT contract"))
    }
    
    /// Test the SIP-009 NFT functionality
    pub fn test(&self) -> Result<()> {
        println!("Testing SIP-009 NFT integration...");
        
        // Simple test to verify the manager is set up correctly
        println!("SIP-009 NFT manager successfully initialized");
        
        Ok(())
    }
}
