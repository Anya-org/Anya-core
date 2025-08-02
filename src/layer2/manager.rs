// [AIR-3][AIS-3][AIM-3][BPC-3][RES-3]
//! Layer2Manager - Comprehensive async protocol coordination
//!
//! This module provides centralized management for all Layer2 Bitcoin protocols,
//! with full async support, event handling, and production-ready coordination.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use crate::layer2::{
    async_coordinator::{AsyncLayer2Coordinator, Layer2Event},
    bob::BobProtocol,
    dlc::DlcProtocol,
    // Protocol implementations
    lightning::LightningProtocol,
    liquid::LiquidProtocol,
    rgb::RgbProtocol,
    rsk::RskProtocol,
    stacks::StacksProtocol,
    state_channels::StateChannelsProtocol,
    taproot_assets::TaprootAssetsProtocol,
    Layer2Error,
    Layer2Protocol,
    Layer2ProtocolType,
    ProtocolHealth,
    ProtocolState,
    TransactionStatus,
};

/// Configuration for Layer2Manager
#[derive(Debug, Clone)]
pub struct Layer2ManagerConfig {
    /// Protocols to enable on startup
    pub enabled_protocols: Vec<Layer2ProtocolType>,
    /// Enable automatic health monitoring
    pub enable_health_monitoring: bool,
    /// Enable event logging
    pub enable_event_logging: bool,
    /// Maximum concurrent operations per protocol
    pub max_concurrent_operations: usize,
}

impl Default for Layer2ManagerConfig {
    fn default() -> Self {
        Self {
            enabled_protocols: vec![
                Layer2ProtocolType::Lightning,
                Layer2ProtocolType::StateChannels,
                Layer2ProtocolType::RGB,
                Layer2ProtocolType::DLC,
                Layer2ProtocolType::Liquid,
            ],
            enable_health_monitoring: true,
            enable_event_logging: true,
            max_concurrent_operations: 100,
        }
    }
}

/// Comprehensive Layer2 manager with async support
pub struct Layer2Manager {
    coordinator: AsyncLayer2Coordinator,
    config: Layer2ManagerConfig,
    protocol_configs: Arc<RwLock<HashMap<Layer2ProtocolType, String>>>,
    event_handler: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
    initialized: Arc<RwLock<bool>>,
}

impl Layer2Manager {
    /// Create a new Layer2Manager with default configuration
    pub fn new() -> Self {
        Self::with_config(Layer2ManagerConfig::default())
    }

