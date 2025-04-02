// Integration Test for HTTP Transport
//
// This test verifies that the HTTP transport server correctly follows the proper startup
// sequence and maintains accurate health status.

use tokio::test;
use tokio::time::{sleep, Duration};
use reqwest::Client;
use serde_json::json;
use std::sync::Arc;
use std::net::TcpListener;

// Import the HttpTransport and related types from our mcp-interface package
use anya_core_mcp_interface::{
    http::HttpTransport,
    types::{McpRequest, McpResponse},
    error::McpError,
};

/// Test helper to find an available TCP port
fn find_available_port() -> u16 {
    // Bind to port 0 to let the OS assign an available port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    listener.local_addr().expect("Failed to get local address").port()
}

#[tokio::test]
async fn test_http_transport_startup_sequence() -> Result<(), Box<dyn std::error::Error>> {
    // Find an available port for the test
    let port = find_available_port();
    println!("Testing HTTP transport on port {}", port);

    // Create a new HTTP transport with the test port
    let transport = HttpTransport::new(port);
    
    // Start the server in the background
    let server_handle = tokio::spawn(async move {
        match transport.start_server().await {
            Ok(_) => println!("Server started successfully"),
            Err(e) => panic!("Failed to start server: {}", e),
        }
    });
    
    // Give the server time to start up
    sleep(Duration::from_millis(500)).await;
    
    // Create a client to test the server
    let client = Client::new();
    
    // Test the health endpoint - should reflect "running" state after startup
    let health_response = client.get(&format!("http://localhost:{}/health", port))
        .send()
        .await?;
    
    assert_eq!(health_response.status(), 200);
    
    let health_json = health_response.json::<serde_json::Value>().await?;
    println!("Health response: {:?}", health_json);
    
    // Verify the health status is "running"
    assert_eq!(health_json["status"], "running");
    
    // Test the readiness endpoint
    let readiness_response = client.get(&format!("http://localhost:{}/health/readiness", port))
        .send()
        .await?;
    
    assert_eq!(readiness_response.status(), 200);
    
    // Request shutdown (this would be implemented in the real HttpTransport)
    // For the test, we'll just abort the task
    server_handle.abort();
    
    println!("HTTP transport startup sequence test passed");
    Ok(())
}

#[tokio::test]
async fn test_http_transport_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Test that the transport correctly handles errors during startup
    
    // First, start a server on a port
    let port = find_available_port();
    
    // Bind to that port to make it unavailable
    let _socket = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    
    // Now try to start the HTTP transport on the same port, which should fail
    let transport = HttpTransport::new(port);
    
    // The start_server call should return an error
    let result = transport.start_server().await;
    
    // Verify we got an error
    assert!(result.is_err());
    
    // Extract the error and verify it's the right type
    if let Err(e) = result {
        println!("Expected error: {}", e);
        match e {
            McpError::TransportError(_) => { /* This is expected */ },
            _ => panic!("Unexpected error type: {:?}", e),
        }
    }
    
    println!("HTTP transport error handling test passed");
    Ok(())
}

#[tokio::test]
async fn test_http_transport_request_response() -> Result<(), Box<dyn std::error::Error>> {
    // Find an available port for the test
    let port = find_available_port();
    
    // Create a new HTTP transport with the test port
    let transport = HttpTransport::new(port);
    
    // Start the server in the background
    let server_handle = tokio::spawn(async move {
        match transport.start_server().await {
            Ok(_) => println!("Server started successfully for request test"),
            Err(e) => panic!("Failed to start server: {}", e),
        }
    });
    
    // Give the server time to start up
    sleep(Duration::from_millis(500)).await;
    
    // Create a client to test the server
    let client = Client::new();
    
    // Test the request endpoint with a sample request
    let request_payload = json!({
        "id": "test-1",
        "method": "echo",
        "params": {
            "message": "Hello, MCP Server!"
        }
    });
    
    let response = client.post(&format!("http://localhost:{}/request", port))
        .json(&request_payload)
        .send()
        .await?;
    
    assert_eq!(response.status(), 200);
    
    let response_json = response.json::<serde_json::Value>().await?;
    println!("Response: {:?}", response_json);
    
    // Verify the response has the expected format
    assert!(response_json.as_object().unwrap().contains_key("id"));
    
    // Request shutdown
    server_handle.abort();
    
    println!("HTTP transport request/response test passed");
    Ok(())
}
