#![forbid(unsafe_code)]
#![warn(missing_docs)]
//! Metrics implementation for Anya Core
//! 
//! This package provides metrics collection, aggregation, and exposition for Anya Core,
//! supporting Prometheus compatible metrics endpoints.

use std::sync::Arc;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use thiserror::Error;
use log::{info, warn, error, debug};
use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use systemstat::{Platform, System};

/// Metrics error type
#[derive(Debug, Error)]
pub enum MetricsError {
    /// Server error
    #[error("Metrics server error: {0}")]
    ServerError(String),
    
    /// Collection error
    #[error("Metrics collection error: {0}")]
    CollectionError(String),
    
    /// Configuration error
    #[error("Metrics configuration error: {0}")]
    ConfigError(String),
}

/// Metrics configuration
#[derive(Debug, Clone)]
pub struct MetricsConfig {
    /// Whether metrics are enabled
    pub enabled: bool,
    /// Metrics server address
    pub address: SocketAddr,
    /// Collection interval in seconds
    pub collection_interval: u64,
    /// Metrics prefix
    pub prefix: String,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            address: "127.0.0.1:9000".parse().unwrap(),
            collection_interval: 15,
            prefix: "anya_core".to_string(),
        }
    }
}

/// Metrics service
pub struct MetricsService {
    /// Metrics configuration
    config: MetricsConfig,
    /// Prometheus handle
    handle: Option<PrometheusHandle>,
    /// Server task handle
    server_handle: Option<JoinHandle<Result<(), MetricsError>>>,
    /// System stats handle
    system_stats_handle: Option<JoinHandle<()>>,
    /// System information
    system: System,
}

impl MetricsService {
    /// Create a new metrics service
    pub fn new(config: MetricsConfig) -> Result<Self, MetricsError> {
        info!("Creating metrics service");
        
        let handle = if config.enabled {
            // Set up Prometheus metrics builder
            let builder = PrometheusBuilder::new()
                .with_namespace(config.prefix.clone())
                .add_global_label("app", "anya-core")
                .add_global_label("version", env!("CARGO_PKG_VERSION"));
            
            // Install prometheus metrics
            let handle = builder.install_recorder()?;
            Some(handle)
        } else {
            None
        };
        
        Ok(Self {
            config,
            handle,
            server_handle: None,
            system_stats_handle: None,
            system: System::new(),
        })
    }
    
    /// Start the metrics service
    pub fn start(&mut self) -> Result<(), MetricsError> {
        if !self.config.enabled {
            info!("Metrics service is disabled");
            return Ok(());
        }
        
        info!("Starting metrics service on {}", self.config.address);
        
        // Create a clone for the server task
        let handle = self.handle
            .as_ref()
            .ok_or_else(|| MetricsError::ServerError("Metrics handle not initialized".to_string()))?
            .clone();
        
        let addr = self.config.address;
        
        // Start metrics server
        self.server_handle = Some(tokio::spawn(async move {
            // Create Axum router
            let app = axum::Router::new()
                .route("/metrics", axum::routing::get(|| async move { handle.render() }))
                .route("/health", axum::routing::get(|| async { "OK" }));
            
            // Start server
            match axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await 
            {
                Ok(_) => Ok(()),
                Err(e) => Err(MetricsError::ServerError(format!("Server error: {}", e))),
            }
        }));
        
        // Start system metrics collection
        let collection_interval = self.config.collection_interval;
        let system = self.system.clone();
        
        self.system_stats_handle = Some(tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(collection_interval));
            
            loop {
                interval.tick().await;
                
                // Collect CPU usage
                if let Ok(cpu) = system.cpu_load_aggregate() {
                    if let Ok(cpu_load) = tokio::time::timeout(
                        Duration::from_secs(1), 
                        async { 
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            cpu.done()
                        }
                    ).await {
                        gauge!("system_cpu_usage_user", cpu_load.user as f64 * 100.0);
                        gauge!("system_cpu_usage_system", cpu_load.system as f64 * 100.0);
                        gauge!("system_cpu_usage_idle", cpu_load.idle as f64 * 100.0);
                    }
                }
                
                // Collect memory usage
                if let Ok(mem) = system.memory() {
                    gauge!("system_memory_total_bytes", mem.total.as_u64() as f64);
                    gauge!("system_memory_used_bytes", (mem.total.as_u64() - mem.free.as_u64()) as f64);
                    gauge!("system_memory_free_bytes", mem.free.as_u64() as f64);
                    
                    let memory_usage_percent = 100.0 * (mem.total.as_u64() - mem.free.as_u64()) as f64 / mem.total.as_u64() as f64;
                    gauge!("system_memory_usage_percent", memory_usage_percent);
                }
                
                // Collect disk usage
                if let Ok(mounts) = system.mounts() {
                    for mount in mounts {
                        if mount.fs_mounted_on == "/" {
                            let total = mount.total.as_u64() as f64;
                            let used = (mount.total.as_u64() - mount.avail.as_u64()) as f64;
                            let avail = mount.avail.as_u64() as f64;
                            
                            gauge!("system_disk_total_bytes", total);
                            gauge!("system_disk_used_bytes", used);
                            gauge!("system_disk_free_bytes", avail);
                            
                            let disk_usage_percent = 100.0 * used / total;
                            gauge!("system_disk_usage_percent", disk_usage_percent);
                        }
                    }
                }
            }
        }));
        
        info!("Metrics service started");
        Ok(())
    }
    
    /// Stop the metrics service
    pub async fn stop(&mut self) -> Result<(), MetricsError> {
        if !self.config.enabled {
            return Ok(());
        }
        
        info!("Stopping metrics service");
        
        // Stop system metrics collection
        if let Some(handle) = self.system_stats_handle.take() {
            handle.abort();
        }
        
        // Stop metrics server
        if let Some(handle) = self.server_handle.take() {
            handle.abort();
        }
        
        info!("Metrics service stopped");
        Ok(())
    }
    
    /// Record a counter increment
    pub fn record_counter(&self, name: &str, value: u64) {
        if !self.config.enabled {
            return;
        }
        
        counter!(name, value as f64);
    }
    
    /// Record a gauge value
    pub fn record_gauge(&self, name: &str, value: f64) {
        if !self.config.enabled {
            return;
        }
        
        gauge!(name, value);
    }
    
    /// Record a histogram value
    pub fn record_histogram(&self, name: &str, value: f64) {
        if !self.config.enabled {
            return;
        }
        
        histogram!(name, value);
    }
}

