//! Anya Core Library
//!
//! This is the core library for the Anya system, providing fundamental
//! functionality for machine learning, Web5 integration, and Bitcoin operations.
//!
//! # Architecture
//!
//! The library is organized into several main modules:
//! - `ml`: Machine learning components and AI agent system
//! - `web5`: Web5 protocol integration and decentralized identity
//! - `bitcoin`: Bitcoin and Lightning Network functionality
//! - `dao`: Decentralized autonomous organization components
//! - `utils`: Common utilities and helper functions
//!
//! # Features
//!
//! - Advanced ML capabilities with federated learning
//! - Web5 protocol implementation for decentralized data management
//! - Bitcoin and Lightning Network support
//! - DAO governance and voting
//! - Comprehensive security and privacy features
//!
//! # Examples
//!
//! ```rust,no_run
//! use anya_core::{ml, web5, bitcoin, AnyaConfig, AnyaCore};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Initialize Anya with default configuration
//! let anya = AnyaCore::default()?;
//!
//! // Or with custom configuration
//! let config = AnyaConfig::default();
//! let anya_custom = AnyaCore::new(config)?;
//!
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
// [AIR-3][AIS-3][BPC-3][RES-3] Removed unused import: std::sync::Arc

pub mod api;
pub mod bip;
pub mod bitcoin;
pub mod compliance;
pub mod dao;
pub mod enterprise; // Enterprise communications and NostrClient
pub mod extensions;
pub mod install;
pub mod ml;
pub mod network; // Network validation and related functionality
pub mod security; // Security and cryptographic functionality
pub mod testing;
pub mod types;
pub mod web5;

// Infrastructure module
pub mod infrastructure;

// Monitoring module
pub mod monitoring;

// Add missing modules to fix compilation errors

pub mod protocols;

pub mod auth {
    //! Authentication module
    //!
    //! This module provides authentication functionality by re-exporting
    //! authentication capabilities from security module and specialized
    //! authentication implementations.
    //!
    //! # Features
    //!
    //! - BIP353 authentication support
    //! - API authentication handlers
    //! - Re-exports from the security module
    //!
    //! # Usage
    //!
    //! ```rust
    //! use anya_core::auth;
    //!
    //! // BIP353 authentication
    //! // let auth_result = auth::bip353_auth::authenticate(...);
    //!
    //! // API authentication handlers
    //! // auth::handlers::auth::login(...);
    //! ```
    pub use crate::api::handlers::auth::*;
    // Removed circular reference: pub use crate::auth::*;
    pub use crate::bip::bip353_auth::*;
    pub use crate::security::*;
}

pub mod hardware_optimization {
    //! Hardware optimization module
    //!
    //! This module provides hardware optimization functionality for Bitcoin
    //! transaction validation and other computationally intensive operations.
    //!
    //! # Features
    //!
    //! - Hardware-specific optimization
    //! - Batch verification configuration
    //! - Intel-specific optimizations
    //!
    //! # Usage
    //!
    //! ```rust
    //! use anya_core::hardware_optimization::{HardwareOptimizationManager, intel::BatchVerificationConfig};
    //!
    //! let manager = HardwareOptimizationManager::new();
    //! let config = BatchVerificationConfig::default();
    //! ```
    
    use std::collections::HashMap;
    
    /// Hardware optimization manager for coordinating various optimization strategies
    #[derive(Debug, Clone)]
    pub struct HardwareOptimizationManager {
        optimizations: HashMap<String, bool>,
    }
    
    impl Default for HardwareOptimizationManager {
        fn default() -> Self {
            Self::new()
        }
    }
    
    impl HardwareOptimizationManager {
        pub fn new() -> Self {
            Self {
                optimizations: HashMap::new(),
            }
        }
        
        pub fn enable_optimization(&mut self, name: &str) {
            self.optimizations.insert(name.to_string(), true);
        }
        
        pub fn is_optimization_enabled(&self, name: &str) -> bool {
            self.optimizations.get(name).copied().unwrap_or(false)
        }
        
        /// Get Intel-specific optimizer if available
        pub fn intel_optimizer(&self) -> Option<intel::IntelOptimizer> {
            if self.is_optimization_enabled("intel") {
                Some(intel::IntelOptimizer::new())
            } else {
                None
            }
        }
    }
    
    /// Intel-specific hardware optimizations
    pub mod intel {
        use std::time::Duration;
        
        /// Configuration for batch verification operations on Intel hardware
        #[derive(Debug, Clone)]
        pub struct BatchVerificationConfig {
            pub batch_size: usize,
            pub timeout: Duration,
            pub use_avx: bool,
            pub use_sse: bool,
        }
        
