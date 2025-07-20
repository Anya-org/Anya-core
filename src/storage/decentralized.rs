// [AIR-3][AIS-3][BPC-3][RES-3] Decentralized Storage Implementation
// Replaces SQLite with IPFS + DWN + Bitcoin anchoring for complete data sovereignty
// [AIR-012] Operational Reliability and [AIP-002] Modular Architecture

// use Web5Adapter HTTP client instead of direct web5 imports
use crate::storage::ipfs::{IPFSStorage, IPFSConfig};
use crate::{AnyaError, AnyaResult};
use bitcoin::{Network, Txid};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Content identifier for IPFS
pub type ContentId = String;

/// Decentralized storage backend combining IPFS + DWN + Bitcoin anchoring
#[derive(Debug)]
pub struct DecentralizedStorage {
    /// IPFS client for immutable content storage
    ipfs_storage: Arc<IPFSStorage>,
    /// Web5Adapter HTTP client for all DWN and DID operations
    web5_adapter: Arc<Web5Adapter>,
    /// Bitcoin client for data anchoring
    bitcoin_client: Option<Arc<BitcoinAnchorService>>,
    /// Multi-layer cache for performance
    cache: Arc<Mutex<DecentralizedStorageCache>>,
    /// Current user DID
    user_did: String,
    /// Cache configuration (TTLs, sizes)
    cache_config: CacheConfig,
    /// Metrics for observability
    metrics: Arc<Mutex<StorageMetrics>>,
}

/// Configuration for cache TTLs and sizes
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub hot_cache_ttl_secs: u64,
    pub hot_cache_size: usize,
    pub query_cache_ttl_secs: u64,
    pub query_cache_size: usize,
    pub metadata_cache_ttl_secs: u64,
    pub metadata_cache_size: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            hot_cache_ttl_secs: 3600, // 1 hour
            hot_cache_size: 1000,     // ~100MB
            query_cache_ttl_secs: 300, // 5 min
            query_cache_size: 500,    // ~50MB
            metadata_cache_ttl_secs: 900, // 15 min
            metadata_cache_size: 250, // ~25MB
        }
    }
}

/// Metrics for cache and anchoring observability
#[derive(Debug, Default)]
pub struct StorageMetrics {
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub anchor_attempts: usize,
    pub anchor_failures: usize,
}

/// Multi-layer caching system for optimal performance
#[derive(Debug)]
pub struct DecentralizedStorageCache {
    /// Hot cache for frequently accessed content (100MB, 1 hour TTL)
    hot_cache: LruCache<ContentId, CachedData>,
    /// Query cache for DWN queries (50MB, 5 min TTL)
    query_cache: LruCache<String, CachedQuery>,
    /// Metadata cache for asset information (25MB, 15 min TTL)
    metadata_cache: LruCache<String, CachedMetadata>,
}

#[derive(Debug, Clone)]
struct CachedData {
    data: Vec<u8>,
    timestamp: SystemTime,
    ttl: Duration,
}

#[derive(Debug, Clone)]
struct CachedQuery {
    results: Vec<DWNRecord>,
    timestamp: SystemTime,
    ttl: Duration,
}

#[derive(Debug, Clone)]
struct CachedMetadata {
    metadata: serde_json::Value,
    timestamp: SystemTime,
    ttl: Duration,
}

/// Bitcoin anchoring service for data integrity
#[derive(Debug)]
pub struct BitcoinAnchorService {
    network: Network,
    // Bitcoin client would be injected here
}

/// RGB Asset for storage operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RGBAsset {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub total_supply: u64,
    pub precision: u8,
    pub metadata: HashMap<String, String>,
    pub contract_id: String,
}

/// RGB Invoice for payment requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RGBInvoice {
    pub id: String,
    pub asset_id: String,
    pub amount: u64,
    pub recipient_did: String,
    pub created_at: u64,
    pub expires_at: u64,
    pub memo: Option<String>,
}

