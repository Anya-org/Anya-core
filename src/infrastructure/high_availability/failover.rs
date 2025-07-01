use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::Instant;
use tracing::{debug, info, instrument, warn};

use crate::infrastructure::high_availability::{
    config::HighAvailabilityConfig, FailoverPhase, HaError,
};

/// Failover manager implementing automatic failover patterns
/// Follows the Leader and Followers pattern from distributed systems
/// [AIR-3][AIS-3][RES-3]
#[derive(Debug)]
pub struct FailoverManager {
    config: Arc<HighAvailabilityConfig>,
    current_phase: Arc<RwLock<FailoverPhase>>,
    failover_history: Arc<RwLock<Vec<FailoverEvent>>>,
    enabled: Arc<RwLock<bool>>,
    last_failover_attempt: Arc<RwLock<Option<Instant>>>,
}

/// Represents a failover event in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverEvent {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub trigger_reason: String,
    pub source_node: Option<String>,
    pub target_node: Option<String>,
    pub phase: FailoverPhase,
    pub duration_ms: Option<u64>,
    pub success: bool,
    pub error: Option<String>,
}

/// Failover triggers
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FailoverTrigger {
    /// Node failure detected
    NodeFailure(String),
    /// Health check failure
    HealthCheckFailure(String),
    /// Manual failover requested
    Manual,
    /// Performance degradation
    PerformanceDegradation(String),
    /// Network partition
    NetworkPartition,
}

impl FailoverManager {
    /// Creates a new failover manager
    pub fn new(config: &HighAvailabilityConfig) -> Self {
        Self {
            config: Arc::new(config.clone()),
            current_phase: Arc::new(RwLock::new(FailoverPhase::Completed)),
            failover_history: Arc::new(RwLock::new(Vec::new())),
            enabled: Arc::new(RwLock::new(config.failover.enabled)),
            last_failover_attempt: Arc::new(RwLock::new(None)),
        }
    }

    /// Initializes the failover manager
    #[instrument(skip(self))]
    pub async fn initialize(&mut self) -> Result<(), HaError> {
        info!("Initializing failover manager");

        if !self.config.failover.enabled {
            warn!("Failover is disabled in configuration");
            *self.enabled.write().await = false;
            return Ok(());
        }

        *self.enabled.write().await = true;
        *self.current_phase.write().await = FailoverPhase::Completed;

        info!("Failover manager initialized");
        Ok(())
    }

    /// Enables the failover manager
    #[instrument(skip(self))]
    pub async fn enable(&mut self) -> Result<(), HaError> {
        info!("Enabling failover manager");
        *self.enabled.write().await = true;
        Ok(())
    }

    /// Disables the failover manager
    #[instrument(skip(self))]
    pub async fn disable(&mut self) -> Result<(), HaError> {
        info!("Disabling failover manager");
        *self.enabled.write().await = false;
        Ok(())
    }

    /// Triggers a manual failover
    #[instrument(skip(self))]
    pub async fn trigger_manual_failover(&mut self) -> Result<(), HaError> {
        self.trigger_failover(FailoverTrigger::Manual, None, None)
            .await
    }

    /// Triggers failover for a specific reason
    #[instrument(skip(self))]
    pub async fn trigger_failover(
        &mut self,
        trigger: FailoverTrigger,
        source_node: Option<String>,
        target_node: Option<String>,
    ) -> Result<(), HaError> {
        if !*self.enabled.read().await {
            warn!("Failover is disabled, ignoring trigger: {:?}", trigger);
            return Ok(());
        }

        let current_phase = *self.current_phase.read().await;
        if current_phase != FailoverPhase::Completed {
            warn!(
                "Failover already in progress (phase: {:?}), ignoring new trigger: {:?}",
                current_phase, trigger
            );
            return Err(HaError::FailoverError(
                "Failover already in progress".to_string(),
            ));
        }

        // Check rate limiting
        if let Some(last_attempt) = *self.last_failover_attempt.read().await {
            let elapsed = last_attempt.elapsed();
            let min_interval = Duration::from_secs(30); // Minimum 30 seconds between failovers

            if elapsed < min_interval {
                warn!("Failover rate limited, last attempt was {:?} ago", elapsed);
                return Err(HaError::FailoverError("Failover rate limited".to_string()));
            }
        }

        info!("Triggering failover: {:?}", trigger);
        *self.last_failover_attempt.write().await = Some(Instant::now());

        let event_id = uuid::Uuid::new_v4().to_string();
        let start_time = Instant::now();

        // Execute failover phases
        let result = self
            .execute_failover_sequence(
                event_id.clone(),
                trigger.clone(),
                source_node.clone(),
                target_node.clone(),
                start_time,
            )
            .await;

        // Record the event
        let duration = start_time.elapsed().as_millis() as u64;
        let (success, error) = match &result {
            Ok(_) => (true, None),
            Err(e) => (false, Some(e.to_string())),
        };

        let event = FailoverEvent {
            id: event_id,
            timestamp: chrono::Utc::now(),
            trigger_reason: format!("{trigger:?}"),
            source_node,
            target_node,
            phase: *self.current_phase.read().await,
            duration_ms: Some(duration),
            success,
            error,
        };

        self.failover_history.write().await.push(event);

        // Ensure we're back to completed state
        *self.current_phase.write().await = FailoverPhase::Completed;

        result
    }

