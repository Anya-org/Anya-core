#![feature(edition2021)]
use log::{info, warn, error};
use std::process::Command;
use std::path::Path;

pub fn run_all() {
    info!("Running all DAO tests...");
    
    // Test DAO contracts
    match test_dao_contracts() {
        Ok(_) => info!("✅ DAO contracts test passed"),
        Err(e) => error!("❌ DAO contracts test failed: {}", e),
    }
    
    // Test governance functions
    match test_governance() {
        Ok(_) => info!("✅ Governance test passed"),
        Err(e) => error!("❌ Governance test failed: {}", e),
    }
    
    // Test voting mechanism
    match test_voting() {
        Ok(_) => info!("✅ Voting mechanism test passed"),
        Err(e) => error!("❌ Voting mechanism test failed: {}", e),
    }
    
    // Test proposal execution
    match test_proposal_execution() {
        Ok(_) => info!("✅ Proposal execution test passed"),
        Err(e) => error!("❌ Proposal execution test failed: {}", e),
    }
    
    info!("DAO tests completed");
}

fn test_dao_contracts() -> Result<(), String> {
    info!("Testing DAO contracts...");
    
    // Check if Clarinet is installed
    let clarinet_output = Command::new("clarinet")
        .arg("--version")
        .output();
        
    match clarinet_output {
        Ok(output) => {
            if !output.status.success() {
                return Err("Clarinet is not properly installed".to_string());
            }
            
            let version_str = String::from_utf8_lossy(&output.stdout);
            info!("Clarinet version: {}", version_str.trim());
        },
        Err(e) => return Err(format!("Failed to execute Clarinet: {}", e)),
    }
    
    // Check for DAO contracts
    let dao_contracts_dir = "config/dao/contracts";
    if !Path::new(dao_contracts_dir).exists() {
        return Err(format!("DAO contracts directory not found: {}", dao_contracts_dir));
    }
    
    // Run Clarinet checks on the contracts
    let check_output = Command::new("clarinet")
        .args(&["check", "--manifest-path", "config/dao/Clarinet.toml"])
        .output();
        
    match check_output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Clarinet check failed: {}", error));
            }
            
            let check_result = String::from_utf8_lossy(&output.stdout);
            info!("Clarinet check passed: {}", check_result);
        },
        Err(e) => return Err(format!("Failed to run Clarinet check: {}", e)),
    }
    
    Ok(())
}

fn test_governance() -> Result<(), String> {
    info!("Testing DAO governance functions...");
    
    // Run Clarinet test for governance
    let test_output = Command::new("clarinet")
        .args(&["test", "tests/governance_test.ts", "--manifest-path", "config/dao/Clarinet.toml"])
        .output();
        
    match test_output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Governance test failed: {}", error));
            }
            
            let test_result = String::from_utf8_lossy(&output.stdout);
            info!("Governance test passed: {}", test_result);
        },
        Err(e) => return Err(format!("Failed to run governance test: {}", e)),
    }
    
    Ok(())
}

fn test_voting() -> Result<(), String> {
    info!("Testing DAO voting mechanism...");
    
    // Run Clarinet test for voting
    let test_output = Command::new("clarinet")
        .args(&["test", "tests/voting_test.ts", "--manifest-path", "config/dao/Clarinet.toml"])
        .output();
        
    match test_output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Voting test failed: {}", error));
            }
            
            let test_result = String::from_utf8_lossy(&output.stdout);
            info!("Voting test passed: {}", test_result);
        },
        Err(e) => return Err(format!("Failed to run voting test: {}", e)),
    }
    
    Ok(())
}

fn test_proposal_execution() -> Result<(), String> {
    info!("Testing DAO proposal execution...");
    
    // Run Clarinet test for proposal execution
    let test_output = Command::new("clarinet")
        .args(&["test", "tests/proposal_execution_test.ts", "--manifest-path", "config/dao/Clarinet.toml"])
        .output();
        
    match test_output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Proposal execution test failed: {}", error));
            }
            
            let test_result = String::from_utf8_lossy(&output.stdout);
            info!("Proposal execution test passed: {}", test_result);
        },
        Err(e) => return Err(format!("Failed to run proposal execution test: {}", e)),
    }
    
    Ok(())
} 