        impl Default for BatchVerificationConfig {
            fn default() -> Self {
                Self {
                    batch_size: 64,
                    timeout: Duration::from_secs(30),
                    use_avx: true,
                    use_sse: true,
                }
            }
        }
        
        impl BatchVerificationConfig {
            pub fn new() -> Self {
                Self::default()
            }
            
            pub fn with_batch_size(mut self, size: usize) -> Self {
                self.batch_size = size;
                self
            }
            
            pub fn with_timeout(mut self, timeout: Duration) -> Self {
                self.timeout = timeout;
                self
            }
        }
        
        /// Intel CPU capabilities detection
        #[derive(Debug, Clone)]
        pub struct CpuCapabilities {
            pub avx2_support: bool,
            pub kaby_lake_optimized: bool,
            pub vendor: String,
            pub model: String,
        }
        
        impl Default for CpuCapabilities {
            fn default() -> Self {
                Self {
                    avx2_support: true, // Assume modern CPU
                    kaby_lake_optimized: false,
                    vendor: "Intel".to_string(),
                    model: "i3-7020U".to_string(),
                }
            }
        }
        
        /// Intel-specific optimizer
        #[derive(Debug, Clone)]
        pub struct IntelOptimizer {
            capabilities: CpuCapabilities,
        }
        
        impl Default for IntelOptimizer {
            fn default() -> Self {
                Self::new()
            }
        }
        
        impl IntelOptimizer {
            pub fn new() -> Self {
                Self {
                    capabilities: CpuCapabilities::default(),
                }
            }
            
            pub fn capabilities(&self) -> &CpuCapabilities {
                &self.capabilities
            }
            
            /// Verify a batch of transactions using Intel optimizations
            pub fn verify_transaction_batch(
                &self, 
                transactions: &[bitcoin::Transaction], 
                config: &BatchVerificationConfig
            ) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
                // Placeholder implementation for batch verification
                // Returns indices of invalid transactions
                
                if transactions.len() > config.batch_size {
                    return Err("Batch too large".into());
                }
                
                let invalid_indices = Vec::new();
                
                // Simulate verification process
                for (i, _tx) in transactions.iter().enumerate() {
                    // Basic validation placeholder
                    // Real implementation would verify signatures, scripts, etc.
                    // For now, assume all transactions are valid
                    let _ = i; // Placeholder to avoid unused variable warning
                }
                
                Ok(invalid_indices)
            }
            
            /// Verify a single Taproot transaction using Intel optimizations
            pub fn verify_taproot_transaction(
                &self,
                tx: &bitcoin::Transaction
            ) -> Result<(), Box<dyn std::error::Error>> {
                // Placeholder implementation for Taproot verification
                // In a real implementation, this would use Intel-specific optimizations
                // for Schnorr signature verification and Taproot script validation
                
                if tx.output.is_empty() {
                    return Err("Transaction has no outputs".into());
                }
                
                // Simulate Taproot validation process
                Ok(())
            }
        }
    }
    
    // Basic optimization functions
    pub fn optimize_for_hardware() -> bool {
        // Placeholder implementation
        true
    }
}

// Re-export key types for crate-wide visibility
// [AIR-3][BPC-3] Following official Bitcoin Improvement Proposals (BIPs)
pub use crate::bitcoin::adapters::BitcoinAdapter;
pub use crate::bitcoin::interface::BitcoinInterface;
pub use crate::dao::DaoLevel;
pub use crate::types::compliance::*;

// Export core types will be defined below
pub mod config;
pub mod core;

// Re-export HSM types for convenience when feature is enabled
#[cfg(feature = "hsm")]
pub use security::hsm;
#[cfg(not(feature = "hsm"))]
pub use security::hsm_shim as hsm;
pub mod layer2;
pub mod tokenomics;
pub mod tools;

/// Core error type for the Anya system
/// [AIR-3][AIS-3][BPC-3][RES-3]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyaError {
    /// ML-related errors
    ML(String),
    /// Web5-related errors
    Web5(String),
    /// Bitcoin-related errors
    Bitcoin(String),
    /// DAO-related errors
    DAO(String),
    /// General system errors
    System(String),
    /// Generic errors
    Custom(String),
    /// Timeout errors
    Timeout(String),
    /// Low confidence AI output errors
    LowConfidence(String),
    /// Resource not found errors
    NotFound(String),
    /// Invalid input errors
    InvalidInput(String),
    /// Performance-related errors
    PerformanceError(String),
}

