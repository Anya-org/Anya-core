// Generated blockchain_port.rs
// Hexagonal Architecture - Primary Port

use async_trait::async_trait;
use crate::core::consensus::{Block, BlockHash, Transaction, Proof};
use crate::error::Result;

#[async_trait]
pub trait BlockchainPort {
    async fn broadcast_transaction(&self, tx: &Transaction) -> Result<BroadcastResult>;
    async fn get_block(&self, hash: BlockHash) -> Result<Block>;
    async fn verify_proof(&self, proof: &Proof) -> Result<VerificationResult>;
}

pub struct BroadcastResult {
    pub transaction_id: String,
    pub status: BroadcastStatus,
}

pub enum BroadcastStatus {
    Accepted,
    Rejected(String),
    Pending,
}

pub struct VerificationResult {
    pub is_valid: bool,
    pub reason: Option<String>,
}

