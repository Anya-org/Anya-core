//! Persistent storage (baseline minimal version)
//!
//! This stripped-down variant intentionally omits Postgres/SQLite integration so the
//! crate can compile without the `enterprise` feature while higher‑level refactors
//! (autoconfig, DWN gating) are stabilized. Reintroduce advanced backends once
//! feature gating is verified. Only RocksDB + in‑memory cache are active.
//! [TEMP-BASELINE]

use anyhow::{Context, Result};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Real persistent storage engine
#[derive(Debug)]
pub struct PersistentStorage {
    config: StorageConfig,
    rocksdb: Option<rocksdb::DB>,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    metrics: Arc<RwLock<StorageMetrics>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Base directory for storage files
    pub base_dir: PathBuf,
    /// Database URL (Postgres). Example: postgres://user:pass@host:5432/db
    pub database_url: String, // retained for future re‑enablement of enterprise path
    /// RocksDB directory
    pub rocksdb_dir: String,
    /// Cache size in MB
    pub cache_size_mb: usize,
    /// Enable WAL mode for SQLite
    pub enable_wal: bool,
    /// RocksDB write buffer size
    pub write_buffer_size: usize,
    /// Maximum number of background jobs
    pub max_background_jobs: i32,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    data: Vec<u8>,
    timestamp: u64,
    access_count: u64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    pub reads: u64,
    pub writes: u64,
    pub deletes: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub total_keys: u64,
    pub storage_size_bytes: u64,
    pub cache_size_bytes: u64,
    pub last_compaction: u64,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            base_dir: PathBuf::from("./data"),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://user:password@localhost:5432/anya".to_string()),
            rocksdb_dir: "rocksdb".to_string(),
            cache_size_mb: 100,
            enable_wal: true,
            write_buffer_size: 64 * 1024 * 1024, // 64MB
            max_background_jobs: 4,
        }
    }
}

impl PersistentStorage {
    /// Create new persistent storage instance
    pub async fn new(config: StorageConfig) -> Result<Self> {
        info!("Initializing persistent storage at: {:?}", config.base_dir);

        // Create storage directories
        std::fs::create_dir_all(&config.base_dir).context("Failed to create storage directory")?;

        let rocksdb_path = config.base_dir.join(&config.rocksdb_dir);
        std::fs::create_dir_all(&rocksdb_path).context("Failed to create RocksDB directory")?;

        let mut storage = Self {
            config,
            rocksdb: None,
            cache: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(StorageMetrics::default())),
        };

        storage.init_rocksdb().await?;

