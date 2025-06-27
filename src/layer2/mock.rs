//! Mock Layer2 Protocol implementation for testing

use crate::layer2::{
    AssetParams, AssetTransfer, Proof, ProtocolState, TransactionStatus, TransferResult,
    ValidationResult, VerificationResult,
};

/// Mock Layer2 Protocol for testing
#[derive(Debug)]
pub struct MockLayer2Protocol {
    /// Mock state
    operational: bool,
}

impl MockLayer2Protocol {
    /// Create a new mock protocol
    pub fn new() -> Self {
        Self { operational: true }
    }
}

impl Default for MockLayer2Protocol {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl crate::layer2::Layer2Protocol for MockLayer2Protocol {
    async fn initialize(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    async fn connect(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    async fn get_state(&self) -> Result<ProtocolState, Box<dyn std::error::Error + Send + Sync>> {
        Ok(crate::layer2::create_protocol_state(
            "mock-1.0.0",
            1,
            Some(1000000),
            self.operational,
        ))
    }

    async fn submit_transaction(
        &self,
        _tx_data: &[u8],
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok("mock_tx_id_12345".to_string())
    }

    async fn check_transaction_status(
        &self,
        _tx_id: &str,
    ) -> Result<TransactionStatus, Box<dyn std::error::Error + Send + Sync>> {
        Ok(TransactionStatus::Confirmed)
    }

    async fn sync_state(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    async fn issue_asset(
        &self,
        params: AssetParams,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok(format!("mock_asset_{}", params.name))
    }

    async fn transfer_asset(
        &self,
        transfer: AssetTransfer,
    ) -> Result<TransferResult, Box<dyn std::error::Error + Send + Sync>> {
        Ok(TransferResult {
            tx_id: format!("mock_transfer_{}", transfer.asset_id),
            status: TransactionStatus::Confirmed,
            fee: Some(500),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }

    async fn verify_proof(
        &self,
        _proof: Proof,
    ) -> Result<VerificationResult, Box<dyn std::error::Error + Send + Sync>> {
        Ok(crate::layer2::create_verification_result(true, None))
    }

    async fn validate_state(
        &self,
        _state_data: &[u8],
    ) -> Result<ValidationResult, Box<dyn std::error::Error + Send + Sync>> {
        Ok(crate::layer2::create_validation_result(true, vec![]))
    }
}

// Implementation of Layer2ProtocolTrait for MockLayer2Protocol
impl crate::layer2::Layer2ProtocolTrait for MockLayer2Protocol {
    fn initialize(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    fn get_state(&self) -> Result<ProtocolState, Box<dyn std::error::Error + Send + Sync>> {
        Ok(crate::layer2::create_protocol_state(
            "mock-1.0.0",
            1,
            Some(1000000),
            self.operational,
        ))
    }

    fn submit_transaction(
        &self,
        _tx_data: &[u8],
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok("mock_tx_id_12345".to_string())
    }

    fn check_transaction_status(
        &self,
        _tx_id: &str,
    ) -> Result<TransactionStatus, Box<dyn std::error::Error + Send + Sync>> {
        Ok(TransactionStatus::Confirmed)
    }

    fn sync_state(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.operational = true;
        Ok(())
    }

    fn issue_asset(
        &self,
        params: AssetParams,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok(format!("mock_asset_{}", params.asset_id))
    }

    fn transfer_asset(
        &self,
        transfer: AssetTransfer,
    ) -> Result<TransferResult, Box<dyn std::error::Error + Send + Sync>> {
        Ok(TransferResult {
            tx_id: format!("mock_transfer_{}", transfer.asset_id),
            status: TransactionStatus::Confirmed,
            fee: Some(100),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }

    fn verify_proof(
        &self,
        _proof: Proof,
    ) -> Result<VerificationResult, Box<dyn std::error::Error + Send + Sync>> {
        Ok(VerificationResult {
            valid: true,
            is_valid: true,
            error: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }

    fn validate_state(
        &self,
        _state_data: &[u8],
    ) -> Result<ValidationResult, Box<dyn std::error::Error + Send + Sync>> {
        Ok(ValidationResult {
            is_valid: true,
            violations: vec![],
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }
}
