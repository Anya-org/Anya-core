//! Integration test for StorageMetrics API exposure

use anya_core::storage::decentralized::{DecentralizedStorage, CacheConfig, StorageMetrics};
use anya_core::storage::ipfs::IpfsClient;
use anya_core::web::web5_adapter::Web5Adapter;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn test_metrics_exposure() {
    let cache_config = CacheConfig::default();
    let metrics = Arc::new(Mutex::new(StorageMetrics::default()));
    let storage = DecentralizedStorage::new(
        Arc::new(Web5Adapter::default()),
        Arc::new(IpfsClient::default()),
        None,
        cache_config,
        metrics.clone(),
    );
    // Simulate some cache hits/misses
    let asset_id = "test_asset";
    let _ = storage.asset_exists(asset_id).await;
    let _ = storage.asset_exists(asset_id).await;
    let m = metrics.lock().await;
    // Metrics should be accessible and reflect operations
    println!("Storage metrics: hits={}, misses={}, anchor_attempts={}, anchor_failures={}", m.cache_hits, m.cache_misses, m.anchor_attempts, m.anchor_failures);
    assert!(m.cache_hits + m.cache_misses > 0, "Metrics should be updated");
}
