use anya::{AnyaCore, AnyaConfig, AnyaResult};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> AnyaResult<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| anya::AnyaError::System(e.to_string()))?;

    info!("Starting Anya Core system...");

    // Initialize Anya with default configuration
    let config = AnyaConfig::default();
    let anya_core = AnyaCore::new(config)?;

    info!("Anya Core system initialized successfully");

    // Set up API server
    let host = "127.0.0.1";
    let port = 8080;
    let addr = format!("{}:{}", host, port);

    info!("Starting API server on {}", addr);
    let listener = TcpListener::bind(&addr)
        .await
        .map_err(|e| anya::AnyaError::System(e.to_string()))?;

    // For now, just keep the server running
    info!("Anya Core is running. Press Ctrl+C to stop.");
    
    // Simple server loop - in a real implementation, this would handle requests
    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                info!("New connection from: {}", addr);
                // Handle connection here
                drop(socket); // Close immediately for now
            }
            Err(e) => {
                tracing::error!("Failed to accept connection: {}", e);
            }
        }
    }
}

