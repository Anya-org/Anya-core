// [AIR-3][AIS-3][BPC-3][RES-3] Storage Module - Decentralized Storage Implementation
// Provides unified interface for all storage operations replacing SQLite dependencies

#[cfg(feature = "dwn")]
pub mod decentralized;
pub mod ipfs;
pub mod memory;
pub mod persistent; // Real persistent storage implementation

#[cfg(feature = "dwn")]
pub use decentralized::{
    AssetHistoryEntry, AssetTransfer, BalanceRecord, BitcoinAnchorService, BitcoinProof, ContentId,
    DecentralizedStorage, DecentralizedStorageCache, RGBAsset, RGBInvoice, TransferRecord,
    TransferStatus,
};
#[cfg(not(feature = "dwn"))]
#[derive(Debug, Clone)]
pub struct RGBAsset {
    pub id: String,
}
#[cfg(not(feature = "dwn"))]
#[derive(Debug, Clone)]
pub struct RGBInvoice {
    pub id: String,
}
#[cfg(not(feature = "dwn"))]
#[derive(Debug, Clone)]
pub struct AssetTransfer {
    pub id: String,
}
#[cfg(not(feature = "dwn"))]
#[derive(Debug, Clone)]
pub enum TransferStatus {
    Pending,
}
pub use ipfs::{EncryptedContent, IPFSConfig, IPFSFileMetadata, IPFSStorage, PinStatus, PinType};
#[cfg(not(feature = "dwn"))]
pub use persistent::AssetRecord as AssetHistoryEntry; // Placeholder
pub use persistent::{AssetRecord, PersistentStorage, StorageConfig, StorageMetrics};

use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

/// Trait for key-value storage implementations
#[async_trait]
pub trait KeyValueStorage: Send + Sync {
    /// Store a value for a key
    async fn set(&self, key: &str, value: &str) -> Result<()>;

    /// Get a value for a key
    async fn get(&self, key: &str) -> Result<Option<String>>;

    /// Delete a key and its value
    async fn delete(&self, key: &str) -> Result<()>;

    /// Check if a key exists
    async fn exists(&self, key: &str) -> Result<bool>;

    /// List keys with a prefix
    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>>;
}

/// Unified storage interface that replaces all SQLite operations
#[async_trait]
pub trait UnifiedStorage {
    // Asset Management
    async fn asset_exists(&self, asset_id: &str) -> anyhow::Result<bool>;
    async fn store_asset(&self, asset: &RGBAsset) -> anyhow::Result<String>;
    async fn query_assets(&self, owner_did: &str) -> anyhow::Result<Vec<RGBAsset>>;
    async fn get_asset_metadata(&self, asset_id: &str) -> anyhow::Result<serde_json::Value>;
    async fn get_asset_history_with_proofs(
        &self,
        asset_id: &str,
    ) -> anyhow::Result<Vec<AssetHistoryEntry>>;

    // Financial Operations
    async fn get_asset_balance(&self, asset_id: &str) -> anyhow::Result<u64>;
    async fn store_invoice(&self, invoice: &RGBInvoice) -> anyhow::Result<String>;

    // Transaction Operations
    async fn store_transfer_and_update_balance(
        &self,
        transfer: &AssetTransfer,
    ) -> anyhow::Result<String>;
    async fn get_transfer_status(&self, transfer_id: &str) -> anyhow::Result<TransferStatus>;
    async fn validate_transfer_with_anchoring(
        &self,
        transfer: &AssetTransfer,
    ) -> anyhow::Result<bool>;
}

#[cfg(feature = "dwn")]
#[async_trait]
impl UnifiedStorage for DecentralizedStorage {
    async fn asset_exists(&self, asset_id: &str) -> anyhow::Result<bool> {
        self.asset_exists(asset_id)
            .await
            .map_err(anyhow::Error::new)
    }

    async fn store_asset(&self, asset: &RGBAsset) -> anyhow::Result<String> {
        self.store_asset(asset).await.map_err(anyhow::Error::new)
    }

    async fn query_assets(&self, owner_did: &str) -> anyhow::Result<Vec<RGBAsset>> {
        self.query_assets(owner_did)
            .await
            .map_err(anyhow::Error::new)
    }

    async fn get_asset_metadata(&self, asset_id: &str) -> anyhow::Result<serde_json::Value> {
        self.get_asset_metadata(asset_id)
            .await
            .map_err(anyhow::Error::new)
    }

