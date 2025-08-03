//! HSM Provider Factory with Intelligent Fallback Strategy
//! [AIR-3][AIS-3][BPC-3][RES-3]
//!
//! This module provides a robust HSM provider factory that implements intelligent
//! fallback strategies for production environments, ensuring high availability
//! while maintaining security standards.

use std::sync::Arc;
use tracing::{error, info, warn};

use crate::security::hsm::{
    config::{HsmConfig, SoftHsmConfig},
    error::HsmError,
    provider::{HsmProvider, HsmProviderStatus, HsmProviderType},
    providers::{
        BitcoinHsmProvider, HardwareHsmProvider, SimulatorHsmProvider, SoftwareHsmProvider,
    },
};

/// [AIR-3][AIS-3][BPC-3][RES-3] HSM Provider Factory with Fallback Strategy
///
/// Provides intelligent provider selection and fallback mechanisms:
/// 1. Attempts to create requested provider type
/// 2. Falls back to software provider if hardware unavailable
/// 3. Maintains audit logging and security warnings
/// 4. Ensures production-grade reliability
pub struct HsmProviderFactory;

impl HsmProviderFactory {
    /// Create HSM provider with automatic fallback strategy
    ///
    /// # Fallback Strategy [AIS-3]
    /// 1. **Primary**: Attempt requested provider type
    /// 2. **Secondary**: Fall back to SoftwareHsmProvider if primary fails
    /// 3. **Tertiary**: Fall back to SimulatorHsmProvider if software fails
    ///
    /// # Security Considerations [BPC-3]
    /// - All fallbacks maintain audit logging
    /// - Security level warnings are logged
    /// - Configuration validation is enforced
    pub async fn create_with_fallback(
        config: &HsmConfig,
    ) -> Result<Arc<dyn HsmProvider>, HsmError> {
        // Attempt primary provider
        match Self::create_primary_provider(config).await {
            Ok(provider) => {
                info!(
                    "Primary HSM provider initialized successfully: {:?}",
                    config.provider_type
                );

                // Verify provider is healthy
                if Self::verify_provider_health(&*provider).await? {
                    return Ok(provider);
                } else {
                    warn!("Primary provider failed health check, falling back");
                }
            }
            Err(e) => {
                warn!(
                    "Primary HSM provider {:?} failed to initialize: {}",
                    config.provider_type, e
                );
            }
        }

        // Fall back to software provider
        match Self::create_software_fallback(config).await {
            Ok(provider) => {
                warn!(
                    "Using software HSM fallback - security level reduced. \
                     Consider configuring hardware HSM for production use."
                );
                return Ok(provider);
            }
            Err(e) => {
                error!("Software HSM fallback failed: {}", e);
            }
        }

        // Final fallback to simulator (development only)
        if cfg!(debug_assertions) {
            warn!(
                "Using simulator HSM as final fallback - DEVELOPMENT ONLY. \
                 This should never happen in production!"
            );
            Self::create_simulator_fallback(config).await
        } else {
            Err(HsmError::InitializationError(
                "All HSM providers failed and simulator is disabled in release builds".into(),
            ))
        }
    }

    /// Create the requested primary provider
    async fn create_primary_provider(config: &HsmConfig) -> Result<Arc<dyn HsmProvider>, HsmError> {
        match config.provider_type {
            HsmProviderType::SoftwareKeyStore => Self::create_software_provider(config).await,
            HsmProviderType::Hardware => Self::create_hardware_provider(config).await,
            HsmProviderType::Bitcoin => Self::create_bitcoin_provider(config).await,
            HsmProviderType::Simulator => Self::create_simulator_provider(config).await,
            HsmProviderType::CloudHsm => Err(HsmError::ProviderNotSupported(
                "CloudHsm provider not yet implemented".into(),
            )),
            HsmProviderType::Tpm => Err(HsmError::ProviderNotSupported(
                "TPM provider not yet implemented".into(),
            )),
            HsmProviderType::Pkcs11 => Err(HsmError::ProviderNotSupported(
                "PKCS#11 provider not yet implemented".into(),
            )),
            HsmProviderType::Custom => Err(HsmError::ProviderNotSupported(
                "Custom provider requires specific implementation".into(),
            )),
        }
    }

