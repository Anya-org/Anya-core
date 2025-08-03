//! Real persistent storage implementation
//!
//! Replaces mock databases with actual persistent storage using SQLite/RocksDB
//! [AIR-3][AIS-3][BPC-3][RES-3]

use anyhow::{anyhow, Context, Result};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Real persistent storage engine
#[derive(Debug)]
pub struct PersistentStorage {
    /// Storage configuration
    config: StorageConfig,
    /// SQLite connection pool for relational data
    sqlite_pool: Option<sqlx::SqlitePool>,
    /// RocksDB instance for key-value data
    rocksdb: Option<rocksdb::DB>,
    /// In-memory cache layer
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    /// Storage metrics
    metrics: Arc<RwLock<StorageMetrics>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Base directory for storage files
    pub base_dir: PathBuf,
    /// SQLite database file
    pub sqlite_file: String,
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
            sqlite_file: "anya.db".to_string(),
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
            sqlite_pool: None,
            rocksdb: None,
            cache: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(StorageMetrics::default())),
        };

        // Initialize storage backends
        storage.init_sqlite().await?;
        storage.init_rocksdb().await?;

        info!("Persistent storage initialized successfully");
        Ok(storage)
    }

    /// Initialize SQLite database
    async fn init_sqlite(&mut self) -> Result<()> {
        let db_path = self.config.base_dir.join(&self.config.sqlite_file);
        let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

        debug!("Connecting to SQLite database: {}", db_url);

        let pool = sqlx::SqlitePool::connect(&db_url)
            .await
            .context("Failed to connect to SQLite database")?;

        // Configure SQLite for performance
        if self.config.enable_wal {
            sqlx::query("PRAGMA journal_mode = WAL;")
                .execute(&pool)
                .await
                .context("Failed to enable WAL mode")?;
        }

        sqlx::query("PRAGMA synchronous = NORMAL;")
            .execute(&pool)
            .await
            .context("Failed to set synchronous mode")?;

        sqlx::query("PRAGMA cache_size = -20000;") // 20MB cache
            .execute(&pool)
            .await
            .context("Failed to set cache size")?;

        // Create tables for structured data
        self.create_tables(&pool).await?;

        self.sqlite_pool = Some(pool);
        info!("SQLite database initialized");
        Ok(())
    }

    /// Create necessary tables
    async fn create_tables(&self, pool: &sqlx::SqlitePool) -> Result<()> {
        // Key-value table for general storage
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS kv_store (
                key TEXT PRIMARY KEY,
                value BLOB NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                metadata TEXT
            );
        "#,
        )
        .execute(pool)
        .await
        .context("Failed to create kv_store table")?;

        // Transaction records
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS transactions (
                tx_id TEXT PRIMARY KEY,
                tx_data BLOB NOT NULL,
                status TEXT NOT NULL,
                block_height INTEGER,
                timestamp INTEGER NOT NULL,
                fee INTEGER,
                confirmations INTEGER DEFAULT 0
            );
        "#,
        )
        .execute(pool)
        .await
        .context("Failed to create transactions table")?;

        // Asset records
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS assets (
                asset_id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                symbol TEXT,
                total_supply INTEGER NOT NULL,
                circulating_supply INTEGER NOT NULL,
                issuer TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                metadata TEXT
            );
        "#,
        )
        .execute(pool)
        .await
        .context("Failed to create assets table")?;

        // Create indexes for performance
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_transactions_status ON transactions(status);")
            .execute(pool)
            .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_transactions_timestamp ON transactions(timestamp);",
        )
        .execute(pool)
        .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_assets_issuer ON assets(issuer);")
            .execute(pool)
            .await?;

        info!("Database tables created successfully");
        Ok(())
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
                drop(metrics);
                drop(cache);
                return Ok(Some(entry.data.clone()));
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

    /// Store structured data in SQLite
    pub async fn store_transaction(&self, tx_id: &str, tx_data: &[u8], status: &str) -> Result<()> {
        if let Some(ref pool) = self.sqlite_pool {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;

            sqlx::query(
                r#"
                INSERT OR REPLACE INTO transactions 
                (tx_id, tx_data, status, timestamp) 
                VALUES (?, ?, ?, ?)
            "#,
            )
            .bind(tx_id)
            .bind(tx_data)
            .bind(status)
            .bind(timestamp)
            .execute(pool)
            .await
            .context("Failed to store transaction")?;

            debug!("Stored transaction: {}", tx_id);
        }

        Ok(())
    }

    /// Retrieve transaction data
    pub async fn get_transaction(&self, tx_id: &str) -> Result<Option<(Vec<u8>, String)>> {
        if let Some(ref pool) = self.sqlite_pool {
            let row = sqlx::query(
                r#"
                SELECT tx_data, status FROM transactions WHERE tx_id = ?
            "#,
            )
            .bind(tx_id)
            .fetch_optional(pool)
            .await
            .context("Failed to query transaction")?;

            if let Some(row) = row {
                let tx_data: Vec<u8> = row.get("tx_data");
                let status: String = row.get("status");
                return Ok(Some((tx_data, status)));
            }
        }

        Ok(None)
    }

    /// Store asset data
    pub async fn store_asset(
        &self,
        asset_id: &str,
        name: &str,
        symbol: Option<&str>,
        total_supply: u64,
        issuer: &str,
        metadata: Option<&str>,
    ) -> Result<()> {
        if let Some(ref pool) = self.sqlite_pool {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;

            sqlx::query(r#"
                INSERT OR REPLACE INTO assets 
                (asset_id, name, symbol, total_supply, circulating_supply, issuer, created_at, metadata) 
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#)
            .bind(asset_id)
            .bind(name)
            .bind(symbol)
            .bind(total_supply as i64)
            .bind(total_supply as i64) // Initially, circulating = total
            .bind(issuer)
            .bind(timestamp)
            .bind(metadata)
            .execute(pool)
            .await
            .context("Failed to store asset")?;

            debug!("Stored asset: {}", asset_id);
        }

        Ok(())
    }

    /// List all assets
    pub async fn list_assets(&self) -> Result<Vec<AssetRecord>> {
        if let Some(ref pool) = self.sqlite_pool {
            let rows = sqlx::query(r#"
                SELECT asset_id, name, symbol, total_supply, circulating_supply, issuer, created_at, metadata
                FROM assets ORDER BY created_at DESC
            "#)
            .fetch_all(pool)
            .await
            .context("Failed to list assets")?;

            let assets = rows
                .into_iter()
                .map(|row| AssetRecord {
                    asset_id: row.get("asset_id"),
                    name: row.get("name"),
                    symbol: row.get("symbol"),
                    total_supply: row.get::<i64, _>("total_supply") as u64,
                    circulating_supply: row.get::<i64, _>("circulating_supply") as u64,
                    issuer: row.get("issuer"),
                    created_at: row.get::<i64, _>("created_at") as u64,
                    metadata: row.get("metadata"),
                })
                .collect();

            return Ok(assets);
        }

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

        // Vacuum SQLite
        if let Some(ref pool) = self.sqlite_pool {
            sqlx::query("VACUUM;")
                .execute(pool)
                .await
                .context("Failed to vacuum SQLite database")?;
            debug!("SQLite vacuum completed");
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
        // Simple LRU eviction - remove 25% of entries
        let target_size = cache.len() * 3 / 4;

        // Sort by access count and timestamp
        let mut entries: Vec<_> = cache.iter().collect();
        entries.sort_by(|a, b| {
            a.1.access_count
                .cmp(&b.1.access_count)
                .then(a.1.timestamp.cmp(&b.1.timestamp))
        });

        // Remove least recently used entries
        for (key, _) in entries.into_iter().take(cache.len() - target_size) {
            cache.remove(key);
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