impl fmt::Display for AnyaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnyaError::ML(msg) => write!(f, "ML error: {msg}"),
            AnyaError::Web5(msg) => write!(f, "Web5 error: {msg}"),
            AnyaError::Bitcoin(msg) => write!(f, "Bitcoin error: {msg}"),
            AnyaError::DAO(msg) => write!(f, "DAO error: {msg}"),
            AnyaError::System(msg) => write!(f, "System error: {msg}"),
            AnyaError::Custom(msg) => write!(f, "Custom error: {msg}"),
            AnyaError::Timeout(msg) => write!(f, "Timeout error: {msg}"),
            AnyaError::LowConfidence(msg) => write!(f, "Low confidence error: {msg}"),
            AnyaError::NotFound(msg) => write!(f, "Not found error: {msg}"),
            AnyaError::InvalidInput(msg) => write!(f, "Invalid input error: {msg}"),
            AnyaError::PerformanceError(msg) => write!(f, "Performance error: {msg}"),
        }
    }
}

impl Error for AnyaError {}

impl From<crate::bitcoin::error::BitcoinError> for AnyaError {
    fn from(err: crate::bitcoin::error::BitcoinError) -> Self {
        AnyaError::Bitcoin(err.to_string())
    }
}

impl From<String> for AnyaError {
    fn from(err: String) -> Self {
        AnyaError::Custom(err)
    }
}

// Use the secp256k1 crate directly
impl From<secp256k1::Error> for AnyaError {
    fn from(err: secp256k1::Error) -> Self {
        AnyaError::Bitcoin(format!("Secp256k1 error: {err}"))
    }
}

// Add conversion from serde_json::Error to AnyaError
impl From<serde_json::Error> for AnyaError {
    fn from(err: serde_json::Error) -> Self {
        AnyaError::System(format!("JSON error: {err}"))
    }
}

/// Result type for Anya operations
pub type AnyaResult<T> = Result<T, AnyaError>;

/// Core configuration for the Anya system
#[derive(Debug, Clone)]
pub struct AnyaConfig {
    /// ML system configuration
    pub ml_config: ml::MLConfig,
    /// Web5 configuration
    pub web5_config: web5::Web5Config,
    /// Bitcoin network configuration
    #[cfg(feature = "hsm")]
    pub bitcoin_config: crate::security::hsm::config::HsmConfig,
    #[cfg(not(feature = "hsm"))]
    pub bitcoin_config: crate::security::hsm_shim::HsmConfig,
    /// DAO configuration
    pub dao_config: dao::DAOConfig,
}

impl Default for AnyaConfig {
    fn default() -> Self {
        Self {
            ml_config: ml::MLConfig::default(),
            web5_config: web5::Web5Config::default(),
            #[cfg(feature = "hsm")]
            bitcoin_config: crate::security::hsm::config::HsmConfig::default(),
            #[cfg(not(feature = "hsm"))]
            bitcoin_config: crate::security::hsm_shim::HsmConfig,
            dao_config: dao::DAOConfig::default(),
        }
    }
}

/// Core Anya system
pub struct AnyaCore {
    /// ML system
    pub ml_system: Option<ml::MLSystem>,
    /// Web5 manager
    pub web5_manager: Option<web5::Web5Manager>,
    /// DAO manager
    pub dao_manager: Option<dao::DAOManager>,
}

impl AnyaCore {
    /// Create a new AnyaCore with the given configuration
    pub fn new(config: AnyaConfig) -> AnyaResult<Self> {
        let ml_system = if config.ml_config.enabled {
            Some(ml::MLSystem::new(config.ml_config)?)
        } else {
            None
        };

        let web5_manager = if config.web5_config.enabled {
            match web5::Web5Manager::new(config.web5_config) {
                Ok(manager) => Some(manager),
                Err(e) => return Err(AnyaError::Web5(e.to_string())),
            }
        } else {
            None
        };

        let dao_manager = if config.dao_config.enabled {
            match dao::DAOManager::new(config.dao_config) {
                Ok(manager) => Some(manager),
                Err(e) => {
                    return Err(AnyaError::Custom(format!(
                        "Failed to initialize DAO manager: {e}"
                    )))
                }
            }
        } else {
            None
        };

        Ok(Self {
            ml_system,
            web5_manager,
            dao_manager,
        })
    }

    /// Initialize the AnyaCore with default configuration
    pub fn with_defaults() -> AnyaResult<Self> {
        Self::new(AnyaConfig::default())
    }

    /// Check if the system is operational
    pub fn is_operational(&self) -> bool {
        // A basic check that at least one core component is enabled
        self.ml_system.is_some() || self.web5_manager.is_some() || self.dao_manager.is_some()
    }

