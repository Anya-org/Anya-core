// Test utilities for standalone tests
use std::fs;
use std::path::{Path, PathBuf};

pub struct TestEnvironment {
    pub test_dir: PathBuf,
}

pub fn setup_test_environment() -> TestEnvironment {
    // Create a temporary directory for the test
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let test_dir = std::env::temp_dir().join(format!("anya_test_{}", timestamp));

    // Create the directory if it doesn't exist
    if !test_dir.exists() {
        fs::create_dir_all(&test_dir).expect("Failed to create test directory");
    }

    TestEnvironment { test_dir }
}

// Helper function to clean up after tests
pub fn cleanup_test_environment(env: &TestEnvironment) {
    if env.test_dir.exists() {
        fs::remove_dir_all(&env.test_dir).expect("Failed to clean up test directory");
    }
}

// Simulate a Bitcoin transaction ID
pub fn simulate_bitcoin_txid() -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    format!("txid{:x}", timestamp)
}

// Simulate a Bitcoin address
pub fn simulate_bitcoin_address() -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    format!("bc1q{:x}qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq", timestamp % 10000)
}

// Simulate a DID (Decentralized Identifier)
pub fn simulate_did() -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    format!("did:key:z{:x}", timestamp)
}

// Simulate an RGB asset ID
pub fn simulate_rgb_asset_id() -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    format!("rgb1{:x}", timestamp)
}
