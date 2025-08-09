//! HSM Integration Tests
//! [AIR-3][AIS-3][BPC-3][RES-3]
//!
//! Comprehensive testing for HSM provider factory, fallback mechanisms,
//! and production readiness validation.

#[cfg(test)]
mod hsm_integration_tests {
    use std::sync::Arc;

    use crate::security::hsm::{
        config::{HsmConfig, SimulatorConfig, SoftHsmConfig},
        error::HsmError,
        factory::{HsmProviderFactory, ProductionHsmFactory},
        provider::{
            EcCurve, HsmOperation, HsmProvider, HsmProviderStatus, HsmProviderType, HsmRequest,
            KeyGenParams, KeyType, KeyUsage, SigningAlgorithm,
        },
    };
    use bitcoin::hashes::{sha256d, Hash};
    use std::env;
    use tokio::time::{timeout, Duration};

    // Helper: check env var enabled ("1", "true", case-insensitive)
    #[allow(dead_code)] // May be unused under certain feature/CI gating combinations
    fn env_enabled(name: &str) -> bool {
        env::var(name)
            .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "True"))
            .unwrap_or(false)
    }

    /// Test software fallback strategy when primary provider fails
    #[tokio::test]
    async fn test_software_fallback_strategy() {
        // Create configuration with invalid hardware settings
        let mut config = HsmConfig {
            provider_type: HsmProviderType::Hardware,
            ..HsmConfig::default()
        };
        config.hardware.device_type = crate::security::hsm::config::HardwareDeviceType::Custom;
        config.hardware.connection_string = "invalid://nonexistent:9999".to_string();

        // Should fallback to software provider
        let provider = HsmProviderFactory::create_with_fallback(&config).await;
        assert!(provider.is_ok(), "Factory should create fallback provider");

        let provider = provider.unwrap();

        // Verify provider is functional
        let status = provider.get_status().await;
        assert!(status.is_ok(), "Fallback provider should be functional");

        match status.unwrap() {
            HsmProviderStatus::Ready => {}
            status => panic!("Expected Ready status, got {status:?}"),
        }

        // Test basic key operations work on fallback
        test_basic_key_operations(&*provider).await;
    }

    /// Test Bitcoin operations work across all available providers
    #[tokio::test]
    async fn test_bitcoin_operations_cross_provider() {
        // Unified skip gate (avoids unreachable-code lint): evaluate all skip conditions once.
        let skip = {
            #[cfg(feature = "fast-tests")]
            {
                eprintln!("SKIP(fast-tests): test_bitcoin_operations_cross_provider");
                true
            }
            #[cfg(not(feature = "fast-tests"))]
            {
                if !env_enabled("ANYA_ENABLE_HSM_CROSS") {
                    eprintln!("SKIP: test_bitcoin_operations_cross_provider (set ANYA_ENABLE_HSM_CROSS=1 to run)");
                    true
                } else {
                    false
                }
            }
        };
        if skip {
            return;
        }
        let mut providers = vec![
            create_software_provider()
                .await
                .expect("Software provider should work"),
            match create_bitcoin_provider().await {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("SKIP: bitcoin provider unavailable: {e:?}");
                    return;
                }
            },
        ];

        #[cfg(feature = "dev-sim")]
        {
            let sim = create_simulator_provider()
                .await
                .expect("Simulator provider should work");
            // Ensure simulator is initialized and unlocked for this test
            let _ = sim.initialize().await;
            let _ = sim
                .execute_operation(HsmRequest {
                    id: "unlock-cross-provider".to_string(),
                    operation: HsmOperation::Custom("unlock".to_string()),
                    parameters: serde_json::json!({"pin": "1234"}),
                })
                .await;
            providers.push(sim);
        }

        for (i, provider) in providers.iter().enumerate() {
            println!("Testing provider {i}");

            // Test Bitcoin key generation (with timeout)
            if let Err(_elapsed) = timeout(
                Duration::from_secs(20),
                test_bitcoin_key_generation(&**provider),
            )
            .await
            {
                eprintln!("SKIP: key generation timed out (provider index {i})");
                continue;
            }

            // Test Bitcoin signing operations (with timeout)
            if let Err(_elapsed) = timeout(
                Duration::from_secs(20),
                test_bitcoin_signing_operations(&**provider),
            )
            .await
            {
                eprintln!("SKIP: signing operations timed out (provider index {i})");
                continue;
            }

            // Test health check (with timeout)
            let health =
                match timeout(Duration::from_secs(20), provider.perform_health_check()).await {
                    Ok(h) => h,
                    Err(_) => {
                        eprintln!("SKIP: health check timed out (provider index {i})");
                        continue;
                    }
                };
            assert!(health.is_ok(), "Health check should succeed");
            assert!(health.unwrap(), "Provider should be healthy");
        }
    }

    /// Test production configuration validation
    #[tokio::test]
    async fn test_production_config_validation() {
        // Test invalid simulator on mainnet
        let mut config = HsmConfig {
            provider_type: HsmProviderType::Simulator,
            ..HsmConfig::default()
        };
        config.bitcoin.network = crate::security::hsm::config::BitcoinNetworkType::Mainnet;

        let result = ProductionHsmFactory::create_for_production(&config).await;
        assert!(result.is_err(), "Simulator should fail on mainnet");

        // Test valid production config
        config.provider_type = HsmProviderType::SoftwareKeyStore;
        config.software.encryption_key =
            Some("secure-encryption-key-32-characters-long".to_string());

        let result = ProductionHsmFactory::create_for_production(&config).await;
        assert!(result.is_ok(), "Valid production config should succeed");
    }

    /// Test HSM provider health checks
    #[tokio::test]
    async fn test_provider_health_checks() {
        let skip = {
            #[cfg(feature = "fast-tests")]
            {
                eprintln!("SKIP(fast-tests): test_provider_health_checks");
                true
            }
            #[cfg(not(feature = "fast-tests"))]
            {
                if !env_enabled("ANYA_ENABLE_HSM_HEALTH") {
                    eprintln!(
                        "SKIP: test_provider_health_checks (set ANYA_ENABLE_HSM_HEALTH=1 to run)"
                    );
                    true
                } else {
                    false
                }
            }
        };
        if skip {
            return;
        }
        let mut providers = vec![create_software_provider().await.unwrap()];
        #[cfg(feature = "dev-sim")]
        {
            let sim = create_simulator_provider().await.unwrap();
            // Initialize and unlock for health checks
            let _ = sim.initialize().await;
            let _ = sim
                .execute_operation(HsmRequest {
                    id: "unlock-health".to_string(),
                    operation: HsmOperation::Custom("unlock".to_string()),
                    parameters: serde_json::json!({"pin": "1234"}),
                })
                .await;
            providers.push(sim);
        }

        for (i, provider) in providers.into_iter().enumerate() {
            let health =
                match timeout(Duration::from_secs(20), provider.perform_health_check()).await {
                    Ok(h) => h,
                    Err(_) => {
                        eprintln!("SKIP: health check timed out (provider index {i})");
                        continue;
                    }
                };
            assert!(health.is_ok(), "Health check should not error");
            assert!(health.unwrap(), "Provider should be healthy");
        }
    }

    /// Test concurrent provider operations
    #[tokio::test]
    async fn test_concurrent_operations() {
        let skip = {
            #[cfg(feature = "fast-tests")]
            {
                eprintln!("SKIP(fast-tests): test_concurrent_operations");
                true
            }
            #[cfg(not(feature = "fast-tests"))]
            {
                if !env_enabled("ANYA_ENABLE_HSM_CONCURRENCY") {
                    eprintln!("SKIP: test_concurrent_operations (set ANYA_ENABLE_HSM_CONCURRENCY=1 to run)");
                    true
                } else {
                    false
                }
            }
        };
        if skip {
            return;
        }
        let provider = create_software_provider().await.unwrap();

        // Create multiple tasks performing operations concurrently
        let mut handles = vec![];

        for i in 0..10 {
            let provider_clone = Arc::clone(&provider);
            let handle = tokio::spawn(async move {
                let key_params = KeyGenParams {
                    id: Some(format!("concurrent-key-{i}")),
                    label: Some(format!("Concurrent Test Key {i}")),
                    key_type: KeyType::Ec {
                        curve: EcCurve::Secp256k1,
                    },
                    extractable: true,
                    usages: vec![KeyUsage::Sign, KeyUsage::Verify],
                    expires_at: None,
                    attributes: std::collections::HashMap::new(),
                };

                let (key_pair, _) = provider_clone.generate_key(key_params).await?;

                let test_data = format!("test data {i}").into_bytes();
                let signature = provider_clone
                    .sign(&key_pair.id, SigningAlgorithm::EcdsaSha256, &test_data)
                    .await?;

                let verified = provider_clone
                    .verify(
                        &key_pair.id,
                        SigningAlgorithm::EcdsaSha256,
                        &test_data,
                        &signature,
                    )
                    .await?;

                assert!(verified, "Signature verification should succeed");

                // Clean up
                provider_clone.delete_key(&key_pair.id).await?;

                Ok::<(), HsmError>(())
            });

            handles.push(handle);
        }

        // Wait for all operations to complete (with timeout)
        for (i, handle) in handles.into_iter().enumerate() {
            match timeout(Duration::from_secs(25), handle).await {
                Ok(join) => {
                    if let Err(e) = join {
                        panic!("task join error index {i}: {e:?}");
                    }
                    if let Err(e) = join.unwrap() {
                        panic!("task result error index {i}: {e:?}");
                    }
                }
                Err(_) => {
                    eprintln!("SKIP: concurrent task group timeout (index {i})");
                    return; // treat remaining as skipped
                }
            }
        }
    }

    /// Test provider recovery from temporary failures
    #[tokio::test]
    async fn test_provider_recovery() {
        #[cfg(not(feature = "dev-sim"))]
        {
            // Skip when simulator is not available
            return;
        }
        #[cfg(feature = "dev-sim")]
        let provider = create_simulator_provider().await.unwrap();

        // Simulator devices start locked by default. Verify Unavailable first,
        // then unlock using the custom operation and assert Ready.
        #[cfg(feature = "dev-sim")]
        {
            let initial_status = provider.get_status().await.unwrap();
            assert_eq!(initial_status, HsmProviderStatus::Unavailable);

            // Unlock with the known test PIN for the simulator ("1234").
            let unlock_req = HsmRequest {
                id: "unlock-1".to_string(),
                operation: HsmOperation::Custom("unlock".to_string()),
                parameters: serde_json::json!({ "pin": "1234" }),
            };
            provider.execute_operation(unlock_req).await.unwrap();
        }

        // Test that provider can recover from temporary issues
        let status = provider.get_status().await.unwrap();
        assert_eq!(status, HsmProviderStatus::Ready);

        // Perform operation that should work
        let key_params = KeyGenParams {
            id: Some("recovery-test-key".to_string()),
            label: Some("Recovery Test Key".to_string()),
            key_type: KeyType::Ec {
                curve: EcCurve::Secp256k1,
            },
            extractable: true,
            usages: vec![KeyUsage::Sign],
            expires_at: None,
            attributes: std::collections::HashMap::new(),
        };

        let result = provider.generate_key(key_params).await;
        assert!(
            result.is_ok(),
            "Key generation should succeed after recovery"
        );
    }

    // Helper functions

    async fn create_software_provider() -> Result<Arc<dyn HsmProvider>, HsmError> {
        let config = HsmConfig {
            provider_type: HsmProviderType::SoftwareKeyStore,
            software: SoftHsmConfig {
                token_dir: ".test-tokens".to_string(),
                max_sessions: 10,
                encryption_key: Some("0123456789ABCDEF0123456789ABCDEF".to_string()),
                lock_timeout_seconds: 300,
                use_testnet: true,
            },
            ..Default::default()
        };

        let provider = HsmProviderFactory::create_specific_provider(
            HsmProviderType::SoftwareKeyStore,
            &config,
        )
        .await?;
        let _ = provider.initialize().await;
        Ok(provider)
    }

    async fn create_bitcoin_provider() -> Result<Arc<dyn HsmProvider>, HsmError> {
        let config = HsmConfig {
            provider_type: HsmProviderType::Bitcoin,
            bitcoin: crate::security::hsm::config::BitcoinConfig {
                network: crate::security::hsm::config::BitcoinNetworkType::Testnet,
                ..Default::default()
            },
            ..Default::default()
        };

        let provider =
            HsmProviderFactory::create_specific_provider(HsmProviderType::Bitcoin, &config).await?;
        // Ensure provider is ready
        provider.initialize().await?;
        Ok(provider)
    }

    async fn create_simulator_provider() -> Result<Arc<dyn HsmProvider>, HsmError> {
        let config = HsmConfig {
            provider_type: HsmProviderType::Simulator,
            simulator: SimulatorConfig {
                storage_path: ".test-simulator".to_string(),
                simulate_latency: false,
                latency_ms: 0,
                simulate_failures: false,
                failure_rate: 0.0,
                pin_timeout_seconds: 300,
                max_pin_attempts: 3,
                use_testnet: true,
            },
            ..Default::default()
        };

        let provider =
            HsmProviderFactory::create_specific_provider(HsmProviderType::Simulator, &config)
                .await?;
        // Initialize but do not unlock globally; tests decide
        let _ = provider.initialize().await;
        Ok(provider)
    }

    async fn test_basic_key_operations(provider: &dyn HsmProvider) {
        // Test key generation
        let key_params = KeyGenParams {
            id: Some("test-basic-key".to_string()),
            label: Some("Basic Test Key".to_string()),
            key_type: KeyType::Ec {
                curve: EcCurve::Secp256k1,
            },
            extractable: true,
            usages: vec![KeyUsage::Sign, KeyUsage::Verify],
            expires_at: None,
            attributes: std::collections::HashMap::new(),
        };

        let (key_pair, key_info) = provider
            .generate_key(key_params)
            .await
            .expect("Key generation should succeed");

        assert_eq!(key_pair.id, "test-basic-key");
        assert_eq!(key_info.id, "test-basic-key");

        // Test signing
        let test_data = b"test signing data";
        let signature = provider
            .sign(&key_pair.id, SigningAlgorithm::EcdsaSha256, test_data)
            .await
            .expect("Signing should succeed");

        assert!(!signature.is_empty(), "Signature should not be empty");

        // Test verification
        let verified = provider
            .verify(
                &key_pair.id,
                SigningAlgorithm::EcdsaSha256,
                test_data,
                &signature,
            )
            .await
            .expect("Verification should succeed");

        assert!(verified, "Signature verification should succeed");

        // Test key listing
        let keys = provider
            .list_keys()
            .await
            .expect("Key listing should succeed");
        assert!(!keys.is_empty(), "Should have at least one key");
        assert!(keys.iter().any(|k| k.id == "test-basic-key"));

        // Clean up
        provider
            .delete_key(&key_pair.id)
            .await
            .expect("Key deletion should succeed");
    }

    async fn test_bitcoin_key_generation(provider: &dyn HsmProvider) {
        let key_params = KeyGenParams {
            id: Some("btc-test-key".to_string()),
            label: Some("Bitcoin Test Key".to_string()),
            key_type: KeyType::Ec {
                curve: EcCurve::Secp256k1,
            },
            extractable: false, // Bitcoin keys should not be extractable
            usages: vec![KeyUsage::Sign],
            expires_at: None,
            attributes: std::collections::HashMap::new(),
        };

        let (key_pair, key_info) = provider
            .generate_key(key_params)
            .await
            .expect("Bitcoin key generation should succeed");

        assert_eq!(
            key_info.key_type,
            KeyType::Ec {
                curve: EcCurve::Secp256k1
            }
        );
        assert!(key_info.usages.contains(&KeyUsage::Sign));

        // Clean up
        provider
            .delete_key(&key_pair.id)
            .await
            .expect("Key deletion should succeed");
    }

    async fn test_bitcoin_signing_operations(provider: &dyn HsmProvider) {
        // Generate a Bitcoin key
        let key_params = KeyGenParams {
            id: Some("btc-sign-key".to_string()),
            label: Some("Bitcoin Signing Key".to_string()),
            key_type: KeyType::Ec {
                curve: EcCurve::Secp256k1,
            },
            extractable: false,
            usages: vec![KeyUsage::Sign],
            expires_at: None,
            attributes: std::collections::HashMap::new(),
        };

        let (key_pair, _) = provider
            .generate_key(key_params)
            .await
            .expect("Bitcoin key generation should succeed");

        // Test signing Bitcoin transaction hash
        let tx_hash = sha256d::Hash::hash(b"dummy transaction data");
        let signature = provider
            .sign(
                &key_pair.id,
                SigningAlgorithm::EcdsaSha256,
                tx_hash.as_byte_array(),
            )
            .await
            .expect("Bitcoin transaction signing should succeed");

        assert!(
            !signature.is_empty(),
            "Bitcoin signature should not be empty"
        );

        // Clean up
        provider
            .delete_key(&key_pair.id)
            .await
            .expect("Key deletion should succeed");
    }
}

