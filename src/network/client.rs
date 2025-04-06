use async_trait::async_trait;
use bitcoin::Transaction;

#[async_trait]
pub trait NetworkClient: Send + Sync {
    async fn connect(&self) -> Result<(), NetworkError>;
    async fn disconnect(&self) -> Result<(), NetworkError>;
    async fn submit_transaction(&self, tx: &[u8]) -> Result<String, NetworkError>;
    async fn get_transaction_status(&self, tx_id: &str) -> Result<TransactionStatus, NetworkError>;
    async fn get_balance(&self, address: &str) -> Result<u64, NetworkError>;
    async fn sync_state(&self) -> Result<NetworkStatus, NetworkError>;
}
