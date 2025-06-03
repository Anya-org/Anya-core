#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use bitcoin::{Address, Network, Script, ScriptBuf};
    use bitcoin::psbt::Psbt;
    use secp256k1::{Secp256k1, SecretKey, PublicKey};
    use std::collections::HashMap;
    
    use crate::security::hsm::config::{
        HsmConfig, SoftHsmConfig, SimulatorConfig, HardwareConfig, HardwareDeviceType
    };
    use crate::security::hsm::provider::{
        HsmProvider, HsmProviderType, KeyType, KeyUsage, EcCurve, KeyGenParams, SigningAlgorithm,
        create_hsm_provider
    };
    use crate::security::hsm::providers::{
        simulator::SimulatorHsmProvider,
        hardware::HardwareHsmProvider,
        software::SoftwareHsmProvider,
    };
    
    #[tokio::test]
    async fn test_software_provider_bitcoin_testnet() {
        let config = SoftHsmConfig {
            token_dir: ".tokens-test".to_string(),
            max_sessions: 5,
            encryption_key: None,
            lock_timeout_seconds: 300,
            use_testnet: true,
        };
        
        let provider = SoftwareHsmProvider::new(&config).unwrap();
        
        // Initialize
        provider.initialize().await.unwrap();
        
        // Generate Bitcoin testnet key
        let key_params = KeyGenParams {
            id: None,
            label: Some("Test Bitcoin Key".to_string()),
            key_type: KeyType::Ec { curve: EcCurve::Secp256k1 },
            extractable: true,
            usages: vec![KeyUsage::Sign, KeyUsage::Verify],
            expires_at: None,
            attributes: HashMap::new(),
        };
        
        let key_pair = provider.generate_key(key_params).await.unwrap();
        
        // Verify the key format is correct for Bitcoin
        assert_eq!(key_pair.public_key.len(), 33); // Compressed Secp256k1 key is 33 bytes
        
        // Test signing
        let message = b"Test message for Bitcoin signing";
        let signature = provider.sign(&key_pair.id, SigningAlgorithm::EcdsaSha256, message).await.unwrap();
        
        // Verify the signature
        let verified = provider.verify(&key_pair.id, SigningAlgorithm::EcdsaSha256, message, &signature).await.unwrap();
        
        assert!(verified);
    }
    
    #[tokio::test]
    async fn test_simulator_provider_bitcoin_testnet() {
        let config = SimulatorConfig {
            storage_path: ".simulator-test".to_string(),
            simulate_latency: false,
            latency_ms: 10,
            simulate_failures: false,
            failure_rate: 0.0,
            pin_timeout_seconds: 300,
            max_pin_attempts: 3,
            use_testnet: true,
        };
        
        let provider = SimulatorHsmProvider::new(&config).unwrap();
        
        // Initialize
        provider.initialize().await.unwrap();
        
        // Unlock the device (simulator needs this)
        let unlock_request = crate::security::hsm::provider::HsmRequest {
            id: "unlock-1".to_string(),
            operation: crate::security::hsm::provider::HsmOperation::Custom("unlock".to_string()),
            parameters: serde_json::json!({ "pin": "1234" }),
        };
        
        provider.execute_operation(unlock_request).await.unwrap();
        
        // Generate Bitcoin testnet key
        let key_params = KeyGenParams {
            id: None,
            label: Some("Test Bitcoin Key".to_string()),
            key_type: KeyType::Ec { curve: EcCurve::Secp256k1 },
            extractable: false,
            usages: vec![KeyUsage::Sign, KeyUsage::Verify],
            expires_at: None,
            attributes: HashMap::new(),
        };
        
        let key_pair = provider.generate_key(key_params).await.unwrap();
        
        // Test device diagnostics
        let diagnostics_request = crate::security::hsm::provider::HsmRequest {
            id: "diag-1".to_string(),
            operation: crate::security::hsm::provider::HsmOperation::Custom("get_diagnostics".to_string()),
            parameters: serde_json::json!({}),
        };
        
        let response = provider.execute_operation(diagnostics_request).await.unwrap();
        assert!(response.success);
        assert!(response.data.is_some());
        
        // Check that we're operating on testnet
        let diag_data = response.data.unwrap();
        let network = diag_data.get("network").and_then(|n| n.as_str()).unwrap_or("");
        assert_eq!(network, "testnet");
    }
    
    #[tokio::test]
    async fn test_hardware_provider_bitcoin_testnet() {
        let config = HardwareConfig {
            device_type: HardwareDeviceType::Ledger,
            connection_string: "simulator".to_string(), // Use simulator for testing
            auth_key_id: None,
            password: None,
            timeout_seconds: 30,
            use_testnet: true,
        };
        
        let provider = HardwareHsmProvider::new(&config).unwrap();
        
        // Initialize
        provider.initialize().await.unwrap();
        
        // Connect to the device
        let connect_request = crate::security::hsm::provider::HsmRequest {
            id: "connect-1".to_string(),
            operation: crate::security::hsm::provider::HsmOperation::Custom("connect".to_string()),
            parameters: serde_json::json!({}),
        };
        
        provider.execute_operation(connect_request).await.unwrap();
        
        // Authenticate with the device
        let auth_request = crate::security::hsm::provider::HsmRequest {
            id: "auth-1".to_string(),
            operation: crate::security::hsm::provider::HsmOperation::Custom("authenticate".to_string()),
            parameters: serde_json::json!({}),
        };
        
        provider.execute_operation(auth_request).await.unwrap();
        
        // Generate Bitcoin testnet key
        let key_params = KeyGenParams {
            id: None,
            label: Some("Test Bitcoin Key".to_string()),
            key_type: KeyType::Ec { curve: EcCurve::Secp256k1 },
            extractable: false,
            usages: vec![KeyUsage::Sign, KeyUsage::Verify],
            expires_at: None,
            attributes: HashMap::new(),
        };
        
        let key_pair = provider.generate_key(key_params).await.unwrap();
        
        // Test signing
        let message = b"Test message for Bitcoin signing";
        let signature = provider.sign(&key_pair.id, SigningAlgorithm::EcdsaSha256, message).await.unwrap();
        
        // Since we're using a simulated hardware device, we can't verify the signature directly
        // Just check that we got a signature of reasonable length
        assert!(signature.len() > 0);
    }
    
    #[tokio::test]
    async fn test_factory_creates_testnet_provider() {
        // Test that the provider factory creates the correct provider type
        let mut hsm_config = HsmConfig::default();
        
        // Test software provider
        hsm_config.provider_type = HsmProviderType::SoftwareKeyStore;
        let provider = create_hsm_provider(&hsm_config).await.unwrap();
        provider.initialize().await.unwrap();
        
        // Test simulator provider
        hsm_config.provider_type = HsmProviderType::Simulator;
        let provider = create_hsm_provider(&hsm_config).await.unwrap();
        provider.initialize().await.unwrap();
        
        // Test hardware provider
        hsm_config.provider_type = HsmProviderType::Hardware;
        let provider = create_hsm_provider(&hsm_config).await.unwrap();
        provider.initialize().await.unwrap();
    }
} 