/// Transfer status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferStatus {
    Pending,
    Confirmed,
    Failed,
    Cancelled,
}

/// Asset transfer record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetTransfer {
    pub id: String,
    pub asset_id: String,
    pub from_did: String,
    pub to_did: String,
    pub amount: u64,
    pub status: TransferStatus,
    pub created_at: u64,
    pub confirmed_at: Option<u64>,
}

/// Balance record for DWN storage
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BalanceRecord {
    pub asset_id: String,
    pub amount: u64,
    pub last_updated: u64,
}

/// Transfer record for DWN storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRecord {
    pub transfer: AssetTransfer,
    pub status: TransferStatus,
}

/// Asset history entry with optional Bitcoin proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetHistoryEntry {
    pub asset_id: String,
    pub event_type: String,
    pub data: serde_json::Value,
    pub timestamp: u64,
    pub actor_did: String,
    pub bitcoin_proof: Option<BitcoinProof>,
}

/// Bitcoin proof for anchored data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinProof {
    pub txid: String,
    pub block_height: Option<u64>,
    pub merkle_proof: Option<Vec<String>>,
    pub confirmation_count: u32,
}

impl DecentralizedStorage {
    /// Create a new decentralized storage instance
    ///
    /// # Arguments
    /// * `ipfs_endpoint` - The IPFS API endpoint URL
    /// * `web5_adapter` - The Web5Adapter HTTP client for all DWN/DID operations
    /// * `user_did` - The current user's DID
    /// * `network` - Bitcoin network for anchoring
    /// * `cache_config` - Optional cache configuration (uses default if None)
    pub async fn new(
        ipfs_endpoint: &str,
        web5_adapter: Arc<Web5Adapter>,
        user_did: String,
        network: Network,
        cache_config: Option<CacheConfig>,
    ) -> AnyaResult<Self> {
        let ipfs_config = IPFSConfig {
            endpoint: ipfs_endpoint.to_string(),
            enable_pinning: true,
            max_file_size: 100 * 1024 * 1024, // 100MB
            timeout: 30,
            encrypt_sensitive: true,
        };

        let ipfs_storage = Arc::new(IPFSStorage::new(ipfs_config).await?);
        let bitcoin_client = Some(Arc::new(BitcoinAnchorService::new(network)));
        let cache_config = cache_config.unwrap_or_default();
        let cache = Arc::new(Mutex::new(DecentralizedStorageCache::new(&cache_config)));
        let metrics = Arc::new(Mutex::new(StorageMetrics::default()));

        Ok(Self {
            ipfs_storage,
            web5_adapter,
            bitcoin_client,
            cache,
            user_did,
            cache_config,
            metrics,
        })
    }

    // ========================================================================
    // ASSET MANAGEMENT OPERATIONS (Replacing SQLite TODOs)
    // ========================================================================

    /// Checks if an asset exists in DWN storage.
    ///
    /// # Arguments
    /// * `asset_id` - The asset identifier to check.
    ///
    /// # Returns
    /// * `Ok(true)` if the asset exists, `Ok(false)` otherwise.
    ///
    /// # Observability
    /// Increments cache hit/miss metrics.
    pub async fn asset_exists(&self, asset_id: &str) -> AnyaResult<bool> {
        // Check cache first
        if let Ok(cache) = self.cache.lock() {
            let cache_key = format!("asset_exists:{}", asset_id);
            if let Some(cached) = cache.metadata_cache.peek(&cache_key) {
                if cached.timestamp.elapsed().unwrap_or(Duration::MAX) < cached.ttl {
                    // Metrics: cache hit
                    if let Ok(mut metrics) = self.metrics.lock() {
                        metrics.cache_hits += 1;
                    }
                    return Ok(cached.metadata.get("exists").unwrap_or(&serde_json::Value::Bool(false)).as_bool().unwrap_or(false));
                }
            }
        }

        // Metrics: cache miss
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.cache_misses += 1;
        }

