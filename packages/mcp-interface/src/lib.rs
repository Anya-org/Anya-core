//! MCP Interface Implementation [AIR-3][AIS-3][BPC-3]
//!
//! This module implements the MCP Interface with HTTP, WebSockets, and optional gRPC support.
//! Includes enhanced server implementation with improved reliability and proper startup sequence.
//! Compliant with the Bitcoin Development Framework v2.5.

use anyhow::Result;
use std::sync::Arc;
use log::info;
use std::net::SocketAddr;
use async_trait::async_trait;

// MCP transport implementations
pub mod http;
pub mod stdio;
pub mod metrics;
pub mod types;
pub mod system;
pub mod compliance;

// Re-exports for convenient API access
#[cfg(feature = "grpc")]
pub mod grpc;

// Re-export types for easy access
pub use types::{McpRequest, McpResponse, McpError};
pub use system::{SystemComponent, SystemIndex, ComponentStatus, Metric};
pub use compliance::{verify_taproot_compatibility, validate_psbt, validate_psbt_v2, get_supported_bips};

/// MCP Transport trait
#[async_trait]
pub trait McpTransport: Send + Sync {
    /// Handle an MCP request
    async fn handle_request(&self, request: McpRequest) -> Result<McpResponse, McpError>;
    
    /// Get the transport health status
    fn health(&self) -> String;
}

// Implement McpTransport for HttpTransport
#[async_trait]
impl McpTransport for http::HttpTransport {
    async fn handle_request(&self, request: McpRequest) -> Result<McpResponse, McpError> {
        // Simply return a successful response for now
        Ok(McpResponse {
            id: request.id,
            result: serde_json::json!({"success": true}),
            error: None,
        })
    }
    
    fn health(&self) -> String {
        self.health()
    }
}

// Implement McpTransport for StdioTransport
#[async_trait]
impl McpTransport for stdio::StdioTransport {
    async fn handle_request(&self, request: McpRequest) -> Result<McpResponse, McpError> {
        // Placeholder implementation
        Ok(McpResponse {
            id: request.id,
            result: serde_json::json!({"success": true}),
            error: None,
        })
    }
    
    fn health(&self) -> String {
        "running".to_string()
    }
}

/// Initialize the MCP interface with HTTP transport
pub async fn init_with_http(port: u16) -> Result<Arc<dyn McpTransport>, McpError> {
    info!("Initializing MCP interface with HTTP transport on port {}", port);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let transport = http::start_server(addr).await
        .map_err(|e| McpError::TransportError(e.to_string()))?;
    
    Ok(Arc::new(transport))
}

/// Initialize the MCP interface with stdio transport
pub fn init_with_stdio() -> Result<Arc<dyn McpTransport>, McpError> {
    info!("Initializing MCP interface with stdio transport");
    
    let transport = stdio::StdioTransport::new();
    Ok(Arc::new(transport))
}

/// Start the MCP server with the specified transport
pub async fn start_server(transport: Arc<dyn McpTransport>) -> Result<(), McpError> {
    info!("Starting MCP server with transport: {}", transport.health());
    
    // The server is already running at this point if using HTTP transport
    // The transport was properly initialized in init_with_http or init_with_stdio
    
    // For a proper server startup sequence:
    // 1. Health status was already set to "starting" in transport initialization
    // 2. HTTP transport was created only once
    // 3. HTTP server was started before storing the transport
    // 4. Errors were explicitly handled during startup
    // 5. Transport was stored only after successful server start
    // 6. Health status was updated to "running" after confirmation
    
    // Any additional startup logic for other transports would go here
    
    Ok(())
}
