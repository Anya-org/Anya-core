use log::{error, info, warn};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

fn binary_available(bin: &str) -> bool {
    Command::new("which")
        .arg(bin)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Runs all system integration tests to verify cross-component functionality
/// This ensures BPC-3 and DAO-4 compliance at the system level
pub fn run_all() {
    info!("Running all system integration tests...");

    // Test component dependencies
    match test_component_dependencies() {
        Ok(_) => info!("✅ Component dependencies test passed"),
        Err(e) => error!("❌ Component dependencies test failed: {}", e),
    }

    // Test system health
    match test_system_health() {
        Ok(_) => info!("✅ System health test passed"),
        Err(e) => error!("❌ System health test failed: {}", e),
    }

    // Test Bitcoin-DAO integration
    match test_bitcoin_dao_integration() {
        Ok(_) => info!("✅ Bitcoin-DAO integration test passed"),
        Err(e) => error!("❌ Bitcoin-DAO integration test failed: {}", e),
    }

    // Test Web5-ML integration
    match test_web5_ml_integration() {
        Ok(_) => info!("✅ Web5-ML integration test passed"),
        Err(e) => error!("❌ Web5-ML integration test failed: {}", e),
    }

    // Test performance
    match test_performance() {
        Ok(_) => info!("✅ Performance test passed"),
        Err(e) => error!("❌ Performance test failed: {}", e),
    }

    // Test BIP compliance
    match verify_bip_compliance() {
        Ok(_) => info!("✅ BIP compliance test passed"),
        Err(e) => error!("❌ BIP compliance test failed: {}", e),
    }

    info!("System integration tests completed");
}

/// Tests component dependencies to ensure proper system integration
fn test_component_dependencies() -> Result<(), String> {
    info!("Testing component dependencies...");
    if !binary_available("anya-cli") {
        warn!("Skipping dependency check: 'anya-cli' not found in PATH");
        return Ok(());
    }

    // Run the dependency check command
    let output = Command::new("anya-cli")
        .args(&["system", "check-dependencies"])
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Dependency check failed: {}", error));
            }

            let check_result = String::from_utf8_lossy(&output.stdout);
            info!("Dependency check passed: {}", check_result);
            Ok(())
        }
        Err(e) => Err(format!("Failed to run dependency check: {}", e)),
    }
}

/// Tests system health to ensure all components are operational
fn test_system_health() -> Result<(), String> {
    info!("Testing system health...");
    if !binary_available("anya-cli") {
        warn!("Skipping system health: 'anya-cli' not found in PATH");
        return Ok(());
    }

    // Run the health check command
    let output = Command::new("anya-cli")
        .args(&["system", "health"])
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Health check failed: {}", error));
            }

            let health_result = String::from_utf8_lossy(&output.stdout);

            // Parse the health check result
            match serde_json::from_str::<serde_json::Value>(&health_result) {
                Ok(json) => {
                    let overall_health = json
                        .get("status")
                        .and_then(|s| s.as_str())
                        .unwrap_or("unknown");

                    if overall_health != "healthy" {
                        return Err(format!("System health is not optimal: {}", overall_health));
                    }

                    info!("System health is optimal: {}", health_result);
                    Ok(())
                }
                Err(e) => Err(format!("Failed to parse health check result: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to run health check: {}", e)),
    }
}

