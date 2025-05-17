use std::error::Error;
use std::sync::{Arc, Mutex};

// Import DaoGovernance for core initialization
use crate::dao::governance::DaoGovernance;

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
            performance::ResourceType::Database,
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
// as per Bitcoin Development Framework v2.5 requirements

pub mod performance;

// Re-export key types
pub use performance::Metrics;

/// Core hexagonal architecture port definitions
pub mod ports {
    // Define all required ports according to BDF v2.5
    pub mod node_communication {
        // P2P networking protocols
    }
    
    pub mod wallet_interface {
        // PSBT/BIP-174 compliant interfaces
    }
    
    pub mod smart_contract {
        // Miniscript execution interfaces
    }
    
    // Additional ports from BDF v2.5
}

// Hexagonal architecture implementation
pub struct AnyaCore {
    bitcoin_adapter: Arc<BitcoinAdapter>,
    web5_adapter: Arc<Web5Adapter>,
    ml_agent_system: Arc<MLAgentSystem>,
    dao_governance: Arc<DaoGovernance>,
    tokenomics: Arc<TokenomicsEngine>,
}

impl AnyaCore {
    // Core initialization with dependency injection
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        // Import necessary adapter types
        use crate::bitcoin::adapters::BitcoinAdapter;
        use crate::web5::adapters::Web5Adapter;
        use crate::ml::agent::MLAgentSystem;
        use crate::tokenomics::engine::TokenomicsEngine;
        
        let bitcoin = BitcoinAdapter::new(config.bitcoin).await?;
        let web5 = Web5Adapter::build(config.web5).await?;
        let agents = MLAgentSystem::init(config.ml).await?;
        let dao = DaoGovernance::initialize(config.dao).await?;
        let tokenomics = TokenomicsEngine::new(config.tokenomics)?;
        
        // Wrap them in Arc since our struct fields expect Arc<T>
        let bitcoin = Arc::new(bitcoin);
        let web5 = Arc::new(web5);
        let agents = Arc::new(agents);
        
        // Both dao and tokenomics are already Arc-wrapped from their respective initialize methods
        Ok(Self {
            bitcoin_adapter: bitcoin,
            web5_adapter: web5,
            ml_agent_system: agents,
            dao_governance: dao, // dao is already an Arc<DaoGovernance>
            tokenomics: tokenomics, // tokenomics is already an Arc<TokenomicsEngine>
        })
    }
}

// [AIR-3][BPC-3] Hexagonal RPC ports
pub mod rpc_ports {
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
    pub struct AnyaRpcAdapter {
        bitcoin: Arc<dyn BitcoinRpc + Send + Sync>,
        lightning: Arc<dyn LightningRpc + Send + Sync>,
        metrics: Arc<Mutex<PrometheusMetrics>>
    }
} 

use crate::bitcoin::adapters::BitcoinAdapter;
use crate::tokenomics::TokenomicsEngine;
