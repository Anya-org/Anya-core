//! Integration tests for DecentralizedStorage cache expiry and anchoring failures

use anya_core::storage::decentralized::{DecentralizedStorage, CacheConfig, StorageMetrics};
use anya_core::storage::ipfs::IpfsClient;
use anya_core::web::web5_adapter::Web5Adapter;
use anya_core::bitcoin::BitcoinClient;
use anya_core::types::{RGBAsset, AssetTransfer};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[tokio::test]
async fn test_cache_expiry_behavior() {
    // Setup: very short TTL for testing expiry
    let cache_config = CacheConfig {
        metadata_ttl: Duration::from_millis(100),
        hot_cache_ttl: Duration::from_millis(100),
        metadata_cache_size: 8,
        hot_cache_size: 8,
    };
    let metrics = Arc::new(Mutex::new(StorageMetrics::default()));
    let storage = DecentralizedStorage::new(
        Arc::new(Web5Adapter::default()),
        Arc::new(IpfsClient::default()),
        None,
        cache_config,
        metrics.clone(),
    );
    let asset = RGBAsset::sample();
    let _ = storage.store_asset(&asset).await;
    // First access: should populate cache
    let _ = storage.asset_exists(&asset.id).await;
    // Wait for cache to expire
    tokio::time::sleep(Duration::from_millis(150)).await;
    // Second access: should be a cache miss
    let _ = storage.asset_exists(&asset.id).await;
    let m = metrics.lock().await;
    assert!(m.cache_hits >= 1, "Should have at least one cache hit");
    assert!(m.cache_misses >= 1, "Should have at least one cache miss after expiry");
}

#[tokio::test]
async fn test_anchoring_failure_metrics() {
    // Setup: inject a BitcoinClient that always fails
    struct FailingBitcoinClient;
    #[async_trait::async_trait]
    impl BitcoinClient for FailingBitcoinClient {
        async fn anchor_data_hash(&self, _hash: &[u8]) -> anyhow::Result<String> {
            Err(anyhow::anyhow!("Simulated anchoring failure"))
        }
    }
    let cache_config = CacheConfig::default();
    let metrics = Arc::new(Mutex::new(StorageMetrics::default()));
    let storage = DecentralizedStorage::new(
        Arc::new(Web5Adapter::default()),
        Arc::new(IpfsClient::default()),
        Some(Arc::new(FailingBitcoinClient)),
        cache_config,
        metrics.clone(),
    );
    let mut asset = RGBAsset::sample();
    asset.total_supply = 2_000_000; // triggers anchoring
    let _ = storage.store_asset(&asset).await;
    let m = metrics.lock().await;
    assert!(m.anchor_attempts >= 1, "Should have at least one anchor attempt");
    assert!(m.anchor_failures >= 1, "Should have at least one anchor failure");
}