        info!("Persistent storage initialized successfully");
        Ok(storage)
    }

    /// Initialize RocksDB
    async fn init_rocksdb(&mut self) -> Result<()> {
        let rocksdb_path = self.config.base_dir.join(&self.config.rocksdb_dir);

        debug!("Opening RocksDB at: {:?}", rocksdb_path);

        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(true);
        opts.set_write_buffer_size(self.config.write_buffer_size);
        opts.set_max_background_jobs(self.config.max_background_jobs);
        opts.set_compression_type(rocksdb::DBCompressionType::Lz4);

        // Configure for performance
        opts.set_level_compaction_dynamic_level_bytes(true);
        opts.set_max_bytes_for_level_base(512 * 1024 * 1024); // 512MB
        opts.set_target_file_size_base(64 * 1024 * 1024); // 64MB

        let db = rocksdb::DB::open(&opts, rocksdb_path).context("Failed to open RocksDB")?;

        self.rocksdb = Some(db);
        info!("RocksDB initialized successfully");
        Ok(())
    }

    /// Store key-value data (uses RocksDB for performance)
    pub async fn put(&self, key: &str, value: &[u8]) -> Result<()> {
        debug!("Storing key: {} ({} bytes)", key, value.len());

        // Update cache
        let cache_entry = CacheEntry {
            data: value.to_vec(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            access_count: 1,
        };

        {
            let mut cache = self.cache.write().await;
            cache.insert(key.to_string(), cache_entry);

            // Evict old entries if cache is too large
            if cache.len() > (self.config.cache_size_mb * 1024) {
                self.evict_cache_entries(&mut cache).await;
            }
        }

        // Store in RocksDB
        if let Some(ref db) = self.rocksdb {
            db.put(key.as_bytes(), value)
                .context("Failed to write to RocksDB")?;
        }

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.writes += 1;
            metrics.total_keys += 1;
            metrics.storage_size_bytes += value.len() as u64;
        }

        Ok(())
    }

    /// Retrieve key-value data
    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        debug!("Retrieving key: {}", key);

        // Check cache first
        {
            let mut cache = self.cache.write().await;
            if let Some(entry) = cache.get_mut(key) {
                entry.access_count += 1;
                let mut metrics = self.metrics.write().await;
                metrics.reads += 1;
                metrics.cache_hits += 1;
                let data = entry.data.clone();
                return Ok(Some(data));
            }
        }

        // Check RocksDB
        if let Some(ref db) = self.rocksdb {
            match db.get(key.as_bytes())? {
                Some(value) => {
                    // Add to cache
                    let cache_entry = CacheEntry {
                        data: value.clone(),
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
                        access_count: 1,
                    };

                    self.cache
                        .write()
                        .await
                        .insert(key.to_string(), cache_entry);

                    // Update metrics
                    let mut metrics = self.metrics.write().await;
                    metrics.reads += 1;
                    metrics.cache_misses += 1;

                    Ok(Some(value))
                }
                None => {
                    let mut metrics = self.metrics.write().await;
                    metrics.reads += 1;
                    metrics.cache_misses += 1;
                    Ok(None)
                }
            }
        } else {
            Ok(None)
        }
    }

    /// Delete key-value data
    pub async fn delete(&self, key: &str) -> Result<()> {
        debug!("Deleting key: {}", key);

        // Remove from cache
        self.cache.write().await.remove(key);

        // Delete from RocksDB
        if let Some(ref db) = self.rocksdb {
            db.delete(key.as_bytes())
                .context("Failed to delete from RocksDB")?;
        }

        // Update metrics
        let mut metrics = self.metrics.write().await;
        metrics.deletes += 1;
        if metrics.total_keys > 0 {
            metrics.total_keys -= 1;
        }

        Ok(())
    }

    // Stub structured data APIs (no-op in minimal baseline)
    pub async fn store_transaction(
        &self,
        _tx_id: &str,
        _tx_data: &[u8],
        _status: &str,
    ) -> Result<()> {
        Ok(())
    }
    pub async fn get_transaction(&self, _tx_id: &str) -> Result<Option<(Vec<u8>, String)>> {
        Ok(None)
    }
    pub async fn store_asset(
        &self,
        _asset_id: &str,
        _name: &str,
        _symbol: Option<&str>,
        _total_supply: u64,
        _issuer: &str,
        _metadata: Option<&str>,
    ) -> Result<()> {
        Ok(())
    }
    pub async fn list_assets(&self) -> Result<Vec<AssetRecord>> {
        Ok(Vec::new())
    }

    /// Get storage metrics
    pub async fn get_metrics(&self) -> StorageMetrics {
        self.metrics.read().await.clone()
    }

    /// Compact storage (RocksDB compaction + SQLite vacuum)
    pub async fn compact(&self) -> Result<()> {
        info!("Starting storage compaction");

        // Compact RocksDB
        if let Some(ref db) = self.rocksdb {
            db.compact_range::<&[u8], &[u8]>(None, None);
            debug!("RocksDB compaction completed");
        }

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.last_compaction = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
        }

        info!("Storage compaction completed");
        Ok(())
    }

    /// Evict old cache entries
    async fn evict_cache_entries(&self, cache: &mut HashMap<String, CacheEntry>) {
        // Simple LRU-style eviction: remove 25% least-used entries
        let target_size = cache.len() * 3 / 4;
        if cache.len() <= target_size || cache.is_empty() {
            return;
        }
        // Collect (key, access_count, timestamp) to avoid holding immutable borrows during removal
        let mut entries: Vec<(String, u64, u64)> = cache
            .iter()
            .map(|(k, v)| (k.clone(), v.access_count, v.timestamp))
            .collect();
        entries.sort_by(|a, b| a.1.cmp(&b.1).then(a.2.cmp(&b.2)));
        let remove_count = cache.len() - target_size;
        for (key, _, _) in entries.into_iter().take(remove_count) {
            cache.remove(&key);
        }
        debug!("Evicted cache entries, new size: {}", cache.len());
    }

    /// Close storage connections
    pub async fn close(&self) -> Result<()> {
        info!("Closing storage connections");

        // SQLite pool will close automatically when dropped
        // RocksDB will close automatically when dropped

        info!("Storage closed successfully");
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRecord {
    pub asset_id: String,
    pub name: String,
    pub symbol: Option<String>,
    pub total_supply: u64,
    pub circulating_supply: u64,
    pub issuer: String,
    pub created_at: u64,
    pub metadata: Option<String>,
}

// Drop implementation to ensure clean shutdown
impl Drop for PersistentStorage {
    fn drop(&mut self) {
        debug!("Dropping PersistentStorage instance");
    }
}
