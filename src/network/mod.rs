use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait NetworkManager: Send + Sync {
    async fn init(&self) -> Result<(), NetworkError>;
    async fn get_client(&self, network_type: NetworkType) -> Result<Arc<dyn NetworkClient>, NetworkError>;
    async fn check_health(&self, network_type: NetworkType) -> Result<NetworkStatus, NetworkError>;
    async fn get_supported_networks(&self) -> Vec<NetworkType>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkType {
    Bob,
    Lightning, 
    TaprootAssets,
    Rgb,
    Rsk,
    Dlc,
    Stacks,
    StateChannels,
}

#[derive(Debug)]
pub struct NetworkStatus {
    pub healthy: bool,
    pub sync_percentage: f64,
    pub peer_count: u32,
    pub last_block: u64,
    pub network_latency: std::time::Duration,
}