    /// Create a new Layer2Manager with custom configuration
    pub fn with_config(config: Layer2ManagerConfig) -> Self {
        Self {
            coordinator: AsyncLayer2Coordinator::new(),
            config,
            protocol_configs: Arc::new(RwLock::new(HashMap::new())),
            event_handler: Arc::new(RwLock::new(None)),
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Initialize the Layer2Manager and all enabled protocols
    pub async fn initialize(&self) -> Result<(), Layer2Error> {
        let mut initialized = self.initialized.write().await;
        if *initialized {
            return Ok(());
        }

        info!(
            "Initializing Layer2Manager with {} protocols",
            self.config.enabled_protocols.len()
        );

        // Register all enabled protocols
        for protocol_type in &self.config.enabled_protocols {
            if let Err(e) = self.register_protocol(*protocol_type).await {
                error!("Failed to register protocol {:?}: {}", protocol_type, e);
                // Continue with other protocols instead of failing completely
                continue;
            }
        }

        // Start health monitoring if enabled
        if self.config.enable_health_monitoring {
            self.coordinator.start_health_monitoring().await;
        }

        // Start event handling if enabled
        if self.config.enable_event_logging {
            self.start_event_handling().await;
        }

        *initialized = true;
        info!("Layer2Manager initialization complete");
        Ok(())
    }

    /// Register a specific protocol with the manager
    async fn register_protocol(
        &self,
        protocol_type: Layer2ProtocolType,
    ) -> Result<(), Layer2Error> {
        let protocol: Arc<dyn Layer2Protocol> = match protocol_type {
            Layer2ProtocolType::Lightning => Arc::new(LightningProtocol::default()),
            Layer2ProtocolType::RGB => Arc::new(RgbProtocol::default()),
            Layer2ProtocolType::DLC => Arc::new(DlcProtocol::default()),
            Layer2ProtocolType::StateChannels => Arc::new(StateChannelsProtocol::default()),
            Layer2ProtocolType::Liquid => Arc::new(LiquidProtocol::default()),
            Layer2ProtocolType::Stacks => Arc::new(StacksProtocol::default()),
            Layer2ProtocolType::BOB => Arc::new(BobProtocol::default()),
            Layer2ProtocolType::RSK => Arc::new(RskProtocol::default()),
            Layer2ProtocolType::TaprootAssets => Arc::new(TaprootAssetsProtocol::default()),
        };

        self.coordinator
            .register_protocol(protocol_type, protocol)
            .await?;
        info!("Registered protocol: {:?}", protocol_type);
        Ok(())
    }

    /// Start event handling in the background
    async fn start_event_handling(&self) {
        if let Some(mut receiver) = self.coordinator.take_event_receiver().await {
            let handle = tokio::spawn(async move {
                while let Some(event) = receiver.recv().await {
                    match event {
                        Layer2Event::ProtocolConnected {
                            protocol,
                            timestamp,
                        } => {
                            info!("Protocol connected: {:?} at {:?}", protocol, timestamp);
                        }
                        Layer2Event::ProtocolDisconnected {
                            protocol,
                            reason,
                            timestamp,
                        } => {
                            warn!(
                                "Protocol disconnected: {:?} - {} at {:?}",
                                protocol, reason, timestamp
                            );
                        }
                        Layer2Event::TransactionSubmitted {
                            protocol,
                            tx_id,
                            timestamp,
                        } => {
                            debug!(
                                "Transaction submitted to {:?}: {} at {:?}",
                                protocol, tx_id, timestamp
                            );
                        }
                        Layer2Event::TransactionStatusChanged {
                            protocol,
                            tx_id,
                            old_status,
                            new_status,
                            timestamp,
                        } => {
                            info!(
                                "Transaction status changed: {:?} {} {:?} -> {:?} at {:?}",
                                protocol, tx_id, old_status, new_status, timestamp
                            );
                        }
                        Layer2Event::HealthChanged {
                            protocol,
                            health,
                            timestamp,
                        } => {
                            if health.healthy {
                                debug!("Protocol health OK: {:?} at {:?}", protocol, timestamp);
                            } else {
                                warn!(
                                    "Protocol health degraded: {:?} (errors: {}) at {:?}",
                                    protocol, health.error_count, timestamp
                                );
                            }
                        }
                        Layer2Event::Error {
                            protocol,
                            error,
                            timestamp,
                        } => {
                            error!(
                                "Protocol error: {:?} - {} at {:?}",
                                protocol, error, timestamp
                            );
                        }
                    }
                }
            });

            let mut event_handler = self.event_handler.write().await;
            *event_handler = Some(handle);
        }
    }

    /// Submit a transaction to a specific protocol
    pub async fn submit_transaction(
        &self,
        protocol_type: Layer2ProtocolType,
        tx_data: &[u8],
    ) -> Result<String, Layer2Error> {
        self.ensure_initialized().await?;
        self.coordinator
            .submit_transaction(protocol_type, tx_data)
            .await
    }

    /// Check transaction status
    pub async fn check_transaction_status(
        &self,
        protocol_type: Layer2ProtocolType,
        tx_id: &str,
    ) -> Result<TransactionStatus, Layer2Error> {
        self.ensure_initialized().await?;
        self.coordinator
            .check_transaction_status(protocol_type, tx_id)
            .await
    }

    /// Get protocol state
    pub async fn get_protocol_state(
        &self,
        protocol_type: Layer2ProtocolType,
    ) -> Result<ProtocolState, Layer2Error> {
        self.ensure_initialized().await?;
        self.coordinator.get_protocol_state(protocol_type).await
    }

    /// Get health status for all protocols
    pub async fn get_all_health_status(
        &self,
    ) -> Result<HashMap<Layer2ProtocolType, ProtocolHealth>, Layer2Error> {
        self.ensure_initialized().await?;
        Ok(self.coordinator.get_all_health_status().await)
    }

    /// Get health status for a specific protocol
    pub async fn get_protocol_health(
        &self,
        protocol_type: Layer2ProtocolType,
    ) -> Result<ProtocolHealth, Layer2Error> {
        let all_health = self.get_all_health_status().await?;
        all_health
            .get(&protocol_type)
            .cloned()
            .ok_or_else(|| Layer2Error::Protocol("Protocol not found".to_string()))
    }

    /// Get list of enabled protocols
    pub fn get_enabled_protocols(&self) -> Vec<Layer2ProtocolType> {
        self.config.enabled_protocols.clone()
    }

    /// Enable a new protocol at runtime
    pub async fn enable_protocol(
        &self,
        protocol_type: Layer2ProtocolType,
    ) -> Result<(), Layer2Error> {
        self.register_protocol(protocol_type).await?;
        // Update config to include the new protocol
        // Note: This would typically involve updating persistent configuration
        info!("Protocol enabled at runtime: {:?}", protocol_type);
        Ok(())
    }

    /// Disable a protocol at runtime
    pub async fn disable_protocol(
        &self,
        protocol_type: Layer2ProtocolType,
    ) -> Result<(), Layer2Error> {
        self.coordinator.unregister_protocol(protocol_type).await?;
        info!("Protocol disabled at runtime: {:?}", protocol_type);
        Ok(())
    }

    /// Get comprehensive status report
    pub async fn get_status_report(&self) -> Result<Layer2StatusReport, Layer2Error> {
        let initialized = *self.initialized.read().await;
        if !initialized {
            return Ok(Layer2StatusReport {
                initialized: false,
                enabled_protocols: self.config.enabled_protocols.clone(),
                protocol_health: HashMap::new(),
                total_protocols: 0,
                healthy_protocols: 0,
                error_count: 0,
                uptime_seconds: 0,
            });
        }

        let health_status = self.get_all_health_status().await?;
        let healthy_count = health_status.values().filter(|h| h.healthy).count();
        let total_errors: u32 = health_status.values().map(|h| h.error_count).sum();

        Ok(Layer2StatusReport {
            initialized: true,
            enabled_protocols: self.config.enabled_protocols.clone(),
            protocol_health: health_status.clone(),
            total_protocols: health_status.len(),
            healthy_protocols: healthy_count,
            error_count: total_errors,
            uptime_seconds: 3600, // This would be calculated from start time
        })
    }

    /// Graceful shutdown
    pub async fn shutdown(&self) -> Result<(), Layer2Error> {
        info!("Shutting down Layer2Manager...");

        // Stop event handling
        let mut event_handler = self.event_handler.write().await;
        if let Some(handle) = event_handler.take() {
            handle.abort();
        }

        // Shutdown coordinator
        self.coordinator.shutdown().await?;

        // Mark as not initialized
        let mut initialized = self.initialized.write().await;
        *initialized = false;

        info!("Layer2Manager shutdown complete");
        Ok(())
    }

    /// Ensure the manager is initialized
    async fn ensure_initialized(&self) -> Result<(), Layer2Error> {
        let initialized = *self.initialized.read().await;
        if !initialized {
            return Err(Layer2Error::Internal(
                "Layer2Manager not initialized".to_string(),
            ));
        }
        Ok(())
    }
}

impl Default for Layer2Manager {
    fn default() -> Self {
        Self::new()
    }
}

/// Comprehensive status report for Layer2Manager
#[derive(Debug, Clone)]
pub struct Layer2StatusReport {
    pub initialized: bool,
    pub enabled_protocols: Vec<Layer2ProtocolType>,
    pub protocol_health: HashMap<Layer2ProtocolType, ProtocolHealth>,
    pub total_protocols: usize,
    pub healthy_protocols: usize,
    pub error_count: u32,
    pub uptime_seconds: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_layer2_manager_initialization() {
        let manager = Layer2Manager::new();
        assert!(manager.initialize().await.is_ok());

        let status = manager.get_status_report().await.unwrap();
        assert!(status.initialized);
        assert!(status.total_protocols > 0);
    }

    #[tokio::test]
    async fn test_layer2_manager_protocol_operations() {
        let manager = Layer2Manager::new();
        manager.initialize().await.unwrap();

        // Test Lightning Network operations
        let tx_data = b"test_lightning_payment";
        let tx_id = manager
            .submit_transaction(Layer2ProtocolType::Lightning, tx_data)
            .await
            .unwrap();

        assert!(!tx_id.is_empty());

        let status = manager
            .check_transaction_status(Layer2ProtocolType::Lightning, &tx_id)
            .await
            .unwrap();

        // Status should be either Pending or Confirmed depending on mock implementation
        assert!(matches!(
            status,
            TransactionStatus::Pending | TransactionStatus::Confirmed
        ));
    }

    #[tokio::test]
    async fn test_layer2_manager_health_monitoring() {
        let manager = Layer2Manager::new();
        manager.initialize().await.unwrap();

        let health_status = manager.get_all_health_status().await.unwrap();
        assert!(!health_status.is_empty());

        // Check specific protocol health
        let lightning_health = manager
            .get_protocol_health(Layer2ProtocolType::Lightning)
            .await
            .unwrap();

        assert!(lightning_health.last_check > 0);
    }

    #[tokio::test]
    async fn test_layer2_manager_lifecycle() {
        let manager = Layer2Manager::new();

        // Initialize
        assert!(manager.initialize().await.is_ok());

        // Get status
        let status = manager.get_status_report().await.unwrap();
        assert!(status.initialized);

        // Shutdown
        assert!(manager.shutdown().await.is_ok());

        // Status after shutdown
        let status = manager.get_status_report().await.unwrap();
        assert!(!status.initialized);
    }
}
