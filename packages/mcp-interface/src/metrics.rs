// MCP Interface Metrics
//
// This module provides integration with the consolidated metrics system
// for the MCP interface, implementing proper status monitoring for the server.

use std::time::{Duration, Instant};
use std::sync::Arc;
use tokio::sync::RwLock;
use log::{debug, info, warn, error};

use anya_core_metrics::{MetricsService, mcp as mcp_metrics};

use crate::error::McpError;
use crate::types::McpHealthStatus;

/// HTTP server metrics
pub struct HttpServerMetrics {
    /// Metrics service
    metrics: Arc<MetricsService>,
    /// Server start time
    start_time: Instant,
    /// Current health status
    health_status: RwLock<McpHealthStatus>,
}

impl HttpServerMetrics {
    /// Create a new HTTP server metrics collector
    pub fn new(metrics: Arc<MetricsService>) -> Self {
        Self {
            metrics,
            start_time: Instant::now(),
            health_status: RwLock::new(McpHealthStatus::Stopped),
        }
    }
    
    /// Update the server health status
    pub async fn update_status(&self, status: McpHealthStatus) {
        let status_str = match status {
            McpHealthStatus::Starting => "starting",
            McpHealthStatus::Running => "running",
            McpHealthStatus::Stopping => "stopping",
            McpHealthStatus::Stopped => "stopped",
            McpHealthStatus::Error => "error",
        };
        
        let uptime = self.start_time.elapsed().as_secs();
        mcp_metrics::record_health_metrics(&self.metrics, status_str, uptime);
        
        // Update internal status
        *self.health_status.write().await = status;
        
        info!("MCP Server health status updated to: {}", status_str);
    }
    
    /// Record a request completion
    pub fn record_request(&self, method: &str, status_code: u16, duration: Duration) {
        let duration_ms = duration.as_secs_f64() * 1000.0;
        mcp_metrics::record_request_metrics(&self.metrics, method, status_code, duration_ms);
        
        debug!("MCP Server request: method={}, status={}, duration={}ms", 
               method, status_code, duration_ms);
    }
    
    /// Get the current health status
    pub async fn get_status(&self) -> McpHealthStatus {
        *self.health_status.read().await
    }
    
    /// Get the current uptime in seconds
    pub fn get_uptime(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}
