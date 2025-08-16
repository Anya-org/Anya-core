//! Hybrid storage orchestrator combining relational (Postgres), KV (RocksDB) and
//! decentralized (IPFS + DWN + Bitcoin anchoring) layers behind a unified trait.
//! Feature gated behind `storage-hybrid`.
//!
//! [AIR-3][AIS-3][BPC-3][RES-3]

#[cfg(feature = "dwn")]
use crate::storage::decentralized::DecentralizedStorage;
use crate::storage::persistent::PersistentStorage;
use crate::AnyaResult;
use async_trait::async_trait;
use std::sync::Arc;

/// Abstract operations common across storage layers.
#[async_trait]
pub trait StorageBackend: Send + Sync {
    async fn put_kv(&self, key: &str, value: &[u8]) -> AnyaResult<()>;
    async fn get_kv(&self, key: &str) -> AnyaResult<Option<Vec<u8>>>;
}

#[async_trait]
impl StorageBackend for PersistentStorage {
    async fn put_kv(&self, key: &str, value: &[u8]) -> AnyaResult<()> {
        self.put(key, value)
            .await
            .map_err(|e| crate::AnyaError::System(format!("Persistent put error: {e}")))
    }
    async fn get_kv(&self, key: &str) -> AnyaResult<Option<Vec<u8>>> {
        self.get(key)
            .await
            .map_err(|e| crate::AnyaError::System(format!("Persistent get error: {e}")))
    }
}

/// Unified hybrid storage façade.
pub struct HybridStorage {
    pub persistent: Arc<PersistentStorage>,
    #[cfg(feature = "dwn")]
    pub decentralized: Arc<DecentralizedStorage>,
}

impl HybridStorage {
    #[allow(clippy::new_without_default)]
    pub fn new(
        persistent: Arc<PersistentStorage>,
        #[cfg(feature = "dwn")] decentralized: Arc<DecentralizedStorage>,
    ) -> Self {
        Self {
            persistent,
            #[cfg(feature = "dwn")]
            decentralized,
        }
    }
}
