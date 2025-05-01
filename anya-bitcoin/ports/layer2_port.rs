// Generated layer2_port.rs
// Hexagonal Architecture - Secondary Port

use async_trait::async_trait;
use crate::error::Result;

#[async_trait]
pub trait ProtocolPort {
    async fn submit_protocol_tx(&self, tx: ProtocolTransaction) -> Result<TransactionId>;
    async fn verify_protocol_state(&self, state: &ProtocolState) -> Result<VerificationResult>;
    async fn sync_protocol_state(&self) -> Result<SyncResult>;
}

#[async_trait]
pub trait AssetPort {
    async fn issue_asset(&self, params: AssetParams) -> Result<AssetId>;
    async fn transfer_asset(&self, transfer: AssetTransfer) -> Result<TransferResult>;
    async fn get_asset_state(&self, asset_id: AssetId) -> Result<AssetState>;
}

// Protocol-specific types
pub struct ProtocolTransaction;
pub struct ProtocolState;
pub struct TransactionId;
pub struct VerificationResult;
pub struct SyncResult;
pub struct AssetParams;
pub struct AssetId;
pub struct AssetTransfer;
pub struct TransferResult;
pub struct AssetState;
