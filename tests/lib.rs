use std::sync::Once;
use tokio::runtime::Runtime;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

static INIT: Once = Once::new();

pub fn setup_test_env() {
    INIT.call_once(|| {
        // Initialize logging for tests
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::DEBUG)
            .with_test_writer()
            .init();

        // Set up test environment variables
        std::env::set_var("ENVIRONMENT", "test");
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
    });
}

pub fn get_test_runtime() -> Runtime {
    Runtime::new().expect("Failed to create Tokio runtime")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::CacheManager;
    use crate::config::CONFIG;
    use crate::security::SecurityManager;
    use std::time::Duration;

    #[test]
    fn test_environment_setup() {
        setup_test_env();
        assert_eq!(std::env::var("ENVIRONMENT").unwrap(), "test");
    }

    #[tokio::test]
    async fn test_config_loading() {
        setup_test_env();
        let config = CONFIG.read().await;
        assert!(config.get_string("environment").is_some());
    }

    #[tokio::test]
    async fn test_security_manager() {
        setup_test_env();
        let security = SecurityManager::new().await.unwrap();

        // Test password hashing
        let password = "test_password";
        let hash = security.hash_password(password).unwrap();
        assert!(security.verify_password(password, &hash).unwrap());

        // Test JWT
        let token = security.generate_jwt("test_user", "user").unwrap();
        let claims = security.verify_jwt(&token).unwrap();
        assert_eq!(claims.sub, "test_user");
    }

    #[tokio::test]
    async fn test_cache_manager() {
        setup_test_env();
        let cache = CacheManager::new(Default::default());

        // Test basic cache operations
        cache
            .set("test_key".to_string(), vec![1, 2, 3])
            .await
            .unwrap();
        let value = cache.get("test_key").await.unwrap();
        assert_eq!(value, vec![1, 2, 3]);

        // Test expiration
        tokio::time::sleep(Duration::from_secs(2)).await;
        cache.cleanup().await;
        let stats = cache.get_stats().await;
        assert!(stats.expired_entries == 0);
    }

    #[tokio::test]
    async fn test_resource_limits() {
        setup_test_env();
        let resource_manager = ResourceManager::new().await;

        // Test connection acquisition
        let conn = resource_manager.acquire_connection().await.unwrap();
        assert!(resource_manager.check_resource_health().await.is_healthy);

        // Test memory allocation
        assert!(resource_manager.allocate_memory(1024).await.is_ok());
        let health = resource_manager.check_resource_health().await;
        assert!(health.memory_usage_percent > 0.0);
    }

    #[tokio::test]
    async fn test_performance_monitoring() {
        setup_test_env();
        let monitor = PerformanceMonitor::new();

        // Record some test metrics
        monitor
            .record_request(Duration::from_millis(100), true)
            .await;
        monitor.update_system_metrics(50.0, 30.0).await;

        let health = monitor.get_health_check().await;
        assert_eq!(health.status, "healthy");

        let report = monitor.generate_performance_report().await;
        assert_eq!(report.total_requests, 1);
        assert_eq!(report.total_errors, 0);
    }

    #[tokio::test]
    async fn test_monitoring_system() {
        setup_test_env();
        let monitoring = MonitoringSystem::new();

        // Test network monitoring
        monitoring.update_metric("network_health", 95.0).unwrap();
        assert_eq!(monitoring.get_metrics()["network_health"], 95.0);

        // Test fee monitoring
        monitoring.update_metric("fee_rate", 2.5).unwrap();
        assert_eq!(monitoring.get_metrics()["fee_rate"], 2.5);

        // Test multiple updates
        monitoring.update_metric("network_health", 98.0).unwrap();
        assert_eq!(monitoring.get_metrics()["network_health"], 98.0);

        // Test network metric
        let registry = Registry::new();
        let metric = NetworkMetric::new(&registry);
        metric.update(95.0);
        assert_eq!(metric.get_value(), 95.0);

        // Test fee metric
        let metric = FeeMetric::new(&registry);
        metric.update(2.5);
        assert_eq!(metric.get_value(), 2.5);

        // Test monitoring registry
        let metrics = monitoring.get_metrics();
        assert!(metrics.contains_key("network_health"));
        assert!(metrics.contains_key("fee_rate"));

        // Test invalid metric
        assert!(monitoring.update_metric("invalid_metric", 1.0).is_err());
    }

    #[tokio::test]
    async fn test_network_metric() {
        setup_test_env();
        let registry = Registry::new();
        let metric = NetworkMetric::new(&registry);

        // Test initial state
        assert_eq!(metric.get_value(), 0.0);

        // Test update
        metric.update(95.0);
        assert_eq!(metric.get_value(), 95.0);

        // Test description
        assert_eq!(metric.description(), "Network health status");
    }

    #[tokio::test]
    async fn test_fee_metric() {
        setup_test_env();
        let registry = Registry::new();
        let metric = FeeMetric::new(&registry);

        // Test initial state
        assert_eq!(metric.get_value(), 0.0);

        // Test update
        metric.update(2.5);
        assert_eq!(metric.get_value(), 2.5);

        // Test histogram
        metric.update(1.5);
        metric.update(3.0);
        assert_eq!(metric.get_value(), 3.0);

        // Test description
        assert_eq!(metric.description(), "Current fee rate and distribution");
    }

    #[tokio::test]
    async fn test_monitoring_registry() {
        setup_test_env();
        let monitoring = MonitoringSystem::new();

        // Test registry
        let metrics = monitoring.get_metrics();
        assert!(metrics.contains_key("network_health"));
        assert!(metrics.contains_key("fee_rate"));

        // Test invalid metric
        assert!(monitoring.update_metric("invalid_metric", 1.0).is_err());
    }

    #[tokio::test]
    async fn test_quantum_resistant_crypto() {
        setup_test_env();
        let crypto = QuantumResistantCrypto::new();

        // Test key generation
        assert_eq!(crypto.private_key.len(), 32);
        assert_eq!(crypto.public_key.len(), 32);

        // Test signing
        let message = "test message".as_bytes();
        let signature = crypto.sign(message).unwrap();
        assert_eq!(signature.len(), 64); // 32 bytes for digest + 32 bytes for private key

        // Test verification
        assert!(crypto.verify(message, &signature).unwrap());

        // Test invalid signature
        let mut invalid_signature = signature.clone();
        invalid_signature[0] ^= 0xFF; // Flip a bit
        assert!(!crypto.verify(message, &invalid_signature).unwrap());

        // Test encryption
        let ciphertext = crypto.encrypt(message).unwrap();
        assert!(ciphertext.len() > message.len()); // Includes nonce

        // Test decryption
        let decrypted = crypto.decrypt(&ciphertext).unwrap();
        assert_eq!(decrypted, message);

        // Test invalid decryption
        let mut invalid_ciphertext = ciphertext.clone();
        invalid_ciphertext[0] ^= 0xFF; // Flip a bit
        assert!(crypto.decrypt(&invalid_ciphertext).is_err());
    }

    #[tokio::test]
    async fn test_quantum_resistant_key_generation() {
        setup_test_env();
        let crypto = QuantumResistantCrypto::new();

        // Test key size
        assert_eq!(crypto.key_size, 32);

        // Test public key generation
        let public_key = QuantumResistantCrypto::generate_public_key(&crypto.private_key);
        assert_eq!(public_key.len(), 32);

        // Test different keys
        let crypto2 = QuantumResistantCrypto::new();
        assert_ne!(crypto.private_key, crypto2.private_key);
        assert_ne!(crypto.public_key, crypto2.public_key);
    }

    #[tokio::test]
    async fn test_quantum_resistant_performance() {
        setup_test_env();
        let crypto = QuantumResistantCrypto::new();

        // Test multiple operations
        let message = "large message to test performance".repeat(100).as_bytes();

        // Test signing performance
        let start = std::time::Instant::now();
        for _ in 0..100 {
            crypto.sign(message).unwrap();
        }
        let duration = start.elapsed();
        assert!(duration.as_secs() < 1); // Should complete in less than 1 second

        // Test encryption performance
        let start = std::time::Instant::now();
        for _ in 0..100 {
            crypto.encrypt(message).unwrap();
        }
        let duration = start.elapsed();
        assert!(duration.as_secs() < 1);
    }

    #[tokio::test]
    async fn test_mobile_sdk() {
        setup_test_env();
        let sdk = MobileSDK::new();

        // Test wallet initialization
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        sdk.initialize_wallet(mnemonic).await.unwrap();

        // Test wallet sync
        sdk.sync_wallet().await.unwrap();

        // Test transaction
        let recipient = "bc1q09vm5lfy0j5reeulh4x5752ryscq5tun4677xu";
        let amount = 100000; // 1 BTC
        let tx_id = sdk.send_transaction(recipient, amount).await.unwrap();
        assert!(!tx_id.is_empty());

        // Test wallet info
        let info = sdk.get_wallet_info().await.unwrap();
        assert!(info.balance >= 0);
        assert!(!info.address.is_empty());
        assert!(info.last_sync <= chrono::Utc::now());
        assert!(info.transaction_count >= 1);
    }

    #[tokio::test]
    async fn test_mobile_wallet() {
        setup_test_env();
        let sdk = MobileSDK::new();

        // Test address generation
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        sdk.initialize_wallet(mnemonic).await.unwrap();

        let wallet = sdk.wallet.lock().await;
        assert!(!wallet.addresses.is_empty());
        assert!(wallet.balance >= 0);
        assert!(!wallet.transactions.is_empty());
        assert!(wallet.last_sync <= chrono::Utc::now());
    }

    #[tokio::test]
    async fn test_mobile_network() {
        setup_test_env();
        let sdk = MobileSDK::new();

        // Test balance retrieval
        let addresses = vec!["bc1q09vm5lfy0j5reeulh4x5752ryscq5tun4677xu".to_string()];
        let balance = sdk.network.get_balance(&addresses).await.unwrap();
        assert!(balance >= 0);

        // Test transaction retrieval
        let transactions = sdk.network.get_transactions(&addresses).await.unwrap();
        assert!(!transactions.is_empty());

        // Test transaction creation
        let sender = "bc1q09vm5lfy0j5reeulh4x5752ryscq5tun4677xu";
        let recipient = "bc1q09vm5lfy0j5reeulh4x5752ryscq5tun4677xu";
        let amount = 100000; // 1 BTC
        let tx = sdk
            .network
            .create_transaction(sender, recipient, amount)
            .await
            .unwrap();
        assert!(!tx.is_empty());

        // Test transaction broadcast
        sdk.network.broadcast_transaction(&tx).await.unwrap();
    }

    #[tokio::test]
    async fn test_mobile_security() {
        setup_test_env();
        let sdk = MobileSDK::new();

        // Test address generation
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let addresses = sdk.security.generate_addresses(mnemonic).unwrap();
        assert_eq!(addresses.len(), 1);
        assert!(!addresses[0].is_empty());

        // Test invalid mnemonic
        let invalid_mnemonic = "invalid invalid invalid";
        assert!(sdk.security.generate_addresses(invalid_mnemonic).is_err());
    }

    #[tokio::test]
    async fn test_tenant_manager() {
        setup_test_env();
        let config = TenantConfig {
            max_tenants: 10,
            storage_quota: 1024 * 1024 * 1024, // 1GB
            rate_limits: RateLimits {
                requests_per_minute: 1000,
                max_concurrent_requests: 100,
            },
        };
        let manager = TenantManager::new(config);

        // Test tenant creation
        let tenant = manager.create_tenant("Test Tenant").await.unwrap();
        assert!(!tenant.id.is_empty());
        assert_eq!(tenant.name, "Test Tenant");

        // Test tenant retrieval
        let retrieved = manager.get_tenant(&tenant.id).await.unwrap();
        assert_eq!(retrieved.id, tenant.id);
        assert_eq!(retrieved.name, tenant.name);

        // Test resource updates
        let usage = ResourceUsage {
            storage: 1024 * 1024, // 1MB
            connections: 5,
            requests: 10,
        };
        manager.update_resources(&tenant.id, usage).await.unwrap();

        // Test access control
        let ip = "192.168.1.1";
        let permission = "read";
        assert!(manager
            .check_access(&tenant.id, ip, permission)
            .await
            .unwrap());

        // Test tenant limit
        for _ in 0..9 {
            manager
                .create_tenant(&format!("Test Tenant {}", _))
                .await
                .unwrap();
        }
        assert!(manager.create_tenant("Excess Tenant").await.is_err());
    }

    #[tokio::test]
    async fn test_tenant_resources() {
        setup_test_env();
        let config = TenantConfig {
            max_tenants: 10,
            storage_quota: 1024 * 1024 * 1024, // 1GB
            rate_limits: RateLimits {
                requests_per_minute: 1000,
                max_concurrent_requests: 100,
            },
        };
        let tenant = Tenant::new("test_id".to_string(), "Test Tenant".to_string(), config);

        // Test resource tracking
        let resources = tenant.get_resources();
        assert_eq!(resources.storage_used, 0);
        assert_eq!(resources.active_connections, 0);
        assert_eq!(resources.request_count, 0);

        // Test resource updates
        let usage = ResourceUsage {
            storage: 1024 * 1024, // 1MB
            connections: 5,
            requests: 10,
        };
        tenant.update_resources(usage);

        let updated = tenant.get_resources();
        assert_eq!(updated.storage_used, 1024 * 1024);
        assert_eq!(updated.active_connections, 5);
        assert_eq!(updated.request_count, 10);
    }

    #[tokio::test]
    async fn test_tenant_security() {
        setup_test_env();
        let security = TenantSecurity::new();

        // Test policy creation
        let policy = AccessPolicy {
            allowed_ips: vec!["192.168.1.1".to_string()],
            rate_limit: RateLimit {
                requests: 100,
                period: chrono::Duration::minutes(1),
            },
            permissions: vec!["read".to_string(), "write".to_string()],
        };
        security.add_policy(policy);

        // Test access control
        assert!(security.check_access("192.168.1.1", "read").unwrap());
        assert!(security.check_access("192.168.1.1", "write").unwrap());
        assert!(!security.check_access("192.168.1.2", "read").unwrap());
        assert!(!security.check_access("192.168.1.1", "admin").unwrap());
    }
}