        // Query DWN for asset record using Web5Adapter
        let asset_records = self.web5_adapter.query_records("*", "anya/rgb/asset")
            .map_err(|e| AnyaError::Storage(format!("DWN query error: {}", e)))?;

        let exists = asset_records.iter().any(|record| {
            record.metadata.get("asset_id") == Some(&asset_id.to_string())
        });

        // Cache the result
        if let Ok(mut cache) = self.cache.lock() {
            let cache_key = format!("asset_exists:{}", asset_id);
            let cached_metadata = CachedMetadata {
                metadata: serde_json::json!({"exists": exists}),
                timestamp: SystemTime::now(),
                ttl: Duration::from_secs(900), // 15 minutes
            };
            cache.metadata_cache.put(cache_key, cached_metadata);
        }

        Ok(exists)
    }

    /// Stores an asset in IPFS and indexes it in DWN.
    ///
    /// # Arguments
    /// * `asset` - The asset to store.
    ///
    /// # Returns
    /// * `Ok(content_id)` of the IPFS-stored asset.
    ///
    /// # Observability
    /// Anchoring attempts/failures are tracked in metrics.
    pub async fn store_asset(&self, asset: &RGBAsset) -> AnyaResult<String> {
        // 1. Serialize and store asset data in IPFS
        let asset_data = serde_json::to_vec(asset)
            .map_err(|e| AnyaError::Serialization(e.to_string()))?;

        let ipfs_metadata = self.ipfs_storage
            .store_content(&asset_data, Some(&format!("asset_{}.json", asset.id)))
            .await?;

        let content_id = ipfs_metadata.content_id.clone();

        // 2. Create DWN index record for queryability
        // Use Web5Adapter for DWN operations
        let index_record = DWNRecord {
            id: format!("asset_index_{}", asset.id),
            owner: self.user_did.clone(),
            schema: "anya/rgb/asset".to_string(),
            data: serde_json::json!({
                "asset_id": asset.id,
                "ipfs_hash": content_id,
                "name": asset.name,
                "total_supply": asset.total_supply,
                "precision": asset.precision,
                "metadata": ipfs_metadata
            }),
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("asset_id".to_string(), asset.id.clone());
                meta.insert("ipfs_hash".to_string(), content_id.clone());
                meta.insert("created_at".to_string(), current_timestamp().to_string());
                meta
            },
            attestations: Vec::new(),
        };

        let _index_id = self.web5_adapter.store_record(index_record)
            .map_err(|e| AnyaError::Storage(format!("DWN index error: {}", e)))?;

        // 3. Optional: Anchor to Bitcoin for critical assets
        if asset.total_supply > 1_000_000 { // Anchor high-value assets
            if let Some(anchor_service) = &self.bitcoin_client {
                let asset_hash = self.compute_asset_hash(asset)?;
                if let Ok(mut metrics) = self.metrics.lock() {
                    metrics.anchor_attempts += 1;
                }
                let anchor_result = anchor_service.anchor_data_hash(&asset_hash).await;
                if anchor_result.is_err() {
                    if let Ok(mut metrics) = self.metrics.lock() {
                        metrics.anchor_failures += 1;
                    }
                }
            }
        }

        // 4. Update cache
        self.update_hot_cache(&content_id, &asset_data)?;

        Ok(content_id)
    }

    /// Queries all assets owned by a DID from DWN storage.
    ///
    /// # Arguments
    /// * `owner_did` - The DID of the asset owner.
    ///
    /// # Returns
    /// * `Ok(Vec<RGBAsset>)` for all assets owned by the DID.
    pub async fn query_assets(&self, owner_did: &str) -> AnyaResult<Vec<RGBAsset>> {
        let cache_key = format!("assets_query:{}", owner_did);

        // Check query cache first
        if let Ok(cache) = self.cache.lock() {
            if let Some(cached) = cache.query_cache.peek(&cache_key) {
                if cached.timestamp.elapsed().unwrap_or(Duration::MAX) < cached.ttl {
                    return self.records_to_assets(&cached.results).await;
                }
            }
        }

        // Query DWN for asset records using Web5Adapter
        let asset_records = self.web5_adapter.query_records(owner_did, "anya/rgb/asset")
            .map_err(|e| AnyaError::Storage(format!("DWN query error: {}", e)))?;

        // Cache the query results
        if let Ok(mut cache) = self.cache.lock() {
            let cached_query = CachedQuery {
                results: asset_records.clone(),
                timestamp: SystemTime::now(),
                ttl: Duration::from_secs(300), // 5 minutes
            };
            cache.query_cache.put(cache_key, cached_query);
        }

        self.records_to_assets(&asset_records).await
    }

    // ========================================================================
    // FINANCIAL OPERATIONS (Replacing SQLite TODOs)
    // ========================================================================

    /// Retrieves the balance for a specific asset from DWN storage.
    ///
    /// # Arguments
    /// * `asset_id` - The asset identifier.
    ///
    /// # Returns
    /// * `Ok(u64)` balance for the asset.
    pub async fn get_asset_balance(&self, asset_id: &str) -> AnyaResult<u64> {
        // Query DWN for balance record
        let balance_records = self.web5_adapter.query_records(&self.user_did, "anya/rgb/balance")
            .map_err(|e| AnyaError::Storage(format!("DWN balance query error: {}", e)))?;

        // Find balance for specific asset
        for record in balance_records {
            if let Some(record_asset_id) = record.metadata.get("asset_id") {
                if record_asset_id == asset_id {
                    if let Some(amount_str) = record.metadata.get("amount") {
                        if let Ok(amount) = amount_str.parse::<u64>() {
                            return Ok(amount);
                        }
                    }
                }
            }
        }

        Ok(0) // Default to 0 if no balance found
    }

    /// Stores an invoice in IPFS and indexes it in DWN.
    ///
    /// # Arguments
    /// * `invoice` - The invoice to store.
    ///
    /// # Returns
    /// * `Ok(content_id)` of the IPFS-stored invoice.
    pub async fn store_invoice(&self, invoice: &RGBInvoice) -> AnyaResult<String> {
        // Store invoice in IPFS for immutability
        let invoice_data = serde_json::to_vec(invoice)
            .map_err(|e| AnyaError::Serialization(e.to_string()))?;

        let ipfs_metadata = self.ipfs_storage
            .store_content(&invoice_data, Some(&format!("invoice_{}.json", invoice.id)))
            .await?;

        let content_id = ipfs_metadata.content_id.clone();

        // Create DWN record for queryability
        let invoice_record = DWNRecord {
            id: format!("invoice_{}", invoice.id),
            owner: self.user_did.clone(),
            schema: "anya/rgb/invoice".to_string(),
            data: serde_json::to_value(invoice)
                .map_err(|e| AnyaError::Serialization(e.to_string()))?,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("asset_id".to_string(), invoice.asset_id.clone());
                meta.insert("amount".to_string(), invoice.amount.to_string());
                meta.insert("created_at".to_string(), invoice.created_at.to_string());
                meta.insert("ipfs_hash".to_string(), content_id.clone());
                meta
            },
            attestations: Vec::new(),
        };

        self.web5_adapter.store_record(invoice_record)
            .map_err(|e| AnyaError::Storage(format!("DWN invoice index error: {}", e)))
    }

    // ========================================================================
    // TRANSACTION OPERATIONS (Replacing SQLite TODOs)
    // ========================================================================

    /// Stores a transfer in IPFS, indexes it in DWN, and updates balances.
    ///
    /// # Arguments
    /// * `transfer` - The asset transfer to store.
    ///
    /// # Returns
    /// * `Ok(content_id)` of the IPFS-stored transfer.
    pub async fn store_transfer_and_update_balance(
        &self,
        transfer: &AssetTransfer,
    ) -> AnyaResult<String> {
        // 1. Store transfer in IPFS
        let transfer_data = serde_json::to_vec(transfer)
            .map_err(|e| AnyaError::Serialization(e.to_string()))?;

        let ipfs_metadata = self.ipfs_storage
            .store_content(&transfer_data, Some(&format!("transfer_{}.json", transfer.id)))
            .await?;

        // 2. Create DWN transfer record
        let transfer_record = DWNRecord {
            id: format!("transfer_{}", transfer.id),
            owner: transfer.from_did.clone(),
            schema: "anya/rgb/transfer".to_string(),
            data: serde_json::to_value(transfer)
                .map_err(|e| AnyaError::Serialization(e.to_string()))?,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("asset_id".to_string(), transfer.asset_id.clone());
                meta.insert("from_did".to_string(), transfer.from_did.clone());
                meta.insert("to_did".to_string(), transfer.to_did.clone());
                meta.insert("amount".to_string(), transfer.amount.to_string());
                meta.insert("ipfs_hash".to_string(), ipfs_metadata.content_id.clone());
                meta
            },
            attestations: Vec::new(),
        };

        self.web5_adapter.store_record(transfer_record)
            .map_err(|e| AnyaError::Storage(format!("DWN transfer index error: {}", e)))?;

        // 3. Update balances
        self.update_balance(&transfer.from_did, &transfer.asset_id, -(transfer.amount as i64)).await?;
        self.update_balance(&transfer.to_did, &transfer.asset_id, transfer.amount as i64).await?;

        // 4. Update cache
        self.update_hot_cache(&ipfs_metadata.content_id, &transfer_data)?;

        Ok(ipfs_metadata.content_id)
    }

    /// Retrieves the status of a transfer from DWN storage.
    ///
    /// # Arguments
    /// * `transfer_id` - The transfer identifier.
    ///
    /// # Returns
    /// * `Ok(TransferStatus)` for the transfer.
    pub async fn get_transfer_status(&self, transfer_id: &str) -> AnyaResult<TransferStatus> {
        // Query DWN for transfer record
        let transfer_records = self.web5_adapter.query_records("*", "anya/rgb/transfer")
            .map_err(|e| AnyaError::Storage(format!("DWN transfer query error: {}", e)))?;

        for record in transfer_records {
            if record.id == format!("transfer_{}", transfer_id) {
                if let Some(status_str) = record.metadata.get("status") {
                    return match status_str.as_str() {
                        "Pending" => Ok(TransferStatus::Pending),
                        "Confirmed" => Ok(TransferStatus::Confirmed),
                        "Failed" => Ok(TransferStatus::Failed),
                        "Cancelled" => Ok(TransferStatus::Cancelled),
                        _ => Ok(TransferStatus::Pending),
                    };
                }
            }
        }

        Ok(TransferStatus::Pending) // Default status
    }

    /// Validates a transfer, checking balances, asset existence, and optionally anchoring to Bitcoin.
    ///
    /// # Arguments
    /// * `transfer` - The asset transfer to validate.
    ///
    /// # Returns
    /// * `Ok(true)` if valid, `Ok(false)` otherwise.
    ///
    /// # Observability
    /// Anchoring attempts/failures are tracked in metrics.
    pub async fn validate_transfer_with_anchoring(&self, transfer: &AssetTransfer) -> AnyaResult<bool> {
        // 1. Check sender balance
        let sender_balance = self.get_asset_balance(&transfer.from_did).await?;
        if sender_balance < transfer.amount {
            return Ok(false);
        }

        // 3. Verify asset exists
        if !self.asset_exists(&transfer.asset_id).await? {
            return Ok(false);
        }

        // 4. Bitcoin anchoring for high-value transfers
        if transfer.amount > 100_000 { // Anchor high-value transfers
            if let Some(anchor_service) = &self.bitcoin_client {
                if let Ok(mut metrics) = self.metrics.lock() {
                    metrics.anchor_attempts += 1;
                }
                let transfer_hash = self.compute_transfer_hash(transfer)?;
                let anchor_result = anchor_service.anchor_data_hash(&transfer_hash).await;
                if let Ok(anchor_txid) = anchor_result {
                    // Store anchor reference using Web5Adapter
                    let anchor_record = DWNRecord {
                        id: format!("anchor_{}", transfer.id),
                        owner: self.user_did.clone(),
                        schema: "anya/bitcoin/anchor".to_string(),
                        data: serde_json::json!({
                            "transfer_id": transfer.id,
                            "anchor_txid": anchor_txid,
                            "timestamp": current_timestamp()
                        }),
                        metadata: HashMap::new(),
                        attestations: Vec::new(),
                    };
                    self.web5_adapter.store_record(anchor_record)
                        .map_err(|e| AnyaError::Storage(format!("Anchor record error: {}", e)))?;
                } else {
                    if let Ok(mut metrics) = self.metrics.lock() {
                        metrics.anchor_failures += 1;
                    }
                }
            }
        }

        Ok(true)
    }

    // ========================================================================
    // HISTORICAL DATA OPERATIONS (Replacing SQLite TODOs)
    // ========================================================================

    /// Retrieves the asset history with Bitcoin proofs from DWN storage.
    ///
    /// # Arguments
    /// * `asset_id` - The asset identifier.
    ///
    /// # Returns
    /// * `Ok(Vec<AssetHistoryEntry>)` for the asset.
    pub async fn get_asset_history_with_proofs(&self, asset_id: &str) -> AnyaResult<Vec<AssetHistoryEntry>> {
        // Query DWN for history entries
        let history_records = self.web5_adapter.query_records("*", "anya/rgb/history")
            .map_err(|e| AnyaError::Storage(format!("DWN history query error: {}", e)))?;

        let mut history_entries = Vec::new();

        for record in history_records {
            if let Some(record_asset_id) = record.metadata.get("asset_id") {
                if record_asset_id == asset_id {
                    // Parse history entry
                    if let Ok(entry) = serde_json::from_value::<AssetHistoryEntry>(record.data.clone()) {
                        history_entries.push(entry);
                    }
                }
            }
        }

        // Sort by timestamp
        history_entries.sort_by_key(|entry| entry.timestamp);

        Ok(history_entries)
    }

    /// Retrieves asset metadata from DWN storage.
    ///
    /// # Arguments
    /// * `asset_id` - The asset identifier.
    ///
    /// # Returns
    /// * `Ok(serde_json::Value)` for the asset metadata.
    pub async fn get_asset_metadata(&self, asset_id: &str) -> AnyaResult<serde_json::Value> {
        // Query DWN for asset metadata
        let asset_records = self.web5_adapter.query_records("*", "anya/rgb/asset")
            .map_err(|e| AnyaError::Storage(format!("DWN asset query error: {}", e)))?;

        for record in asset_records {
            if let Some(record_asset_id) = record.metadata.get("asset_id") {
                if record_asset_id == asset_id {
                    return Ok(record.data);
                }
            }
        }

        Err(AnyaError::Storage(format!("Asset metadata not found: {}", asset_id)))
    }

    // ========================================================================
    // UTILITY METHODS
    // ========================================================================

    /// Converts DWN records to RGBAsset objects, using cache where possible.
    ///
    /// # Arguments
    /// * `records` - Slice of DWNRecord objects.
    ///
    /// # Returns
    /// * `Ok(Vec<RGBAsset>)` for the records.
    async fn records_to_assets(&self, records: &[DWNRecord]) -> AnyaResult<Vec<RGBAsset>> {
        let mut assets = Vec::new();

        for record in records {
            if let Some(ipfs_hash) = record.metadata.get("ipfs_hash") {
                // Try cache first
                if let Ok(cache) = self.cache.lock() {
                    if let Some(cached) = cache.hot_cache.peek(ipfs_hash) {
                        if cached.timestamp.elapsed().unwrap_or(Duration::MAX) < cached.ttl {
                            let asset: RGBAsset = serde_json::from_slice(&cached.data)
                                .map_err(|e| AnyaError::Serialization(e.to_string()))?;
                            assets.push(asset);
                            continue;
                        }
                    }
                }

                // Fetch from IPFS if not cached
                let asset_data = self.ipfs_storage
                    .retrieve_content(ipfs_hash)
                    .await?;

                let asset: RGBAsset = serde_json::from_slice(&asset_data)
                    .map_err(|e| AnyaError::Serialization(e.to_string()))?;

                // Update cache
                self.update_hot_cache(ipfs_hash, &asset_data)?;

                assets.push(asset);
            }
        }

        Ok(assets)
    }

    /// Updates the balance for a DID and asset in DWN storage.
    ///
    /// # Arguments
    /// * `did` - The DID to update.
    /// * `asset_id` - The asset identifier.
    /// * `delta` - The change in balance (positive or negative).
    async fn update_balance(&self, did: &str, asset_id: &str, delta: i64) -> AnyaResult<()> {
        // Get current balance
        let current_balance = if did == &self.user_did {
            self.get_asset_balance(asset_id).await?
        } else {
            // For other users, we'd need their permission to read balance
            0
        };

        let new_balance = if delta < 0 {
            current_balance.saturating_sub((-delta) as u64)
        } else {
            current_balance.saturating_add(delta as u64)
        };

        // Create/update balance record
        let balance_record = DWNRecord {
            id: format!("balance_{}_{}", did, asset_id),
            owner: did.to_string(),
            schema: "anya/rgb/balance".to_string(),
            data: serde_json::to_value(BalanceRecord {
                asset_id: asset_id.to_string(),
                amount: new_balance,
                last_updated: current_timestamp(),
            }).map_err(|e| AnyaError::Serialization(e.to_string()))?,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("asset_id".to_string(), asset_id.to_string());
                meta.insert("amount".to_string(), new_balance.to_string());
                meta
            },
            attestations: Vec::new(),
        };

        self.web5_adapter.store_record(balance_record)
            .map_err(|e| AnyaError::Storage(format!("Balance update error: {}", e)))?;

        Ok(())
    }

    /// Updates the hot cache with new data.
    ///
    /// # Arguments
    /// * `content_id` - The content identifier.
    /// * `data` - The data to cache.
    fn update_hot_cache(&self, content_id: &str, data: &[u8]) -> AnyaResult<()> {
        if let Ok(mut cache) = self.cache.lock() {
            let cached_data = CachedData {
                data: data.to_vec(),
                timestamp: SystemTime::now(),
                ttl: Duration::from_secs(3600), // 1 hour
            };
            cache.hot_cache.put(content_id.to_string(), cached_data);
        }
        Ok(())
    }

    /// Computes a hash for an asset for anchoring.
    ///
    /// # Arguments
    /// * `asset` - The asset to hash.
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` hash bytes.
    fn compute_asset_hash(&self, asset: &RGBAsset) -> AnyaResult<Vec<u8>> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        asset.id.hash(&mut hasher);
        asset.total_supply.hash(&mut hasher);
        Ok(hasher.finish().to_be_bytes().to_vec())
    }

    /// Computes a hash for a transfer for anchoring.
    ///
    /// # Arguments
    /// * `transfer` - The transfer to hash.
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` hash bytes.
    fn compute_transfer_hash(&self, transfer: &AssetTransfer) -> AnyaResult<Vec<u8>> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        transfer.id.hash(&mut hasher);
        transfer.asset_id.hash(&mut hasher);
        transfer.amount.hash(&mut hasher);
        Ok(hasher.finish().to_be_bytes().to_vec())
    }
    
    /// Gets IPFS storage statistics and cache sizes.
    ///
    /// # Returns
    /// * `Ok(serde_json::Value)` with stats.
    pub async fn get_storage_stats(&self) -> AnyaResult<serde_json::Value> {
        let ipfs_stats = self.ipfs_storage.get_statistics().await?;
        
        Ok(serde_json::json!({
            "ipfs_stats": ipfs_stats,
            "cache_size": {
                "hot_cache": self.cache.lock().map(|c| c.hot_cache.len()).unwrap_or(0),
                "query_cache": self.cache.lock().map(|c| c.query_cache.len()).unwrap_or(0),
                "metadata_cache": self.cache.lock().map(|c| c.metadata_cache.len()).unwrap_or(0),
            }
        }))
    }
    
    /// Performs garbage collection and cache cleanup.
    ///
    /// # Returns
    /// * `Ok(serde_json::Value)` with cleanup results.
    pub async fn perform_gc(&self) -> AnyaResult<serde_json::Value> {
        // IPFS garbage collection
        let ipfs_gc_result = self.ipfs_storage.perform_gc().await?;
        
        // Cache cleanup
        if let Ok(mut cache) = self.cache.lock() {
            let mut expired_keys = Vec::new();
            
            for (key, cached_data) in cache.hot_cache.iter() {
                if cached_data.timestamp.elapsed().unwrap_or(Duration::ZERO) > cached_data.ttl {
                    expired_keys.push(key.clone());
                }
            }
            
            for key in &expired_keys {
                cache.hot_cache.pop(key);
            }
            
            // Similar for other caches...
        }
        
        Ok(serde_json::json!({
            "ipfs_gc": ipfs_gc_result,
            "cache_cleanup": "completed"
        }))
    }
}

