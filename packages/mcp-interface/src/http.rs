// MCP HTTP Interface Implementation
//
// This module provides HTTP transport for the MCP interfaces
// It implements both the client and server components

use axum::{
    extract::Path,
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::{
    net::{TcpListener, TcpSocket},
    sync::broadcast,
    time,
};
use log::{debug, info, warn, error};

use crate::{
    McpRequest, McpResponse, McpError,
};
use crate::metrics::MetricsCollector;

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
    
    /// Update the health status
    pub fn set_health_status(&self, status: &str) {
        let mut health = self.health.lock().unwrap();
        *health = status.to_string();
    }
    
    /// Get the current health status
    pub fn get_health_status(&self) -> String {
        let health = self.health.lock().unwrap();
        health.clone()
    }
}

/// HTTP transport for MCP
#[derive(Clone)]
pub struct HttpTransport {
    /// Server address
    addr: SocketAddr,
    /// Server state
    state: HttpServerState,
}

impl HttpTransport {
    /// Create a new HTTP transport
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            state: HttpServerState::new(),
        }
    }
    
    /// Start the HTTP server
    pub async fn start(&mut self) -> Result<(), String> {
        // Set health status to starting
        self.state.set_health_status("starting");
        
        // Create a TCP socket
        let socket = if cfg!(target_os = "linux") {
            match TcpSocket::new_v4() {
                Ok(s) => s,
                Err(e) => return Err(format!("Failed to create socket: {}", e)),
            }
        } else {
            match TcpSocket::new_v4() {
                Ok(s) => s,
                Err(e) => return Err(format!("Failed to create socket: {}", e)),
            }
        };
        
        // Set socket options
        if let Err(e) = socket.set_reuseaddr(true) {
            return Err(format!("Failed to set SO_REUSEADDR: {}", e));
        }
        
        // Bind to the address
        if let Err(e) = socket.bind(self.addr) {
            return Err(format!("Failed to bind to {}: {}", self.addr, e));
        }
        
        // Create a listener
        let listener = match socket.listen(1024) {
            Ok(listener) => listener,
            Err(e) => return Err(format!("Failed to listen on socket: {}", e)),
        };
        
        // Create the router
        let app = Router::new()
            .route("/", get(root_handler))
            .route("/health", get(health_handler))
            .route("/api/:version", post(api_handler))
            .with_state(self.state.clone());
            
        // Start the server
        info!("Starting HTTP server on {}", self.addr);
        
        // Spawn the server task
        tokio::spawn(async move {
            if let Err(e) = axum::serve(listener, app).await {
                error!("HTTP server error: {}", e);
            }
        });
        
        // Set health status to running
        self.state.set_health_status("running");
        
        // Start metrics reporting task
        self.start_metrics_task();
        
        Ok(())
    }
    
    /// Start a background task to report metrics
    fn start_metrics_task(&self) {
        let state = self.state.clone();
        tokio::spawn(async move {
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
}

/// Root handler
async fn root_handler() -> &'static str {
    "MCP HTTP Interface"
}

/// Health check handler
async fn health_handler(State(state): State<HttpServerState>) -> (StatusCode, Json<Value>) {
    let status = state.get_health_status();
    let response = json!({
        "status": status,
        "version": env!("CARGO_PKG_VERSION"),
    });
    
    (StatusCode::OK, Json(response))
}

/// API handler
async fn api_handler(
    Path(version): Path<String>,
    State(state): State<HttpServerState>,
    Json(request): Json<McpRequest>,
) -> Json<McpResponse> {
    debug!("Received API request: {} (v{})", request.method, version);
    
    let start_time = Instant::now();
    
    // In a real implementation, this would handle the request
    // For now, just return a dummy response
    let response = McpResponse {
        id: request.id.clone(),
        result: json!({"success": true}),
        error: None,
    };
    
    // Record metrics
    let duration = start_time.elapsed();
    state.metrics.record_request(&request.method, 200, duration);
    
    Json(response)
}

/// Start the MCP HTTP server
pub async fn start_server(addr: SocketAddr) -> Result<HttpTransport, String> {
    // Create the transport
    let mut transport = HttpTransport::new(addr);
    
    // Start the server
    if let Err(e) = transport.start().await {
        return Err(e);
    }
    
    // Subscribe to events
    let _rx = transport.state.tx.subscribe();
    
    Ok(transport)
}
