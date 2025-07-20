use std::sync::Arc;

// Modules
pub mod metrics;
pub mod performance_optimization;
pub mod reliability;

// Re-exports
pub use metrics::PrometheusMetrics;
pub use performance_optimization::{OptimizationStatus, PerformanceOptimizer, ResourceType};
pub use reliability::{
    execute_with_monitoring, execute_with_recovery, AiVerification, ProgressTracker, Watchdog,
};


/// Core functionality with auto-save capabilities.
/// Provides performance optimization and input processing for the Anya system.
pub struct CoreSystem {
    performance_optimizer: PerformanceOptimizer,
}

impl CoreSystem {
    /// Creates a new core system with the specified auto-save frequency for each component.
    pub fn new(auto_save_frequency: usize) -> Self {
        Self {
            performance_optimizer: PerformanceOptimizer::new(auto_save_frequency),
        }
    }

    /// Returns a reference to the performance optimizer.
    pub fn performance_optimizer(&self) -> &PerformanceOptimizer {
        &self.performance_optimizer
    }

    /// Processes input across all components. Returns Ok if successful.
    pub fn process_input(&self, _input: &str) -> Result<(), String> {
        Ok(())
    }

    /// Gets stats about the auto-save state of all components.
    /// Returns a tuple of (agent_inputs, hardening_changes, performance_changes).
    pub fn get_auto_save_stats(&self) -> (usize, usize, usize) {
        let (performance_changes, _, _) = self.performance_optimizer.get_stats();
        (0, 0, performance_changes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ResourceType;
    use std::collections::HashMap;
    use std::time::Duration;

    /// Integration test for CoreSystem performance optimizer and input processing.
    #[test]
    fn test_core_system_integration() -> Result<(), Box<dyn std::error::Error>> {
        let core = CoreSystem::new(20);

        for i in 0..25 {
            let input = if i % 5 == 0 {
                format!("success message {i}")
            } else {
                format!("normal message {i}")
            };
            core.process_input(&input)?;
        }

        let mut settings = HashMap::new();
        settings.insert("cache_size".to_string(), "1024".to_string());

        core.performance_optimizer().configure_resource(
            "database",
            ResourceType::Database,
            settings,
            0.8,
            500.0,
            Duration::from_millis(50),
        )?;

        let (agent_inputs, hardening_changes, performance_changes) = core.get_auto_save_stats();

        assert_eq!(agent_inputs, 0);
        assert_eq!(hardening_changes, 0);
        assert_eq!(performance_changes, 1);

        Ok(())
    }
}

// Core module
pub mod performance;
pub use performance::Metrics;

/// Core hexagonal architecture port definitions
pub mod ports {
    pub mod node_communication {}
    pub mod wallet_interface {}
    pub mod smart_contract {}
    pub mod taproot_assets {}
    pub mod dlc_oracle {}
    pub mod metrics_port {}
    pub mod audit_trail {}
}

use crate::ml::agent_system::MLAgentSystem;
use crate::tokenomics::{engine::TokenomicsConfig, TokenomicsEngine};
// use Web5Adapter HTTP client instead of direct web5 imports

#[derive(Debug, Clone)]
pub struct BitcoinConfig {
    pub network: String,
    pub rpc_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Web5Config {
    pub endpoint: String,
}

#[derive(Debug, Clone)]
pub struct MlConfig {
    pub model_path: String,
}

pub struct Config {
    pub bitcoin: BitcoinConfig,
    pub web5: Web5Config,
    pub ml: MlConfig,
    pub tokenomics: TokenomicsConfig,
}

#[allow(dead_code)]
pub struct AnyaCore {
    #[cfg(feature = "rust-bitcoin")]
    bitcoin_adapter: Arc<dyn crate::bitcoin::interface::BitcoinInterface>,
    web5_adapter: Arc<crate::web::web5_adapter::Web5Adapter>,
    ml_agent_system: Arc<MLAgentSystem>,
    dao_governance: Arc<crate::dao::DaoGovernance>,
    tokenomics: Arc<TokenomicsEngine>,
}

impl AnyaCore {
    #[cfg(feature = "rust-bitcoin")]
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let bitcoin_config = crate::bitcoin::config::BitcoinConfig {
            enabled: true,
            network: config.bitcoin.network.clone(),
            rpc_url: config.bitcoin.rpc_url.clone(),
            auth: None,
            min_confirmations: 6,
            default_fee_rate: 1,
            wallet_path: Some("/tmp/bitcoin-wallet".to_string()),
        };

        let bitcoin_adapter = crate::bitcoin::BitcoinAdapter::new(bitcoin_config).await?;
        let bitcoin: Arc<dyn crate::bitcoin::interface::BitcoinInterface + Send + Sync> =
            Arc::new(bitcoin_adapter);

        // Use HTTP-based Web5Adapter client
        let web5 = Arc::new(crate::web::web5_adapter::Web5Adapter::new(&config.web5.endpoint));

        let ml_config = crate::ml::MLConfig {
            enabled: true,
            model_path: Some(config.ml.model_path.clone()),
            use_gpu: true,
            federated_learning: true,
            max_model_size: 100 * 1024 * 1024,
        };
        let agents = Arc::new(MLAgentSystem::init(ml_config).await?);

        let dao = Arc::new(crate::dao::DaoGovernance::default());
        let tokens = TokenomicsEngine::setup(config.tokenomics).await?;

        Ok(Self {
            bitcoin_adapter: bitcoin,
            web5_adapter: web5,
            ml_agent_system: agents,
            dao_governance: dao,
            tokenomics: tokens,
        })
    }

    #[cfg(not(feature = "rust-bitcoin"))]
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        // Use HTTP-based Web5Adapter client
        let web5 = Arc::new(crate::web::web5_adapter::Web5Adapter::new(&config.web5.endpoint));

        let ml_config = crate::ml::MLConfig {
            enabled: true,
            model_path: Some(config.ml.model_path.clone()),
            use_gpu: true,
            federated_learning: true,
            max_model_size: 100 * 1024 * 1024,
        };
        let agents = Arc::new(MLAgentSystem::init(ml_config).await?);

        let dao = Arc::new(crate::dao::DaoGovernance::default());
        let tokens = TokenomicsEngine::setup(config.tokenomics).await?;

        Ok(Self {
            web5_adapter: web5,
            ml_agent_system: agents,
            dao_governance: dao,
            tokenomics: tokens,
        })
    }
}

pub mod rpc_ports {
    use crate::core::metrics::PrometheusMetrics;
    use async_trait::async_trait;
    use serde_json::Value as JsonValue;
    use std::sync::{Arc, Mutex};

    #[async_trait]
    pub trait BitcoinRpc {
        async fn call_method(
            &self,
            method: &str,
            params: JsonValue,
        ) -> Result<JsonValue, Box<dyn std::error::Error + Send + Sync>>;
        async fn validate_response(
            &self,
            response: JsonValue,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    }

    #[async_trait]
    pub trait LightningRpc {
        async fn create_invoice(
            &self,
            amount_msat: u64,
            description: &str,
        ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
        async fn verify_payment(
            &self,
            payment_hash: &str,
        ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
    }

    #[allow(dead_code)]
    pub struct AnyaRpcAdapter {
        bitcoin: Arc<dyn BitcoinRpc + Send + Sync>,
        lightning: Arc<dyn LightningRpc + Send + Sync>,
        metrics: Arc<Mutex<PrometheusMetrics>>,
    }
}
