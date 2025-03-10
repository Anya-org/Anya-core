use crate::{
    AnyaResult,
    layer2::{
        Layer2Protocol,
        ProtocolState,
        TransactionStatus,
        AssetParams,
        AssetTransfer,
        TransferResult,
        Proof,
        VerificationResult,
        ValidationResult,
    },
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
    async fn initialize(&self) -> AnyaResult<()> {
        Ok(())
    }

    async fn connect(&self) -> AnyaResult<()> {
        Ok(())
    }

    async fn disconnect(&self) -> AnyaResult<()> {
        Ok(())
    }

    async fn submit_transaction(&self, _tx: &[u8]) -> AnyaResult<String> {
        Ok("mock_tx_id".to_string())
    }

    async fn get_transaction_status(&self, _tx_id: &str) -> AnyaResult<TransactionStatus> {
        Ok(TransactionStatus::Confirmed)
    }

    async fn get_state(&self) -> AnyaResult<ProtocolState> {
        Ok(ProtocolState {
            height: 0,
            hash: "mock_hash".to_string(),
            timestamp: 0,
        })
    }

    async fn sync_state(&self) -> AnyaResult<()> {
        Ok(())
    }

    async fn issue_asset(&self, _params: AssetParams) -> AnyaResult<String> {
        Ok("mock_asset_id".to_string())
    }

    async fn transfer_asset(&self, _transfer: AssetTransfer) -> AnyaResult<TransferResult> {
        Ok(TransferResult {
            tx_id: "mock_tx_id".to_string(),
            status: TransactionStatus::Confirmed,
            timestamp: 0,
        })
    }

    async fn verify_proof(&self, _proof: &Proof) -> AnyaResult<VerificationResult> {
        Ok(VerificationResult {
            valid: true,
            error: None,
        })
    }

    async fn validate_state(&self, _state: &ProtocolState) -> AnyaResult<ValidationResult> {
        Ok(ValidationResult {
            valid: true,
            error: None,
        })
    }
} 