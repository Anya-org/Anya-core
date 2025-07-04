// Generated transaction_port.rs
// Hexagonal Architecture - Primary Port

use crate::core::consensus::{Transaction, TransactionId, ValidationResult};
use crate::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionPort {
    async fn submit_transaction(&self, tx: Transaction) -> Result<TransactionId>;
    async fn get_transaction(&self, id: TransactionId) -> Result<Transaction>;
    async fn validate_transaction(&self, tx: &Transaction) -> Result<ValidationResult>;
}
