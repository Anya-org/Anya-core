use crate::layer2::{
    Layer2Protocol, ProtocolState, TransactionStatus, AssetParams, 
    AssetTransfer, TransferResult, Proof, VerificationResult, ValidationResult
};
use async_trait::async_trait;
use mockall::{mock, predicate::*};

mock! {
    pub Layer2Protocol {}

    #[async_trait]
    impl Layer2Protocol for Layer2Protocol {
        async fn initialize(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
        async fn connect(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
        async fn get_state(&self) -> Result<ProtocolState, Box<dyn std::error::Error + Send + Sync>>;
        async fn submit_transaction(&self, tx_data: &[u8]) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
        async fn check_transaction_status(&self, tx_id: &str) -> Result<TransactionStatus, Box<dyn std::error::Error + Send + Sync>>;
        async fn sync_state(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
        async fn issue_asset(&self, params: AssetParams) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
        async fn transfer_asset(&self, transfer: AssetTransfer) -> Result<TransferResult, Box<dyn std::error::Error + Send + Sync>>;
        async fn verify_proof(&self, proof: Proof) -> Result<VerificationResult, Box<dyn std::error::Error + Send + Sync>>;
        async fn validate_state(&self, state_data: &[u8]) -> Result<ValidationResult, Box<dyn std::error::Error + Send + Sync>>;
    }
}

// The mock is automatically generated with the name MockLayer2Protocol
// by the mockall library, so we don't need to create an alias
