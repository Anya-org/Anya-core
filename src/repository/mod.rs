use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::validation::{Validator, ValidationResult};

#[async_trait]
pub trait Repository<T, E> {
    async fn create(&self, item: T) -> Result<T, E>;
    async fn read(&self, id: &str) -> Result<Option<T>, E>;
    async fn update(&self, id: &str, item: T) -> Result<T, E>;
    async fn delete(&self, id: &str) -> Result<(), E>;
    async fn list(&self) -> Result<Vec<T>, E>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryMetrics {
    pub total_items: usize,
    pub operations_count: HashMap<String, u64>,
    pub last_operation: chrono::DateTime<chrono::Utc>,
}

pub mod asset_repository;
pub mod transaction_repository;
pub mod protocol_repository;