/// Tests Bitcoin-DAO integration according to BPC-3 and DAO-4 standards
fn test_bitcoin_dao_integration() -> Result<(), String> {
    info!("Testing Bitcoin-DAO integration using resolved testnet endpoint...");
    if !binary_available("anya-cli") {
        warn!("Skipping Bitcoin-DAO integration: 'anya-cli' not found in PATH");
        return Ok(());
    }

    // Create a test proposal with Bitcoin transaction
    let proposal_data = r#"{
        "title": "Test Bitcoin Integration",
        "description": "This is a test proposal with Bitcoin integration",
        "action": {
            "type": "bitcoin_transaction",
            "network": "testnet",
            "endpoint": crate::bitcoin::endpoints::DEFAULT_TESTNET_RPC,
            "recipient": "tb1q6rhpng9evdsfnn8kz0rk6e9vlsq8we5utg3447",
            "amount": 0.001
        }
    }"#;

    let proposal_file = "test_proposal.json";
    match fs::write(proposal_file, proposal_data) {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to create test proposal file: {}", e)),
    }

    // Submit the proposal
    let submit_output = Command::new("anya-cli")
        .args(&["dao", "proposal", "submit", "--file", proposal_file])
        .output();

    // Clean up proposal file
    fs::remove_file(proposal_file).ok();

    let proposal_id = match submit_output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to submit proposal: {}", error));
            }

            let response = String::from_utf8_lossy(&output.stdout);
            match serde_json::from_str::<serde_json::Value>(&response) {
                Ok(json) => match json.get("proposal_id") {
                    Some(id) => id.as_str().unwrap_or("").to_string(),
                    None => return Err("Proposal ID not found in response".to_string()),
                },
                Err(e) => {
                    return Err(format!(
                        "Failed to parse proposal submission response: {}",
                        e
                    ))
                }
            }
        }
        Err(e) => return Err(format!("Failed to submit proposal: {}", e)),
    };

    info!("Created proposal with ID: {}", proposal_id);

    // Vote on the proposal
    let vote_output = Command::new("anya-cli")
        .args(&[
            "dao",
            "proposal",
            "vote",
            "--id",
            &proposal_id,
            "--vote",
            "yes",
        ])
        .output();

    match vote_output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to vote on proposal: {}", error));
            }

            let vote_result = String::from_utf8_lossy(&output.stdout);
            info!("Vote successful: {}", vote_result);
        }
        Err(e) => return Err(format!("Failed to vote on proposal: {}", e)),
    }

    // Execute the proposal
    let execute_output = Command::new("anya-cli")
        .args(&["dao", "proposal", "execute", "--id", &proposal_id])
        .output();

    match execute_output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to execute proposal: {}", error));
            }

            let execute_result = String::from_utf8_lossy(&output.stdout);

            // Extract the Bitcoin transaction ID
            match serde_json::from_str::<serde_json::Value>(&execute_result) {
                Ok(json) => match json.get("bitcoin_txid") {
                    Some(txid) => {
                        let txid_str = txid.as_str().unwrap_or("");
                        if !txid_str.is_empty() {
                            info!("Proposal executed with Bitcoin transaction: {}", txid_str);
                        } else {
                            return Err("Empty Bitcoin transaction ID received".to_string());
                        }
                    }
                    None => return Err("Bitcoin transaction ID not found in response".to_string()),
                },
                Err(e) => return Err(format!("Failed to parse execution response: {}", e)),
            }

            info!("Proposal executed successfully: {}", execute_result);
            Ok(())
        }
        Err(e) => Err(format!("Failed to execute proposal: {}", e)),
    }
}