    /// Get system status information
    pub fn get_status(&self) -> AnyaResult<SystemStatus> {
        let mut status = SystemStatus {
            ml_enabled: self.ml_system.is_some(),
            web5_enabled: self.web5_manager.is_some(),
            // Temporarily disable bitcoin_enabled check as the field doesn't exist
            bitcoin_enabled: false,
            dao_enabled: self.dao_manager.is_some(),
            component_status: Vec::new(),
            metrics: HashMap::new(),
        };

        // Add component-specific status
        if let Some(ml_system) = &self.ml_system {
            status
                .metrics
                .insert("ml".to_string(), ml_system.get_model_health_metrics());
        }

        // Add status for each component
        status.component_status.push(ComponentStatus {
            name: "ml".to_string(),
            operational: self.ml_system.is_some(),
            health_score: if self.ml_system.is_some() { 1.0 } else { 0.0 },
        });

        status.component_status.push(ComponentStatus {
            name: "web5".to_string(),
            operational: self.web5_manager.is_some(),
            health_score: if self.web5_manager.is_some() {
                1.0
            } else {
                0.0
            },
        });

        status.component_status.push(ComponentStatus {
            name: "dao".to_string(),
            operational: self.dao_manager.is_some(),
            health_score: if self.dao_manager.is_some() { 1.0 } else { 0.0 },
        });

        Ok(status)
    }
}

/// System status information
#[derive(Debug, Clone)]
pub struct SystemStatus {
    /// Whether ML is enabled
    pub ml_enabled: bool,
    /// Whether Web5 is enabled
    pub web5_enabled: bool,
    /// Whether Bitcoin is enabled
    pub bitcoin_enabled: bool,
    /// Whether DAO is enabled
    pub dao_enabled: bool,
    /// Status of individual components
    pub component_status: Vec<ComponentStatus>,
    /// Metrics for all components
    pub metrics: HashMap<String, HashMap<String, HashMap<String, f64>>>,
}

/// Component status information
#[derive(Debug, Clone)]
pub struct ComponentStatus {
    /// Component name
    pub name: String,
    /// Whether the component is operational
    pub operational: bool,
    /// Health score (0.0-1.0)
    pub health_score: f64,
}

/// Utils module for common functionality
pub mod utils {
    /// Generate a random ID string
    pub fn generate_id() -> String {
        format!("id:{:x}", rand::random::<u64>())
    }

    /// Log a message
    pub fn log(msg: &str) {
        println!("[{}] {}", chrono::Utc::now(), msg);
    }
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(feature = "bitcoin_integration")]
pub mod integration {
    pub fn bitcoin_enabled() -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = AnyaConfig::default();
        // [AIR-3][AIS-3][BPC-3][RES-3] Use correct field structure as per BDF v2.5 hexagonal architecture
        assert!(config.ml_config.enabled);
        assert!(config.web5_config.enabled);
        // Check HSM config through the bitcoin_config field
        // For HSM config, we check the general.enabled field as per BDF v2.5 standards
        #[cfg(feature = "hsm")]
        assert!(config.bitcoin_config.general.enabled);
        // In non-HSM builds, the config is just an empty struct, so no assertions needed
        #[cfg(not(feature = "hsm"))]
        {
            // Just check that the config exists
            let _ = &config.bitcoin_config;
        }
        assert!(config.dao_config.enabled);
    }

    #[test]
    fn test_error_display() {
        let err = AnyaError::ML("test error".to_string());
        assert_eq!(err.to_string(), "ML error: test error");
    }
}

// Initialize all modules
pub fn init() {
    // Initialize Bitcoin module
    // bitcoin::init();
}

// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

// Add From implementations for Web5Error and BitcoinError
impl From<web5::Web5Error> for AnyaError {
    fn from(error: web5::Web5Error) -> Self {
        AnyaError::Web5(error.to_string())
    }
}

#[cfg(feature = "hsm")]
impl From<crate::security::hsm::HsmError> for AnyaError {
    #[cfg(feature = "hsm")]
    fn from(error: crate::security::hsm::HsmError) -> Self {
        AnyaError::Bitcoin(error.to_string())
    }
}

/// Protocol version
pub const PROTOCOL_VERSION: &str = "2.0.0";

/// Year of implementation
pub const IMPLEMENTATION_YEAR: u16 = 2025;

/// Build identifier
pub const BUILD_ID: &str = env!("CARGO_PKG_VERSION");

/// Module re-exports for convenience
pub mod prelude {
    pub use crate::dao::governance::DaoGovernance;
    // pub use crate::dao::DaoLevel; // Now re-exported at crate root
    // pub use crate::bitcoin::interface::BitcoinInterface;
    pub use crate::bitcoin::adapters::BitcoinAdapter; // Now re-exported at crate root
                                                      // pub use crate::tools::markdown::DocumentationValidator;
                                                      // pub use crate::security::hsm::TaprootValidator;
}

mod error;
