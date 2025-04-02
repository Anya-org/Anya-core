//! MCP types
//!
//! This module defines the core types used by the MCP interface

use thiserror::Error;
use serde::{Serialize, Deserialize};
use serde_json::Value;

/// MCP request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest {
    /// Request ID
    pub id: String,
    
    /// Request method
    pub method: String,
    
    /// Request parameters
    pub params: Option<Value>,
}

/// MCP response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse {
    /// Response ID matching the request ID
    pub id: String,
    
    /// Response result
    pub result: Value,
    
    /// Response error, if any
    pub error: Option<McpError>,
}

/// MCP error
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum McpError {
    /// Transport error
    #[error("Transport error: {0}")]
    TransportError(String),
    
    /// Protocol error
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    
    /// Authentication error
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    /// Internal server error
    #[error("Internal server error: {0}")]
    InternalError(String),
}
