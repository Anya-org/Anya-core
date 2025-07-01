use crate::layer2::{
    Layer2Protocol,
    ProtocolState,
    TransactionStatus,
    AssetParams,
    AssetTransfer,
    TransferResult,
    Proof,
    VerificationResult,
    ValidationResult,
};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct MockLayer2Protocol {
    pub initialized: bool,
    pub connected: bool,
}

impl Default for MockLayer2Protocol {
    fn default() -> Self {
        Self {
            initialized: false,
            connected: false,
        }
    }
}

impl MockLayer2Protocol {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Layer2Protocol for MockLayer2Protocol {
    async fn initialize(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    async fn connect(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    async fn get_state(&self) -> Result<ProtocolState, Box<dyn std::error::Error + Send + Sync>> {
        Ok(ProtocolState {
            height: 0,
            hash: "mock_hash".to_string(),
            timestamp: 0,
        })
    }

    async fn submit_transaction(&self, _tx_data: &[u8]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok("mock_tx_id".to_string())
    }

    async fn check_transaction_status(&self, _tx_id: &str) -> Result<TransactionStatus, Box<dyn std::error::Error + Send + Sync>> {
        Ok(TransactionStatus::Confirmed)
    }

    async fn sync_state(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    async fn issue_asset(&self, _params: AssetParams) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok("mock_asset_id".to_string())
    }

    async fn transfer_asset(&self, _transfer: AssetTransfer) -> Result<TransferResult, Box<dyn std::error::Error + Send + Sync>> {
        Ok(TransferResult {
            tx_id: "mock_tx_id".to_string(),
            status: TransactionStatus::Confirmed,
            timestamp: 0,
        })
    }

    async fn verify_proof(&self, _proof: Proof) -> Result<VerificationResult, Box<dyn std::error::Error + Send + Sync>> {
        Ok(VerificationResult {
            valid: true,
            error: None,
        })
    }

    async fn validate_state(&self, _state_data: &[u8]) -> Result<ValidationResult, Box<dyn std::error::Error + Send + Sync>> {
        Ok(ValidationResult {
            valid: true,
            error: None,
        })
    }
} 
