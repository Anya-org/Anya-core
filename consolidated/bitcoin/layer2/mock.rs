use std::error::Error;
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
    fn default() -> Self  -> Result<(), Box<dyn Error>> {
        Self {
            initialized: false,
            connected: false,
        }
    }
}

impl MockLayer2Protocol {
    pub fn new() -> Self  -> Result<(), Box<dyn Error>> {
        Self::default()
    }
}

#[async_trait]
impl Layer2Protocol for MockLayer2Protocol {
    async fn initialize(&self) -> AnyaResult<()>  -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn connect(&self) -> AnyaResult<()>  -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn disconnect(&self) -> AnyaResult<()>  -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn submit_transaction(&self, __tx: tx: &[u8][u8]) -> AnyaResult<String>  -> Result<(), Box<dyn Error>> {
        Ok("mock_tx_id".to_string())
    }

    async fn get_transaction_status(&self, __tx_id: tx_id: &strstr) -> AnyaResult<TransactionStatus>  -> Result<(), Box<dyn Error>> {
        Ok(TransactionStatus::Confirmed)
    }

    async fn get_state(&self) -> AnyaResult<ProtocolState>  -> Result<(), Box<dyn Error>> {
        Ok(ProtocolState {
            height: 0,
            hash: "mock_hash".to_string(),
            timestamp: 0,
        })
    }

    async fn sync_state(&self) -> AnyaResult<()>  -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn issue_asset(&self, __params: AssetParams) -> AnyaResult<String>  -> Result<(), Box<dyn Error>> {
        Ok("mock_asset_id".to_string())
    }

    async fn transfer_asset(&self, __transfer: AssetTransfer) -> AnyaResult<TransferResult>  -> Result<(), Box<dyn Error>> {
        Ok(TransferResult {
            tx_id: "mock_tx_id".to_string(),
            status: TransactionStatus::Confirmed,
            timestamp: 0,
        })
    }

    async fn verify_proof(&self, __proof: proof: &ProofProof) -> AnyaResult<VerificationResult>  -> Result<(), Box<dyn Error>> {
        Ok(VerificationResult {
            valid: true,
            error: None,
        })
    }

    async fn validate_state(&self, __state: state: &ProtocolStateProtocolState) -> AnyaResult<ValidationResult>  -> Result<(), Box<dyn Error>> {
        Ok(ValidationResult {
            valid: true,
            error: None,
        })
    }
} 
