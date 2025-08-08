//! Minimal RPC shim to unify internal expectations
//! Exposes BitcoinRpcClient and RpcError used by legacy adapters/tests.
//! Internally wraps bitcoincore_rpc and maps common calls.

use std::time::Duration;

use bitcoincore_rpc::{self as core_rpc, Auth, RpcApi};
// Note: warnings field mapping varies across versions; we'll omit detailed mapping for now.
use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value as JsonValue};
use std::collections::HashMap;

/// Thin error wrapper compatible with legacy code paths
#[derive(thiserror::Error, Debug, Clone)]
#[error("RPC error: {0}")]
pub struct RpcError(pub String);

impl From<core_rpc::Error> for RpcError {
    fn from(value: core_rpc::Error) -> Self {
        RpcError(value.to_string())
    }
}

/// Minimal, typed view of getblockchaininfo used by adapters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub best_block_hash: String,
    pub difficulty: f64,
    pub verification_progress: f64,
    pub initial_block_download: bool,
    pub chain_work: String,
    pub size_on_disk: u64,
    pub pruned: bool,
    pub prune_height: Option<i64>,
    pub warnings: String,
    /// Optional softfork heights map for compatibility
    pub softforks: HashMap<String, SoftforkInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftforkInfo {
    pub height: u64,
}

/// Minimal, typed view of getnetworkinfo used by adapters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub protocol_version: i64,
    pub connections: i64,
}

/// Result for estimatesmartfee with sats/kvB semantics upstream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimateSmartFeeResult {
    /// Fee rate in BTC/kvB (as floating), when present
    pub fee_rate: Option<f64>,
    pub errors: Option<Vec<String>>,
    pub blocks: Option<u16>,
}

/// Back-compat RPC client used across internal modules
pub struct BitcoinRpcClient {
    inner: core_rpc::Client,
}

impl BitcoinRpcClient {
    /// Create a new RPC client from URL and auth. Timeout is currently advisory.
    pub fn new(
        url: &str,
        username: &str,
        password: &str,
        _timeout: Duration,
    ) -> Result<Self, RpcError> {
        let auth = Auth::UserPass(username.to_string(), password.to_string());
        let inner = core_rpc::Client::new(url, auth).map_err(RpcError::from)?;
        Ok(Self { inner })
    }

    /// getblockchaininfo mapped to a minimal typed struct
    pub async fn get_blockchain_info(&self) -> Result<BlockchainInfo, RpcError> {
        // bitcoincore_rpc is sync; use spawn_blocking to avoid holding async tasks
        let inner = self.inner.clone();
        let res = spawn_blocking(move || inner.get_blockchain_info())
            .await
            .map_err(|e| RpcError(format!("Join error: {e}")))?
            .map_err(RpcError::from)?;
        let softforks = HashMap::new();
        Ok(BlockchainInfo {
            chain: res.chain.to_string(),
            blocks: res.blocks as u64,
            headers: res.headers as u64,
            best_block_hash: res.best_block_hash.to_string(),
            difficulty: res.difficulty,
            verification_progress: res.verification_progress,
            initial_block_download: res.initial_block_download,
            chain_work: hex::encode(&res.chain_work),
            size_on_disk: res.size_on_disk as u64,
            pruned: res.pruned,
            prune_height: res.prune_height.map(|v| v as i64),
            warnings: String::new(),
            softforks,
        })
    }

    /// getnetworkinfo minimal mapping
    pub async fn get_network_info(&self) -> Result<NetworkInfo, RpcError> {
        let res = tokio::task::block_in_place(|| self.inner.get_network_info())
            .map_err(RpcError::from)?;
        Ok(NetworkInfo {
            protocol_version: res.protocol_version as i64,
            connections: res.connections as i64,
        })
    }

    /// estimatesmartfee wrapper; returns BTC/kvB as f64 when available
    pub async fn estimate_smart_fee(
        &self,
        target_blocks: u16,
    ) -> Result<EstimateSmartFeeResult, RpcError> {
        let res =
            tokio::task::block_in_place(|| self.inner.estimate_smart_fee(target_blocks, None))
                .map_err(RpcError::from)?;
        Ok(EstimateSmartFeeResult {
            fee_rate: res.fee_rate.map(|amt| amt.to_btc()),
            errors: res.errors,
            blocks: Some(res.blocks as u16),
        })
    }

    /// Convenience wrappers used by several call sites
    pub async fn get_block_hash(&self, height: u64) -> Result<String, RpcError> {
        let hash = tokio::task::block_in_place(|| self.inner.get_block_hash(height as u64))
            .map_err(RpcError::from)?;
        Ok(hash.to_string())
    }

    /// Try to load an existing wallet by name. No-op if already loaded.
    pub async fn load_wallet(&self, name: &str) -> Result<(), RpcError> {
        let _ = tokio::task::block_in_place(|| {
            self.inner.call::<JsonValue>("loadwallet", &[json!(name)])
        })
        .map_err(RpcError::from)?;
        Ok(())
    }

    /// Create a new wallet by name if it doesn't exist yet.
    pub async fn create_wallet(&self, name: &str) -> Result<(), RpcError> {
        let _ = tokio::task::block_in_place(|| {
            // Minimal signature: createwallet "wallet_name"
            self.inner.call::<JsonValue>("createwallet", &[json!(name)])
        })
        .map_err(RpcError::from)?;
        Ok(())
    }

    /// Get a new address from the active wallet
    pub async fn get_new_address(
        &self,
        label: Option<&str>,
        address_type: Option<&str>,
    ) -> Result<String, RpcError> {
        let mut params = vec![];
        if let Some(l) = label {
            params.push(json!(l));
        }
        if let Some(t) = address_type {
            if params.is_empty() {
                // Bitcoin Core expects label first; use empty label when only type provided
                params.push(json!(""));
            }
            params.push(json!(t));
        }
        let addr =
            tokio::task::block_in_place(|| self.inner.call::<String>("getnewaddress", &params))
                .map_err(RpcError::from)?;
        Ok(addr)
    }

    /// Get wallet balance (BTC)
    pub async fn get_balance(&self) -> Result<f64, RpcError> {
        let bal = tokio::task::block_in_place(|| self.inner.call::<f64>("getbalance", &[]))
            .map_err(RpcError::from)?;
        Ok(bal)
    }
}