// Implement conversion between error types
impl From<metrics::SetRecorderError> for MetricsError {
    fn from(err: metrics::SetRecorderError) -> Self {
        MetricsError::ConfigError(format!("Failed to set metrics recorder: {}", err))
    }
}

/// Bitcoin Network Metrics
pub mod bitcoin {
    use super::*;
    
    /// Record Bitcoin P2P network metrics
    pub fn record_network_metrics(metrics: &MetricsService, peers: u64, connections: u64, download_rate: f64, upload_rate: f64) {
        metrics.record_gauge("bitcoin_peers", peers as f64);
        metrics.record_gauge("bitcoin_connections", connections as f64);
        metrics.record_gauge("bitcoin_download_rate_bytes", download_rate);
        metrics.record_gauge("bitcoin_upload_rate_bytes", upload_rate);
    }
    
    /// Record Bitcoin mempool metrics
    pub fn record_mempool_metrics(metrics: &MetricsService, tx_count: u64, fee_rate: f64, size_bytes: u64) {
        metrics.record_gauge("bitcoin_mempool_tx_count", tx_count as f64);
        metrics.record_gauge("bitcoin_mempool_fee_rate_sats", fee_rate);
        metrics.record_gauge("bitcoin_mempool_size_bytes", size_bytes as f64);
    }
    
    /// Record Bitcoin block metrics
    pub fn record_block_metrics(metrics: &MetricsService, height: u64, tx_count: u64, size_bytes: u64) {
        metrics.record_gauge("bitcoin_block_height", height as f64);
        metrics.record_gauge("bitcoin_block_tx_count", tx_count as f64);
        metrics.record_gauge("bitcoin_block_size_bytes", size_bytes as f64);
    }
}

/// MCP Server Metrics
pub mod mcp {
    use super::*;
    
    /// Record MCP request metrics
    pub fn record_request_metrics(metrics: &MetricsService, method: &str, status_code: u16, duration_ms: f64) {
        metrics.record_counter(&format!("mcp_request_{}", method), 1);
        metrics.record_counter(&format!("mcp_status_{}", status_code), 1);
        metrics.record_histogram("mcp_request_duration_ms", duration_ms);
    }
    
    /// Record MCP health metrics
    pub fn record_health_metrics(metrics: &MetricsService, status: &str, uptime_sec: u64) {
        metrics.record_gauge("mcp_health_status", if status == "running" { 1.0 } else { 0.0 });
        metrics.record_gauge("mcp_uptime_seconds", uptime_sec as f64);
    }
}

/// Validation Metrics 
pub mod validation {
    use super::*;
    
    /// Record validation metrics for transactions
    pub fn record_transaction_validation_metrics(metrics: &MetricsService, success: bool, duration_ms: f64, tx_size: u64) {
        metrics.record_counter("validation_transactions_total", 1);
        metrics.record_counter(
            if success { "validation_transactions_success" } else { "validation_transactions_failure" }, 
            1
        );
        metrics.record_histogram("validation_transaction_duration_ms", duration_ms);
        metrics.record_histogram("validation_transaction_size_bytes", tx_size as f64);
    }
    
    /// Record validation metrics for blocks
    pub fn record_block_validation_metrics(metrics: &MetricsService, success: bool, duration_ms: f64, tx_count: u64) {
        metrics.record_counter("validation_blocks_total", 1);
        metrics.record_counter(
            if success { "validation_blocks_success" } else { "validation_blocks_failure" }, 
            1
        );
        metrics.record_histogram("validation_block_duration_ms", duration_ms);
        metrics.record_gauge("validation_block_tx_count", tx_count as f64);
    }
}

/// System Metrics
pub mod system {
    use super::*;
    
    /// Record system metrics
    pub fn record_system_metrics(metrics: &MetricsService) -> Result<(), MetricsError> {
        let sys = System::new();
        
        // CPU metrics
        if let Ok(cpu) = sys.cpu_load_aggregate() {
            if let Ok(cpu_load) = tokio::task::block_in_place(|| {
                std::thread::sleep(Duration::from_millis(200));
                cpu.done()
            }) {
                metrics.record_gauge("system_cpu_user", cpu_load.user * 100.0);
                metrics.record_gauge("system_cpu_system", cpu_load.system * 100.0);
                metrics.record_gauge("system_cpu_idle", cpu_load.idle * 100.0);
            }
        }
        
        // Memory metrics
        if let Ok(mem) = sys.memory() {
            metrics.record_gauge("system_memory_total_bytes", mem.total.as_u64() as f64);
            metrics.record_gauge("system_memory_used_bytes", (mem.total.as_u64() - mem.free.as_u64()) as f64);
        }
        
        // Disk metrics
        if let Ok(disk) = sys.mount_at("/") {
            metrics.record_gauge("system_disk_total_bytes", disk.total.as_u64() as f64);
            metrics.record_gauge("system_disk_used_bytes", (disk.total.as_u64() - disk.free.as_u64()) as f64);
        }
        
        Ok(())
    }
}