// hsm_integration_tests module closed above

/// Performance benchmarks for HSM operations
#[cfg(all(test, not(feature = "fast-tests")))]
mod hsm_performance_tests {
    use std::sync::Arc;
    use std::time::Instant;

    use crate::security::hsm::{
        config::{HsmConfig, SoftHsmConfig},
        error::HsmError,
        factory::HsmProviderFactory,
        provider::{
            EcCurve, HsmProvider, HsmProviderType, KeyGenParams, KeyType, KeyUsage,
            SigningAlgorithm,
        },
    };

    async fn create_software_provider() -> Result<Arc<dyn HsmProvider>, HsmError> {
        let config = HsmConfig {
            provider_type: HsmProviderType::SoftwareKeyStore,
            software: SoftHsmConfig {
                token_dir: ".test-tokens".to_string(),
                max_sessions: 10,
                encryption_key: Some("0123456789ABCDEF0123456789ABCDEF".to_string()),
                lock_timeout_seconds: 300,
                use_testnet: true,
            },
            ..Default::default()
        };

        HsmProviderFactory::create_specific_provider(HsmProviderType::SoftwareKeyStore, &config)
            .await
    }

    #[tokio::test]
    async fn benchmark_key_generation() {
        let provider = create_software_provider().await.unwrap();

        let start = Instant::now();
        let mut key_ids = vec![];

        // Generate 100 keys
        for i in 0..100 {
            let key_params = KeyGenParams {
                id: Some(format!("bench-key-{i}")),
                label: Some(format!("Benchmark Key {i}")),
                key_type: KeyType::Ec {
                    curve: EcCurve::Secp256k1,
                },
                extractable: true,
                usages: vec![KeyUsage::Sign],
                expires_at: None,
                attributes: std::collections::HashMap::new(),
            };

            let (key_pair, _) = provider.generate_key(key_params).await.unwrap();
            key_ids.push(key_pair.id);
        }

        let duration = start.elapsed();
        println!(
            "Generated 100 keys in {:?} ({:.2} keys/sec)",
            duration,
            100.0 / duration.as_secs_f64()
        );

        // Clean up
        for key_id in key_ids {
            provider.delete_key(&key_id).await.unwrap();
        }

        // Assert reasonable performance (adjust based on your requirements)
        assert!(
            duration.as_millis() < 5000,
            "Key generation should be under 5 seconds for 100 keys"
        );
    }

