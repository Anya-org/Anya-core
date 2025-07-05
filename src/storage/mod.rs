// [AIR-3][AIS-3][BPC-3][RES-3] Storage Module - Decentralized Storage Implementation
// Provides unified interface for all storage operations replacing SQLite dependencies

pub mod decentralized;
pub mod memory;
pub mod ipfs;

pub use decentralized::{
    AssetHistoryEntry, AssetTransfer, BalanceRecord, BitcoinAnchorService, BitcoinProof,
    ContentId, DecentralizedStorage, DecentralizedStorageCache, RGBAsset, RGBInvoice,
    TransferRecord, TransferStatus,
};
pub use ipfs::{
    IPFSStorage, IPFSConfig, IPFSFileMetadata, IPFSBatch, BatchResult, PinStatus, PinType,
    EncryptedContent,
};

use anyhow::Result;
use async_trait::async_trait;

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
    async fn get_asset_history_with_proofs(&self, asset_id: &str) -> anyhow::Result<Vec<AssetHistoryEntry>>;

    // Financial Operations
    async fn get_asset_balance(&self, asset_id: &str) -> anyhow::Result<u64>;
    async fn store_invoice(&self, invoice: &RGBInvoice) -> anyhow::Result<String>;

    // Transaction Operations
    async fn store_transfer_and_update_balance(&self, transfer: &AssetTransfer) -> anyhow::Result<String>;
    async fn get_transfer_status(&self, transfer_id: &str) -> anyhow::Result<TransferStatus>;
    async fn validate_transfer_with_anchoring(&self, transfer: &AssetTransfer) -> anyhow::Result<bool>;
}

#[async_trait]
impl UnifiedStorage for DecentralizedStorage {
    async fn asset_exists(&self, asset_id: &str) -> anyhow::Result<bool> {
        self.asset_exists(asset_id).await.map_err(Into::into)
    }

    async fn store_asset(&self, asset: &RGBAsset) -> anyhow::Result<String> {
        self.store_asset(asset).await.map_err(Into::into)
    }

    async fn query_assets(&self, owner_did: &str) -> anyhow::Result<Vec<RGBAsset>> {
        self.query_assets(owner_did).await.map_err(Into::into)
    }

    async fn get_asset_metadata(&self, asset_id: &str) -> anyhow::Result<serde_json::Value> {
        self.get_asset_metadata(asset_id).await.map_err(Into::into)
    }

    async fn get_asset_history_with_proofs(&self, asset_id: &str) -> anyhow::Result<Vec<AssetHistoryEntry>> {
        self.get_asset_history_with_proofs(asset_id).await.map_err(Into::into)
    }

    async fn get_asset_balance(&self, asset_id: &str) -> anyhow::Result<u64> {
        self.get_asset_balance(asset_id).await.map_err(Into::into)
    }

    async fn store_invoice(&self, invoice: &RGBInvoice) -> anyhow::Result<String> {
        self.store_invoice(invoice).await.map_err(Into::into)
    }

    async fn store_transfer_and_update_balance(&self, transfer: &AssetTransfer) -> anyhow::Result<String> {
        self.store_transfer_and_update_balance(transfer).await.map_err(Into::into)
    }

    async fn get_transfer_status(&self, transfer_id: &str) -> anyhow::Result<TransferStatus> {
        self.get_transfer_status(transfer_id).await.map_err(Into::into)
    }

    async fn validate_transfer_with_anchoring(&self, transfer: &AssetTransfer) -> anyhow::Result<bool> {
        self.validate_transfer_with_anchoring(transfer).await.map_err(Into::into)
    }
}
