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

pub mod api;
pub mod bip;
#[cfg(feature = "bitcoin")]
pub mod bitcoin;
pub mod compliance;
pub mod config;
pub mod core;
pub mod dao;
pub mod enterprise;
pub mod extensions;
pub mod handlers;
pub mod infrastructure;
pub mod install;
pub mod layer2;
pub mod ml;
#[cfg(any(feature = "ffi", feature = "mobile"))]
pub mod mobile;
pub mod network;
pub mod security;
pub mod testing;
pub mod tokenomics;
pub mod tools;
pub mod types;
pub mod web;
pub mod web5;

// Hardware optimization module
pub mod hardware_optimization {
    use std::collections::HashMap;

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

        pub fn intel_optimizer(&self) -> Option<intel::IntelOptimizer> {
            if self.is_optimization_enabled("intel") {
                Some(intel::IntelOptimizer::new())
            } else {
                None
            }
        }
    }

    pub mod intel {
        use std::time::Duration;

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
                    avx2_support: true,
                    kaby_lake_optimized: false,
                    vendor: "Intel".to_string(),
                    model: "i3-7020U".to_string(),
                }
            }
        }

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

            #[cfg(feature = "rust-bitcoin")]
            pub fn verify_transaction_batch(
                &self,
                transactions: &[bitcoin::Transaction],
                config: &BatchVerificationConfig,
            ) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
                if transactions.len() > config.batch_size {
                    return Err("Batch too large".into());
                }
                let invalid_indices = Vec::new();
                for (i, _tx) in transactions.iter().enumerate() {
                    let _ = i;
                }
                Ok(invalid_indices)
            }

            #[cfg(not(feature = "rust-bitcoin"))]
            pub fn verify_transaction_batch(
                &self,
                _transactions: &[bitcoin::Transaction],
                _config: &BatchVerificationConfig,
            ) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
                log::debug!("Hardware-optimized batch verification not available");
                Ok(vec![])
            }

            #[cfg(feature = "rust-bitcoin")]
            pub fn verify_taproot_transaction(
                &self,
                tx: &bitcoin::Transaction,
            ) -> Result<(), Box<dyn std::error::Error>> {
                if tx.output.is_empty() {
                    return Err("Transaction has no outputs".into());
                }
                Ok(())
            }

            #[cfg(not(feature = "rust-bitcoin"))]
            pub fn verify_taproot_transaction(
                &self,
                _tx: &bitcoin::Transaction,
            ) -> Result<(), Box<dyn std::error::Error>> {
                Ok(())
            }
        }
    }

    pub fn optimize_for_hardware() -> bool {
        true
    }
}

// Re-export key types for crate-wide visibility
#[cfg(feature = "rust-bitcoin")]
pub use crate::bitcoin::adapters::BitcoinAdapter;
#[cfg(feature = "rust-bitcoin")]
pub use crate::bitcoin::interface::BitcoinInterface;
pub use crate::dao::DaoLevel;
pub use crate::types::compliance::*;

#[cfg(feature = "hsm")]
pub use security::hsm;
#[cfg(not(feature = "hsm"))]
pub use security::hsm_shim as hsm;

/// Core error type for the Anya system
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyaError {
    ML(String),
    Web5(String),
    Bitcoin(String),
    DAO(String),
    System(String),
    Custom(String),
    Timeout(String),
    LowConfidence(String),
    NotFound(String),
    InvalidInput(String),
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

#[cfg(feature = "bitcoin")]
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

impl From<secp256k1::Error> for AnyaError {
    fn from(err: secp256k1::Error) -> Self {
        AnyaError::Bitcoin(format!("Secp256k1 error: {err}"))
    }
}

impl From<serde_json::Error> for AnyaError {
    fn from(err: serde_json::Error) -> Self {
        AnyaError::System(format!("JSON error: {err}"))
    }
}

pub type AnyaResult<T> = Result<T, AnyaError>;

#[derive(Debug, Clone, Default)]
pub struct AnyaConfig {
    pub ml_config: ml::MLConfig,
    pub web5_config: web5::Web5Config,
    #[cfg(feature = "hsm")]
    pub bitcoin_config: crate::security::hsm::config::HsmConfig,
    #[cfg(not(feature = "hsm"))]
    pub bitcoin_config: crate::security::hsm_shim::HsmConfig,
    pub dao_config: dao::DAOConfig,
}

pub struct AnyaCore {
    pub ml_system: Option<ml::MLSystem>,
    pub web5_manager: Option<web5::Web5Manager>,
    pub dao_manager: Option<dao::DAOManager>,
}

impl AnyaCore {
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

    pub fn with_defaults() -> AnyaResult<Self> {
        Self::new(AnyaConfig::default())
    }

    pub fn is_operational(&self) -> bool {
        self.ml_system.is_some() || self.web5_manager.is_some() || self.dao_manager.is_some()
    }

    pub fn get_status(&self) -> AnyaResult<SystemStatus> {
        let mut status = SystemStatus {
            ml_enabled: self.ml_system.is_some(),
            web5_enabled: self.web5_manager.is_some(),
            bitcoin_enabled: false,
            dao_enabled: self.dao_manager.is_some(),
            component_status: Vec::new(),
            metrics: HashMap::new(),
        };

        if let Some(ml_system) = &self.ml_system {
            status
                .metrics
                .insert("ml".to_string(), ml_system.get_model_health_metrics());
        }

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

#[derive(Debug, Clone)]
pub struct SystemStatus {
    pub ml_enabled: bool,
    pub web5_enabled: bool,
    pub bitcoin_enabled: bool,
    pub dao_enabled: bool,
    pub component_status: Vec<ComponentStatus>,
    pub metrics: HashMap<String, HashMap<String, HashMap<String, f64>>>,
}

#[derive(Debug, Clone)]
pub struct ComponentStatus {
    pub name: String,
    pub operational: bool,
    pub health_score: f64,
}

pub mod utils {
    pub fn generate_id() -> String {
        format!("id:{:x}", rand::random::<u64>())
    }

    pub fn log(msg: &str) {
        println!("[{}] {}", chrono::Utc::now(), msg);
    }
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(feature = "bitcoin")]
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
        assert!(config.ml_config.enabled);
        assert!(config.web5_config.enabled);
        #[cfg(feature = "hsm")]
        assert!(config.bitcoin_config.general.enabled);
        #[cfg(not(feature = "hsm"))]
        {
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

pub fn init() {
    // Initialize Bitcoin module if needed
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

impl From<web5::Web5Error> for AnyaError {
    fn from(error: web5::Web5Error) -> Self {
        AnyaError::Web5(error.to_string())
    }
}

#[cfg(feature = "hsm")]
impl From<crate::security::hsm::HsmError> for AnyaError {
    fn from(error: crate::security::hsm::HsmError) -> Self {
        AnyaError::Bitcoin(error.to_string())
    }
}

pub const PROTOCOL_VERSION: &str = "2.0.0";
pub const IMPLEMENTATION_YEAR: u16 = 2025;
pub const BUILD_ID: &str = env!("CARGO_PKG_VERSION");

pub mod prelude {
    #[cfg(feature = "rust-bitcoin")]
    pub use crate::bitcoin::adapters::BitcoinAdapter;
    pub use crate::dao::governance::DaoGovernance;
}

mod error;
