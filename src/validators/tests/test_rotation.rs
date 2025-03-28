use super::*;
use tempfile::tempdir;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_validator_initialization() {
    // Create a temporary directory for test files
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("validators.json").to_str().unwrap().to_string();
    
    // Create a rotation manager
    let mut manager = ValidatorRotationManager::new(&config_path).expect("Failed to create manager");
    
    // Initialize validators (3-of-5 multisig)
    let threshold = 3;
    let num_validators = 5;
    let use_hsm = false; // Use software keys for testing
    
    manager.initialize_validators(threshold, num_validators, use_hsm)
        .expect("Failed to initialize validators");
    
    // Verify the config file was created
    assert!(Path::new(&config_path).exists(), "Config file not created");
    
    // Load the config file to verify the content
    let config_content = fs::read_to_string(&config_path).expect("Failed to read config file");
    let config: ValidatorConfig = serde_json::from_str(&config_content).expect("Failed to parse config");
    
    assert_eq!(config.threshold, threshold, "Incorrect threshold value");
    assert_eq!(config.validators.len(), num_validators, "Incorrect number of validators");
    assert_eq!(config.previous_validators.len(), 0, "Should have no previous validators");
    
    // All validators should be active and not HSM-backed
    for validator in &config.validators {
        assert!(validator.is_active, "Validator should be active");
        assert!(!validator.hsm_backed, "Validator should not be HSM-backed");
    }
}

#[test]
fn test_validator_rotation() {
    // Create a temporary directory for test files
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("validators.json").to_str().unwrap().to_string();
    
    // Create and initialize rotation manager
    let mut manager = ValidatorRotationManager::new(&config_path).expect("Failed to create manager");
    let threshold = 2;
    let num_validators = 3;
    manager.initialize_validators(threshold, num_validators, false)
        .expect("Failed to initialize validators");
    
    // Store original validators for comparison
    let config_content = fs::read_to_string(&config_path).expect("Failed to read config file");
    let original_config: ValidatorConfig = serde_json::from_str(&config_content).expect("Failed to parse config");
    let original_validators = original_config.validators.clone();
    
    // Manipulate the last_rotation time to force rotation
    let mut config = manager.config.clone();
    config.last_rotation = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() - (config.rotation_period_days * 86400 + 3600); // Add 1 hour to ensure it's overdue
    
    // Update config file with manipulated time
    let config_json = serde_json::to_string_pretty(&config).expect("Failed to serialize config");
    fs::write(&config_path, config_json).expect("Failed to write config");
    
    // Create a new manager to load the updated config
    let mut manager = ValidatorRotationManager::new(&config_path).expect("Failed to reload manager");
    
    // Check rotation status
    match manager.check_rotation_status() {
        RotationStatus::RotationNeeded { days_overdue, .. } => {
            assert!(days_overdue > 0, "Rotation should be needed");
        },
        _ => panic!("Rotation should be needed"),
    }
    
    // Perform rotation
    let rotation_result = manager.rotate_validators(false).expect("Failed to rotate validators");
    
    // Verify rotation results
    assert!(rotation_result.rotated, "Rotation should have occurred");
    assert_eq!(rotation_result.new_validators.len(), num_validators, "Should have same number of validators");
    assert_eq!(rotation_result.deactivated_validators.len(), num_validators, "Should have deactivated all validators");
    
    // Load the updated config
    let config_content = fs::read_to_string(&config_path).expect("Failed to read updated config");
    let updated_config: ValidatorConfig = serde_json::from_str(&config_content).expect("Failed to parse updated config");
    
    // Compare configs
    assert_eq!(updated_config.validators.len(), num_validators, "Should have same number of validators");
    assert_eq!(updated_config.previous_validators.len(), num_validators, "Should have previous validators");
    
    // Verify keys are different after rotation
    let mut all_different = true;
    for (old_key, new_key) in original_validators.iter().zip(updated_config.validators.iter()) {
        if old_key.public_key == new_key.public_key {
            all_different = false;
            break;
        }
    }
    assert!(all_different, "New keys should be different from old keys");
    
    // Verify addresses are generated
    let addresses = manager.get_multisig_addresses().expect("Failed to get multisig addresses");
    assert_eq!(addresses.threshold, threshold, "Address threshold should match");
    assert_eq!(addresses.validator_count, num_validators, "Address validator count should match");
    
    // Verify we have all address types
    assert!(!addresses.legacy.is_empty(), "Legacy address should not be empty");
    assert!(!addresses.segwit.is_empty(), "SegWit address should not be empty");
    assert!(!addresses.native_segwit.is_empty(), "Native SegWit address should not be empty");
    assert!(!addresses.taproot.is_empty(), "Taproot address should not be empty");
}

#[test]
fn test_hsm_integration() {
    // Create a temporary directory for test files
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("validators.json").to_str().unwrap().to_string();
    
    // Create a rotation manager
    let mut manager = ValidatorRotationManager::new(&config_path).expect("Failed to create manager");
    
    // Initialize validators with HSM
    let use_hsm = true;
    manager.initialize_validators(2, 3, use_hsm)
        .expect("Failed to initialize validators with HSM");
    
    // Verify the validators are HSM-backed
    let config_content = fs::read_to_string(&config_path).expect("Failed to read config file");
    let config: ValidatorConfig = serde_json::from_str(&config_content).expect("Failed to parse config");
    
    // All validators should be HSM-backed
    for validator in &config.validators {
        assert!(validator.hsm_backed, "Validator should be HSM-backed");
    }
} 