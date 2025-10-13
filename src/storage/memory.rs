use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::RwLock;

use super::KeyValueStorage;

/// In-memory storage implementation
///
/// This is meant for development and testing only.
/// For production, use a persistent storage implementation.
pub struct MemoryStorage {
    data: RwLock<HashMap<String, String>>,
}

impl MemoryStorage {
    /// Create a new memory storage
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl KeyValueStorage for MemoryStorage {
    async fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut data = self.data.write().unwrap();
        data.insert(key.to_string(), value.to_string());
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<String>> {
        let data = self.data.read().unwrap();
        Ok(data.get(key).cloned())
    }

    async fn delete(&self, key: &str) -> Result<()> {
        let mut data = self.data.write().unwrap();
        data.remove(key);
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool> {
        let data = self.data.read().unwrap();
        Ok(data.contains_key(key))
    }

    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>> {
        let data = self.data.read().unwrap();
        let keys = data
            .keys()
            .filter(|k| k.starts_with(prefix))
            .cloned()
            .collect();
        Ok(keys)
    }
}
