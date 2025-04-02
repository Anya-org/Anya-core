// Standard IO Transport Implementation for MCP Interface
//
// Provides a simple stdio-based transport for command-line interaction with MCP

use std::io::{self, BufRead, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use super::{McpError, McpTransport, McpRequest, McpResponse, McpNotification};
use log::{info, warn, error, debug};
use tokio::sync::mpsc;

/// Stdio Transport for MCP Interface
#[derive(Clone)]
pub struct StdioTransport {
    /// Message sender
    tx: Arc<Mutex<mpsc::Sender<String>>>,
}

impl StdioTransport {
    /// Create a new stdio transport
    pub fn new() -> Self {
        // Create a channel for message passing
        let (tx, mut rx) = mpsc::channel::<String>(100);
        let tx = Arc::new(Mutex::new(tx));
        
        // Start a background thread for reading from stdin
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let stdin = io::stdin();
            let mut stdin_lock = stdin.lock();
            let mut buf = String::new();
            
            loop {
                buf.clear();
                match stdin_lock.read_line(&mut buf) {
                    Ok(0) => {
                        // EOF
                        break;
                    }
                    Ok(_) => {
                        // Send the line to the channel
                        if let Err(e) = tx_clone.lock().unwrap().blocking_send(buf.clone()) {
                            eprintln!("Failed to send message: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read from stdin: {}", e);
                        break;
                    }
                }
            }
        });
        
        // Start a background task for writing to stdout
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                let stdout = io::stdout();
                let mut stdout_lock = stdout.lock();
                if let Err(e) = writeln!(stdout_lock, "{}", message) {
                    error!("Failed to write to stdout: {}", e);
                    break;
                }
            }
        });
        
        Self { tx }
    }
}

#[async_trait::async_trait]
impl McpTransport for StdioTransport {
    /// Send a request
    async fn send_request(&self, request: McpRequest) -> Result<McpResponse, McpError> {
        debug!("Sending request: {:?}", request);
        
        match serde_json::to_string(&request) {
            Ok(message) => {
                match self.tx.lock().unwrap().try_send(message) {
                    Ok(_) => {
                        // For simplicity, return a dummy response
                        // In a real implementation, we would parse the response from stdin
                        Ok(McpResponse {
                            id: request.id,
                            result: serde_json::json!({"success": true}),
                            error: None,
                        })
                    }
                    Err(e) => Err(McpError::TransportError(format!("Failed to send message: {}", e)))
                }
            }
            Err(e) => Err(McpError::TransportError(format!("Serialization error: {}", e)))
        }
    }
    
    /// Send a notification
    async fn send_notification(&self, notification: McpNotification) -> Result<(), McpError> {
        debug!("Sending notification: {:?}", notification);
        
        match serde_json::to_string(&notification) {
            Ok(message) => {
                match self.tx.lock().unwrap().try_send(message) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(McpError::TransportError(format!("Failed to send message: {}", e)))
                }
            }
            Err(e) => Err(McpError::TransportError(format!("Serialization error: {}", e)))
        }
    }
}