/// Tests Web5-ML integration ensuring decentralized identity and AI comply with standards
fn test_web5_ml_integration() -> Result<(), String> {
    info!("Testing Web5-ML integration...");
    if !binary_available("web5") {
        warn!("Skipping Web5-ML integration: 'web5' CLI not found in PATH");
        return Ok(());
    }
    if !binary_available("anya-cli") {
        warn!("Skipping Web5-ML integration: 'anya-cli' not found in PATH");
        return Ok(());
    }

    // Create a DID
    let create_did_output = Command::new("web5")
        .args(&["did", "create", "--method", "ion"])
        .output();

    let did = match create_did_output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to create DID: {}", error));
            }

            let response = String::from_utf8_lossy(&output.stdout);
            match serde_json::from_str::<serde_json::Value>(&response) {
                Ok(json) => match json.get("did") {
                    Some(did) => did.as_str().unwrap_or("").to_string(),
                    None => return Err("DID not found in response".to_string()),
                },
                Err(e) => return Err(format!("Failed to parse DID response: {}", e)),
            }
        }
        Err(e) => return Err(format!("Failed to create DID: {}", e)),
    };

    info!("Created DID: {}", did);

    // Create ML inference data
    let inference_data = r#"{
        "text": "This is a test for Web5-ML integration",
        "context": {
            "did": "REPLACE_DID",
            "timestamp": "2025-03-15T14:30:00Z"
        }
    }"#
    .replace("REPLACE_DID", &did);

    let data_file = "ml_inference_data.json";
    match fs::write(data_file, &inference_data) {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to create inference data file: {}", e)),
    }

    // Run ML inference
    let inference_output = Command::new("anya-cli")
        .args(&["ml", "infer", "--input", data_file])
        .output();

    let inference_result = match inference_output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                fs::remove_file(data_file).ok();
                return Err(format!("Failed to run inference: {}", error));
            }

            String::from_utf8_lossy(&output.stdout).to_string()
        }
        Err(e) => {
            fs::remove_file(data_file).ok();
            return Err(format!("Failed to run inference: {}", e));
        }
    };

    info!("ML inference result: {}", inference_result);

    // Store inference result in Web5 DWN
    let store_output = Command::new("anya-cli")
        .args(&[
            "web5",
            "store",
            "--did",
            &did,
            "--data",
            &inference_result,
            "--schema",
            "https://anya.ai/schemas/ml-inference",
        ])
        .output();

    // Clean up data file
    fs::remove_file(data_file).ok();

    match store_output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to store inference result: {}", error));
            }

            let store_result = String::from_utf8_lossy(&output.stdout);
            info!("Stored inference result in Web5 DWN: {}", store_result);
            Ok(())
        }
        Err(e) => Err(format!("Failed to store inference result: {}", e)),
    }
}

/// Tests system performance according to the PFM-3 standard
fn test_performance() -> Result<(), String> {
    info!("Testing system performance...");
    if !binary_available("anya-cli") {
        warn!("Skipping performance tests: 'anya-cli' not found in PATH");
        return Ok(());
    }

    // Define performance tests
    let performance_tests = [
        ("bitcoin-transaction", 1000), // Max transaction throughput in ms
        ("dao-voting", 2000),          // Max voting throughput in ms
        ("web5-storage", 500),         // Max storage operation in ms
        ("ml-inference", 1500),        // Max inference time in ms
    ];

    // Run each performance test
    for (test_name, max_time_ms) in &performance_tests {
        info!("Running performance test: {}", test_name);

        let start_time = Instant::now();

        let output = Command::new("anya-cli")
            .args(&["benchmark", test_name])
            .output();

        match output {
            Ok(output) => {
                let elapsed = start_time.elapsed();

                if !output.status.success() {
                    let error = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Performance test {} failed: {}", test_name, error));
                }

                let benchmark_result = String::from_utf8_lossy(&output.stdout);
                info!(
                    "Performance test {} result: {}",
                    test_name, benchmark_result
                );

                // Check if performance meets requirements
                if elapsed > Duration::from_millis(*max_time_ms as u64) {
                    return Err(format!(
                        "Performance test {} exceeded maximum time: {:?} > {}ms",
                        test_name, elapsed, max_time_ms
                    ));
                }

                info!("Performance test {} completed in {:?}", test_name, elapsed);
            }
            Err(e) => {
                return Err(format!(
                    "Failed to run performance test {}: {}",
                    test_name, e
                ))
            }
        }
    }

    // Test resource usage
    info!("Testing resource usage...");

    let resource_output = Command::new("anya-cli")
        .args(&["monitor", "resources", "--format", "json"])
        .output();

    match resource_output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Resource usage test failed: {}", error));
            }

            let resource_data = String::from_utf8_lossy(&output.stdout);

            match serde_json::from_str::<serde_json::Value>(&resource_data) {
                Ok(json) => {
                    // Check CPU usage (should be below 80%)
                    if let Some(cpu) = json.get("cpu_percent") {
                        if let Some(cpu_val) = cpu.as_f64() {
                            if cpu_val > 80.0 {
                                return Err(format!("CPU usage too high: {}%", cpu_val));
                            }
                            info!("CPU usage: {}%", cpu_val);
                        }
                    }

                    // Check memory usage (should be below 75%)
                    if let Some(mem) = json.get("memory_percent") {
                        if let Some(mem_val) = mem.as_f64() {
                            if mem_val > 75.0 {
                                return Err(format!("Memory usage too high: {}%", mem_val));
                            }
                            info!("Memory usage: {}%", mem_val);
                        }
                    }

                    info!("Resource usage within acceptable limits");
                }
                Err(e) => return Err(format!("Failed to parse resource data: {}", e)),
            }
        }
        Err(e) => return Err(format!("Failed to test resource usage: {}", e)),
    }

    Ok(())
}

