use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for high availability system
/// [AIR-3][AIS-3][PFM-3][SCL-3][RES-3]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HighAvailabilityConfig {
    /// General high availability settings
    pub general: GeneralConfig,

    /// Cluster management configuration
    pub cluster: ClusterConfig,

    /// Health check configuration
    pub health_check: HealthCheckConfig,

    /// Failover configuration
    pub failover: FailoverConfig,

    /// Replication configuration
    pub replication: ReplicationConfig,

    /// Load balancing configuration
    pub load_balancing: LoadBalancingConfig,

    /// Disaster recovery configuration
    pub disaster_recovery: DisasterRecoveryConfig,
}

impl HighAvailabilityConfig {
    /// Creates a new configuration for development environment
    pub fn development() -> Self {
        Self {
            general: GeneralConfig {
                enabled: true,
                environment: Environment::Development,
                log_level: LogLevel::Debug,
            },
            cluster: ClusterConfig {
                node_count: 2,
                cluster_name: "anya-dev-cluster".to_string(),
                discovery_method: DiscoveryMethod::Static,
                static_nodes: vec!["localhost:5001".to_string(), "localhost:5002".to_string()],
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Creates a new configuration for production environment
    pub fn production() -> Self {
        Self {
            general: GeneralConfig {
                enabled: true,
                environment: Environment::Production,
                log_level: LogLevel::Info,
            },
            cluster: ClusterConfig {
                node_count: 3,
                cluster_name: "anya-prod-cluster".to_string(),
                discovery_method: DiscoveryMethod::Dns,
                dns_discovery_url: Some("anya-cluster.example.com".to_string()),
                heartbeat_interval: Duration::from_secs(5),
                ..Default::default()
            },
            health_check: HealthCheckConfig {
                enabled: true,
                check_interval: Duration::from_secs(10),
                critical_threshold: 3,
                warning_threshold: 2,
                ..Default::default()
            },
            failover: FailoverConfig {
                enabled: true,
                auto_failover: true,
                failover_timeout: Duration::from_secs(30),
                min_nodes_for_failover: 2,
                ..Default::default()
            },
            replication: ReplicationConfig {
                mode: ReplicationMode::Synchronous,
                sync_timeout: Duration::from_secs(10),
                ..Default::default()
            },
            load_balancing: LoadBalancingConfig {
                algorithm: LoadBalancingAlgorithm::RoundRobin,
                health_check_enabled: true,
                auto_scaling: true,
                ..Default::default()
            },
            disaster_recovery: DisasterRecoveryConfig {
                backup_interval: Duration::from_secs(3600),
                backup_retention: 7,
                auto_restore: true,
                ..Default::default()
            },
        }
    }
}

/// General high availability settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Whether high availability is enabled
    pub enabled: bool,

    /// Environment type
    pub environment: Environment,

    /// Log level
    pub log_level: LogLevel,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            environment: Environment::Development,
            log_level: LogLevel::Info,
        }
    }
}

/// Environment types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Cluster configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// Number of nodes in the cluster
    pub node_count: usize,

    /// Cluster name
    pub cluster_name: String,

    /// Method used for node discovery
    pub discovery_method: DiscoveryMethod,

    /// List of static nodes (for static discovery)
    #[serde(default)]
    pub static_nodes: Vec<String>,

    /// DNS address for DNS discovery
    pub dns_discovery_url: Option<String>,

    /// Kubernetes service name for K8s discovery
    pub k8s_service_name: Option<String>,

    /// Heartbeat interval
    #[serde(with = "humantime_serde")]
    pub heartbeat_interval: Duration,

    /// Node timeout
    #[serde(with = "humantime_serde")]
    pub node_timeout: Duration,

    /// Gossip interval
    #[serde(with = "humantime_serde")]
    pub gossip_interval: Duration,
}

impl Default for ClusterConfig {
    fn default() -> Self {
        Self {
            node_count: 1,
            cluster_name: "anya-cluster".to_string(),
            discovery_method: DiscoveryMethod::Static,
            static_nodes: vec!["localhost:5001".to_string()],
            dns_discovery_url: None,
            k8s_service_name: None,
            heartbeat_interval: Duration::from_secs(10),
            node_timeout: Duration::from_secs(30),
            gossip_interval: Duration::from_secs(1),
        }
    }
}