    /// Executes the complete failover sequence
    async fn execute_failover_sequence(
        &mut self,
        event_id: String,
        trigger: FailoverTrigger,
        source_node: Option<String>,
        target_node: Option<String>,
        start_time: Instant,
    ) -> Result<(), HaError> {
        // Phase 1: Detection
        *self.current_phase.write().await = FailoverPhase::Detection;
        info!("Failover {}: Detection phase", event_id);
        self.detect_failure(&trigger).await?;

        // Phase 2: Election
        *self.current_phase.write().await = FailoverPhase::Election;
        info!("Failover {}: Election phase", event_id);
        let new_leader = self
            .elect_new_leader(source_node.as_deref(), target_node.as_deref())
            .await?;

        // Phase 3: Promotion
        *self.current_phase.write().await = FailoverPhase::Promotion;
        info!(
            "Failover {}: Promotion phase - promoting {}",
            event_id, new_leader
        );
        self.promote_new_leader(&new_leader).await?;

        // Phase 4: Redirection
        *self.current_phase.write().await = FailoverPhase::Redirection;
        info!("Failover {}: Redirection phase", event_id);
        self.redirect_traffic(&new_leader).await?;

        // Phase 5: Recovery (optional, for failed node)
        if let Some(failed_node) = &source_node {
            *self.current_phase.write().await = FailoverPhase::Recovery;
            info!("Failover {}: Recovery phase for {}", event_id, failed_node);
            self.initiate_recovery(failed_node).await?;
        }

        info!(
            "Failover {} completed successfully in {}ms",
            event_id,
            start_time.elapsed().as_millis()
        );

        Ok(())
    }

    /// Detects and validates the failure
    async fn detect_failure(&self, trigger: &FailoverTrigger) -> Result<(), HaError> {
        debug!("Detecting failure: {:?}", trigger);

        match trigger {
            FailoverTrigger::NodeFailure(node) => {
                // Verify the node is actually down
                if !self.verify_node_failure(node).await? {
                    return Err(HaError::FailoverError(format!(
                        "Node {node} appears to be healthy"
                    )));
                }
            }
            FailoverTrigger::HealthCheckFailure(component) => {
                // Verify health check failure is critical
                if !self.verify_health_failure(component).await? {
                    return Err(HaError::FailoverError(format!(
                        "Component {component} health check not critical"
                    )));
                }
            }
            FailoverTrigger::Manual => {
                // Manual failover always proceeds
                info!("Manual failover requested");
            }
            FailoverTrigger::PerformanceDegradation(reason) => {
                debug!("Performance degradation detected: {}", reason);
            }
            FailoverTrigger::NetworkPartition => {
                debug!("Network partition detected");
            }
        }

        Ok(())
    }

    /// Elects a new leader node
    async fn elect_new_leader(
        &self,
        failed_node: Option<&str>,
        preferred_node: Option<&str>,
    ) -> Result<String, HaError> {
        debug!("Electing new leader");

        // If a preferred node is specified, use it
        if let Some(node) = preferred_node {
            info!("Using preferred node as new leader: {}", node);
            return Ok(node.to_string());
        }

        // In a real implementation, this would:
        // 1. Query available nodes
        // 2. Check their health and eligibility
        // 3. Apply election algorithm (e.g., highest priority, least loaded)
        // 4. Ensure consensus among nodes

        // For now, simulate election
        let available_nodes = self.get_available_nodes(failed_node).await?;

        if available_nodes.is_empty() {
            return Err(HaError::FailoverError(
                "No available nodes for promotion".to_string(),
            ));
        }

        // Select the first available node (in real implementation, use proper election logic)
        let new_leader = available_nodes[0].clone();
        info!("Elected new leader: {}", new_leader);

        Ok(new_leader)
    }

    /// Promotes a node to leader
    async fn promote_new_leader(&self, node: &str) -> Result<(), HaError> {
        debug!("Promoting {} to leader", node);

        // In a real implementation, this would:
        // 1. Update the node's role to leader
        // 2. Initialize leader-specific services
        // 3. Update cluster metadata
        // 4. Notify other nodes

        tokio::time::sleep(Duration::from_millis(100)).await; // Simulate promotion time

        info!("Successfully promoted {} to leader", node);
        Ok(())
    }

