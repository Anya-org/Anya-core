fn test_psbt_handling() -> Result<(), String> {
    info!("Testing PSBT handling...");
    
    // Create a new PSBT
    let create_psbt_output = Command::new("bitcoin-cli")
        .args(&["-rpcwallet=taproot_test", "walletcreatefundedpsbt", "[]", "[{\"bcrt1q6rhpng9evdsfnn8kz0rk6e9vlsq8we5utg3447\":0.001}]"])
        .output();
        
    let psbt_base64 = match create_psbt_output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to create PSBT: {}", error));
            }
            
            // Parse the JSON response to extract the PSBT
            let response = String::from_utf8_lossy(&output.stdout);
            match serde_json::from_str::<serde_json::Value>(&response) {
                Ok(json) => {
                    match json.get("psbt") {
                        Some(psbt) => psbt.as_str().unwrap_or("").to_string(),
                        None => return Err("PSBT not found in response".to_string()),
                    }
                },
                Err(e) => return Err(format!("Failed to parse PSBT response: {}", e)),
            }
        },
        Err(e) => return Err(format!("Failed to create PSBT: {}", e)),
    };
    
    if psbt_base64.is_empty() {
        return Err("Empty PSBT received".to_string());
    }
    
    info!("Created PSBT: {}", psbt_base64);
    
    // Process the PSBT
    let process_psbt_output = Command::new("bitcoin-cli")
        .args(&["-rpcwallet=taproot_test", "walletprocesspsbt", &psbt_base64])
        .output();
        
    match process_psbt_output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to process PSBT: {}", error));
            }
            
            let response = String::from_utf8_lossy(&output.stdout);
            info!("Processed PSBT: {}", response);
            Ok(())
        },
        Err(e) => Err(format!("Failed to process PSBT: {}", e)),
    }
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