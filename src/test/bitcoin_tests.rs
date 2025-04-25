use std::error::Error;
use log::{info, warn, error};
use bitcoin::{Network, Transaction};
use bitcoin::consensus::encode::deserialize;
use std::process::Command;
use crate::testing::UnifiedTester;
use std::sync::Arc;
use crate::config;
use url;

/// Runs all Bitcoin protocol tests to verify BPC-3 compliance
pub fn run_all() {
    info!("Running all Bitcoin tests...");
    
    // Initialize the unified tester with Bitcoin validator
    let tester = Arc::new(UnifiedTester::new());
    
    // Test Bitcoin Core connection
    match test_bitcoin_core_connection(&tester) {
        Ok(_) => info!("✅ Bitcoin Core connection test passed"),
        Err(e) => error!("❌ Bitcoin Core connection test failed: {}", e),
    }
    
    // Test Taproot support (BIP-341)
    match test_taproot_support(&tester) {
        Ok(_) => info!("✅ Taproot support test passed"),
        Err(e) => error!("❌ Taproot support test failed: {}", e),
    }
    
    // Test transaction validation
    match test_transaction_validation(&tester) {
        Ok(_) => info!("✅ Transaction validation test passed"),
        Err(e) => error!("❌ Transaction validation test failed: {}", e),
    }
    
    // Test PSBT handling (BIP-174)
    match test_psbt_handling(&tester) {
        Ok(_) => info!("✅ PSBT handling test passed"),
        Err(e) => error!("❌ PSBT handling test failed: {}", e),
    }
    
    info!("Bitcoin tests completed");
}

/// Tests connection to Bitcoin Core
fn test_bitcoin_core_connection(tester: &Arc<UnifiedTester>) -> Result<(), String> {
    // Get the appropriate RPC endpoint from configuration
    let config = config::load_config("config/anya.conf").map_err(|e| e.to_string())?;
    
    let rpc_url = if !config.network.bitcoin_custom_rpc_url.is_empty() {
        config.network.bitcoin_custom_rpc_url
    } else if config.network.network_type == "mainnet" {
        config.network.bitcoin_mainnet_rpc_url
    } else {
        config.network.bitcoin_testnet_rpc_url
    };
    
    info!("Testing Bitcoin Core connection to {}...", rpc_url);
    
    // Extract host and port from URL
    let url = url::Url::parse(&rpc_url).map_err(|e| format!("Invalid URL: {}", e))?;
    let host = url.host_str().unwrap_or("localhost");
    let port = url.port().unwrap_or(if url.scheme() == "https" { 443 } else { 8332 });
    let use_ssl = url.scheme() == "https";
    
    // Build appropriate command arguments
    let mut args = vec!["-rpcconnect", host];
    args.push("-rpcport");
    args.push(&port.to_string());
    if use_ssl {
        args.push("-rpcssl");
    }
    args.push("getnetworkinfo");
    
    let output = Command::new("bitcoin-cli")
        .args(&args)
        .output();
        
    match output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to connect to Bitcoin: {}", error));
            }
            
            let info = String::from_utf8_lossy(&output.stdout);
            info!("Connection to Bitcoin successful: {}", info);
            Ok(())
        },
        Err(e) => Err(format!("Failed to execute bitcoin-cli: {}", e)),
    }
}

/// Tests Taproot (BIP-341) support according to BPC-3 standard
fn test_taproot_support(tester: &Arc<UnifiedTester>) -> Result<(), String> {
    info!("Testing Taproot support on testnet...");
    
    // Updated to use the public testnet RPC endpoint
    tester.bitcoin_validator.verify_taproot_support_with_endpoint(
        "https://bitcoin-testnet-rpc.publicnode.com")
}

/// Tests transaction validation according to BPC-3 standard
fn test_transaction_validation(tester: &Arc<UnifiedTester>) -> Result<(), String> {
    info!("Testing transaction validation...");
    
    // Use the Bitcoin validator from the unified tester
    tester.bitcoin_validator.validate_transaction()
}

/// Tests PSBT (BIP-174) handling according to BPC-3 standard
fn test_psbt_handling(tester: &Arc<UnifiedTester>) -> Result<(), String> {
    info!("Testing PSBT handling...");
    
    // Use the Bitcoin validator from the unified tester
    tester.bitcoin_validator.test_psbt_workflow()
}

fn test_mempool_access() -> Result<(), String> {
    info!("Testing mempool access...");
    
    let output = Command::new("bitcoin-cli")
        .args(&["getmempoolinfo"])
        .output();
        
    match output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to access mempool: {}", error));
            }
            
            let info = String::from_utf8_lossy(&output.stdout);
            info!("Mempool access successful: {}", info);
            Ok(())
        },
        Err(e) => Err(format!("Failed to access mempool: {}", e)),
    }
}

fn test_blockchain_info() -> Result<(), String> {
    info!("Testing blockchain info...");
    
    let output = Command::new("bitcoin-cli")
        .args(&["getblockchaininfo"])
        .output();
        
    match output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to get blockchain info: {}", error));
            }
            
            let info = String::from_utf8_lossy(&output.stdout);
            
            // Check for BIP-341 (Taproot) activation
            match serde_json::from_str::<serde_json::Value>(&info) {
                Ok(json) => {
                    if let Some(softforks) = json.get("softforks") {
                        if let Some(taproot) = softforks.get("taproot") {
                            if let Some(active) = taproot.get("active") {
                                if active.as_bool().unwrap_or(false) {
                                    info!("Taproot is active on this network");
                                } else {
                                    warn!("Taproot is not active on this network");
                                }
                            }
                        }
                    }
                },
                Err(e) => warn!("Failed to parse blockchain info: {}", e),
            }
            
            info!("Blockchain info successful");
            Ok(())
        },
        Err(e) => Err(format!("Failed to get blockchain info: {}", e)),
    }
}

// Utility function to wait for a transaction to confirm
fn wait_for_confirmation(txid: &str, confirmations: u32) -> Result<(), String> {
    info!("Waiting for transaction {} to confirm ({} confirmations)...", txid, confirmations);
    
    let max_attempts = 30;
    let mut attempts = 0;
    
    while attempts < max_attempts {
        let output = Command::new("bitcoin-cli")
            .args(&["gettransaction", txid])
            .output();
            
        match output {
            Ok(output) => {
                if output.status.success() {
                    let tx_info = String::from_utf8_lossy(&output.stdout);
                    
                    match serde_json::from_str::<serde_json::Value>(&tx_info) {
                        Ok(json) => {
                            if let Some(confs) = json.get("confirmations") {
                                if let Some(n) = confs.as_u64() {
                                    if n >= confirmations as u64 {
                                        info!("Transaction confirmed with {} confirmations", n);
                                        return Ok(());
                                    } else {
                                        info!("Transaction has {} confirmations, waiting for {}...", n, confirmations);
                                    }
                                }
                            }
                        },
                        Err(e) => warn!("Failed to parse transaction info: {}", e),
                    }
                }
            },
            Err(e) => warn!("Failed to get transaction info: {}", e),
        }
        
        // Generate a new block if in regtest mode
        let _ = Command::new("bitcoin-cli")
            .args(&["generatetoaddress", "1", "bcrt1qekmmdpuyd7kpetypht0fd3znt5lev2g2ykm3le"])
            .output();
            
        std::thread::sleep(std::time::Duration::from_secs(1));
        attempts += 1;
    }
    
    Err(format!("Transaction {} failed to confirm after {} attempts", txid, max_attempts))
} 
