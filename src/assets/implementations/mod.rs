use super::{Asset, AssetMetadata, AssetType};
use crate::repository::Repository;
use crate::validation::{Validator, ValidationResult};
use async_trait::async_trait;
use std::sync::Arc;

pub struct BaseAsset<R, V> {
    repository: Arc<R>,
    validator: Arc<V>,
    asset_type: AssetType,
}

impl<R, V> BaseAsset<R, V> 
where
    R: Repository<AssetMetadata, anyhow::Error>,
    V: Validator<AssetMetadata>,
{
    pub fn new(repository: Arc<R>, validator: Arc<V>, asset_type: AssetType) -> Self {
        Self {
            repository,
            validator,
            asset_type,
        }
    }
}

#[async_trait]
impl<R, V> Asset for BaseAsset<R, V>
where
    R: Repository<AssetMetadata, anyhow::Error> + Send + Sync,
    V: Validator<AssetMetadata> + Send + Sync,
{
    async fn create(&self, metadata: AssetMetadata) -> anyhow::Result<String> {
        // Validate metadata before creation
        match self.validator.validate(&metadata).await? {
            ValidationResult::Valid => {
                let created = self.repository.create(metadata).await?;
                Ok(created.name.clone())
            },
            ValidationResult::Invalid(reason) => {
                Err(anyhow::anyhow!("Invalid asset metadata: {}", reason))
            },
            ValidationResult::NeedsReview(reason) => {
                Err(anyhow::anyhow!("Asset needs review: {}", reason))
            }
        }
    }

    async fn transfer(&self, asset_id: &str, recipient: &str, amount: u64) -> anyhow::Result<String> {
        let mut asset = self.repository.read(asset_id).await?
            .ok_or_else(|| anyhow::anyhow!("Asset not found: {}", asset_id))?;
        
        // Validate transfer amount
        if amount == 0 {
            return Err(anyhow::anyhow!("Transfer amount must be greater than 0"));
        }

        // Validate transfer
        let validation = self.validator.validate(&asset).await?;
        match validation {
            ValidationResult::Valid => {
                // Update asset state after transfer
                asset.metadata.insert("last_transfer".to_string(), chrono::Utc::now().to_string());
                asset.metadata.insert("last_recipient".to_string(), recipient.to_string());
                self.repository.update(asset_id, asset).await?;
                Ok(format!("Transferred {} units to {}", amount, recipient))
            },
            ValidationResult::Invalid(reason) => Err(anyhow::anyhow!("Invalid transfer: {}", reason)),
            ValidationResult::NeedsReview(reason) => Err(anyhow::anyhow!("Transfer needs review: {}", reason)),
        }
    }

    async fn get_metadata(&self, asset_id: &str) -> anyhow::Result<AssetMetadata> {
        self.repository.read(asset_id).await?
            .ok_or_else(|| anyhow::anyhow!("Asset not found"))
    }

    async fn validate(&self, asset_id: &str) -> anyhow::Result<bool> {
        let asset = self.repository.read(asset_id).await?
            .ok_or_else(|| anyhow::anyhow!("Asset not found"))?;
        
        Ok(matches!(self.validator.validate(&asset).await?, ValidationResult::Valid))
    }
}
