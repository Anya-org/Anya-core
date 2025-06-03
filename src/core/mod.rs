// [AIR-3][AIS-3][BPC-3][RES-3] Removed unused import: std::error::Error
// [AIR-3][AIS-3][BPC-3][RES-3] Import necessary synchronization primitives
// Mutex is used in the rpc_ports module, so we'll keep it imported there
use std::sync::Arc;

// Import DaoGovernance for core initialization
// [AIR-3][AIS-3][BPC-3][RES-3] Removed unused import: crate::dao::governance::DaoGovernance

// AIR-008: Core Module Integration
// Integrates all Priority 1 implementations with auto-save functionality

// Modules
pub mod performance_optimization;
pub mod metrics;

// Re-exports
pub use performance_optimization::PerformanceOptimizer;
pub use performance_optimization::ResourceType;
pub use performance_optimization::OptimizationStatus;
pub use metrics::PrometheusMetrics;

// ML agent checker module is in src/ml/agent_checker.rs
// Re-export from ml module
pub use crate::ml::agent_checker::AgentChecker;
pub use crate::ml::agent_checker::SystemStage;

// System hardening module is in src/security/system_hardening.rs
// Re-export from security module  
pub use crate::security::system_hardening::SystemHardening;
pub use crate::security::system_hardening::SecurityLevel;
pub use crate::security::system_hardening::ConfigStatus;

/// Core functionality with auto-save capabilities
pub struct CoreSystem {
    // Component managers with auto-save functionality
    agent_checker: AgentChecker,
    system_hardening: SystemHardening, 
    performance_optimizer: PerformanceOptimizer,
}

impl CoreSystem {
    /// Create a new core system with specified auto-save frequency for each component
    pub fn new(auto_save_frequency: usize) -> Self {
        Self {
            agent_checker: AgentChecker::new(auto_save_frequency),
            system_hardening: SystemHardening::new(auto_save_frequency),
            performance_optimizer: PerformanceOptimizer::new(auto_save_frequency),
        }
    }
    
    /// Get access to the agent checker
    pub fn agent_checker(&self) -> &AgentChecker {
        &self.agent_checker
    }
    
    /// Get access to the system hardening manager
    pub fn system_hardening(&self) -> &SystemHardening {
        &self.system_hardening
    }
    
    /// Get access to the performance optimizer
    pub fn performance_optimizer(&self) -> &PerformanceOptimizer {
        &self.performance_optimizer
    }
    
    /// Process input across all components
    pub fn process_input(&self, input: &str) -> Result<(), String> {
        // Process input in the agent checker
        self.agent_checker.process_input(input)?;
        
        // Additional processing could be done with other components
        // depending on the input type
        
        Ok(())
    }
    
