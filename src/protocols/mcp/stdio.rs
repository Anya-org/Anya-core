use std::io::{self, BufRead, Write};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use super::{McpError, McpTransport, McpRequest, McpResponse, McpNotification};

pub struct StdioTransport {
    reader: tokio::io::BufReader<tokio::io::Stdin>,
    writer: tokio::io::Stdout,
}

impl StdioTransport {
    pub fn new() -> Self {
        Self {
            reader: tokio::io::BufReader::new(tokio::io::stdin()),
            writer: tokio::io::stdout(),
        }
    }

    async fn read_message(&mut self) -> Result<String, McpError> {
        let mut line = String::new();
        self.reader.read_line(&mut line).await.map_err(|e| {
            McpError::TransportError(format!("Failed to read from stdin: {}", e))
        })?;
        Ok(line)
    }

    async fn write_message(&mut self, message: &str) -> Result<(), McpError> {
        self.writer.write_all(message.as_bytes()).await.map_err(|e| {
            McpError::TransportError(format!("Failed to write to stdout: {}", e))
        })?;
        self.writer.write_all(b"\n").await.map_err(|e| {
            McpError::TransportError(format!("Failed to write newline to stdout: {}", e))
        })?;
        self.writer.flush().await.map_err(|e| {
            McpError::TransportError(format!("Failed to flush stdout: {}", e))
        })?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl McpTransport for StdioTransport {
    async fn send_request(&self, request: McpRequest) -> Result<McpResponse, McpError> {
        let request_json = serde_json::to_string(&request).map_err(|e| {
            McpError::ProtocolError(format!("Failed to serialize request: {}", e))
        })?;

        let mut transport = StdioTransport::new();
        transport.write_message(&request_json).await?;

        let response_json = transport.read_message().await?;
        let response: McpResponse = serde_json::from_str(&response_json).map_err(|e| {
            McpError::ProtocolError(format!("Failed to deserialize response: {}", e))
        })?;

        Ok(response)
    }

    async fn send_notification(&self, notification: McpNotification) -> Result<(), McpError> {
        let notification_json = serde_json::to_string(&notification).map_err(|e| {
            McpError::ProtocolError(format!("Failed to serialize notification: {}", e))
        })?;

        let mut transport = StdioTransport::new();
        transport.write_message(&notification_json).await?;
        Ok(())
    }
}
