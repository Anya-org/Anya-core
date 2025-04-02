//! Stdio transport for MCP
//!
//! This module provides a simple stdin/stdout transport for MCP

use std::{
    io::{self, BufRead, Write},
    sync::Mutex,
    thread,
};
use log::{debug, info, warn, error};
use serde_json::Value;

use crate::{McpRequest, McpResponse, McpError};

/// Stdio transport for MCP
pub struct StdioTransport {
    /// Input buffer
    input: Mutex<Vec<String>>,
}

impl StdioTransport {
    /// Create a new stdio transport
    pub fn new() -> Self {
        let transport = Self {
            input: Mutex::new(Vec::new()),
        };
        
        transport.start_reader();
        
        transport
    }
    
    /// Start the stdin reader thread
    fn start_reader(&self) {
        // Clone the input buffer for the reader thread
        let input = self.input.clone();
        
        thread::spawn(move || {
            let stdin = io::stdin();
            let mut lines = stdin.lock().lines();
            
            while let Some(Ok(line)) = lines.next() {
                let mut input = input.lock().unwrap();
                input.push(line);
            }
        });
    }
    
    /// Read a line from stdin
    pub fn read_line(&self) -> Option<String> {
        let mut input = self.input.lock().unwrap();
        if input.is_empty() {
            None
        } else {
            Some(input.remove(0))
        }
    }
    
    /// Write a line to stdout
    pub fn write_line(&self, line: &str) -> io::Result<()> {
        let mut stdout = io::stdout();
        writeln!(stdout, "{}", line)?;
        stdout.flush()
    }
    
    /// Process an incoming request
    pub fn process_request(&self, request: &str) -> Result<McpResponse, McpError> {
        let request: McpRequest = serde_json::from_str(request)
            .map_err(|e| McpError::ProtocolError(format!("Invalid request: {}", e)))?;
            
        // Simple echo response for now
        Ok(McpResponse {
            id: request.id,
            result: serde_json::json!({"echo": request.method}),
            error: None,
        })
    }
}