/// Methods for node discovery
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    Static,
    Dns,
    Kubernetes,
    Consul,
    Etcd,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Whether health checking is enabled
    pub enabled: bool,

    /// Health check interval
    #[serde(with = "humantime_serde")]
    pub check_interval: Duration,

    /// Number of failed checks to trigger warning
    pub warning_threshold: u32,

    /// Number of failed checks to trigger critical
    pub critical_threshold: u32,

    /// Timeout for health checks
    #[serde(with = "humantime_serde")]
    pub check_timeout: Duration,

    /// Components to check
    #[serde(default)]
    pub components: Vec<String>,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval: Duration::from_secs(30),
            warning_threshold: 2,
            critical_threshold: 3,
            check_timeout: Duration::from_secs(5),
            components: vec![
                "cluster".to_string(),
                "storage".to_string(),
                "network".to_string(),
            ],
        }
    }
}

/// Failover configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfig {
    /// Whether failover is enabled
    pub enabled: bool,

    /// Whether automatic failover is enabled
    pub auto_failover: bool,

    /// Timeout before triggering failover
    #[serde(with = "humantime_serde")]
    pub failover_timeout: Duration,

    /// Minimum number of nodes required for failover
    pub min_nodes_for_failover: usize,

    /// Maximum number of automatic failovers per period
    pub max_auto_failovers: Option<u32>,

    /// Period for auto failover limits
    #[serde(with = "humantime_serde")]
    pub auto_failover_period: Duration,

    /// Fencing enabled
    pub fencing_enabled: bool,
}

impl Default for FailoverConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_failover: true,
            failover_timeout: Duration::from_secs(60),
            min_nodes_for_failover: 2,
            max_auto_failovers: Some(3),
            auto_failover_period: Duration::from_secs(3600),
            fencing_enabled: true,
        }
    }
}

/// Replication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationConfig {
    /// Replication mode
    pub mode: ReplicationMode,

    /// Timeout for synchronous replication
    #[serde(with = "humantime_serde")]
    pub sync_timeout: Duration,

    /// Maximum lag for semi-sync replication
    #[serde(with = "humantime_serde")]
    pub max_lag: Duration,

    /// Number of acknowledgments required (for semi-sync)
    pub ack_count: Option<usize>,

    /// Compression enabled
    pub compression_enabled: bool,

    /// Encryption enabled
    pub encryption_enabled: bool,
}

impl Default for ReplicationConfig {
    fn default() -> Self {
        Self {
            mode: ReplicationMode::SemiSynchronous,
            sync_timeout: Duration::from_secs(5),
            max_lag: Duration::from_millis(500),
            ack_count: None,
            compression_enabled: true,
            encryption_enabled: true,
        }
    }
}

/// Replication modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplicationMode {
    Synchronous,
    SemiSynchronous,
    Asynchronous,
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,

    /// Whether to check health for load balancing
    pub health_check_enabled: bool,

    /// Whether to use sticky sessions
    pub sticky_sessions: bool,

    /// Whether to enable auto scaling
    pub auto_scaling: bool,

    /// Minimum number of nodes
    pub min_nodes: Option<usize>,

    /// Maximum number of nodes
    pub max_nodes: Option<usize>,

    /// Scale up threshold (load percentage)
    pub scale_up_threshold: Option<f32>,

    /// Scale down threshold (load percentage)
    pub scale_down_threshold: Option<f32>,
}

impl Default for LoadBalancingConfig {
    fn default() -> Self {
        Self {
            algorithm: LoadBalancingAlgorithm::LeastConnections,
            health_check_enabled: true,
            sticky_sessions: false,
            auto_scaling: false,
            min_nodes: Some(1),
            max_nodes: Some(10),
            scale_up_threshold: Some(0.75),
            scale_down_threshold: Some(0.25),
        }
    }
}

/// Load balancing algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    LeastConnections,
    LeastResponseTime,
    WeightedRoundRobin,
    ResourceBased,
}

/// Disaster recovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterRecoveryConfig {
    /// Backup interval
    #[serde(with = "humantime_serde")]
    pub backup_interval: Duration,

    /// Backup retention days
    pub backup_retention: u32,

    /// Whether auto restore is enabled
    pub auto_restore: bool,

    /// Remote backup location
    pub remote_backup_location: Option<String>,

    /// Encryption key for backups
    pub backup_encryption_key: Option<String>,
}

impl Default for DisasterRecoveryConfig {
    fn default() -> Self {
        Self {
            backup_interval: Duration::from_secs(3600), // 1 hour
            backup_retention: 30,                       // 30 days
            auto_restore: false,
            remote_backup_location: None,
            backup_encryption_key: None,
        }
    }
}
