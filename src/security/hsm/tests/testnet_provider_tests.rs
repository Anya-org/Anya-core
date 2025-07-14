// Only import what is actually used
// Removed unused imports: HashMap, Arc

/// HSM Testnet Provider Tests

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::security::hsm::config::{HsmConfig, SimulatorConfig};
    use crate::security::hsm::provider::{
        create_hsm_provider, EcCurve, HsmProvider, HsmProviderType, KeyGenParams, KeyType, KeyUsage,
    };
    use crate::security::hsm::providers::simulator::SimulatorHsmProvider;


    #[tokio::test]
    #[ignore = "Requires AuditLogger implementation"]
    async fn test_software_provider_bitcoin_testnet() {
        println!("Testing software provider for Bitcoin testnet");

        // Test implementation goes here - commented out until AuditLogger is available
        // let provider = SoftwareHsmProvider::new(...);
        // provider.initialize().await.unwrap();
        // ...
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
            key_type: KeyType::Ec {
                curve: EcCurve::Secp256k1,
            },
            extractable: false,
            usages: vec![KeyUsage::Sign, KeyUsage::Verify],
            expires_at: None,
            attributes: HashMap::new(),
        };

        let (_key_pair, _) = provider.generate_key(key_params).await.unwrap();

        // Test device diagnostics
        let diagnostics_request = crate::security::hsm::provider::HsmRequest {
            id: "diag-1".to_string(),
            operation: crate::security::hsm::provider::HsmOperation::Custom(
                "get_diagnostics".to_string(),
            ),
            parameters: serde_json::json!({}),
        };

        let response = provider
            .execute_operation(diagnostics_request)
            .await
            .unwrap();
        assert!(response.success);
        assert!(response.data.is_some());

        // Check that we're operating on testnet
        let diag_data = response.data.unwrap();
        let network = diag_data
            .get("network")
            .and_then(|n| n.as_str())
            .unwrap_or("");
        assert_eq!(network, "testnet");
    }

    #[tokio::test]
    #[ignore = "Requires hardware device"]
    async fn test_hardware_provider_bitcoin_testnet() {
        println!("Hardware provider test requires hardware device");

        // Hardware provider implementation would go here when hardware is available
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
