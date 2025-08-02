use crate::layer2::{
    AssetParams, AssetTransfer, FeeEstimate, Layer2Error, Layer2Protocol, Proof,
    ProtocolCapabilities, ProtocolHealth, ProtocolState, TransactionResult, TransactionStatus,
    TransferResult, ValidationResult, VerificationResult,
};
use async_trait::async_trait;
use mockall::{mock, predicate::*};

mock! {
    pub Layer2Protocol {}

    #[async_trait]
    impl Layer2Protocol for Layer2Protocol {
        async fn initialize(&self) -> Result<(), Layer2Error>;
        async fn connect(&self) -> Result<(), Layer2Error>;
        async fn disconnect(&self) -> Result<(), Layer2Error>;
        async fn health_check(&self) -> Result<ProtocolHealth, Layer2Error>;
        async fn get_state(&self) -> Result<ProtocolState, Layer2Error>;
        async fn sync_state(&mut self) -> Result<(), Layer2Error>;
        async fn validate_state(&self, state: &ProtocolState) -> Result<ValidationResult, Layer2Error>;
        async fn submit_transaction(&self, tx_data: &[u8]) -> Result<String, Layer2Error>;
        async fn check_transaction_status(&self, tx_id: &str) -> Result<TransactionStatus, Layer2Error>;
        async fn get_transaction_history(&self, limit: Option<u32>) -> Result<Vec<TransactionResult>, Layer2Error>;
        async fn issue_asset(&self, params: AssetParams) -> Result<String, Layer2Error>;
        async fn transfer_asset(&self, transfer: AssetTransfer) -> Result<TransferResult, Layer2Error>;
        async fn verify_proof(&self, proof: Proof) -> Result<VerificationResult, Layer2Error>;
        async fn generate_proof(&self, transaction_id: &str) -> Result<Proof, Layer2Error>;
        async fn get_capabilities(&self) -> Result<ProtocolCapabilities, Layer2Error>;
        async fn estimate_fees(&self, operation: &str, params: &[u8]) -> Result<FeeEstimate, Layer2Error>;
    }
}

// The mock is automatically generated with the name MockLayer2Protocol
// by the mockall library, so we don't need to create an alias
