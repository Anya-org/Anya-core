use async_trait::async_trait;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationResult {
    Valid,
    Invalid(String),
    NeedsReview(String),
}

#[async_trait]
pub trait Validator<T> {
    async fn validate(&self, item: &T) -> anyhow::Result<ValidationResult>;
}

#[async_trait]
pub trait L4Validator {
    async fn validate_taproot(&self, script: &bitcoin::Script) -> anyhow::Result<ValidationResult>;
    async fn validate_psbt(&self, psbt: &bitcoin::psbt::Psbt) -> anyhow::Result<ValidationResult>;
}

pub mod transaction;
pub mod asset;
pub mod protocol;
pub mod l4_validation;

pub use transaction::TransactionValidator;
pub use asset::AssetValidator;
pub use protocol::ProtocolValidator;
