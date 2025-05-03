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

use std::error::Error;
use std::fmt;
use std::collections::HashMap;

// Core modules
pub mod ml;
pub mod web5;
pub mod bitcoin;
pub mod dao;
pub mod extensions;
// Remove config since it has conflicts
// pub mod config;
pub mod core;
pub mod security;

// Main tools module export
mod tools {
    pub mod markdown;
}

/// Core error type for the Anya system
#[derive(Debug)]
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
    Generic(String),
}

impl fmt::Display for AnyaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnyaError::ML(msg) => write!(f, "ML error: {}", msg),
            AnyaError::Web5(msg) => write!(f, "Web5 error: {}", msg),
            AnyaError::Bitcoin(msg) => write!(f, "Bitcoin error: {}", msg),
            AnyaError::DAO(msg) => write!(f, "DAO error: {}", msg),
            AnyaError::System(msg) => write!(f, "System error: {}", msg),
            AnyaError::Generic(msg) => write!(f, "Generic error: {}", msg),
        }
    }
}

impl Error for AnyaError {}

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
    pub bitcoin_config: bitcoin::BitcoinConfig,
    /// DAO configuration
    pub dao_config: dao::DAOConfig,
}

impl Default for AnyaConfig {
    fn default() -> Self {
        Self {
            ml_config: ml::MLConfig::default(),
            web5_config: web5::Web5Config::default(),
            bitcoin_config: bitcoin::BitcoinConfig::default(),
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
    /// Bitcoin manager
    pub bitcoin_manager: Option<bitcoin::BitcoinManager>,
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

        // Initialize metrics for Bitcoin Core alignment
        let metrics = HashMap::new();

        let web5_manager = if config.web5_config.enabled {
            Some(web5::Web5Manager::new(config.web5_config)?)
        } else {
            None
        };

        let bitcoin_manager = if config.bitcoin_config.enabled {
            Some(bitcoin::BitcoinManager::new(config.bitcoin_config)?)
        } else {
            None
        };

        let dao_manager = if config.dao_config.enabled {
            Some(dao::DAOManager::new(config.dao_config)?)
        } else {
            None
        };

        Ok(Self {
            ml_system,
            web5_manager,
            bitcoin_manager,
            dao_manager,
        })
    }

    /// Initialize the AnyaCore with default configuration
    pub fn default() -> AnyaResult<Self> {
        Self::new(AnyaConfig::default())
    }

    /// Check if the system is operational
    pub fn is_operational(&self) -> bool {
        // A basic check that at least one core component is enabled
        self.ml_system.is_some() || self.web5_manager.is_some() || 
        self.bitcoin_manager.is_some() || self.dao_manager.is_some()
    }

    /// Get system status information
    pub fn get_status(&self) -> AnyaResult<SystemStatus> {
        let mut status = SystemStatus {
            ml_enabled: self.ml_system.is_some(),
            web5_enabled: self.web5_manager.is_some(),
            bitcoin_enabled: self.bitcoin_manager.is_some(),
            dao_enabled: self.dao_manager.is_some(),
            component_status: Vec::new(),
            metrics: HashMap::new(),
        };

        // Add component-specific status
        if let Some(ml_system) = &self.ml_system {
            status.metrics.insert("ml".to_string(), ml_system.get_health_metrics());
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
            health_score: if self.web5_manager.is_some() { 1.0 } else { 0.0 },
        });

        status.component_status.push(ComponentStatus {
            name: "bitcoin".to_string(),
            operational: self.bitcoin_manager.is_some(),
            health_score: if self.bitcoin_manager.is_some() { 1.0 } else { 0.0 },
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
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        println!("[{}] {}", now.as_secs(), msg);
    }
}

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(feature = "bitcoin_integration")]
pub mod integration {
    pub fn bitcoin_enabled() -> bool {
        true
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
    pub use crate::bitcoin::{BitcoinProtocol, BPCLevel};
    pub use crate::bitcoin::taproot::TaprootValidator;
    pub use crate::dao::DaoGovernance;
    pub use crate::dao::DaoLevel;
    pub use crate::tools::markdown::DocumentationValidator;
}

/// Public exports for use in other crates
pub use bitcoin::protocol as bitcoin_protocol;
pub use web5::identity as web5_identity;
pub use security::crypto as security_crypto;

/// Test modules 
#[cfg(test)]
pub mod test_modules {
    /// Bitcoin test modules
    pub mod bitcoin {
        /// Protocol tests
        pub mod protocol {
            // Placeholder for future test runner
        }
    }
}

// Re-export core types from dependencies
pub use secp256k1;
pub use lightning;

// Export primary error type
pub use anyhow::Error as AnyhowError;

/// Check if the library is compiled with a specific feature
#[inline]
pub fn has_feature(feature: &str) -> bool {
    match feature {
        "bitcoin" => cfg!(feature = "bitcoin"),
        "dao" => cfg!(feature = "dao"),
        "web5" => cfg!(feature = "web5"),
        "enterprise" => cfg!(feature = "enterprise"),
        "p2p" => cfg!(feature = "p2p"),
        "machine-learning" => cfg!(feature = "machine-learning"),
        _ => false,
    }
}

// Initialize the library
pub fn initialize() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Log library initialization
    tracing::info!("Anya Core v{} initialized", VERSION);
    tracing::info!("Features: bitcoin={}, dao={}, web5={}, enterprise={}", 
        has_feature("bitcoin"), 
        has_feature("dao"),
        has_feature("web5"),
        has_feature("enterprise")
    );
    
    Ok(())
} 