    /// Create software HSM provider
    async fn create_software_provider(
        config: &HsmConfig,
    ) -> Result<Arc<dyn HsmProvider>, HsmError> {
        let audit_config = crate::security::hsm::audit::AuditLoggerConfig::default();
        let audit_logger =
            Arc::new(crate::security::hsm::audit::AuditLogger::new(&audit_config).await?);

        let provider = SoftwareHsmProvider::new(
            config.software.clone(),
            bitcoin::Network::from(config.bitcoin.network),
            audit_logger,
        )
        .await?;

        Ok(Arc::new(provider))
    }

    /// Create hardware HSM provider
    async fn create_hardware_provider(
        config: &HsmConfig,
    ) -> Result<Arc<dyn HsmProvider>, HsmError> {
        let audit_config = crate::security::hsm::audit::AuditLoggerConfig::default();
        let audit_logger =
            Arc::new(crate::security::hsm::audit::AuditLogger::new(&audit_config).await?);

        let provider = HardwareHsmProvider::new(
            &config.hardware,
            bitcoin::Network::from(config.bitcoin.network),
            audit_logger,
        )
        .await?;

        // Initialize and authenticate
        provider.initialize().await?;

        Ok(Arc::new(provider))
    }

    /// Create Bitcoin HSM provider
    async fn create_bitcoin_provider(config: &HsmConfig) -> Result<Arc<dyn HsmProvider>, HsmError> {
        let provider = BitcoinHsmProvider::new(&config.bitcoin).await?;
        Ok(Arc::new(provider))
    }

    /// Create simulator HSM provider
    async fn create_simulator_provider(
        config: &HsmConfig,
    ) -> Result<Arc<dyn HsmProvider>, HsmError> {
        let provider = SimulatorHsmProvider::new(&config.simulator)?;
        Ok(Arc::new(provider))
    }

    /// Software fallback provider - always available
    async fn create_software_fallback(
        config: &HsmConfig,
    ) -> Result<Arc<dyn HsmProvider>, HsmError> {
        // Use secure defaults for fallback
        let fallback_config = SoftHsmConfig {
            token_dir: config.software.token_dir.clone(),
            max_sessions: config.software.max_sessions,
            encryption_key: Some(Self::generate_fallback_encryption_key()?),
            lock_timeout_seconds: config.software.lock_timeout_seconds,
            use_testnet: config.bitcoin.network
                != crate::security::hsm::config::BitcoinNetworkType::Mainnet,
        };

        let audit_config = crate::security::hsm::audit::AuditLoggerConfig::default();
        let audit_logger =
            Arc::new(crate::security::hsm::audit::AuditLogger::new(&audit_config).await?);

        let provider = SoftwareHsmProvider::new(
            fallback_config,
            bitcoin::Network::from(config.bitcoin.network),
            audit_logger,
        )
        .await?;

        Ok(Arc::new(provider))
    }

    /// Simulator fallback (development only)
    async fn create_simulator_fallback(
        config: &HsmConfig,
    ) -> Result<Arc<dyn HsmProvider>, HsmError> {
        let fallback_config = crate::security::hsm::config::SimulatorConfig {
            storage_path: ".anya-hsm-sim".into(),
            simulate_latency: false, // Disable for fallback
            latency_ms: 0,
            simulate_failures: false, // Disable for fallback
            failure_rate: 0.0,
            pin_timeout_seconds: 300,
            max_pin_attempts: 3,
            use_testnet: config.bitcoin.network
                != crate::security::hsm::config::BitcoinNetworkType::Mainnet,
        };

        let provider = SimulatorHsmProvider::new(&fallback_config)?;
        Ok(Arc::new(provider))
    }

    /// Generate secure encryption key for fallback software HSM
    fn generate_fallback_encryption_key() -> Result<String, HsmError> {
        use rand::RngCore;

        let mut key = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);

