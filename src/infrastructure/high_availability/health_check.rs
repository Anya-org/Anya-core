use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::Instant;
use tracing::{debug, error, info, instrument, warn};
use serde::{Deserialize, Serialize};

use crate::infrastructure::high_availability::{
    HealthStatus, HealthState, ComponentHealth, HaError,
    config::HighAvailabilityConfig,
};

/// Health checker for monitoring system components
/// Implements the Health Check API pattern from distributed systems
/// [AIR-3][AIS-3][RES-3]
#[derive(Debug)]
pub struct HealthChecker {
    config: Arc<HighAvailabilityConfig>,
    component_states: Arc<RwLock<HashMap<String, ComponentHealth>>>,
    last_check: Arc<RwLock<Option<Instant>>>,
    monitoring_active: Arc<RwLock<bool>>,
}

/// Health check result for a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub component: String,
    pub status: HealthState,
    pub details: Option<String>,
    pub latency_ms: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl HealthChecker {
    /// Creates a new health checker with the given configuration
    pub fn new(config: &HighAvailabilityConfig) -> Self {
        Self {
            config: Arc::new(config.clone()),
            component_states: Arc::new(RwLock::new(HashMap::new())),
            last_check: Arc::new(RwLock::new(None)),
            monitoring_active: Arc::new(RwLock::new(false)),
        }
    }

    /// Starts continuous health monitoring
    #[instrument(skip(self))]
    pub async fn start_monitoring(&mut self) -> Result<(), HaError> {
        info!("Starting health monitoring");

        let mut monitoring_active = self.monitoring_active.write().await;
        if *monitoring_active {
            warn!("Health monitoring already active");
            return Ok(());
        }
        *monitoring_active = true;

        // Initialize component states
        let mut states = self.component_states.write().await;
        for component in &self.config.health_check.components {
            states.insert(component.clone(), ComponentHealth {
                name: component.clone(),
                status: HealthState::Unknown,
                details: Some("Initializing".to_string()),
                last_check: chrono::Utc::now(),
            });
        }
        drop(states);

        // Start monitoring loop
        let config = self.config.clone();
        let component_states = self.component_states.clone();
        let last_check = self.last_check.clone();
        let monitoring_active = self.monitoring_active.clone();

        tokio::spawn(async move {
            Self::monitoring_loop(config, component_states, last_check, monitoring_active).await;
        });

        info!("Health monitoring started");
        Ok(())
    }

    /// Stops health monitoring
    #[instrument(skip(self))]
    pub async fn stop_monitoring(&mut self) -> Result<(), HaError> {
        info!("Stopping health monitoring");

        let mut monitoring_active = self.monitoring_active.write().await;
        *monitoring_active = false;

        info!("Health monitoring stopped");
        Ok(())
    }

    /// Gets the current health status
    #[instrument(skip(self))]
    pub async fn get_status(&self) -> Result<HealthStatus, HaError> {
        let components = self.component_states.read().await;
        let last_check = self.last_check.read().await;

        // Determine overall status
        let overall_status = if components.is_empty() {
            HealthState::Unknown
        } else {
            let has_critical = components.values().any(|c| c.status == HealthState::Critical);
            let has_degraded = components.values().any(|c| c.status == HealthState::Degraded);
            
            if has_critical {
                HealthState::Critical
            } else if has_degraded {
                HealthState::Degraded
            } else {
                HealthState::Healthy
            }
        };

        Ok(HealthStatus {
            status: overall_status,
            components: components.clone(),
            last_check: instant_to_datetime(last_check.unwrap_or(Instant::now())),
            message: Some(format!("Checked {} components", components.len())),
        })
    }

    /// Updates the health checker configuration
    #[instrument(skip(self, config))]
    pub async fn update_config(&mut self, config: &HighAvailabilityConfig) -> Result<(), HaError> {
        info!("Updating health checker configuration");
        self.config = Arc::new(config.clone());
        Ok(())
    }

    /// Performs a health check on a specific component
    #[instrument(skip(self))]
    pub async fn check_component(&self, component: &str) -> Result<HealthCheckResult, HaError> {
        let start_time = Instant::now();
        
        debug!("Checking health of component: {}", component);

        // Simulate component-specific health checks
        let (status, details) = match component {
            "cluster" => self.check_cluster_health().await,
            "storage" => self.check_storage_health().await,
            "network" => self.check_network_health().await,
            "api" => self.check_api_health().await,
            _ => {
                warn!("Unknown component: {}", component);
                (HealthState::Unknown, Some("Unknown component".to_string()))
            }
        };

        let latency = start_time.elapsed().as_millis() as u64;

        let result = HealthCheckResult {
            component: component.to_string(),
            status,
            details,
            latency_ms: latency,
            timestamp: chrono::Utc::now(),
        };

        // Update component state
        let mut states = self.component_states.write().await;
        states.insert(component.to_string(), ComponentHealth {
            name: component.to_string(),
            status,
            details: result.details.clone(),
            last_check: result.timestamp,
        });

        debug!("Health check completed for {}: {:?} ({}ms)", component, status, latency);
        Ok(result)
    }

    /// Main monitoring loop
    async fn monitoring_loop(
        config: Arc<HighAvailabilityConfig>,
        component_states: Arc<RwLock<HashMap<String, ComponentHealth>>>,
        last_check: Arc<RwLock<Option<Instant>>>,
        monitoring_active: Arc<RwLock<bool>>,
    ) {
        let mut interval = tokio::time::interval(config.health_check.check_interval);
        
        while *monitoring_active.read().await {
            interval.tick().await;

            debug!("Running health check cycle");
            *last_check.write().await = Some(Instant::now());

            // Check each component
            for component in &config.health_check.components {
                let checker = HealthChecker {
                    config: config.clone(),
                    component_states: component_states.clone(),
                    last_check: last_check.clone(),
                    monitoring_active: monitoring_active.clone(),
                };

                if let Err(e) = checker.check_component(component).await {
                    error!("Failed to check component {}: {}", component, e);
                }
            }
        }

        info!("Health monitoring loop ended");
    }

    /// Check cluster health
    async fn check_cluster_health(&self) -> (HealthState, Option<String>) {
        // In a real implementation, this would check:
        // - Node connectivity
        // - Leader election status
        // - Quorum availability
        // - Network partitions
        
        (HealthState::Healthy, Some("Cluster operational".to_string()))
    }

    /// Check storage health
    async fn check_storage_health(&self) -> (HealthState, Option<String>) {
        // In a real implementation, this would check:
        // - Disk space
        // - I/O latency
        // - Data consistency
        // - Backup status
        
        (HealthState::Healthy, Some("Storage operational".to_string()))
    }

    /// Check network health
    async fn check_network_health(&self) -> (HealthState, Option<String>) {
        // In a real implementation, this would check:
        // - Network connectivity
        // - Latency between nodes
        // - Bandwidth utilization
        // - Packet loss
        
        (HealthState::Healthy, Some("Network operational".to_string()))
    }

    /// Check API health
    async fn check_api_health(&self) -> (HealthState, Option<String>) {
        // In a real implementation, this would check:
        // - HTTP endpoint availability
        // - Response times
        // - Error rates
        // - Authentication services
        
        (HealthState::Healthy, Some("API operational".to_string()))
    }
}

