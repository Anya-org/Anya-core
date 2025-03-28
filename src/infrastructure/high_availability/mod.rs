#![feature(edition2021)]
pub mod cluster;
pub mod failover;
pub mod health_check;
pub mod replication;
pub mod load_balancing;

mod config;

pub use config::HighAvailabilityConfig;
pub use health_check::HealthChecker;
pub use failover::FailoverManager;
pub use cluster::ClusterManager;
pub use replication::ReplicationManager;
pub use load_balancing::LoadBalancer;

/// High Availability module for Anya Core
/// [AIR-3][AIS-3][AIT-3][PFM-3][SCL-3][RES-3]
///
/// Provides resilient infrastructure capabilities including:
/// - Cluster management
/// - Automatic failover
/// - Health monitoring
/// - Data replication
/// - Load balancing
///
/// This module coordinates all high availability features to ensure
/// system reliability and fault tolerance.
pub struct HighAvailabilityManager {
    config: HighAvailabilityConfig,
    cluster_manager: ClusterManager,
    failover_manager: FailoverManager,
    health_checker: HealthChecker,
    replication_manager: ReplicationManager,
    load_balancer: LoadBalancer,
}

impl HighAvailabilityManager {
    /// Creates a new HighAvailabilityManager with the specified configuration
    pub fn new(config: HighAvailabilityConfig) -> Self {
        let cluster_manager = ClusterManager::new(&config);
        let failover_manager = FailoverManager::new(&config);
        let health_checker = HealthChecker::new(&config);
        let replication_manager = ReplicationManager::new(&config);
        let load_balancer = LoadBalancer::new(&config);

        Self {
            config,
            cluster_manager,
            failover_manager,
            health_checker,
            replication_manager,
            load_balancer,
        }
    }

    /// Initializes all high availability components
    pub async fn initialize(&mut self) -> Result<(), HaError> {
        self.cluster_manager.initialize().await?;
        self.health_checker.start_monitoring().await?;
        self.replication_manager.initialize().await?;
        self.load_balancer.initialize().await?;
        self.failover_manager.initialize().await?;
        
        Ok(())
    }

    /// Starts all high availability services
    pub async fn start(&mut self) -> Result<(), HaError> {
        self.cluster_manager.join_cluster().await?;
        self.health_checker.start_monitoring().await?;
        self.replication_manager.start_replication().await?;
        self.load_balancer.start().await?;
        self.failover_manager.enable().await?;
        
        Ok(())
    }

    /// Stops all high availability services
    pub async fn stop(&mut self) -> Result<(), HaError> {
        self.failover_manager.disable().await?;
        self.load_balancer.stop().await?;
        self.replication_manager.stop_replication().await?;
        self.health_checker.stop_monitoring().await?;
        self.cluster_manager.leave_cluster().await?;
        
        Ok(())
    }

    /// Gets the current cluster status
    pub async fn get_cluster_status(&self) -> Result<ClusterStatus, HaError> {
        self.cluster_manager.get_status().await
    }

    /// Gets the current health status
    pub async fn get_health_status(&self) -> Result<HealthStatus, HaError> {
        self.health_checker.get_status().await
    }

    /// Triggers a manual failover
    pub async fn trigger_failover(&mut self) -> Result<(), HaError> {
        self.failover_manager.trigger_manual_failover().await
    }

    /// Updates the high availability configuration
    pub async fn update_config(&mut self, config: HighAvailabilityConfig) -> Result<(), HaError> {
        self.config = config.clone();
        self.cluster_manager.update_config(&config).await?;
        self.failover_manager.update_config(&config).await?;
        self.health_checker.update_config(&config).await?;
        self.replication_manager.update_config(&config).await?;
        self.load_balancer.update_config(&config).await?;
        
        Ok(())
    }
}

/// Error types for high availability operations
#[derive(Debug, thiserror::Error)]
pub enum HaError {
    #[error("Cluster operation failed: {0}")]
    ClusterError(String),
    
    #[error("Failover operation failed: {0}")]
    FailoverError(String),
    
    #[error("Health check failed: {0}")]
    HealthCheckError(String),
    
    #[error("Replication error: {0}")]
    ReplicationError(String),
    
    #[error("Load balancing error: {0}")]
    LoadBalancerError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Status of the cluster
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClusterStatus {
    /// Cluster is healthy with all nodes operating
    Healthy,
    
    /// Cluster is operating with degraded service
    Degraded { 
        active_nodes: usize,
        total_nodes: usize,
        details: String,
    },
    
    /// Cluster is in failover state
    Failover {
        primary_node: String,
        failing_node: Option<String>,
        failover_phase: FailoverPhase,
    },
    
    /// Cluster is initializing
    Initializing,
    
    /// Cluster is down
    Down { reason: String },
}

/// Phase of the failover process
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FailoverPhase {
    /// Detecting failure
    Detection,
    
    /// Electing new primary
    Election,
    
    /// Promoting standby to primary
    Promotion,
    
    /// Redirecting clients
    Redirection,
    
    /// Recovering failed node
    Recovery,
    
    /// Completed failover
    Completed,
}

/// Health status of the system
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthStatus {
    /// Overall status
    pub status: HealthState,
    
    /// Component-specific health
    pub components: std::collections::HashMap<String, ComponentHealth>,
    
    /// Last check timestamp
    pub last_check: chrono::DateTime<chrono::Utc>,
    
    /// Status message
    pub message: Option<String>,
}

/// General health state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthState {
    Healthy,
    Degraded,
    Critical,
    Unknown,
}

/// Health of a specific component
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentHealth {
    pub name: String,
    pub status: HealthState,
    pub details: Option<String>,
    pub last_check: chrono::DateTime<chrono::Utc>,
} 