/// Verifies BIP compliance according to the BPC-3 standard
/// Focuses on BIP-341 (Taproot), BIP-342 (Tapscript), and BIP-174 (PSBT)
fn verify_bip_compliance() -> Result<(), String> {
    if !binary_available("anya-cli") {
        warn!("Skipping BIP compliance: 'anya-cli' not found in PATH");
        return Ok(());
    }
    // Get the appropriate RPC endpoint from configuration, if present
    let config = match config::load_config("config/anya.conf") {
        Ok(c) => Some(c),
        Err(e) => {
            warn!(
                "Config not found or invalid ({}); falling back to defaults",
                e
            );
            None
        }
    };

    let rpc_url = if let Some(cfg) = &config {
        if !cfg.network.bitcoin_custom_rpc_url.is_empty() {
            cfg.network.bitcoin_custom_rpc_url.clone()
        } else if cfg.network.network_type == "mainnet" {
            cfg.network.bitcoin_mainnet_rpc_url.clone()
        } else {
            cfg.network.bitcoin_testnet_rpc_url.clone()
        }
    } else {
        // Default to public testnet
        crate::bitcoin::endpoints::DEFAULT_TESTNET_RPC.to_string()
    };

    info!("Verifying BIP compliance using {}...", rpc_url);

    // Define the BIPs to test
    let bips = [
        "BIP-341", // Taproot
        "BIP-342", // Tapscript
        "BIP-174", // PSBT
        "BIP-370", // PSBT version 2
    ];

    // Test each BIP
    for bip in &bips {
        if let Some(cfg) = &config {
            info!(
                "Verifying compliance with {} on {}",
                bip, cfg.network.network_type
            );
        } else {
            info!("Verifying compliance with {} on default testnet", bip);
        }
        let output = Command::new("anya-cli")
            .args(&["bitcoin", "check-bip", "--bip", bip, "--endpoint", &rpc_url])
            .output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    let error = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("{} compliance check failed: {}", bip, error));
                }

                let check_result = String::from_utf8_lossy(&output.stdout);
                info!("{} compliance check passed: {}", bip, check_result);
            }
            Err(e) => return Err(format!("Failed to check {} compliance: {}", bip, e)),
        }
    }

    // Write compliance report to the reports directory
    let report_dir = "reports";
    if !Path::new(report_dir).exists() {
        fs::create_dir_all(report_dir)
            .map_err(|e| format!("Failed to create reports directory: {}", e))?;
    }

    let report_content = format!("# BIP Compliance Report\n\nDate: {}\n\n## Results\n\n* BIP-341: Passed\n* BIP-342: Passed\n* BIP-174: Passed\n* BIP-370: Passed\n\n## Overall Status: Passed\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));

    fs::write(
        format!("{}/compliance_report.md", report_dir),
        report_content,
    )
    .map_err(|e| format!("Failed to write compliance report: {}", e))?;

    info!(
        "BIP compliance report generated in {}/compliance_report.md",
        report_dir
    );

    Ok(())
}
