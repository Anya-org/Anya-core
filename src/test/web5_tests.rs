use std::error::Error;
use log::{info, warn, error};
use std::process::Command;
use std::path::Path;
use std::fs;

pub fn run_all() {
    info!("Running all Web5 tests...");
    
    // Test DWN connections
    match test_dwn_connection() {
        Ok(_) => info!("✅ DWN connection test passed"),
        Err(e) => error!("❌ DWN connection test failed: {}", e),
    }
    
    // Test DID creation and resolution
    match test_did_operations() {
        Ok(_) => info!("✅ DID operations test passed"),
        Err(e) => error!("❌ DID operations test failed: {}", e),
    }
    
    // Test data storage and retrieval
    match test_data_operations() {
        Ok(_) => info!("✅ Data operations test passed"),
        Err(e) => error!("❌ Data operations test failed: {}", e),
    }
    
    // Test protocol definitions
    match test_protocols() {
        Ok(_) => info!("✅ Protocol definitions test passed"),
        Err(e) => error!("❌ Protocol definitions test failed: {}", e),
    }
    
    info!("Web5 tests completed");
}

fn test_dwn_connection() -> Result<(), String> {
    info!("Testing DWN connection...");
    
    // Check if the DWN configuration exists
    let dwn_config_path = "config/web5/dwn/config.json";
    if !Path::new(dwn_config_path).exists() {
        return Err(format!("DWN configuration not found: {}", dwn_config_path));
    }
    
    // Read the configuration to get the endpoint
    let config_content = match fs::read_to_string(dwn_config_path) {
        Ok(content) => content,
        Err(e) => return Err(format!("Failed to read DWN configuration: {}", e)),
    };
    
    let config: serde_json::Value = match serde_json::from_str(&config_content) {
        Ok(config) => config,
        Err(e) => return Err(format!("Failed to parse DWN configuration: {}", e)),
    };
    
    let endpoint = match config.get("endpoint").and_then(|e| e.as_str()) {
        Some(endpoint) => endpoint,
        None => return Err("DWN endpoint not found in configuration".to_string()),
    };
    
    // Test connection to the DWN endpoint
    let output = Command::new("curl")
        .args(&["-s", "-o", "/dev/null", "-w", "%{http_code}", endpoint])
        .output();
        
    match output {
        Ok(output) => {
            let status_code = String::from_utf8_lossy(&output.stdout);
            match status_code.trim().parse::<u16>() {
                Ok(code) if code >= 200 && code < 300 => {
                    info!("DWN connection successful, status code: {}", code);
                    Ok(())
                },
                Ok(code) => Err(format!("DWN connection failed, status code: {}", code)),
                Err(e) => Err(format!("Failed to parse status code: {}", e)),
            }
        },
        Err(e) => Err(format!("Failed to connect to DWN: {}", e)),
    }
}

fn test_did_operations() -> Result<(), String> {
    info!("Testing DID operations...");
    
    // Check for Web5 CLI tool
    let web5_output = Command::new("web5")
        .arg("--version")
        .output();
        
    match web5_output {
        Ok(output) => {
            if !output.status.success() {
                return Err("Web5 CLI tool is not properly installed".to_string());
            }
            
            let version_str = String::from_utf8_lossy(&output.stdout);
            info!("Web5 CLI version: {}", version_str.trim());
        },
        Err(e) => return Err(format!("Failed to execute Web5 CLI: {}", e)),
    }
    
    // Create a test DID
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
                Ok(json) => {
                    match json.get("did") {
                        Some(did) => did.as_str().unwrap_or("").to_string(),
                        None => return Err("DID not found in response".to_string()),
                    }
                },
                Err(e) => return Err(format!("Failed to parse DID response: {}", e)),
            }
        },
        Err(e) => return Err(format!("Failed to create DID: {}", e)),
    };
    
    if did.is_empty() || !did.starts_with("did:") {
        return Err(format!("Invalid DID created: {}", did));
    }
    
    info!("Created DID: {}", did);
    
    // Resolve the DID
    let resolve_did_output = Command::new("web5")
        .args(&["did", "resolve", &did])
        .output();
        
    match resolve_did_output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to resolve DID: {}", error));
            }
            
            let response = String::from_utf8_lossy(&output.stdout);
            info!("DID resolved successfully: {}", response);
            Ok(())
        },
        Err(e) => Err(format!("Failed to resolve DID: {}", e)),
    }
}

fn test_data_operations() -> Result<(), String> {
    info!("Testing Web5 data operations...");
    
    // Create a simple record
    let record_data = r#"{"test": "data", "timestamp": "2025-03-15T12:00:00Z"}"#;
    let temp_file = "temp_record.json";
    
    match fs::write(temp_file, record_data) {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to create temporary record file: {}", e)),
    }
    
    // Store the record in DWN
    let store_output = Command::new("web5")
        .args(&["dwn", "records", "write", "--file", temp_file, "--schema", "https://example.com/testRecord"])
        .output();
        
    let record_id = match store_output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                fs::remove_file(temp_file).ok();
                return Err(format!("Failed to store record: {}", error));
            }
            
            let response = String::from_utf8_lossy(&output.stdout);
            match serde_json::from_str::<serde_json::Value>(&response) {
                Ok(json) => {
                    match json.get("recordId") {
                        Some(id) => id.as_str().unwrap_or("").to_string(),
                        None => {
                            fs::remove_file(temp_file).ok();
                            return Err("Record ID not found in response".to_string());
                        },
                    }
                },
                Err(e) => {
                    fs::remove_file(temp_file).ok();
                    return Err(format!("Failed to parse store response: {}", e));
                },
            }
        },
        Err(e) => {
            fs::remove_file(temp_file).ok();
            return Err(format!("Failed to store record: {}", e));
        },
    };
    
    // Clean up temporary file
    fs::remove_file(temp_file).ok();
    
    if record_id.is_empty() {
        return Err("Empty record ID received".to_string());
    }
    
    info!("Stored record with ID: {}", record_id);
    
    // Retrieve the record
    let retrieve_output = Command::new("web5")
        .args(&["dwn", "records", "read", "--recordId", &record_id])
        .output();
        
    match retrieve_output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to retrieve record: {}", error));
            }
            
            let response = String::from_utf8_lossy(&output.stdout);
            info!("Record retrieved successfully: {}", response);
            Ok(())
        },
        Err(e) => Err(format!("Failed to retrieve record: {}", e)),
    }
}

fn test_protocols() -> Result<(), String> {
    info!("Testing Web5 protocol definitions...");
    
    // Check for protocol definition file
    let protocol_path = "config/web5/protocols/test-protocol.json";
    if !Path::new(protocol_path).exists() {
        return Err(format!("Protocol definition not found: {}", protocol_path));
    }
    
    // Create protocol instance
    let create_protocol_output = Command::new("web5")
        .args(&["protocols", "configure", "--from", protocol_path])
        .output();
        
    match create_protocol_output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to create protocol: {}", error));
            }
            
            let response = String::from_utf8_lossy(&output.stdout);
            info!("Protocol created successfully: {}", response);
            Ok(())
        },
        Err(e) => Err(format!("Failed to create protocol: {}", e)),
    }
} 
