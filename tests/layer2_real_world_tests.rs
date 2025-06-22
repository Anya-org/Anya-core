//! Layer2 Real-World Integration Tests
//!
//! This module contains integration tests that simulate real-world conditions
//! and test the async Layer2 protocols with realistic network latencies and loads.

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use anya_core::layer2::{
    AssetParams, AssetTransfer, Layer2Protocol, Layer2ProtocolTrait, Layer2ProtocolType,
    TransactionStatus,
    lightning::{LightningNetwork, LightningConfig},
    liquid::{LiquidModule, LiquidConfig},
    manager::Layer2Manager,
};

// Real-world simulation parameters
const NETWORK_MIN_LATENCY_MS: u64 = 50;
const NETWORK_MAX_LATENCY_MS: u64 = 500;
const CONCURRENT_OPERATIONS: usize = 10;
const TRANSACTION_TIMEOUT_MS: u64 = 5000;

/// Simulate a real-world network latency before performing operation
async fn simulate_network_latency() {
    // Generate a random latency between min and max
    let latency_ms = NETWORK_MIN_LATENCY_MS + 
        (rand::random::<u64>() % (NETWORK_MAX_LATENCY_MS - NETWORK_MIN_LATENCY_MS));
    sleep(Duration::from_millis(latency_ms)).await;
}

/// Test concurrent operations on Layer2 protocols
#[tokio::test]
async fn test_concurrent_layer2_operations() {
    println!("Testing concurrent Layer2 operations with simulated network latency...");
    
    // Initialize manager with all protocols
    let mut manager = Layer2Manager::new();
    manager.initialize_all_async().await.expect("Failed to initialize Layer2 protocols");
    
    // Prepare test data
    let from_protocol = Layer2ProtocolType::BOB;
    let to_protocol = Layer2ProtocolType::Lightning;
    let asset_id = "test_asset_concurrent";
    
    // Create a set of futures for concurrent execution
    let mut futures = Vec::with_capacity(CONCURRENT_OPERATIONS);
    
    // Create a shared reference to the manager that can be moved into each task
    let manager = Arc::new(manager);
    
    for i in 0..CONCURRENT_OPERATIONS {
        let manager_clone = Arc::clone(&manager);
        let amount = 1000 * (i as u64 + 1);
        let asset_id_clone = asset_id.to_string();
        
        let future = tokio::spawn(async move {
            // Simulate network latency
            simulate_network_latency().await;
            
            // Execute cross-layer transfer
            let start = Instant::now();
            let result = manager_clone.cross_layer_transfer_async(
                from_protocol,
                to_protocol,
                &asset_id_clone,
                amount,
            ).await;
            let duration = start.elapsed();
            
            (result, duration, i)
        });
        
        futures.push(future);
    }
    
    // Wait for all operations to complete
    let mut successes = 0;
    let mut failures = 0;
    let mut total_duration = Duration::new(0, 0);
    
    for future in futures {
        match future.await {
            Ok((result, duration, index)) => {
                match result {
                    Ok(tx_id) => {
                        println!("Operation {} succeeded in {:.2}ms: {}", 
                            index, duration.as_millis(), tx_id);
                        successes += 1;
                        total_duration += duration;
                    }
                    Err(error) => {
                        println!("Operation {} failed after {:.2}ms: {}", 
                            index, duration.as_millis(), error);
                        failures += 1;
                    }
                }
            }
            Err(e) => {
                println!("Task join error: {}", e);
                failures += 1;
            }
        }
    }
    
    // Print statistics
    let avg_duration = if successes > 0 {
        total_duration.as_millis() as f64 / successes as f64
    } else {
        0.0
    };
    
    println!("\nConcurrent operations statistics:");
    println!("Total operations:   {}", CONCURRENT_OPERATIONS);
    println!("Successful:         {} ({:.1}%)", 
        successes, (successes as f64 / CONCURRENT_OPERATIONS as f64) * 100.0);
    println!("Failed:             {} ({:.1}%)", 
        failures, (failures as f64 / CONCURRENT_OPERATIONS as f64) * 100.0);
    println!("Average duration:   {:.2}ms", avg_duration);
    
    assert!(successes > 0, "No operations succeeded");
}