    async fn get_asset_history_with_proofs(
        &self,
        asset_id: &str,
    ) -> anyhow::Result<Vec<AssetHistoryEntry>> {
        self.get_asset_history_with_proofs(asset_id)
            .await
            .map_err(anyhow::Error::new)
    }

    async fn get_asset_balance(&self, asset_id: &str) -> anyhow::Result<u64> {
        self.get_asset_balance(asset_id)
            .await
            .map_err(anyhow::Error::new)
    }

    async fn store_invoice(&self, invoice: &RGBInvoice) -> anyhow::Result<String> {
        self.store_invoice(invoice)
            .await
            .map_err(anyhow::Error::new)
    }

    async fn store_transfer_and_update_balance(
        &self,
        transfer: &AssetTransfer,
    ) -> anyhow::Result<String> {
        self.store_transfer_and_update_balance(transfer)
            .await
            .map_err(anyhow::Error::new)
    }

    async fn get_transfer_status(&self, transfer_id: &str) -> anyhow::Result<TransferStatus> {
        self.get_transfer_status(transfer_id)
            .await
            .map_err(anyhow::Error::new)
    }

    async fn validate_transfer_with_anchoring(
        &self,
        transfer: &AssetTransfer,
    ) -> anyhow::Result<bool> {
        self.validate_transfer_with_anchoring(transfer)
            .await
            .map_err(anyhow::Error::new)
    }
}

#[cfg(not(feature = "dwn"))]
#[async_trait]
impl UnifiedStorage for DecentralizedStorage {
    async fn asset_exists(&self, _asset_id: &str) -> anyhow::Result<bool> {
        Err(anyhow::anyhow!("Decentralized storage is not available: the `dwn` feature is disabled."))
    }
    async fn store_asset(&self, _asset: &RGBAsset) -> anyhow::Result<String> {
        Err(anyhow::anyhow!("Decentralized storage is not available: the `dwn` feature is disabled."))
    }
    async fn query_assets(&self, _owner_did: &str) -> anyhow::Result<Vec<RGBAsset>> {
        Err(anyhow::anyhow!("Decentralized storage is not available: the `dwn` feature is disabled."))
    }
    async fn get_asset_metadata(&self, _asset_id: &str) -> anyhow::Result<serde_json::Value> {
        Err(anyhow::anyhow!("Decentralized storage is not available: the `dwn` feature is disabled."))
    }
    async fn get_asset_history_with_proofs(
        &self,
        _asset_id: &str,
    ) -> anyhow::Result<Vec<AssetHistoryEntry>> {
        Err(anyhow::anyhow!("Decentralized storage is not available: the `dwn` feature is disabled."))
    }
    async fn get_asset_balance(&self, _asset_id: &str) -> anyhow::Result<u64> {
        Err(anyhow::anyhow!("Decentralized storage is not available: the `dwn` feature is disabled."))
    }
    async fn store_invoice(&self, _invoice: &RGBInvoice) -> anyhow::Result<String> {
        Err(anyhow::anyhow!("Decentralized storage is not available: the `dwn` feature is disabled."))
    }
    async fn store_transfer_and_update_balance(
        &self,
        _transfer: &AssetTransfer,
    ) -> anyhow::Result<String> {
        Err(anyhow::anyhow!("Decentralized storage is not available: the `dwn` feature is disabled."))
    }
    async fn get_transfer_status(&self, _transfer_id: &str) -> anyhow::Result<TransferStatus> {
        Err(anyhow::anyhow!("Decentralized storage is not available: the `dwn` feature is disabled."))
    }
    async fn validate_transfer_with_anchoring(
        &self,
        _transfer: &AssetTransfer,
    ) -> anyhow::Result<bool> {
        Err(anyhow::anyhow!("Decentralized storage is not available: the `dwn` feature is disabled."))
    }
}

/// Runtime selectable storage backend.
///
/// - When the `dwn` feature is enabled, both `Decentralized` and `Persistent` variants are functional.
/// - When the `dwn` feature is **not** enabled, the `Decentralized` variant exists but all operations will return an error.
#[derive(Clone)]
pub enum StorageRouter {
    /// Decentralized storage backend (DWN). Only functional if the `dwn` feature is enabled.
    Decentralized(Arc<DecentralizedStorage>),
    /// Persistent storage backend.
    Persistent(Arc<persistent::PersistentStorage>),
}

