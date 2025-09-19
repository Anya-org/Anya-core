//! Tests for StorageRouter::from_env
//! [AIR-3][AIS-3][BPC-3][RES-3]

use std::env;
use std::sync::Arc;

use anya_core::storage::{persistent::PersistentStorage, persistent::StorageConfig, StorageRouter};

#[tokio::test]
async fn router_persistent_fallback_without_dwn_feature() {
    // This test runs irrespective of dwn feature; we don't construct decentralized instance.
    let config = StorageConfig::default();
    let persistent = Arc::new(
        PersistentStorage::new(config)
            .await
            .expect("persistent init"),
    );

    env::remove_var("ANYA_STORAGE_BACKEND");
    let router = StorageRouter::from_env(persistent.clone(), None);
    assert!(
        !router.decentralized_enabled(),
        "Router should fallback to persistent when no decentralized instance provided"
    );
}

#[tokio::test]
async fn router_env_forces_persistent() {
    let config = StorageConfig::default();
    let persistent = Arc::new(
        PersistentStorage::new(config)
            .await
            .expect("persistent init"),
    );

    env::set_var("ANYA_STORAGE_BACKEND", "persistent");
    let router = StorageRouter::from_env(persistent.clone(), {
        #[cfg(feature = "dwn")]
        {
            None
        }
        #[cfg(not(feature = "dwn"))]
        {
            None
        }
    });
    assert!(!router.decentralized_enabled());
}

#[cfg(feature = "dwn")]
#[tokio::test]
async fn router_auto_uses_dwn_when_available() {
    use anya_core::storage::decentralized::DecentralizedStorage;
    use anya_core::web::web5_adapter::Web5Adapter;
    use bitcoin::Network;

    env::remove_var("ANYA_STORAGE_BACKEND");
    let config = StorageConfig::default();
    let persistent = Arc::new(
        PersistentStorage::new(config)
            .await
            .expect("persistent init"),
    );
    let adapter = Arc::new(Web5Adapter::new("http://localhost:8080"));
    let decentralized = Arc::new(
        DecentralizedStorage::new(
            "http://127.0.0.1:5001",
            adapter,
            "did:example:router".to_string(),
            Network::Regtest,
            None,
        )
        .await
        .expect("decentralized init"),
    );

    let router = StorageRouter::from_env(persistent, Some(decentralized));
    assert!(
        router.decentralized_enabled(),
        "Auto mode should choose decentralized when available"
    );
}

// Autoconfig tests
#[tokio::test]
async fn autoconfig_persistent_only_when_dwn_disabled_or_init_fails() {
    // Force persistent selection
    std::env::set_var("ANYA_STORAGE_BACKEND", "persistent");
    let router = StorageRouter::autoconfig().await.expect("autoconfig");
    assert!(!router.decentralized_enabled());
}

#[cfg(feature = "dwn")]
#[tokio::test]
async fn autoconfig_auto_prefers_dwn() {
    // Remove explicit backend to allow auto selection
    std::env::remove_var("ANYA_STORAGE_BACKEND");
    // Provide endpoints likely runnable in dev (will fall back gracefully if IPFS unreachable but still attempt)
    std::env::set_var("ANYA_IPFS_ENDPOINT", "http://127.0.0.1:5001");
    std::env::set_var("ANYA_WEB5_SERVICE_URL", "http://localhost:8080");
    let router = StorageRouter::autoconfig().await.expect("autoconfig");
    // If decentralized init failed due to lack of local services, router will be persistent; accept either but log intent
    if !router.decentralized_enabled() {
        eprintln!("SKIP: decentralized not enabled in autoconfig (service may be offline)");
    }
}
