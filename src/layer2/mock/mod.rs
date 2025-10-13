use crate::layer2::{
    AssetParams, AssetTransfer, FeeEstimate, Layer2Error, Layer2Protocol, Proof,
    ProtocolCapabilities, ProtocolHealth, ProtocolState, TransactionResult, TransactionStatus,
    TransferResult, ValidationResult, VerificationResult,
};
use async_trait::async_trait;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Simple mock implementation for testing
pub struct MockLayer2Protocol {
    pub connected: bool,
}

impl Default for MockLayer2Protocol {
    fn default() -> Self {
        Self::new()
    }
}

impl MockLayer2Protocol {
    pub fn new() -> Self {
        Self { connected: false }
    }
}

#[async_trait]
impl Layer2Protocol for MockLayer2Protocol {
    async fn initialize(&self) -> Result<(), Layer2Error> {
        Ok(())
    }

    async fn connect(&self) -> Result<(), Layer2Error> {
        Ok(())
    }

    async fn disconnect(&self) -> Result<(), Layer2Error> {
        Ok(())
    }

    async fn health_check(&self) -> Result<ProtocolHealth, Layer2Error> {
        Ok(ProtocolHealth {
            healthy: true,
            last_check: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            error_count: 0,
            uptime_seconds: 3600,
        })
    }

    async fn get_state(&self) -> Result<ProtocolState, Layer2Error> {
        Ok(ProtocolState {
            version: "0.1.0".to_string(),
            connections: 1,
            capacity: Some(1000000),
            operational: true,
            height: 100,
            hash: "0".repeat(64),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        })
    }

    async fn sync_state(&mut self) -> Result<(), Layer2Error> {
        Ok(())
    }

    async fn validate_state(
        &self,
        _state: &ProtocolState,
    ) -> Result<ValidationResult, Layer2Error> {
        Ok(ValidationResult {
            is_valid: true,
            violations: vec![],
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        })
    }

    async fn submit_transaction(&self, _tx_data: &[u8]) -> Result<String, Layer2Error> {
        Ok(Uuid::new_v4().to_string())
    }

    async fn check_transaction_status(
        &self,
        _tx_id: &str,
    ) -> Result<TransactionStatus, Layer2Error> {
        Ok(TransactionStatus::Pending)
    }

    async fn get_transaction_history(
        &self,
        _limit: Option<u32>,
    ) -> Result<Vec<TransactionResult>, Layer2Error> {
        Ok(vec![])
    }

    async fn issue_asset(&self, _params: AssetParams) -> Result<String, Layer2Error> {
        Ok(Uuid::new_v4().to_string())
    }

    async fn transfer_asset(
        &self,
        _transfer: AssetTransfer,
    ) -> Result<TransferResult, Layer2Error> {
        Ok(TransferResult {
            tx_id: Uuid::new_v4().to_string(),
            status: TransactionStatus::Pending,
            fee: Some(100),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        })
    }

    async fn verify_proof(&self, _proof: Proof) -> Result<VerificationResult, Layer2Error> {
        Ok(VerificationResult {
            valid: true,
            is_valid: true,
            error: None,
            error_message: None,
            confidence_score: 1.0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        })
    }

    async fn generate_proof(&self, _transaction_id: &str) -> Result<Proof, Layer2Error> {
        Ok(Proof {
            proof_type: "mock".to_string(),
            data: vec![],
            block_height: Some(100),
            witness: None,
            merkle_root: "0".repeat(64),
            merkle_proof: vec![],
            block_header: "0".repeat(160),
        })
    }

    async fn get_capabilities(&self) -> Result<ProtocolCapabilities, Layer2Error> {
        Ok(ProtocolCapabilities {
            supports_assets: true,
            supports_smart_contracts: true,
            supports_privacy: true,
            max_transaction_size: 1000000,
            fee_estimation: true,
        })
    }

    async fn estimate_fees(
        &self,
        _operation: &str,
        _params: &[u8],
    ) -> Result<FeeEstimate, Layer2Error> {
        Ok(FeeEstimate {
            estimated_fee: 1000,
            fee_rate: 1.0,
            confirmation_target: 6,
            slow_fee: 500,
            normal_fee: 1000,
            fast_fee: 2000,
            estimated_confirmation_time: 6,
        })
    }
}