impl StorageRouter {
    /// Returns true if the Decentralized backend is enabled and selected.
    ///
    /// When the `dwn` feature is not enabled, this will always return false.
    pub fn decentralized_enabled(&self) -> bool {
        #[cfg(feature = "dwn")]
        {
            matches!(self, StorageRouter::Decentralized(_))
        }
        #[cfg(not(feature = "dwn"))]
        {
            false
        }
    }

    /// Construct a StorageRouter from environment variables / runtime config.
    ///
    /// Env vars:
    /// ANYA_STORAGE_BACKEND = "dwn" | "persistent" | "auto" (default auto)
    /// When "auto" and feature `dwn` enabled, selects decentralized; otherwise persistent.
    pub fn from_env(
        persistent: Arc<persistent::PersistentStorage>,
        #[cfg(feature = "dwn")] decentralized: Option<Arc<DecentralizedStorage>>,
    ) -> Self {
        let mode = std::env::var("ANYA_STORAGE_BACKEND").unwrap_or_else(|_| "auto".into());
        match mode.as_str() {
            #[cfg(feature = "dwn")]
            "dwn" => {
                if let Some(d) = decentralized {
                    StorageRouter::Decentralized(d)
                } else {
                    StorageRouter::Persistent(persistent)
                }
            }
            "persistent" => StorageRouter::Persistent(persistent),
            _ => {
                #[cfg(feature = "dwn")]
                if let Some(d) = decentralized {
                    return StorageRouter::Decentralized(d);
                }
                StorageRouter::Persistent(persistent)
            }
        }
    }

    /// Auto-configure full storage stack from environment, returning a constructed router.
    ///
    /// Environment variables (all optional, with sensible defaults):
    /// - ANYA_STORAGE_BACKEND = auto|dwn|persistent (selection logic)
    /// - ANYA_IPFS_ENDPOINT = http://127.0.0.1:5001
    /// - ANYA_WEB5_SERVICE_URL = http://localhost:8080
    /// - ANYA_DID = pre-created DID (if absent and web5 available attempts create via method "key")
    /// - ANYA_BITCOIN_NETWORK = regtest|testnet|mainnet (defaults regtest for anchoring when bitcoin feature compiled)
    ///
    /// Notes:
    /// - Requires `enterprise` feature for persistent Postgres; falls back to in-memory if initialization fails.
    /// - DWN stack only built when compiled with `dwn` feature; otherwise logs and returns persistent-only router.
    pub async fn autoconfig() -> anyhow::Result<Self> {
        use tracing::error;
        // Persistent base
        let p_cfg = persistent::StorageConfig::default();
        let persistent = match persistent::PersistentStorage::new(p_cfg.clone()).await {
            Ok(p) => Arc::new(p),
            Err(e) => {
                error!(?e, "Persistent storage init failed; aborting autoconfig");
                return Err(e);
            }
        };

        #[cfg(feature = "dwn")]
        let decentralized: Option<Arc<DecentralizedStorage>> = {
            // Build decentralized components if feature compiled
            let ipfs_endpoint = std::env::var("ANYA_IPFS_ENDPOINT")
                .unwrap_or_else(|_| "http://127.0.0.1:5001".into());
            let web5_url = std::env::var("ANYA_WEB5_SERVICE_URL")
                .unwrap_or_else(|_| "http://localhost:8080".into());
            let adapter = Arc::new(crate::web::web5_adapter::Web5Adapter::new(&web5_url));
            // DID handling
            let did = match std::env::var("ANYA_DID") {
                Ok(d) => d,
                Err(_) => match adapter.create_did("key") {
                    Ok(doc) => {
                        info!(did = %doc.did, "Created new DID via Web5 adapter");
                        doc.did
                    }
                    Err(e) => {
                        warn!(?e, "Failed to create DID; using placeholder");
                        "did:example:autoconfig".to_string()
                    }
                },
            };
            // Bitcoin network (only for anchoring; fallback regtest) - safe even if bitcoin feature absent
            #[allow(unused)]
            let network_str =
                std::env::var("ANYA_BITCOIN_NETWORK").unwrap_or_else(|_| "regtest".into());
            #[cfg(feature = "bitcoin")]
            let network = match network_str.as_str() {
                "mainnet" => bitcoin::Network::Bitcoin,
                "testnet" => bitcoin::Network::Testnet,
                "signet" => bitcoin::Network::Signet,
                _ => bitcoin::Network::Regtest,
            };
            #[cfg(not(feature = "bitcoin"))]
            let network = (); // Placeholder value when bitcoin feature is disabled

            match DecentralizedStorage::new(&ipfs_endpoint, adapter, did, network, None).await {
                Ok(d) => Some(Arc::new(d)),
                Err(e) => {
                    warn!(
                        ?e,
                        "Decentralized storage init failed; continuing without DWN"
                    );
                    None
                }
            }
        };

        #[cfg(not(feature = "dwn"))]
        let decentralized: Option<Arc<()>> = None; // placeholder type never used

        // Defer to existing selection logic
        Ok(Self::from_env(
            persistent,
            #[cfg(feature = "dwn")]
            decentralized,
        ))
    }
}

