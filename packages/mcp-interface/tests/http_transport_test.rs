// Integration Test for HTTP Transport
//
// This test verifies that the HTTP transport server correctly follows the proper startup
// sequence and maintains accurate health status.

use tokio::time::Duration;
use std::net::{TcpListener, SocketAddr};

// Import the HttpTransport and related types from our mcp-interface package
use anya_core_mcp_interface::{
    http::{HttpTransport, start_server},
};

/// Test helper to find an available TCP port and return a socket address
fn find_available_port() -> SocketAddr {
    // Bind to port 0 to let the OS assign an available port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    listener.local_addr().expect("Failed to get local address")
}

#[tokio::test]
async fn test_http_transport_startup_sequence() -> Result<(), Box<dyn std::error::Error>> {
    // Find an available port for the test
    let addr = find_available_port();
    println!("Testing HTTP transport on {}", addr);

    // Start the server using our new pattern which handles the startup sequence
    let transport = start_server(addr).await?;
    
    // Verify that the server is in the "running" state as expected
    assert_eq!(transport.health(), "running", "Server should be in 'running' state");
    
    println!("HTTP transport startup sequence test passed");
    Ok(())
}

#[tokio::test]
async fn test_http_transport_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Test that our HTTP server startup correctly handles errors
    
    // First, find a port and make it unavailable by binding to it
    let addr = find_available_port();
    let _socket = TcpListener::bind(addr)?;
    
    // Now try to start the HTTP transport on the same address, which should fail
    let result = start_server(addr).await;
    
    // Verify we got an error
    assert!(result.is_err(), "Expected an error when port is already in use");
    
    // Print the error for debugging
    if let Err(e) = &result {
        println!("Expected error: {}", e);
    }
    
    println!("HTTP transport error handling test passed");
    Ok(())
}

#[tokio::test]
async fn test_http_transport_health_check() -> Result<(), Box<dyn std::error::Error>> {
    // Find an available port for the test
    let addr = find_available_port();
    
    // Start the server using our new pattern which handles the startup sequence
    let transport = start_server(addr).await?;
    
    // Verify the health status is correct
    assert_eq!(transport.health(), "running", "Health status should be 'running'");
    
    // Simple test that the transport is working
    let response = transport.health();
    assert_eq!(response, "running", "Health endpoint should return 'running'");
    
    println!("Health check test passed");
    Ok(())
}
