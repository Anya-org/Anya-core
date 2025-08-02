// [AIR-3][AIS-3][AIM-3][BPC-3][RES-3]
//! Async Layer2 Protocol Coordinator
//!
//! This module provides centralized coordination for all Layer2 Bitcoin protocols,
//! managing async operations, event handling, and cross-protocol communication.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, RwLock, Semaphore};
use tokio::time::timeout;
use tracing::{error, info, warn};

use crate::layer2::{
    Layer2Error, Layer2Protocol, Layer2ProtocolType, ProtocolHealth, ProtocolState,
    TransactionStatus,
};

/// Maximum number of concurrent operations per protocol
const MAX_CONCURRENT_OPS: usize = 100;
/// Default timeout for protocol operations
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
/// Health check interval
const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(60);

/// Events emitted by Layer2 protocols
#[derive(Debug, Clone)]
pub enum Layer2Event {
    /// Protocol connected successfully
    ProtocolConnected {
        protocol: Layer2ProtocolType,
        timestamp: Instant,
    },
    /// Protocol disconnected
    ProtocolDisconnected {
        protocol: Layer2ProtocolType,
        reason: String,
        timestamp: Instant,
    },
    /// Transaction submitted
    TransactionSubmitted {
        protocol: Layer2ProtocolType,
        tx_id: String,
        timestamp: Instant,
    },
    /// Transaction status changed
    TransactionStatusChanged {
        protocol: Layer2ProtocolType,
        tx_id: String,
        old_status: TransactionStatus,
        new_status: TransactionStatus,
        timestamp: Instant,
    },
    /// Protocol health changed
    HealthChanged {
        protocol: Layer2ProtocolType,
        health: ProtocolHealth,
        timestamp: Instant,
    },
    /// Error occurred
    Error {
        protocol: Layer2ProtocolType,
        error: Layer2Error,
        timestamp: Instant,
    },
}

/// Protocol wrapper with async coordination
pub struct AsyncProtocolWrapper {
    protocol: Arc<dyn Layer2Protocol>,
    protocol_type: Layer2ProtocolType,
    semaphore: Arc<Semaphore>,
    health: Arc<RwLock<ProtocolHealth>>,
    last_activity: Arc<RwLock<Instant>>,
}

impl AsyncProtocolWrapper {
    pub fn new(protocol: Arc<dyn Layer2Protocol>, protocol_type: Layer2ProtocolType) -> Self {
        Self {
            protocol,
            protocol_type,
            semaphore: Arc::new(Semaphore::new(MAX_CONCURRENT_OPS)),
            health: Arc::new(RwLock::new(ProtocolHealth {
                healthy: false,
                last_check: 0,
                error_count: 0,
                uptime_seconds: 0,
            })),
            last_activity: Arc::new(RwLock::new(Instant::now())),
        }
    }

    /// Execute an async operation with timeout and concurrency control
    pub async fn execute_with_semaphore<F, T>(&self, operation: F) -> Result<T, Layer2Error>
    where
        F: std::future::Future<Output = Result<T, Layer2Error>>,
    {
        // Acquire semaphore permit
        let _permit =
            self.semaphore.acquire().await.map_err(|e| {
                Layer2Error::Internal(format!("Failed to acquire semaphore: {}", e))
            })?;

        // Update last activity
        {
            let mut last_activity = self.last_activity.write().await;
            *last_activity = Instant::now();
        }

        // Execute with timeout
        match timeout(DEFAULT_TIMEOUT, operation).await {
            Ok(result) => {
                // Update health on success
                if result.is_ok() {
                    let mut health = self.health.write().await;
                    health.last_check = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                } else {
                    // Increment error count on failure
                    let mut health = self.health.write().await;
                    health.error_count += 1;
                }
                result
            }
            Err(_) => {
                // Update health on timeout
                let mut health = self.health.write().await;
                health.error_count += 1;
                Err(Layer2Error::Internal("Operation timed out".to_string()))
            }
        }
    }

    /// Check protocol health
    pub async fn check_health(&self) -> Result<ProtocolHealth, Layer2Error> {
        let health_result = self
            .execute_with_semaphore(self.protocol.health_check())
            .await;

        let mut health = self.health.write().await;
        match health_result {
            Ok(new_health) => {
                *health = new_health.clone();
                Ok(new_health)
            }
            Err(e) => {
                health.healthy = false;
                health.error_count += 1;
                health.last_check = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                Err(e)
            }
        }
    }
}

