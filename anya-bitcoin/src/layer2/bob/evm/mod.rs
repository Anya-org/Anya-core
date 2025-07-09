// EVM compatibility module for BOB
// Implements EVM compatibility for Bitcoin Optimistic Blockchain
// as per official Bitcoin Improvement Proposals (BIPs) requirements

use crate::layer2::bob::{BobConfig, BobError, EvmTransaction, EvmTransactionReceipt};
use ethers::prelude::{TransactionRequest, U256};
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use reqwest::StatusCode;
use std::convert::TryFrom;
use std::str::FromStr;
use std::time::Duration;

/// EVM adapter for BOB
pub struct EvmAdapter {
    config: BobConfig,
    client: HttpClient,
}

impl EvmAdapter {
    /// Create a new EVM adapter
    pub async fn new(config: &BobConfig) -> Result<Self, BobError> {
        let timeout = Duration::from_millis(config.timeout_ms);
        let client = HttpClientBuilder::default()
            .request_timeout(timeout)
            .build(&config.rpc_url)
            .map_err(|e| BobError::ConnectionError(format!("Failed to build EVM client: {}", e)))?;
        
        Ok(Self {
            config: config.clone(),
            client,
        })
    }

    /// Check connection to the EVM network
    pub async fn check_connection(&self) -> Result<bool, BobError> {
        // Try to get the network ID as a basic connectivity check
        match self.client.request::<String, &str>("net_version", &[]).await {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::warn!("EVM connection check failed: {}", e);
                Err(BobError::ConnectionError(format!("Failed to connect to EVM RPC: {}", e)))
            }
        }
    }

    /// Get the current block number
    pub async fn get_block_number(&self) -> Result<u64, BobError> {
        let block_number: String = self.client.request("eth_blockNumber", &[])
            .await
            .map_err(|e| BobError::RpcError(format!("Failed to get block number: {}", e)))?;
            
        // Convert hex string to u64
        let block_number = u64::from_str_radix(block_number.trim_start_matches("0x"), 16)
            .map_err(|e| BobError::ParseError(format!("Failed to parse block number: {}", e)))?;
            
        Ok(block_number)
    }

    /// Send a transaction to the EVM network
    pub async fn send_transaction(
        &self,
        transaction: EvmTransaction,
    ) -> Result<EvmTransactionReceipt, BobError> {
        // Convert our transaction to an ethers TransactionRequest
        let tx_request = self.convert_to_tx_request(&transaction)?;
        
        // Serialize the transaction for RPC
        let tx_serialized = serde_json::to_value(tx_request)
            .map_err(|e| BobError::SerializationError(format!("Failed to serialize transaction: {}", e)))?;
            
        // Send the transaction
        let tx_hash: String = self.client.request("eth_sendTransaction", &[tx_serialized])
            .await
            .map_err(|e| BobError::TransactionError(format!("Failed to send transaction: {}", e)))?;
            
        // Wait for the transaction to be mined with exponential backoff
        let receipt = self.wait_for_transaction(&tx_hash).await?;
        
        Ok(receipt)
    }
    
    /// Convert our transaction model to ethers TransactionRequest
    fn convert_to_tx_request(&self, tx: &EvmTransaction) -> Result<TransactionRequest, BobError> {
        let mut request = TransactionRequest::new();
        
        // Set from address
        let from = tx.from.parse()
            .map_err(|e| BobError::ParseError(format!("Invalid from address: {}", e)))?;
        request = request.from(from);
        
        // Set to address if present
        if let Some(to) = &tx.to {
            let to = to.parse()
                .map_err(|e| BobError::ParseError(format!("Invalid to address: {}", e)))?;
            request = request.to(to);
        }
        
        // Set value, gas limit and gas price
        request = request
            .value(U256::from(tx.value))
            .gas(U256::from(tx.gas_limit))
            .gas_price(U256::from(tx.gas_price));
            
        // Set data if present
        if !tx.data.is_empty() {
            request = request.data(tx.data.clone());
        }
        
        Ok(request)
    }
    
    /// Wait for a transaction to be mined with exponential backoff
    async fn wait_for_transaction(&self, tx_hash: &str) -> Result<EvmTransactionReceipt, BobError> {
        let backoff = backoff::ExponentialBackoff {
            initial_interval: Duration::from_millis(1000),
            max_interval: Duration::from_millis(20000),
            max_elapsed_time: Some(Duration::from_millis(self.config.timeout_ms)),
            ..Default::default()
        };
        
        let operation = || async {
            // Request transaction receipt
            let receipt: Option<serde_json::Value> = self.client
                .request("eth_getTransactionReceipt", &[serde_json::Value::String(tx_hash.to_string())])
                .await
                .map_err(|e| {
                    backoff::Error::transient(BobError::RpcError(format!(
                        "Failed to get transaction receipt: {}", e
                    )))
                })?;
                
            match receipt {
                Some(receipt) => {
                    // Extract status
                    let status = receipt
                        .get("status")
                        .and_then(|s| s.as_str())
                        .ok_or_else(|| {
                            backoff::Error::transient(BobError::ParseError(
                                "Receipt missing status field".to_string()
                            ))
                        })?;
                        
                    let status = status == "0x1";
                    
                    // Extract block number
                    let block_number = receipt
                        .get("blockNumber")
                        .and_then(|b| b.as_str())
                        .ok_or_else(|| {
                            backoff::Error::transient(BobError::ParseError(
                                "Receipt missing blockNumber field".to_string()
                            ))
                        })?;
                        
                    let block_number = u64::from_str_radix(block_number.trim_start_matches("0x"), 16)
                        .map_err(|e| {
                            backoff::Error::permanent(BobError::ParseError(format!(
                                "Failed to parse block number: {}", e
                            )))
                        })?;
                        
                    // Extract gas used
                    let gas_used = receipt
                        .get("gasUsed")
                        .and_then(|g| g.as_str())
                        .ok_or_else(|| {
                            backoff::Error::transient(BobError::ParseError(
                                "Receipt missing gasUsed field".to_string()
                            ))
                        })?;
                        
                    let gas_used = u64::from_str_radix(gas_used.trim_start_matches("0x"), 16)
                        .map_err(|e| {
                            backoff::Error::permanent(BobError::ParseError(format!(
                                "Failed to parse gas used: {}", e
                            )))
                        })?;
                        
                    // Construct receipt
                    let receipt = EvmTransactionReceipt {
                        tx_hash: tx_hash.to_string(),
                        block_number,
                        gas_used,
                        status,
                    };
                    
                    Ok(receipt)
                },
                None => {
                    // Transaction not yet mined
                    Err(backoff::Error::transient(BobError::TransactionError(
                        "Transaction not yet mined".to_string()
                    )))
                }
            }
        };
        
        backoff::future::retry(backoff, operation)
            .await
            .map_err(|e| match e {
                backoff::Error::Transient(e) => e,
                backoff::Error::Permanent(e) => e,
            })
    }
}
