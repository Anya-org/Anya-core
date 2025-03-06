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
//! - `core`: Core system components including configuration management
//! - `utils`: Common utilities and helper functions
//!
//! # Features
//!
//! - Advanced ML capabilities with federated learning
//! - Web5 protocol implementation for decentralized data management
//! - Bitcoin and Lightning Network support
//! - DAO governance and voting
//! - Comprehensive security and privacy features
//! - Centralized configuration management with multi-source support
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
use std::path::PathBuf;

pub mod ml;
pub mod web5;
pub mod bitcoin;
pub mod dao;
pub mod extensions;
pub mod config;
pub mod core;
pub mod security;
pub mod enterprise;
pub mod layer2;

// Re-export core components for easy access
pub use core::CoreSystem;
pub use core::config_management::{
    ConfigManager, 
    ConfigValue, 
    ConfigSource, 
    ValidationRule, 
    CONFIG_MANAGER
};

/// Errors that can occur in the Anya system
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
    /// Configuration-related errors
    Config(String),
    /// General system errors
    System(String),
}

impl fmt::Display for AnyaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnyaError::ML(msg) => write!(f, "ML error: {}", msg),
            AnyaError::Web5(msg) => write!(f, "Web5 error: {}", msg),
            AnyaError::Bitcoin(msg) => write!(f, "Bitcoin error: {}", msg),
            AnyaError::DAO(msg) => write!(f, "DAO error: {}", msg),
            AnyaError::Config(msg) => write!(f, "Configuration error: {}", msg),
            AnyaError::System(msg) => write!(f, "System error: {}", msg),
        }
    }
}

impl Error for AnyaError {}

/// Result type for Anya operations
pub type AnyaResult<T> = Result<T, AnyaError>;

/// Configuration for Anya Core
pub struct AnyaConfig {
    /// ML system configuration
    pub ml_config: ml::MLConfig,
    /// Web5 configuration
    pub web5_config: web5::Web5Config,
    /// Bitcoin network configuration
    pub bitcoin_config: bitcoin::BitcoinConfig,
    /// DAO configuration
    pub dao_config: dao::DAOConfig,
    /// Configuration file path (optional)
    pub config_file_path: Option<PathBuf>,
    /// Environment variable prefix
    pub env_prefix: Option<String>,
}

impl Default for AnyaConfig {
    fn default() -> Self {
        Self {
            ml_config: ml::MLConfig::default(),
            web5_config: web5::Web5Config::default(),
            bitcoin_config: bitcoin::BitcoinConfig::default(),
            dao_config: dao::DAOConfig::default(),
            config_file_path: None,
            env_prefix: Some("ANYA_".to_string()),
        }
    }
}

/// Core system for Anya functionality
pub struct AnyaCore {
    /// ML system
    pub ml_system: Option<ml::MLSystem>,
    /// Web5 manager
    pub web5_manager: Option<web5::Web5Manager>,
    /// Bitcoin manager
    pub bitcoin_manager: Option<bitcoin::BitcoinManager>,
    /// DAO manager
    pub dao_manager: Option<dao::DAOManager>,
    /// Core system (includes configuration management)
    pub core_system: core::CoreSystem,
}

impl AnyaCore {
    /// Create a new Anya Core instance with the given configuration
    pub fn new(config: AnyaConfig) -> AnyaResult<Self> {
        // Create the core system with configuration management
        let core_system = core::CoreSystem::new(20);
        
        // Initialize default configuration
        core_system.initialize_default_config().map_err(|e| AnyaError::Config(e))?;
        
        // Load configuration from file if provided
        if let Some(path) = &config.config_file_path {
            core_system.config_manager().load_from_file(path)
                .map_err(|e| AnyaError::Config(format!("Failed to load config file: {}", e)))?;
        }
        
        // Load environment variables if prefix is provided
        if let Some(prefix) = &config.env_prefix {
            core_system.config_manager().load_from_env(prefix)
                .map_err(|e| AnyaError::Config(format!("Failed to load environment variables: {}", e)))?;
        }
        
        // Initialize ML system if enabled
        let ml_system = if config.ml_config.enabled {
            Some(ml::MLSystem::new(&config.ml_config)
                .map_err(|e| AnyaError::ML(format!("Failed to initialize ML system: {}", e)))?)
        } else {
            None
        };
        
        // Initialize Web5 manager if enabled
        let web5_manager = if config.web5_config.enabled {
            Some(web5::Web5Manager::new(&config.web5_config)
                .map_err(|e| AnyaError::Web5(format!("Failed to initialize Web5 manager: {}", e)))?)
        } else {
            None
        };
        
        // Initialize Bitcoin manager if enabled
        let bitcoin_manager = if config.bitcoin_config.enabled {
            Some(bitcoin::BitcoinManager::new(&config.bitcoin_config)
                .map_err(|e| AnyaError::Bitcoin(format!("Failed to initialize Bitcoin manager: {}", e)))?)
        } else {
            None
        };
        
        // Initialize DAO manager if enabled
        let dao_manager = if config.dao_config.enabled {
            Some(dao::DAOManager::new(&config.dao_config)
                .map_err(|e| AnyaError::DAO(format!("Failed to initialize DAO manager: {}", e)))?)
        } else {
            None
        };
        
        Ok(Self {
            ml_system,
            web5_manager,
            bitcoin_manager,
            dao_manager,
            core_system,
        })
    }
    