        // Encode as base64 for configuration
        Ok(base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            &key,
        ))
    }

    /// Verify provider health before returning
    async fn verify_provider_health(provider: &dyn HsmProvider) -> Result<bool, HsmError> {
        // Check provider status
        match provider.get_status().await {
            Ok(status) => {
                match status {
                    HsmProviderStatus::Ready => {
                        // Perform basic health check if available
                        provider.perform_health_check().await
                    }
                    HsmProviderStatus::Initializing => {
                        warn!("Provider still initializing");
                        Ok(false)
                    }
                    HsmProviderStatus::Unavailable => {
                        warn!("Provider unavailable");
                        Ok(false)
                    }
                    HsmProviderStatus::NeedsAuthentication => {
                        warn!("Provider needs authentication");
                        Ok(false)
                    }
                    HsmProviderStatus::Error(e) => {
                        warn!("Provider error: {}", e);
                        Ok(false)
                    }
                }
            }
            Err(e) => {
                warn!("Failed to get provider status: {}", e);
                Ok(false)
            }
        }
    }

    /// Create provider without fallback (for testing)
    pub async fn create_specific_provider(
        provider_type: HsmProviderType,
        config: &HsmConfig,
    ) -> Result<Arc<dyn HsmProvider>, HsmError> {
        let mut specific_config = config.clone();
        specific_config.provider_type = provider_type;
        Self::create_primary_provider(&specific_config).await
    }
}

/// [AIR-3][AIS-3][BPC-3] Enhanced HSM Provider Factory for Production
///
/// Provides additional production-specific functionality:
/// - Configuration validation
/// - Security policy enforcement
/// - Provider capability detection
pub struct ProductionHsmFactory;

impl ProductionHsmFactory {
    /// Create HSM provider with production validation
    pub async fn create_for_production(
        config: &HsmConfig,
    ) -> Result<Arc<dyn HsmProvider>, HsmError> {
        // Validate configuration for production use
        Self::validate_production_config(config)?;

        // Create provider with fallback
        let provider = HsmProviderFactory::create_with_fallback(config).await?;

        // Verify production readiness
        Self::verify_production_readiness(&*provider).await?;

        Ok(provider)
    }

    /// Validate configuration meets production requirements
    fn validate_production_config(config: &HsmConfig) -> Result<(), HsmError> {
        // Network validation
        if config.bitcoin.network == crate::security::hsm::config::BitcoinNetworkType::Mainnet {
            // Mainnet requires hardware HSM or secure software configuration
            match config.provider_type {
                HsmProviderType::Simulator => {
                    return Err(HsmError::ConfigurationError(
                        "Simulator HSM cannot be used with Bitcoin mainnet".into(),
                    ));
                }
                HsmProviderType::SoftwareKeyStore => {
                    if config.software.encryption_key.is_none() {
                        return Err(HsmError::ConfigurationError(
                            "Software HSM requires encryption key for mainnet".into(),
                        ));
                    }
                }
                _ => {}
            }
        }

        // Audit logging validation
        if !config.audit_enabled {
            warn!("Audit logging is disabled - consider enabling for production");
        }

        Ok(())
    }

    /// Verify provider meets production readiness requirements
    async fn verify_production_readiness(provider: &dyn HsmProvider) -> Result<(), HsmError> {
        // Perform comprehensive health check
        if !provider.perform_health_check().await? {
            return Err(HsmError::InitializationError(
                "Provider failed production health check".into(),
            ));
        }

        info!("HSM provider passed production readiness verification");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::hsm::config::{BitcoinNetworkType, HsmConfig};

    #[tokio::test]
    async fn test_software_fallback_strategy() {
        let mut config = HsmConfig::default();
        config.provider_type = HsmProviderType::Hardware;

        // Force hardware failure by using invalid config
        config.hardware.connection_string = "invalid://connection".to_string();

        // Should fallback to software provider
        let provider = HsmProviderFactory::create_with_fallback(&config).await;
        assert!(provider.is_ok());

        let provider = provider.unwrap();
        let status = provider.get_status().await.unwrap();
        assert_ne!(status, HsmProviderStatus::Error("".into()));
    }

    #[tokio::test]
    async fn test_production_config_validation() {
        let mut config = HsmConfig::default();
        config.provider_type = HsmProviderType::Simulator;
        config.bitcoin.network = BitcoinNetworkType::Mainnet;

        // Should fail validation
        let result = ProductionHsmFactory::validate_production_config(&config);
        assert!(result.is_err());
    }
}
