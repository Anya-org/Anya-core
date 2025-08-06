// Bitcoin RPC Adapter
//
// Implements real RPC adapters for Bitcoin node communication
// [AIR-3][AIS-3][BPC-3]

use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose, Engine as _};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;

/// Real Bitcoin RPC client with HTTP communication
pub struct BitcoinRpcAdapter {
    /// RPC URL (e.g., http://localhost:8332)
    url: String,
    /// RPC username
    username: String,
    /// RPC password
    password: String,
    /// HTTP client for real network communication
    client: reqwest::Client,
    /// Request timeout
    timeout: Duration,
    /// Current request ID counter
    request_id: std::sync::atomic::AtomicU64,
}

#[derive(Serialize)]
struct RpcRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    params: Vec<Value>,
}

#[derive(Deserialize, Debug)]
struct RpcResponse {
    id: u64,
    result: Option<Value>,
    error: Option<RpcError>,
}

#[derive(Deserialize, Debug)]
struct RpcError {
    code: i32,
    message: String,
}

impl BitcoinRpcAdapter {
    /// Create a new Bitcoin RPC adapter with real HTTP client
    pub fn new(url: &str, username: &str, password: &str) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            url: url.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            client,
            timeout: Duration::from_secs(30),
            request_id: std::sync::atomic::AtomicU64::new(1),
        }
    }

    /// Set custom timeout for RPC requests
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Make authenticated RPC request to Bitcoin node
    async fn make_request(&self, method: &str, params: Vec<Value>) -> Result<Value> {
        let id = self
            .request_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let request = RpcRequest {
            jsonrpc: "1.0".to_string(),
            id,
            method: method.to_string(),
            params,
        };

        // Create basic auth header
        let auth_string = format!("{}:{}", self.username, self.password);
        let auth_encoded = general_purpose::STANDARD.encode(auth_string.as_bytes());
        let auth_header = format!("Basic {}", auth_encoded);

        debug!(
            "Making RPC request: {} with {} params",
            method,
            request.params.len()
        );

        let response = self
            .client
            .post(&self.url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .json(&request)
            .timeout(self.timeout)
            .send()
            .await
            .context("Failed to send RPC request")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!(
                "RPC request failed with status {}: {}",
                status,
                error_text
            ));
        }

        let rpc_response: RpcResponse = response
            .json()
            .await
            .context("Failed to parse RPC response")?;

        if let Some(error) = rpc_response.error {
            return Err(anyhow!("RPC error {}: {}", error.code, error.message));
        }

        rpc_response
            .result
            .ok_or_else(|| anyhow!("RPC response missing result"))
    }

    /// Get blockchain information
    pub async fn get_blockchain_info(&self) -> Result<Value> {
        self.make_request("getblockchaininfo", vec![]).await
    }

    /// Get network information
    pub async fn get_network_info(&self) -> Result<Value> {
        self.make_request("getnetworkinfo", vec![]).await
    }

    /// Get current block count
    pub async fn get_block_count(&self) -> Result<u64> {
        let result = self.make_request("getblockcount", vec![]).await?;
        result
            .as_u64()
            .ok_or_else(|| anyhow!("Invalid block count response"))
    }

    /// Get block hash by height
    pub async fn get_block_hash(&self, height: u64) -> Result<String> {
        let result = self
            .make_request("getblockhash", vec![json!(height)])
            .await?;
        result
            .as_str()
            .ok_or_else(|| anyhow!("Invalid block hash response"))
            .map(|s| s.to_string())
    }

    /// Get block by hash
    pub async fn get_block(&self, hash: &str, verbose: bool) -> Result<Value> {
        let verbosity = if verbose { 2 } else { 1 };
        self.make_request("getblock", vec![json!(hash), json!(verbosity)])
            .await
    }

    /// Get transaction by txid
    pub async fn get_transaction(&self, txid: &str, verbose: bool) -> Result<Value> {
        self.make_request("getrawtransaction", vec![json!(txid), json!(verbose)])
            .await
    }

    /// Send raw transaction
    pub async fn send_raw_transaction(&self, hex: &str) -> Result<String> {
        let result = self
            .make_request("sendrawtransaction", vec![json!(hex)])
            .await?;
        result
            .as_str()
            .ok_or_else(|| anyhow!("Invalid transaction ID response"))
            .map(|s| s.to_string())
    }

    /// Get mempool information
    pub async fn get_mempool_info(&self) -> Result<Value> {
        self.make_request("getmempoolinfo", vec![]).await
    }

    /// Get raw mempool
    pub async fn get_raw_mempool(&self, verbose: bool) -> Result<Value> {
        self.make_request("getrawmempool", vec![json!(verbose)])
            .await
    }

    /// Estimate smart fee
    pub async fn estimate_smart_fee(
        &self,
        conf_target: u32,
        estimate_mode: Option<&str>,
    ) -> Result<Value> {
        let mut params = vec![json!(conf_target)];
        if let Some(mode) = estimate_mode {
            params.push(json!(mode));
        }
        self.make_request("estimatesmartfee", vec![json!(conf_target)])
            .await
    }

    /// Get wallet info (if wallet is loaded)
    pub async fn get_wallet_info(&self) -> Result<Value> {
        self.make_request("getwalletinfo", vec![]).await
    }

    /// Get new address (if wallet is available)
    pub async fn get_new_address(
        &self,
        label: Option<&str>,
        address_type: Option<&str>,
    ) -> Result<String> {
        let mut params = vec![];
        if let Some(label) = label {
            params.push(json!(label));
            if let Some(addr_type) = address_type {
                params.push(json!(addr_type));
            }
        }

        let result = self.make_request("getnewaddress", params).await?;
        result
            .as_str()
            .ok_or_else(|| anyhow!("Invalid address response"))
            .map(|s| s.to_string())
    }

    /// Get balance (if wallet is available)
    pub async fn get_balance(&self) -> Result<f64> {
        let result = self.make_request("getbalance", vec![]).await?;
        result
            .as_f64()
            .ok_or_else(|| anyhow!("Invalid balance response"))
    }

    /// List unspent outputs (if wallet is available)
    pub async fn list_unspent(
        &self,
        min_conf: Option<u32>,
        max_conf: Option<u32>,
    ) -> Result<Value> {
        let mut params = vec![];
        if let Some(min) = min_conf {
            params.push(json!(min));
            if let Some(max) = max_conf {
                params.push(json!(max));
            }
        }
        self.make_request("listunspent", params).await
    }

    /// Create raw transaction
    pub async fn create_raw_transaction(
        &self,
        inputs: &[Value],
        outputs: &Value,
    ) -> Result<String> {
        let result = self
            .make_request("createrawtransaction", vec![json!(inputs), json!(outputs)])
            .await?;
        result
            .as_str()
            .ok_or_else(|| anyhow!("Invalid raw transaction response"))
            .map(|s| s.to_string())
    }

    /// Sign raw transaction (if wallet is available)
    pub async fn sign_raw_transaction_with_wallet(&self, hex: &str) -> Result<Value> {
        self.make_request("signrawtransactionwithwallet", vec![json!(hex)])
            .await
    }

    /// Test connection to Bitcoin node
    pub async fn test_connection(&self) -> Result<bool> {
        match self.get_network_info().await {
            Ok(info) => {
                info!("Successfully connected to Bitcoin node");
                debug!("Network info: {}", serde_json::to_string_pretty(&info)?);
                Ok(true)
            }
            Err(e) => {
                warn!("Failed to connect to Bitcoin node: {}", e);
                Ok(false)
            }
        }
    }

    /// Get node uptime
    pub async fn uptime(&self) -> Result<u64> {
        let result = self.make_request("uptime", vec![]).await?;
        result
            .as_u64()
            .ok_or_else(|| anyhow!("Invalid uptime response"))
    }

    /// Get peer info
    pub async fn get_peer_info(&self) -> Result<Value> {
        self.make_request("getpeerinfo", vec![]).await
    }

    /// Get connection count
    pub async fn get_connection_count(&self) -> Result<u32> {
        let result = self.make_request("getconnectioncount", vec![]).await?;
        result
            .as_u64()
            .ok_or_else(|| anyhow!("Invalid connection count response"))
            .map(|n| n as u32)
    }
}

// Backward compatibility trait implementation
use crate::bitcoin::adapters::BitcoinRpcPort;

impl BitcoinRpcPort for BitcoinRpcAdapter {
    /// Execute a Bitcoin RPC command (sync wrapper for async method)
    fn execute_command(&self, command: &str, params: &[Value]) -> Result<Value> {
        // Note: This is a blocking wrapper around async code
        // In production, consider using async throughout or tokio::Runtime
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { self.make_request(command, params.to_vec()).await })
        })
    }

    /// Get transaction via RPC (sync wrapper)
    fn get_transaction_rpc(&self, txid: &str) -> Result<Value> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { self.get_transaction(txid, true).await })
        })
    }

    /// Send raw transaction via RPC (sync wrapper)
    fn send_raw_transaction(&self, hex: &str) -> Result<String> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { self.send_raw_transaction(hex).await })
        })
    }
}
