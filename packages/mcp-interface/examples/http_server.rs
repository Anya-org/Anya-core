// MCP HTTP Interface Demo [AIR-3][BPC-3]
//
// This example demonstrates the MCP HTTP interface functionality
// following the Bitcoin Development Framework v2.5 requirements

// Arc is not needed in this example
use std::net::SocketAddr;
use tokio::time::{sleep, Duration};

// Import necessary components from the MCP interface
use anya_core_mcp_interface::{
    http::HttpTransport,
    SystemIndex,
    SystemComponent,
};

// Required by Bitcoin Development Framework v2.5 for hexagonal architecture
// and system observability through the @SystemIndex

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a system index for component registration as required by [AIR-3]
    let mut system_index = SystemIndex::new();
    
    println!("Starting MCP HTTP interface demo...");
    
    // Create HTTP transport
    let mut transport = HttpTransport::new();
    println!("Initial health status: {}", transport.health());
    
    // Initialize HTTP interface on port 8080
    // Following hexagonal architecture port/adapter pattern
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    
    // Start the server - performs BIP compliance checks 
    // as required by the Bitcoin Development Framework v2.5
    transport.start(addr).await
        .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;
    
    println!("\nHTTP interface initialized with status: {}", transport.health());
    
    // Register with system index for component discovery [AIR-3]
    transport.register_with_index(&mut system_index);
    println!("Component registered with system index");
    
    // Get component status for observability requirements
    let status = transport.get_status();
    println!("\nComponent status: {:?}", status);
    
    // Display available hexagonal architecture ports (endpoints)
    println!("\nServer running at http://localhost:8080");
    println!("Available endpoints (BDF v2.5 compliant):");
    println!("  - / (Root endpoint)");
    println!("  - /health (Health status - reports current component health)");
    println!("  - /bip-status (BIP compatibility status - shows compliance with BIPs)");
    println!("  - /metrics (Server metrics - exposes Prometheus-compatible metrics)");
    println!("  - /api/v1/ping (API ping endpoint - demonstrates API versioning)");
    println!("\nPress Ctrl+C to stop the server");
    
    // Wait for user to stop the server
    let mut uptime_seconds = 0;
    loop {
        sleep(Duration::from_secs(5)).await;
        uptime_seconds += 5;
        println!("Server uptime: {} seconds - Metrics updated", uptime_seconds);
        
        // System monitoring as required by Bitcoin Development Framework v2.5
        let updated_status = transport.get_status();
        println!("Health: {}, Metrics: {:?}", 
            updated_status.health, 
            updated_status.metrics
                .iter()
                .map(|m| format!("{}: {}", m.name, m.value))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}
