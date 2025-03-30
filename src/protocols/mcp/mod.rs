use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub name: String,
    pub version: String,
    pub transport_type: TransportType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportType {
    Stdio,
    Http,
}

#[derive(Debug)]
pub struct McpServer {
    config: McpServerConfig,
    state: Arc<RwLock<ServerState>>,
}

#[derive(Debug, Default)]
struct ServerState {
    resources: Vec<Resource>,
    active_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub uri: String,
    pub name: String,
    pub resource_type: ResourceType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    File,
    Directory,
    Service,
    Tool,
}

impl McpServer {
    pub fn new(config: McpServerConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(ServerState::default())),
        }
    }

    pub async fn initialize(&self) -> Result<(), McpError> {
        // Initialize server components
        self.setup_transport().await?;
        self.register_default_handlers().await?;
        Ok(())
    }

    async fn setup_transport(&self) -> Result<(), McpError> {
        match self.config.transport_type {
            TransportType::Stdio => {
                // Setup stdio transport
                Ok(())
            }
            TransportType::Http => {
                // Setup HTTP transport with improved logging
                tracing::info!("Setting up HTTP transport");
                Ok(())
            }
        }
    }
    
    /// Start the MCP server with the configured transport
    pub async fn start(&self, port: u16) -> Result<(), McpError> {
        tracing::info!("Starting MCP server with port {}", port);
        
        // First update status to starting
        tracing::info!("Updating server status to 'starting'");
        
        match self.config.transport_type {
            TransportType::Http => {
                use crate::protocols::mcp::http::HttpTransport;
                
                // Create the HTTP transport only once
                let transport = HttpTransport::new(port);
                
                // Start the HTTP server before storing the transport
                // and explicitly handle errors
                tracing::info!("Starting HTTP transport on port {}", port);
                transport.start_server().await
            },
            TransportType::Stdio => {
                tracing::info!("Starting Stdio transport");
                // Stdio transport implementation
                Ok(())
            }
        }
    }

    async fn register_default_handlers(&self) -> Result<(), McpError> {
        // Register basic request handlers
        Ok(())
    }

    pub async fn add_resource(&self, resource: Resource) -> Result<(), McpError> {
        let mut state = self.state.write().await;
        state.resources.push(resource);
        Ok(())
    }

    pub async fn list_resources(&self) -> Result<Vec<Resource>, McpError> {
        let state = self.state.read().await;
        Ok(state.resources.clone())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum McpError {
    #[error("Transport error: {0}")]
    TransportError(String),
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    #[error("Internal error: {0}")]
    InternalError(String),
}

// Transport trait for implementing different transport mechanisms
#[async_trait::async_trait]
pub trait McpTransport: Send + Sync {
    async fn send_request(&self, request: McpRequest) -> Result<McpResponse, McpError>;
    async fn send_notification(&self, notification: McpNotification) -> Result<(), McpError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest {
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse {
    pub result: Option<serde_json::Value>,
    pub error: Option<McpErrorResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpNotification {
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpErrorResponse {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}
