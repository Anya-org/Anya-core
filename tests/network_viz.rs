//! Network Visualization Integration Tests
use anya_core::chaos::visualization::{run_server, NetworkState};
use reqwest::StatusCode;

#[tokio::test]
async fn test_network_visualization() {
    let port = 8081;
    let server_task = tokio::spawn(async move {
        run_server(port).await.unwrap();
    });

    // Test dashboard accessibility
    let resp = reqwest::get(&format!("http://localhost:{}/", port))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    
    // Test data endpoint
    let data: NetworkState = reqwest::get(&format!("http://localhost:{}/data", port))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    
    assert!(!data.nodes.is_empty(), "No nodes in network visualization");
    assert!(data.tps >= 0.0, "Invalid TPS metric");
    
    server_task.abort();
} 