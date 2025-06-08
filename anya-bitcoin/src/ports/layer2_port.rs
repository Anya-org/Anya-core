// Generated layer2_port.rs
// Hexagonal Architecture - Secondary Port

use async_trait::async_trait;
use crate::core::error::AnyaResult;

// Import the TransactionStatus from framework

#[async_trait]
pub trait ProtocolPort {
    async fn submit_protocol_tx(&self, tx: ProtocolTransaction) -> AnyaResult<TransactionId>;
    async fn verify_protocol_state(&self, state: &ProtocolState) -> AnyaResult<VerificationResult>;
    async fn sync_protocol_state(&self) -> AnyaResult<SyncResult>;
}

#[async_trait]
pub trait AssetPort {
    async fn issue_asset(&self, params: AssetParams) -> AnyaResult<AssetId>;
    async fn transfer_asset(&self, transfer: AssetTransfer) -> AnyaResult<TransferResult>;
    async fn get_asset_state(&self, asset_id: AssetId) -> AnyaResult<AssetState>;
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

