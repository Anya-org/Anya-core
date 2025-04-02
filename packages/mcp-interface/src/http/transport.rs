// HTTP Transport Implementation [AIR-3][BPC-3][AIS-3]
//
// This module provides the HTTP transport implementation for the MCP interface
// following the Bitcoin Development Framework v2.5 requirements

use axum::{
    routing::{get, post},
    Router,
};
// use serde_json::json;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    net::TcpSocket,
    sync::broadcast,
    task::JoinHandle,
    time,
};
use log::{debug, info, error};
use std::any::Any;

use crate::{
    
    SystemComponent, ComponentStatus, Metric, SystemIndex
};
use crate::metrics::MetricsCollector;
#[cfg(feature = "bitcoin")]
use crate::compliance;
use super::error::TransportError;
use super::handlers;

/// HTTP server state shared across all handlers
#[derive(Clone)]
pub struct HttpServerState {
    /// Channel for sending events
    pub tx: broadcast::Sender<String>,
    /// Metrics collector
    pub metrics: Arc<MetricsCollector>,
    /// Health status
    pub health: Arc<Mutex<String>>,
}

impl HttpServerState {
    /// Create a new HTTP server state
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            tx,
            metrics: Arc::new(MetricsCollector::new()),
            health: Arc::new(Mutex::new("starting".to_string())),
        }
    }
    
    /// Set health status
    pub fn set_health_status(&self, status: &str) {
        let mut health = self.health.lock().unwrap();
        *health = status.to_string();
    }
    
    /// Get health status
    pub fn get_health_status(&self) -> String {
        let health = self.health.lock().unwrap();
        health.clone()
    }
}

/// HTTP Transport implementation
pub struct HttpTransport {
    /// Server state
    pub state: HttpServerState,
    /// Server handle
    server: Option<JoinHandle<()>>,
}

impl HttpTransport {
    /// Create a new HTTP transport
    pub fn new() -> Self {
        Self {
            state: HttpServerState::new(),
            server: None,
        }
    }
    
    /// Start the HTTP server [AIR-3][AIS-3]
    pub async fn start(&mut self, addr: SocketAddr) -> Result<(), TransportError> {
        // Set health status to starting
        self.state.set_health_status("starting");
        
        // Check Taproot compatibility (BIP-341)
        #[cfg(feature = "bitcoin")]
        if !compliance::verify_taproot_compatibility() {
            return Err(TransportError::ComplianceError(
                "BIP-341 (Taproot) compatibility check failed".to_string()
            ));
        }
        
        // Create socket
        let socket = TcpSocket::new_v4().map_err(|e| TransportError::SocketError(e.to_string()))?;
        
        // Bind to address
        socket.bind(addr).map_err(|e| TransportError::BindingError(e.to_string()))?;
        
        // Create listener
        let listener = socket.listen(1024).map_err(|e| TransportError::SocketError(e.to_string()))?;
        
        info!("HTTP server listening on {}", addr);
        
        // Create application state
        let state = self.state.clone();
        
        // Create router with handlers
        let app = Router::new()
            .route("/", get(handlers::root_handler))
            .route("/health", get(handlers::health_handler))
            .route("/bip-status", get(handlers::bip_status_handler))
            .route("/metrics", get(handlers::metrics_handler))
            .route("/api/:version/:endpoint", post(handlers::api_handler))
            .with_state(state.clone());
        
        // Start server in background task
        let server_handle = tokio::spawn(async move {
            // Convert tokio TcpListener to a hyper-compatible server
            match axum::Server::from_tcp(listener.into_std().unwrap()) {
                Ok(server) => {
                    if let Err(e) = server
                        .serve(app.into_make_service())
                        .with_graceful_shutdown(shutdown_signal())
                        .await {
                        error!("HTTP server error: {}", e);
                    }
                },
                Err(e) => {
                    error!("Failed to create HTTP server: {}", e);
                }
            }
        });
        
        // Store the server handle
        self.server = Some(server_handle);
        
        // Set health status to running
        self.state.set_health_status("running");
        
        // Start metrics reporting task
        self.start_metrics_task();
        
        Ok(())
    }
    
    /// Verify Taproot compatibility (BIP-341) [AIS-3][BPC-3]
    #[cfg(feature = "bitcoin")]
    async fn verify_taproot_compatibility(&self) -> bool {
        // This would perform an actual check against the bitcoin network
        // For now, we just return true as if Taproot is supported
        true
    }
    
    /// Start metrics reporting task
    fn start_metrics_task(&self) {
        let state = self.state.clone();
        
        tokio::spawn(async move {
            // Report metrics every 60 seconds
            let mut interval = time::interval(Duration::from_secs(60));
            
            loop {
                interval.tick().await;
                
                // Report metrics
                let metrics = state.metrics.get_metrics();
                debug!("Metrics: {:?}", metrics);
            }
        });
    }
    
    /// Get metrics collector
    pub fn metrics(&self) -> Arc<MetricsCollector> {
        self.state.metrics.clone()
    }
    
    /// Get health status
    pub fn health(&self) -> String {
        self.state.get_health_status()
    }
    
    /// Convert to Any for downcasting
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implement SystemComponent for HttpTransport [AIR-3]
impl SystemComponent for HttpTransport {
    /// Register with the system index
    fn register_with_index(&self, index: &mut SystemIndex) {
        // Extract metrics as key-value pairs
        let metrics = self.state.metrics.get_metrics();
        let metric_pairs = vec![
            ("requests_total".to_string(), metrics.requests_total as i64),
            ("uptime_seconds".to_string(), metrics.uptime_seconds as i64),
        ];
        
        // Register with the index
        index.register_component(
            "mcp_http_transport",
            self.state.health.clone(),
            metric_pairs,
        );
    }
    
    /// Get component status
    fn get_status(&self) -> ComponentStatus {
        let metrics = self.state.metrics.get_metrics();
        
        ComponentStatus {
            name: "MCP HTTP Transport".to_string(),
            health: self.state.get_health_status(),
            metrics: vec![
                Metric::new("requests_total", metrics.requests_total as i64),
                Metric::new("uptime_seconds", metrics.uptime_seconds as i64),
            ],
        }
    }
}

/// Shutdown signal handler
async fn shutdown_signal() {
    // Wait for CTRL+C
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    
    // Received shutdown signal
    info!("Received shutdown signal, gracefully stopping server");
}

/// Start HTTP server and return a transport instance
pub async fn start_server(addr: SocketAddr) -> Result<HttpTransport, TransportError> {
    let mut transport = HttpTransport::new();
    transport.start(addr).await?;
    Ok(transport)
}