    /// Redirects traffic to the new leader
    async fn redirect_traffic(&self, new_leader: &str) -> Result<(), HaError> {
        debug!("Redirecting traffic to {}", new_leader);

        // In a real implementation, this would:
        // 1. Update load balancer configuration
        // 2. Update DNS records
        // 3. Notify clients
        // 4. Update service discovery

        tokio::time::sleep(Duration::from_millis(50)).await; // Simulate redirection time

        info!("Successfully redirected traffic to {}", new_leader);
        Ok(())
    }

    /// Initiates recovery for a failed node
    async fn initiate_recovery(&self, failed_node: &str) -> Result<(), HaError> {
        debug!("Initiating recovery for {}", failed_node);

        // In a real implementation, this would:
        // 1. Try to restart the node
        // 2. Check if it can rejoin the cluster
        // 3. Sync any missed data
        // 4. Update its role appropriately

        info!("Recovery initiated for {}", failed_node);
        Ok(())
    }

    /// Verifies that a node has actually failed
    async fn verify_node_failure(&self, node: &str) -> Result<bool, HaError> {
        debug!("Verifying failure of node: {}", node);

        // In a real implementation, this would:
        // 1. Try to ping the node
        // 2. Check heartbeat timestamps
        // 3. Verify with other nodes
        // 4. Check network connectivity

        // For simulation, assume the node is indeed failed
        Ok(true)
    }

    /// Verifies that a health check failure is critical
    async fn verify_health_failure(&self, component: &str) -> Result<bool, HaError> {
        debug!("Verifying health failure of component: {}", component);

        // In a real implementation, this would:
        // 1. Check the severity of the health failure
        // 2. Verify with multiple health checks
        // 3. Check if it affects critical functionality

        // For simulation, assume it's critical
        Ok(true)
    }

    /// Gets list of available nodes for promotion
    async fn get_available_nodes(
        &self,
        exclude_node: Option<&str>,
    ) -> Result<Vec<String>, HaError> {
        // In a real implementation, this would query the cluster manager
        let mut nodes = vec![
            "node-1".to_string(),
            "node-2".to_string(),
            "node-3".to_string(),
        ];

        // Remove the failed node
        if let Some(failed) = exclude_node {
            nodes.retain(|n| n != failed);
        }

        Ok(nodes)
    }

    /// Updates the failover manager configuration
    #[instrument(skip(self, config))]
    pub async fn update_config(&mut self, config: &HighAvailabilityConfig) -> Result<(), HaError> {
        info!("Updating failover manager configuration");
        self.config = Arc::new(config.clone());
        *self.enabled.write().await = config.failover.enabled;
        Ok(())
    }

    /// Gets the current failover phase
    pub async fn get_current_phase(&self) -> FailoverPhase {
        *self.current_phase.read().await
    }

    /// Gets the failover history
    pub async fn get_failover_history(&self) -> Vec<FailoverEvent> {
        self.failover_history.read().await.clone()
    }

    /// Checks if failover is currently active
    pub async fn is_failover_active(&self) -> bool {
        *self.current_phase.read().await != FailoverPhase::Completed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::high_availability::config::FailoverConfig;

    fn create_test_config() -> HighAvailabilityConfig {
        HighAvailabilityConfig {
            failover: FailoverConfig {
                enabled: true,
                auto_failover: true,
                failover_timeout: Duration::from_secs(30),
                min_nodes_for_failover: 2,
                max_auto_failovers: Some(3),
                auto_failover_period: Duration::from_secs(3600),
                fencing_enabled: true,
            },
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_failover_manager_creation() {
        let config = create_test_config();
        let manager = FailoverManager::new(&config);

        assert!(!manager.is_failover_active().await);
        assert_eq!(manager.get_current_phase().await, FailoverPhase::Completed);
    }

    #[tokio::test]
    async fn test_enable_disable() {
        let config = create_test_config();
        let mut manager = FailoverManager::new(&config);

        manager.initialize().await.unwrap();
        assert!(*manager.enabled.read().await);

        manager.disable().await.unwrap();
        assert!(!*manager.enabled.read().await);

        manager.enable().await.unwrap();
        assert!(*manager.enabled.read().await);
    }

    #[tokio::test]
    async fn test_manual_failover() {
        let config = create_test_config();
        let mut manager = FailoverManager::new(&config);

        manager.initialize().await.unwrap();

        let result = manager.trigger_manual_failover().await;
        assert!(result.is_ok());

        let history = manager.get_failover_history().await;
        assert_eq!(history.len(), 1);
        assert!(history[0].success);
    }
}
