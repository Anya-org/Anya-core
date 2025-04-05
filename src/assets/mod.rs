use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMetadata {
    pub name: String,
    pub supply: u64,
    pub precision: u8,
    pub issuer: String,
    pub metadata: HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Fungible,
    NonFungible,
    Taproot,
    RGB,
}

#[async_trait]
pub trait Asset {
    async fn create(&self, metadata: AssetMetadata) -> anyhow::Result<String>;
    async fn transfer(&self, asset_id: &str, recipient: &str, amount: u64) -> anyhow::Result<String>;
    async fn get_metadata(&self, asset_id: &str) -> anyhow::Result<AssetMetadata>;
    async fn validate(&self, asset_id: &str) -> anyhow::Result<bool>;
}
