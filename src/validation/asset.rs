use super::{Validator, ValidationResult};
use crate::assets::AssetMetadata;
use async_trait::async_trait;
use chrono::Utc;

pub struct AssetValidator;

#[async_trait]
impl Validator<AssetMetadata> for AssetValidator {
    async fn validate(&self, metadata: &AssetMetadata) -> anyhow::Result<ValidationResult> {
        // Validate basic requirements
        if metadata.name.is_empty() || metadata.name.len() > 100 {
            return Ok(ValidationResult::Invalid("Asset name must be between 1 and 100 characters".into()));
        }

        if metadata.supply == 0 {
            return Ok(ValidationResult::Invalid("Asset supply must be greater than 0".into()));
        }

        if metadata.precision > 18 {
            return Ok(ValidationResult::Invalid("Asset precision cannot exceed 18".into()));
        }

        if metadata.issuer.is_empty() {
            return Ok(ValidationResult::Invalid("Issuer cannot be empty".into()));
        }

        // Check creation time
        let age = chrono::Utc::now() - metadata.created_at;
        if age.num_seconds() < 0 {
            return Ok(ValidationResult::Invalid("Creation time cannot be in the future".into()));
        }

        Ok(ValidationResult::Valid)
    }
}