// Helper function to convert Instant to DateTime - avoids orphan rule violation
fn instant_to_datetime(_instant: Instant) -> chrono::DateTime<chrono::Utc> {
    // Convert Instant to DateTime - this is an approximation
    // In a real implementation, you'd want to track creation time
    chrono::Utc::now()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::high_availability::config::HealthCheckConfig;

    fn create_test_config() -> HighAvailabilityConfig {
        HighAvailabilityConfig {
            health_check: HealthCheckConfig {
                enabled: true,
                check_interval: Duration::from_millis(100),
                warning_threshold: 2,
                critical_threshold: 3,
                check_timeout: Duration::from_secs(1),
                components: vec!["cluster".to_string(), "storage".to_string()],
            },
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_health_checker_creation() {
        let config = create_test_config();
        let checker = HealthChecker::new(&config);
        
        assert!(!*checker.monitoring_active.read().await);
    }

    #[tokio::test]
    async fn test_component_health_check() {
        let config = create_test_config();
        let checker = HealthChecker::new(&config);
        
        let result = checker.check_component("cluster").await.unwrap();
        assert_eq!(result.component, "cluster");
        assert_eq!(result.status, HealthState::Healthy);
    }

    #[tokio::test]
    async fn test_start_stop_monitoring() {
        let config = create_test_config();
        let mut checker = HealthChecker::new(&config);
        
        checker.start_monitoring().await.unwrap();
        assert!(*checker.monitoring_active.read().await);
        
        checker.stop_monitoring().await.unwrap();
        assert!(!*checker.monitoring_active.read().await);
    }
}
