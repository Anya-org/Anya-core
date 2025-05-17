use std::sync::Arc;
use tokio::sync::RwLock;

use super::*;
use crate::security::hsm::audit::AuditLogger;
use crate::security::hsm::providers::software::SoftwareHsmProvider;
use crate::security::hsm::config::{HsmConfig, SoftHsmConfig};
use crate::security::hsm::operations::*;
use chrono::Utc;

/// Creates a test HSM configuration with a software provider
fn create_test_config() -> HsmConfig {
    HsmConfig {
        provider_type: provider::HsmProviderType::SoftwareKeyStore,
        enabled: true,
        software: Some(SoftHsmConfig {
            storage_path: std::path::PathBuf::from("/tmp/hsm_test"),
            encryption_key: "test_encryption_key".to_string(),
        }),
        cloud: None,
        tpm: None,
        pkcs11: None,
        ..Default::default()
    }
}

/// Creates a test HSM manager for testing
async fn create_test_manager() -> Result<HsmManager, HsmError> {
    let config = create_test_config();
    let manager = HsmManager::new(config)?;
    manager.initialize().await?;
    Ok(manager)
}

/// Creates a mock audit logger that doesn't actually log anything
fn create_mock_audit_logger() -> Arc<AuditLogger> {
    Arc::new(AuditLogger::new_mock())
}

#[tokio::test]
async fn test_hsm_manager_initialization() {
    let manager = create_test_manager().await.expect("Failed to create HSM manager");
    
    // Check that the manager is properly initialized
    assert!(manager.is_enabled());
    
    // Check that the status is Ready
    let status = manager.status.read().await;
    assert_eq!(*status, HsmStatus::Ready);
}

#[tokio::test]
async fn test_key_generation() {
    let manager = create_test_manager().await.expect("Failed to create HSM manager");
    
    // Generate a test key
    let key_info = manager.generate_key_pair(
        KeyType::Ec { curve: provider::EcCurve::Secp256k1 },
        "test_key"
    ).await.expect("Failed to generate key pair");
    
    // Verify key info
    assert_eq!(key_info.label, Some("test_key".to_string()));
    assert!(matches!(key_info.key_type, KeyType::Ec { curve: provider::EcCurve::Secp256k1 }));
}

#[tokio::test]
async fn test_sign_and_verify() {
    let manager = create_test_manager().await.expect("Failed to create HSM manager");
    
    // Generate a test key
    let key_info = manager.generate_key_pair(
        KeyType::Ec { curve: provider::EcCurve::Secp256k1 },
        "signing_test_key"
    ).await.expect("Failed to generate key pair");
    
    // Test data to sign
    let test_data = b"Hello, world!";
    
    // Sign the data
    let signature = manager.sign_data(
        "signing_test_key",
        test_data,
        SignatureAlgorithm::EcdsaSecp256k1Sha256
    ).await.expect("Failed to sign data");
    
    // Verify the signature
    let verified = manager.verify_signature(
        "signing_test_key",
        test_data,
        &signature,
        SignatureAlgorithm::EcdsaSecp256k1Sha256
    ).await.expect("Failed to verify signature");
    
    assert!(verified, "Signature verification failed");
}

#[tokio::test]
async fn test_health_check() {
    let manager = create_test_manager().await.expect("Failed to create HSM manager");
    
    // Run health check
    let result = manager.run_health_check().await.expect("Failed to run health check");
    
    // Health check should pass for a newly initialized manager
    assert!(result, "Health check failed for new manager");
}

#[tokio::test]
async fn test_soft_hsm_provider() {
    // Create a software HSM provider directly for testing
    let config = SoftHsmConfig {
        storage_path: std::path::PathBuf::from("/tmp/hsm_test_provider"),
        encryption_key: "test_key".to_string(),
    };
    
    let provider = SoftwareHsmProvider::new(&config).expect("Failed to create provider");
    
    // Initialize the provider
    provider.initialize().await.expect("Failed to initialize provider");
    
    // Generate a key
    let params = provider::KeyGenParams {
        key_type: KeyType::Ec { curve: provider::EcCurve::Secp256k1 },
        label: Some("test_key".to_string()),
        extractable: true,
        usages: vec![provider::KeyUsage::Sign, provider::KeyUsage::Verify],
        attributes: Default::default(),
    };
    
    let (key_pair, key_info) = provider.generate_key(params).await.expect("Failed to generate key");
    
    // Verify key info
    assert_eq!(key_info.label, Some("test_key".to_string()));
    
    // Sign some data
    let test_data = b"Test data for signing";
    let signature = provider.sign(&key_info.id, provider::SigningAlgorithm::EcdsaSecp256k1Sha256, test_data)
        .await.expect("Failed to sign");
        
    // Verify the signature
    let verified = provider.verify(&key_info.id, provider::SigningAlgorithm::EcdsaSecp256k1Sha256, test_data, &signature)
        .await.expect("Failed to verify");
        
    assert!(verified, "Signature verification failed");
}

// Test for the HsmAuditEvent structure
#[test]
fn test_hsm_audit_event() {
    let event = HsmAuditEvent {
        event_type: AuditEventType::HsmOperation,
        result: AuditEventResult::Success,
        severity: AuditEventSeverity::Info,
        timestamp: Utc::now(),
        id: "test-id".to_string(),
        user_id: Some("test-user".to_string()),
        key_id: Some("test-key".to_string()),
        parameters: Some(serde_json::json!({ "param": "value" })),
        error: None,
        metadata: Some(serde_json::json!({ "meta": "data" })),
    };
    
    // Test serialization/deserialization
    let json = serde_json::to_string(&event).expect("Failed to serialize event");
    let deserialized: HsmAuditEvent = serde_json::from_str(&json).expect("Failed to deserialize event");
    
    assert_eq!(event.id, deserialized.id);
    assert_eq!(event.event_type, deserialized.event_type);
    assert_eq!(event.result, deserialized.result);
}

// Test for execute_operation with different operation types
#[tokio::test]
async fn test_execute_operation() {
    let manager = create_test_manager().await.expect("Failed to create HSM manager");
    
    // Test key generation operation
    let gen_params = operations::KeyGenRequest {
        key_type: KeyType::Ec { curve: provider::EcCurve::Secp256k1 },
        label: "exec_test_key".to_string(),
        extractable: true,
        usages: vec![provider::KeyUsage::Sign, provider::KeyUsage::Verify],
    };
    
    let result = manager.execute(HsmOperation::GenerateKey, gen_params).await
        .expect("Failed to execute key generation");
    
    if let OperationResult::KeyGen(key_info) = result {
        assert_eq!(key_info.label, Some("exec_test_key".to_string()));
        
        // Now test signing operation with the generated key
        let sign_params = operations::SignRequest {
            key_id: key_info.id.clone(),
            algorithm: SignatureAlgorithm::EcdsaSecp256k1Sha256,
            data: b"Testing execute operation".to_vec(),
        };
        
        let sign_result = manager.execute(HsmOperation::Sign, sign_params).await
            .expect("Failed to execute signing operation");
            
        if let OperationResult::Sign(signature) = sign_result {
            assert!(!signature.is_empty(), "Signature should not be empty");
        } else {
            panic!("Expected Sign result, got: {:?}", sign_result);
        }
    } else {
        panic!("Expected KeyGen result, got: {:?}", result);
    }
}