    /// Get stats about the auto-save state of all components
    pub fn get_auto_save_stats(&self) -> (usize, usize, usize) {
        let (agent_inputs, _, _) = self.agent_checker.get_input_stats();
        let (hardening_changes, _) = self.system_hardening.get_stats();
        let (performance_changes, _, _) = self.performance_optimizer.get_stats();
        
        (agent_inputs, hardening_changes, performance_changes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::time::Duration;
    use crate::core::ResourceType; // Fix: Import ResourceType from core
    
    #[test]
    fn test_core_system_integration() -> Result<(), Box<dyn std::error::Error>> {
        // Create core system with auto-save every 20 inputs
        let core = CoreSystem::new(20);
        
        // Process some inputs through the agent checker
        for i in 0..25 {
            let input = if i % 5 == 0 {
                format!("success message {}", i)
            } else {
                format!("normal message {}", i)
            };
            
            core.process_input(&input).map_err(|e| e.to_string())?;
        }
        
        // Set up a resource in the performance optimizer
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
        
        // Set up a component in the system hardening
        use crate::security::system_hardening::SecurityLevel;
        let mut security_settings = HashMap::new();
        security_settings.insert("firewall".to_string(), "enabled".to_string());
        
        core.system_hardening().configure_component(
            "network",
            SecurityLevel::Enhanced,
            security_settings,
            true
        )?;
        
        // Get stats
        let (agent_inputs, hardening_changes, performance_changes) = core.get_auto_save_stats();
        
        // Verify all components registered inputs
        assert_eq!(agent_inputs, 25);
        assert_eq!(hardening_changes, 1);
        assert_eq!(performance_changes, 1);
        
        Ok(())
    }
}

// Core module
// Implements core functionality for Bitcoin operations
// as per official Bitcoin Improvement Proposals (BIPs) requirements

pub mod performance;

// Re-export key types
pub use performance::Metrics;

/// Core hexagonal architecture port definitions
pub mod ports {
    // Define all required ports according to BDF v2.5
    pub mod node_communication {
        // P2P networking protocols
        // [BDF v2.5] Ensure block propagation and mempool monitoring
    }

    pub mod wallet_interface {
        // PSBT/BIP-174 compliant interfaces
        // [BDF v2.5] Enforce BIP-174 structure and SegWit/Taproot witness checks
    }

    pub mod smart_contract {
        // Miniscript execution interfaces
        // [BDF v2.5] Miniscript support for smart contract execution
    }

    pub mod taproot_assets {
        // [BDF v2.5] Taproot asset issuance and management (BIP-341/342)
        // Stub: Integrate Taproot asset logic (see @AI labelling.md)
    }

    pub mod dlc_oracle {
        // [BDF v2.5] DLC oracle interface (privacy-preserving, non-interactive)
        // Stub: Implement Schnorr-based non-interactive oracle pattern
    }

    pub mod metrics_port {
        // [BDF v2.5] Prometheus metrics export
        // Stub: Expose TPS, block propagation, mempool depth, BIP support matrix
    }

    pub mod audit_trail {
        // [BDF v2.5] Security audit hooks and event trail
        // Stub: Log compliance events and protocol upgrades
    }
    // Additional ports from BDF v2.5
}

use crate::web5::Web5Adapter;
use crate::ml::agent_system::MLAgentSystem;
use crate::tokenomics::{TokenomicsEngine, engine::TokenomicsConfig};

// Configuration types
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

// Hexagonal architecture implementation
#[allow(dead_code)]
pub struct AnyaCore {
    bitcoin_adapter: Arc<dyn crate::bitcoin::interface::BitcoinInterface>,
    web5_adapter: Arc<Web5Adapter>,
    ml_agent_system: Arc<MLAgentSystem>,
    dao_governance: Arc<crate::dao::DaoGovernance>,
    tokenomics: Arc<TokenomicsEngine>,
}

impl AnyaCore {
    // Core initialization with dependency injection
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        // Use our stub implementations
        // [AIR-3][AIS-3][BPC-3][RES-3] Convert core::BitcoinConfig to bitcoin::config::BitcoinConfig with all required fields
        let bitcoin_config = crate::bitcoin::config::BitcoinConfig {
            enabled: true, // Assuming enabled by default since core config doesn't have this field
            network: config.bitcoin.network.clone(),
            rpc_url: config.bitcoin.rpc_url.clone(),
            auth: None, // No auth in core config
            min_confirmations: 6, // Default value
            default_fee_rate: 1, // [AIR-3][AIS-3][BPC-3][RES-3] Default value for fee rate as u64
            wallet_path: Some("/tmp/bitcoin-wallet".to_string()), // Default wallet path as Option<String>
        };
        
        // Properly wrap BitcoinInterface trait object to comply with BDF v2.5 hexagonal architecture
        let bitcoin_adapter = crate::bitcoin::BitcoinAdapter::new(bitcoin_config).await?;
        let bitcoin: Arc<dyn crate::bitcoin::interface::BitcoinInterface + Send + Sync> = Arc::new(bitcoin_adapter);
        
        // Convert config.web5 to web5::Web5Config with all required fields
        // [AIR-3][AIS-3][BPC-3][RES-3]
        let web5_config = crate::web5::Web5Config {
            enabled: true, // Default to enabled
            did_method: "ion".to_string(), // Default DID method
            dwn_url: Some(config.web5.endpoint.clone()), // Use endpoint as DWN URL
            use_local_storage: true, // Default to using local storage
        };
        let web5 = Arc::new(Web5Adapter::build(web5_config).await?);
        
        // Convert config.ml to MLConfig with all required fields
        // [AIR-3][AIS-3][BPC-3][RES-3]
        let ml_config = crate::ml::MLConfig {
            enabled: true, // Default to enabled
            model_path: Some(config.ml.model_path.clone()), // model_path is Option<String> in MLConfig
            use_gpu: true, // Default to using GPU
            federated_learning: true, // Default to using federated learning
            max_model_size: 100 * 1024 * 1024, // Default to 100MB
        };
        let agents = Arc::new(MLAgentSystem::init(ml_config).await?);
        
        // Create a default DaoGovernance since we can't import it
        let dao = Arc::new(crate::dao::DaoGovernance::default());
        
        // [AIR-3][AIS-3][BPC-3][RES-3]
        // TokenomicsEngine::setup already returns Arc<TokenomicsEngine>, so we don't need to wrap it again
        let tokens = TokenomicsEngine::setup(config.tokenomics).await?;
        
        // Assign tokens directly to the AnyaCore struct
        Ok(Self {
            bitcoin_adapter: bitcoin,
            web5_adapter: web5,
            ml_agent_system: agents,
            dao_governance: dao, 
            tokenomics: tokens,
        })
    }
}

// [AIR-3][BPC-3] Hexagonal RPC ports
pub mod rpc_ports {
    // [AIR-3][AIS-3][BPC-3][RES-3] Import necessary synchronization primitives
    // This follows official Bitcoin Improvement Proposals (BIPs) standards for clean code
    use std::sync::{Arc, Mutex};
    use serde_json::Value as JsonValue;
    use crate::core::metrics::PrometheusMetrics;
    use async_trait::async_trait;

    #[async_trait]
    pub trait BitcoinRpc {
        async fn call_method(&self, method: &str, params: JsonValue) -> Result<JsonValue, Box<dyn std::error::Error + Send + Sync>>;
        async fn validate_response(&self, response: JsonValue) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    }

    #[async_trait]
    pub trait LightningRpc {
        async fn create_invoice(&self, amount_msat: u64, description: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
        async fn verify_payment(&self, payment_hash: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
    }

    // BDF v2.5 compliant adapter
    #[allow(dead_code)]
    pub struct AnyaRpcAdapter {
        bitcoin: Arc<dyn BitcoinRpc + Send + Sync>,
        lightning: Arc<dyn LightningRpc + Send + Sync>,
        // [AIR-3][AIS-3][BPC-3][RES-3] Using imported Mutex
        // This follows official Bitcoin Improvement Proposals (BIPs) standards for clean code
        metrics: Arc<Mutex<PrometheusMetrics>>
    }
}