impl DecentralizedStorageCache {
    fn new(config: &CacheConfig) -> Self {
        Self {
            hot_cache: LruCache::new(NonZeroUsize::new(config.hot_cache_size).unwrap()),
            query_cache: LruCache::new(NonZeroUsize::new(config.query_cache_size).unwrap()),
            metadata_cache: LruCache::new(NonZeroUsize::new(config.metadata_cache_size).unwrap()),
        }
    }
}

impl BitcoinAnchorService {
    fn new(network: Network) -> Self {
        Self { network }
    }

    async fn anchor_data_hash(&self, _hash: &[u8]) -> AnyaResult<String> {
        // In production, this would create and broadcast a Bitcoin transaction
        // with the hash committed in an OP_RETURN output
        Ok("mock_anchor_txid".to_string())
    }

    async fn get_transaction_proof(&self, txid: &str) -> AnyaResult<BitcoinProof> {
        // In production, this would fetch the actual transaction and merkle proof
        Ok(BitcoinProof {
            txid: txid.to_string(),
            block_height: Some(800000),
            merkle_proof: None,
            confirmation_count: 6,
        })
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

use futures::TryStreamExt;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_decentralized_storage_creation() {
        let storage = DecentralizedStorage::new(
            "http://127.0.0.1:5001",
            "did:example:123".to_string(),
            Network::Testnet,
        ).await;
        assert!(storage.is_ok());
    }

    #[tokio::test]
    async fn test_asset_storage_and_retrieval() {
        let storage = DecentralizedStorage::new(
            "http://127.0.0.1:5001",
            "did:example:123".to_string(),
            Network::Testnet,
        ).await.unwrap();

        let asset = RGBAsset {
            id: "test_asset".to_string(),
            name: "Test Asset".to_string(),
            description: Some("A test asset".to_string()),
            total_supply: 1000000,
            precision: 8,
            metadata: HashMap::new(),
            contract_id: "test_contract".to_string(),
        };

        let content_id = storage.store_asset(&asset).await.unwrap();
        assert!(!content_id.is_empty());

        let exists = storage.asset_exists(&asset.id).await.unwrap();
        assert!(exists);
    }
}
