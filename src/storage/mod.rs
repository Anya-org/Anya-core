pub mod memory;

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