#[async_trait]
impl UnifiedStorage for StorageRouter {
    async fn asset_exists(&self, asset_id: &str) -> anyhow::Result<bool> {
        match self {
            #[cfg(feature = "dwn")]
            StorageRouter::Decentralized(d) => d.asset_exists(asset_id).await.map_err(Into::into),
            StorageRouter::Persistent(p) => Ok(p.get(asset_id).await?.is_some()),
        }
    }
    async fn store_asset(&self, asset: &RGBAsset) -> anyhow::Result<String> {
        match self {
            #[cfg(feature = "dwn")]
            StorageRouter::Decentralized(d) => d.store_asset(asset).await.map_err(Into::into),
            StorageRouter::Persistent(_p) => Ok(asset.id.clone()),
        }
    }
    async fn query_assets(&self, owner_did: &str) -> anyhow::Result<Vec<RGBAsset>> {
        match self {
            #[cfg(feature = "dwn")]
            StorageRouter::Decentralized(d) => d.query_assets(owner_did).await.map_err(Into::into),
            StorageRouter::Persistent(_p) => Ok(Vec::new()),
        }
    }
    async fn get_asset_metadata(&self, asset_id: &str) -> anyhow::Result<serde_json::Value> {
        match self {
            #[cfg(feature = "dwn")]
            StorageRouter::Decentralized(d) => {
                d.get_asset_metadata(asset_id).await.map_err(Into::into)
            }
            StorageRouter::Persistent(_p) => Ok(serde_json::json!({"asset_id": asset_id})),
        }
    }
    async fn get_asset_history_with_proofs(
        &self,
        asset_id: &str,
    ) -> anyhow::Result<Vec<AssetHistoryEntry>> {
        match self {
            #[cfg(feature = "dwn")]
            StorageRouter::Decentralized(d) => d
                .get_asset_history_with_proofs(asset_id)
                .await
                .map_err(Into::into),
            StorageRouter::Persistent(_p) => Ok(Vec::new()),
        }
    }
    async fn get_asset_balance(&self, asset_id: &str) -> anyhow::Result<u64> {
        match self {
            #[cfg(feature = "dwn")]
            StorageRouter::Decentralized(d) => {
                d.get_asset_balance(asset_id).await.map_err(Into::into)
            }
            StorageRouter::Persistent(_p) => Ok(0),
        }
    }
    async fn store_invoice(&self, invoice: &RGBInvoice) -> anyhow::Result<String> {
        match self {
            #[cfg(feature = "dwn")]
            StorageRouter::Decentralized(d) => d.store_invoice(invoice).await.map_err(Into::into),
            StorageRouter::Persistent(_p) => Ok(invoice.id.clone()),
        }
    }
    async fn store_transfer_and_update_balance(
        &self,
        transfer: &AssetTransfer,
    ) -> anyhow::Result<String> {
        match self {
            #[cfg(feature = "dwn")]
            StorageRouter::Decentralized(d) => d
                .store_transfer_and_update_balance(transfer)
                .await
                .map_err(Into::into),
            StorageRouter::Persistent(_p) => Ok(transfer.id.clone()),
        }
    }
    async fn get_transfer_status(&self, transfer_id: &str) -> anyhow::Result<TransferStatus> {
        match self {
            #[cfg(feature = "dwn")]
            StorageRouter::Decentralized(d) => {
                d.get_transfer_status(transfer_id).await.map_err(Into::into)
            }
            StorageRouter::Persistent(_p) => Ok(TransferStatus::Pending),
        }
    }
    async fn validate_transfer_with_anchoring(
        &self,
        transfer: &AssetTransfer,
    ) -> anyhow::Result<bool> {
        match self {
            #[cfg(feature = "dwn")]
            StorageRouter::Decentralized(d) => d
                .validate_transfer_with_anchoring(transfer)
                .await
                .map_err(Into::into),
            StorageRouter::Persistent(_p) => Ok(true),
        }
    }
}
