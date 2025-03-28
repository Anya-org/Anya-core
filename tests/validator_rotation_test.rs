use anya_core::validators::rotation::{
    ValidatorRotationManager,
    ValidatorConfig,
    ValidatorKey,
    MultisigAddresses,
    RotationResult,
    RotationStatus
};
use tempfile::tempdir;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_validator_lifecycle() {
    // Create temp directory for config
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("validators.json").to_str().unwrap().to_string();
    
    // 1. Create and initialize validators
    let mut manager = ValidatorRotationManager::new(&config_path).expect("Failed to create manager");
    manager.initialize_validators(2, 3, false).expect("Failed to initialize validators");
    
    // Verify config was created
    let config_file = Path::new(&config_path);
    assert!(config_file.exists(), "Config file should exist");
    
    // 2. Get multisig addresses
    let addresses = manager.get_multisig_addresses().expect("Failed to get addresses");
    println!("Multisig Addresses ({}-of-{}):", addresses.threshold, addresses.validator_count);
    println!("Legacy: {}", addresses.legacy);
    println!("SegWit: {}", addresses.segwit);
    println!("Native SegWit: {}", addresses.native_segwit);
    println!("Taproot: {}", addresses.taproot);
    
    // Validate addresses
    assert_eq!(addresses.threshold, 2, "Threshold should be 2");
    assert_eq!(addresses.validator_count, 3, "Should have 3 validators");
    
    // 3. Check rotation status (should be valid since we just created it)
    match manager.check_rotation_status() {
        RotationStatus::Valid { days_remaining, .. } => {
            println!("Rotation not needed. Days remaining: {}", days_remaining);
            assert!(days_remaining > 0, "Should have days remaining");
        },
        RotationStatus::RotationNeeded { .. } => {
            panic!("Rotation should not be needed yet");
        }
    }
    
    // 4. Force rotation by manipulating the config
    // Load the config, update the last_rotation time, and save it back
    let mut config: ValidatorConfig = {
        let config_json = fs::read_to_string(&config_path).expect("Failed to read config");
        serde_json::from_str(&config_json).expect("Failed to parse config")
    };
    
    // Set last_rotation to 31 days ago
    config.last_rotation = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to calculate timestamp")
        .as_secs() - (31 * 24 * 60 * 60);
    
    // Save the modified config
    let config_json = serde_json::to_string_pretty(&config).expect("Failed to serialize config");
    fs::write(&config_path, config_json).expect("Failed to write config");
    
    // Reload the manager to apply changed config
    let mut manager = ValidatorRotationManager::new(&config_path).expect("Failed to reload manager");
    
    // 5. Check rotation status again (should need rotation now)
    match manager.check_rotation_status() {
        RotationStatus::Valid { .. } => {
            panic!("Rotation should be needed");
        },
        RotationStatus::RotationNeeded { days_overdue, .. } => {
            println!("Rotation needed. Days overdue: {}", days_overdue);
            assert!(days_overdue > 0, "Should have days overdue");
        }
    }
    
    // 6. Perform rotation
    let result = manager.rotate_validators(false).expect("Failed to rotate validators");
    assert!(result.rotated, "Rotation should have happened");
    assert_eq!(result.new_validators.len(), 3, "Should have 3 new validators");
    assert_eq!(result.deactivated_validators.len(), 3, "Should have 3 deactivated validators");
    
    println!("Validator rotation successful!");
    println!("Next rotation time: {} (unix timestamp)", result.next_rotation_time);
    
    // 7. Verify rotation result in config file
    let config: ValidatorConfig = {
        let config_json = fs::read_to_string(&config_path).expect("Failed to read config after rotation");
        serde_json::from_str(&config_json).expect("Failed to parse config after rotation")
    };
    
    assert_eq!(config.validators.len(), 3, "Should have 3 validators");
    assert_eq!(config.previous_validators.len(), 3, "Should have 3 previous validators");
    
    println!("Test successful: Validator rotation lifecycle validated ✅");
} 