/// Central coordinator for all Layer2 protocols
pub struct AsyncLayer2Coordinator {
    protocols: Arc<RwLock<HashMap<Layer2ProtocolType, AsyncProtocolWrapper>>>,
    event_sender: mpsc::UnboundedSender<Layer2Event>,
    event_receiver: Arc<RwLock<Option<mpsc::UnboundedReceiver<Layer2Event>>>>,
    health_check_handle: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl AsyncLayer2Coordinator {
    /// Create a new async coordinator
    pub fn new() -> Self {
        let (event_sender, event_receiver) = mpsc::unbounded_channel();

        Self {
            protocols: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
            event_receiver: Arc::new(RwLock::new(Some(event_receiver))),
            health_check_handle: Arc::new(RwLock::new(None)),
        }
    }

    /// Register a protocol with the coordinator
    pub async fn register_protocol(
        &self,
        protocol_type: Layer2ProtocolType,
        protocol: Arc<dyn Layer2Protocol>,
    ) -> Result<(), Layer2Error> {
        let wrapper = AsyncProtocolWrapper::new(protocol, protocol_type);

        // Initialize the protocol
        wrapper
            .execute_with_semaphore(wrapper.protocol.initialize())
            .await?;

        // Connect the protocol
        wrapper
            .execute_with_semaphore(wrapper.protocol.connect())
            .await?;

        // Register in the coordinator
        {
            let mut protocols = self.protocols.write().await;
            protocols.insert(protocol_type, wrapper);
        }

        // Send connection event
        self.emit_event(Layer2Event::ProtocolConnected {
            protocol: protocol_type,
            timestamp: Instant::now(),
        });

        info!("Registered and connected protocol: {:?}", protocol_type);
        Ok(())
    }

    /// Unregister a protocol
    pub async fn unregister_protocol(
        &self,
        protocol_type: Layer2ProtocolType,
    ) -> Result<(), Layer2Error> {
        let wrapper = {
            let mut protocols = self.protocols.write().await;
            protocols.remove(&protocol_type)
        };

        if let Some(wrapper) = wrapper {
            // Disconnect the protocol
            let _ = wrapper
                .execute_with_semaphore(wrapper.protocol.disconnect())
                .await;

            // Send disconnection event
            self.emit_event(Layer2Event::ProtocolDisconnected {
                protocol: protocol_type,
                reason: "Manual unregistration".to_string(),
                timestamp: Instant::now(),
            });

            info!("Unregistered protocol: {:?}", protocol_type);
        }

        Ok(())
    }

    /// Submit transaction to a specific protocol
    pub async fn submit_transaction(
        &self,
        protocol_type: Layer2ProtocolType,
        tx_data: &[u8],
    ) -> Result<String, Layer2Error> {
        let protocols = self.protocols.read().await;
        let wrapper = protocols
            .get(&protocol_type)
            .ok_or_else(|| Layer2Error::Protocol("Protocol not registered".to_string()))?;

        let result = wrapper
            .execute_with_semaphore(wrapper.protocol.submit_transaction(tx_data))
            .await;

        match &result {
            Ok(tx_id) => {
                self.emit_event(Layer2Event::TransactionSubmitted {
                    protocol: protocol_type,
                    tx_id: tx_id.clone(),
                    timestamp: Instant::now(),
                });
                info!("Transaction submitted to {:?}: {}", protocol_type, tx_id);
            }
            Err(e) => {
                self.emit_event(Layer2Event::Error {
                    protocol: protocol_type,
                    error: e.clone(),
                    timestamp: Instant::now(),
                });
                error!("Failed to submit transaction to {:?}: {}", protocol_type, e);
            }
        }

        result
    }

    /// Check transaction status across protocols
    pub async fn check_transaction_status(
        &self,
        protocol_type: Layer2ProtocolType,
        tx_id: &str,
    ) -> Result<TransactionStatus, Layer2Error> {
        let protocols = self.protocols.read().await;
        let wrapper = protocols
            .get(&protocol_type)
            .ok_or_else(|| Layer2Error::Protocol("Protocol not registered".to_string()))?;

        wrapper
            .execute_with_semaphore(wrapper.protocol.check_transaction_status(tx_id))
            .await
    }

    /// Get protocol state
    pub async fn get_protocol_state(
        &self,
        protocol_type: Layer2ProtocolType,
    ) -> Result<ProtocolState, Layer2Error> {
        let protocols = self.protocols.read().await;
        let wrapper = protocols
            .get(&protocol_type)
            .ok_or_else(|| Layer2Error::Protocol("Protocol not registered".to_string()))?;

        wrapper
            .execute_with_semaphore(wrapper.protocol.get_state())
            .await
    }

    /// Get health status for all protocols
    pub async fn get_all_health_status(&self) -> HashMap<Layer2ProtocolType, ProtocolHealth> {
        let protocols = self.protocols.read().await;
        let mut health_map = HashMap::new();

        for (protocol_type, wrapper) in protocols.iter() {
            match wrapper.check_health().await {
                Ok(health) => {
                    health_map.insert(*protocol_type, health);
                }
                Err(e) => {
                    warn!("Failed to get health for {:?}: {}", protocol_type, e);
                    health_map.insert(
                        *protocol_type,
                        ProtocolHealth {
                            healthy: false,
                            last_check: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs(),
                            error_count: 1,
                            uptime_seconds: 0,
                        },
                    );
                }
            }
        }

        health_map
    }

    /// Start background health monitoring
    pub async fn start_health_monitoring(&self) {
        let protocols = Arc::clone(&self.protocols);
        let event_sender = self.event_sender.clone();

        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(HEALTH_CHECK_INTERVAL);

            loop {
                interval.tick().await;

                let protocols_read = protocols.read().await;
                for (protocol_type, wrapper) in protocols_read.iter() {
                    match wrapper.check_health().await {
                        Ok(health) => {
                            let _ = event_sender.send(Layer2Event::HealthChanged {
                                protocol: *protocol_type,
                                health,
                                timestamp: Instant::now(),
                            });
                        }
                        Err(e) => {
                            let _ = event_sender.send(Layer2Event::Error {
                                protocol: *protocol_type,
                                error: e,
                                timestamp: Instant::now(),
                            });
                        }
                    }
                }
            }
        });