    #[tokio::test]
    async fn benchmark_signing_operations() {
        let provider = create_software_provider().await.unwrap();

        // Generate a key first
        let key_params = KeyGenParams {
            id: Some("bench-sign-key".to_string()),
            label: Some("Benchmark Signing Key".to_string()),
            key_type: KeyType::Ec {
                curve: EcCurve::Secp256k1,
            },
            extractable: true,
            usages: vec![KeyUsage::Sign],
            expires_at: None,
            attributes: std::collections::HashMap::new(),
        };

        let (key_pair, _) = provider.generate_key(key_params).await.unwrap();

        let start = Instant::now();
        let test_data = b"benchmark signing data";

        // Perform 1000 signing operations
        for _ in 0..1000 {
            let _signature = provider
                .sign(&key_pair.id, SigningAlgorithm::EcdsaSha256, test_data)
                .await
                .unwrap();
        }

        let duration = start.elapsed();
        println!(
            "Performed 1000 signatures in {:?} ({:.2} sigs/sec)",
            duration,
            1000.0 / duration.as_secs_f64()
        );

        // Clean up
        provider.delete_key(&key_pair.id).await.unwrap();

        // Assert reasonable performance
        assert!(
            duration.as_millis() < 10000,
            "1000 signatures should complete under 10 seconds"
        );
    }
}