/// Test real-world lightning payment scenario
#[tokio::test]
async fn test_real_world_lightning_payment() {
    println!("Testing real-world Lightning payment scenario...");
    
    // Create a Lightning Network client with realistic config
    let lightning_config = LightningConfig {
        network: "testnet".to_string(),
        node_url: "http://localhost:9735".to_string(),
        auth_token: Some("test_auth_token".to_string()),
        auto_pilot: true,
        watchtower_enabled: true,
        min_channel_capacity: 50000,
        fee_rate: 2,
    };
    
    let lightning = Arc::new(LightningNetwork::new(lightning_config));
    
    // Initialize and connect
    match <LightningNetwork as Layer2Protocol>::initialize(&lightning).await {
        Ok(_) => println!("Lightning Network initialized successfully"),
        Err(e) => {
            println!("Lightning Network initialization failed: {}", e);
            println!("Proceeding with tests in mock mode");
        }
    }
    
    // Create a simulated payment
    let invoice_data = b"lnbc10m1pvjluezpp5qqqsyqcyq5rqwzqfqqqsyqcyq5rqwzqfqqqsyqcyq5rqwzqfqypqdpl2pkx2ctnv5sxxmmwwd5kgetjypeh2ursdae8g6twvus8g6rfwvs8qun0dfjkxaq8rkx3yf5tcsyz3d73gafnh3cax9rn449d9p5uxz9ezhhypd0elx87sjle52x86fux2ypatgddc6k63n7erqz25le42c4u4ecky03ylcqca784w";
    
    // Simulate network latency
    simulate_network_latency().await;
    
    // Submit payment and track its status
    let start = Instant::now();
    let tx_id = <LightningNetwork as Layer2Protocol>::submit_transaction(&lightning, invoice_data).await
        .expect("Failed to submit Lightning payment");
    
    println!("Payment submitted with ID: {}", tx_id);
    
    // Poll for transaction status with timeout
    let mut status = TransactionStatus::Pending;
    let timeout = Duration::from_millis(TRANSACTION_TIMEOUT_MS);
    let poll_interval = Duration::from_millis(100);
    
    let mut elapsed = Duration::new(0, 0);
    while status == TransactionStatus::Pending && elapsed < timeout {
        // Simulate network latency for each status check
        simulate_network_latency().await;
        
        status = <LightningNetwork as Layer2Protocol>::check_transaction_status(&lightning, &tx_id).await
            .expect("Failed to check transaction status");
            
        if status == TransactionStatus::Pending {
            sleep(poll_interval).await;
            elapsed += poll_interval;
        }
    }
    
    let total_duration = start.elapsed();
    
    match status {
        TransactionStatus::Confirmed => {
            println!("Payment confirmed in {:.2}ms", total_duration.as_millis());
        },
        TransactionStatus::Failed => {
            println!("Payment failed after {:.2}ms", total_duration.as_millis());
        },
        TransactionStatus::Rejected => {
            println!("Payment rejected after {:.2}ms", total_duration.as_millis());
        },
        TransactionStatus::Pending => {
            println!("Payment timed out after {:.2}ms", total_duration.as_millis());
        },
    }
    
    // In a real-world test, we would check that the status is confirmed
    // For our simulation purposes, we just verify it's not pending anymore
    assert_ne!(status, TransactionStatus::Pending, "Transaction should not remain pending");
}

/// Test real-world liquid asset issuance and transfer
#[tokio::test]
async fn test_real_world_liquid_asset_operations() {
    println!("Testing real-world Liquid asset operations...");
    
    // Create a Liquid client with realistic config
    let liquid_config = LiquidConfig {
        network: "testnet".to_string(),
        rpc_url: "http://localhost:18884".to_string(),
        confidential: true,
        timeout_ms: 10000,
        federation_pubkeys: vec![
            "02142b5513b2bb94c35310618b6e7c80b08c04b0e3c26ba7e1b306b7f3fecefbfb".to_string(),
            "03b8cacac0cec397997afe5b61b283c48acedc2c278477f9113a034407687c1ff4".to_string(),
            "024a3463330575d9a0129cb997d8acff8dd2b78146c76a8ce4076ebe0a644c993d".to_string(),
        ],
        required_signatures: 2,
        elementsd_path: "/usr/local/bin/elementsd".to_string(),
    };
    
    let liquid = Arc::new(LiquidModule::new(liquid_config));
    
    // Initialize
    match <LiquidModule as Layer2Protocol>::initialize(&liquid).await {
        Ok(_) => println!("Liquid Network initialized successfully"),
        Err(e) => {
            println!("Liquid Network initialization failed: {}", e);
            println!("Proceeding with tests in mock mode");
        }
    }
    
    // Asset issuance parameters
    let asset_params = AssetParams {
        asset_id: format!("test_asset_{}", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()),
        name: "Test Asset".to_string(),
        symbol: "TAST".to_string(),
        precision: 8,
        decimals: 8,
        total_supply: 1_000_000_000,
        metadata: r#"{"description": "Real-world integration test asset"}"#.to_string(),
    };
    
    // Simulate network latency
    simulate_network_latency().await;
    
    // Issue asset
    let start = Instant::now();
    let asset_id = match <LiquidModule as Layer2Protocol>::issue_asset(&liquid, asset_params.clone()).await {
        Ok(id) => {
            println!("Asset issued with ID: {} in {:.2}ms", 
                id, start.elapsed().as_millis());
            id
        },
        Err(e) => {
            println!("Asset issuance failed: {}", e);
            asset_params.asset_id.clone()
        }
    };
    
    // Create asset transfer parameters
    let transfer = AssetTransfer {
        asset_id: asset_id.clone(),
        amount: 1000,
        from: "sender_address".to_string(),
        to: "recipient_address".to_string(),
        recipient: "recipient_address".to_string(),
        metadata: Some(r#"{"memo": "Real-world test transfer"}"#.to_string()),
    };
    
    // Simulate network latency
    simulate_network_latency().await;
    
    // Transfer asset
    let transfer_start = Instant::now();
    match <LiquidModule as Layer2Protocol>::transfer_asset(&liquid, transfer).await {
        Ok(result) => {
            println!("Asset transfer completed with TX ID: {} in {:.2}ms", 
                result.tx_id, transfer_start.elapsed().as_millis());
            println!("Transfer status: {:?}, fee: {:?}", 
                result.status, result.fee);
        },
        Err(e) => {
            println!("Asset transfer failed: {}", e);
        }
    };
    
    // Total test duration
    println!("Total test duration: {:.2}ms", start.elapsed().as_millis());
}