    /// Create a new Anya Core instance with default configuration
    pub fn default() -> AnyaResult<Self> {
        Self::new(AnyaConfig::default())
    }
    
    /// Check if the system is operational
    pub fn is_operational(&self) -> bool {
        // System is operational if at least one component is enabled
        self.ml_system.is_some() || self.web5_manager.is_some() || 
        self.bitcoin_manager.is_some() || self.dao_manager.is_some()
    }
    
    /// Get the status of the system
    pub fn get_status(&self) -> AnyaResult<SystemStatus> {
        let mut component_status = Vec::new();
        let mut metrics = HashMap::new();
        
        // Add ML component status if available
        if let Some(ml) = &self.ml_system {
            let (operational, health) = ml.get_status();
            component_status.push(ComponentStatus {
                name: "ML".to_string(),
                operational,
                health_score: health,
            });
            
            // Add ML metrics
            let ml_metrics = ml.get_metrics();
            metrics.insert("ml".to_string(), ml_metrics);
        }
        
        // Add Web5 component status if available
        if let Some(web5) = &self.web5_manager {
            let (operational, health) = web5.get_status();
            component_status.push(ComponentStatus {
                name: "Web5".to_string(),
                operational,
                health_score: health,
            });
            
            // Add Web5 metrics
            let web5_metrics = web5.get_metrics();
            metrics.insert("web5".to_string(), web5_metrics);
        }
        
        // Add Bitcoin component status if available
        if let Some(bitcoin) = &self.bitcoin_manager {
            let (operational, health) = bitcoin.get_status();
            component_status.push(ComponentStatus {
                name: "Bitcoin".to_string(),
                operational,
                health_score: health,
            });
            
            // Add Bitcoin metrics
            let bitcoin_metrics = bitcoin.get_metrics();
            metrics.insert("bitcoin".to_string(), bitcoin_metrics);
        }
        
        // Add DAO component status if available
        if let Some(dao) = &self.dao_manager {
            let (operational, health) = dao.get_status();
            component_status.push(ComponentStatus {
                name: "DAO".to_string(),
                operational,
                health_score: health,
            });
            
            // Add DAO metrics
            let dao_metrics = dao.get_metrics();
            metrics.insert("dao".to_string(), dao_metrics);
        }
        
        // Add Core system metrics
        let core_metrics = HashMap::new();
        metrics.insert("core".to_string(), core_metrics);
        
        Ok(SystemStatus {
            ml_enabled: self.ml_system.is_some(),
            web5_enabled: self.web5_manager.is_some(),
            bitcoin_enabled: self.bitcoin_manager.is_some(),
            dao_enabled: self.dao_manager.is_some(),
            component_status,
            metrics,
        })
    }
    
    /// Get the configuration manager
    pub fn config_manager(&self) -> &ConfigManager {
        self.core_system.config_manager()
    }
    
    /// Save current configuration to a file
    pub fn save_config(&self, path: &PathBuf) -> AnyaResult<()> {
        self.config_manager().save_to_file(path)
            .map_err(|e| AnyaError::Config(format!("Failed to save configuration: {}", e)))
    }
}

/// Status of the system and its components
#[derive(Debug)]
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

/// Status of an individual component
#[derive(Debug)]
pub struct ComponentStatus {
    /// Component name
    pub name: String,
    /// Whether the component is operational
    pub operational: bool,
    /// Health score (0.0-1.0)
    pub health_score: f64,
}

/// Utility functions
pub mod utils {
    /// Generate a unique ID
    pub fn generate_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }
    
    /// Log a message
    pub fn log(msg: &str) {
        println!("[ANYA] {}", msg);
    }
}

/// Get the version of Anya Core
pub fn version() -> &'static str {
    VERSION
}

/// Integration utilities
pub mod integration {
    /// Check if Bitcoin integration is enabled
    pub fn bitcoin_enabled() -> bool {
        #[cfg(feature = "bitcoin")]
        return true;
        
        #[cfg(not(feature = "bitcoin"))]
        return false;
    }
}

/// Initialize the Anya Core library
pub fn init() {
    // Initialize the configuration manager
    let _ = &*core::config_management::CONFIG_MANAGER;
    
    // Set up core system
    let core = core::CoreSystem::new(20);
    let _ = core.initialize_default_config();
}

/// Version of the Anya Core library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn test_config_management() {
        use super::*;
        
        // Initialize the library
        init();
        
        // Get the global configuration manager
        let config = &*CONFIG_MANAGER;
        
        // Set a configuration value
        let result = config.set_value(
            "test.key", 
            ConfigValue::String("test value".to_string()), 
            ConfigSource::Default
        );
        assert!(result.is_ok());
        
        // Get the configuration value
        let value = config.get_string("test.key");
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), "test value");
    }
} 