        let mut health_check_handle = self.health_check_handle.write().await;
        *health_check_handle = Some(handle);
    }

    /// Stop health monitoring
    pub async fn stop_health_monitoring(&self) {
        let mut handle = self.health_check_handle.write().await;
        if let Some(handle) = handle.take() {
            handle.abort();
        }
    }

    /// Get event receiver for listening to coordinator events
    pub async fn take_event_receiver(&self) -> Option<mpsc::UnboundedReceiver<Layer2Event>> {
        let mut receiver = self.event_receiver.write().await;
        receiver.take()
    }

    /// Emit an event
    fn emit_event(&self, event: Layer2Event) {
        let _ = self.event_sender.send(event);
    }

    /// Graceful shutdown of all protocols
    pub async fn shutdown(&self) -> Result<(), Layer2Error> {
        info!("Shutting down Layer2 coordinator...");

        // Stop health monitoring
        self.stop_health_monitoring().await;

        // Disconnect all protocols
        let protocol_types: Vec<Layer2ProtocolType> = {
            let protocols = self.protocols.read().await;
            protocols.keys().copied().collect()
        };

        for protocol_type in protocol_types {
            if let Err(e) = self.unregister_protocol(protocol_type).await {
                warn!("Error during protocol shutdown {:?}: {}", protocol_type, e);
            }
        }

        info!("Layer2 coordinator shutdown complete");
        Ok(())
    }
}

impl Default for AsyncLayer2Coordinator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layer2::mock::MockLayer2Protocol;

    #[tokio::test]
    async fn test_coordinator_lifecycle() {
        let coordinator = AsyncLayer2Coordinator::new();

        // Register a mock protocol
        let mock_protocol = Arc::new(MockLayer2Protocol::new());
        coordinator
            .register_protocol(Layer2ProtocolType::Lightning, mock_protocol)
            .await
            .unwrap();

        // Check protocol is registered
        let health_status = coordinator.get_all_health_status().await;
        assert!(health_status.contains_key(&Layer2ProtocolType::Lightning));

        // Unregister protocol
        coordinator
            .unregister_protocol(Layer2ProtocolType::Lightning)
            .await
            .unwrap();

        // Check protocol is unregistered
        let health_status = coordinator.get_all_health_status().await;
        assert!(!health_status.contains_key(&Layer2ProtocolType::Lightning));
    }

    #[tokio::test]
    async fn test_transaction_operations() {
        let coordinator = AsyncLayer2Coordinator::new();
        let mock_protocol = Arc::new(MockLayer2Protocol::new());

        coordinator
            .register_protocol(Layer2ProtocolType::Lightning, mock_protocol)
            .await
            .unwrap();

        // Submit transaction
        let tx_data = b"test_transaction";
        let tx_id = coordinator
            .submit_transaction(Layer2ProtocolType::Lightning, tx_data)
            .await
            .unwrap();

        assert!(!tx_id.is_empty());

        // Check transaction status
        let status = coordinator
            .check_transaction_status(Layer2ProtocolType::Lightning, &tx_id)
            .await
            .unwrap();

        assert_eq!(status, TransactionStatus::Pending);
    }
}
