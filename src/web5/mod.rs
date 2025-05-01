use std::error::Error;
// Web5 Module for Anya Core
// Implements Web5 specification with focus on decentralized identity and protocols
// [AIR-012] Operational Reliability and [AIP-002] Modular Architecture

// Re-export modules
pub mod identity;
pub mod protocols;
pub mod dwn;  // Decentralized Web Node
pub mod vc;   // Verifiable Credentials

// Re-export important types for easy access
pub use identity::{Web5Error, Web5Result, DIDManager, DID, DIDDocument};
pub use protocols::{ProtocolHandler, ProtocolManager, ProtocolDefinition};

use std::collections::HashMap;

/// Web5 configuration with focused parameters
#[derive(Clone, Debug)]
pub struct Web5Config {
    /// Whether Web5 functionality is enabled
    pub enabled: bool,
    /// Default DID method to use (e.g., "ion", "key", "web")
    pub did_method: String,
    /// DWN endpoint URL (if applicable)
    pub dwn_url: Option<String>,
    /// Whether to use local storage for DIDs
    pub use_local_storage: bool,
}

impl Default for Web5Config {
    fn default() -> Self {
        Self {
            enabled: true,
            did_method: "ion".to_string(),
            dwn_url: None,
            use_local_storage: true,
        }
    }
}

/// Web5Manager: Lightweight coordinator for Web5 functionality following hexagonal architecture
/// Implements ports and adapters pattern for clean interfaces
pub struct Web5Manager {
    /// Configuration
    config: Web5Config,
    /// DID manager - Core identity functionality
    did_manager: identity::DIDManager,
    /// Protocol manager - Core protocol functionality 
    protocol_manager: protocols::ProtocolManager,
}

impl Web5Manager {
    /// Create a new Web5 manager with the specified configuration
    pub fn new(config: Web5Config) -> Web5Result<Self> {
        let did_manager = identity::DIDManager::new(&config.did_method);
        let protocol_manager = protocols::ProtocolManager::new();
        
        Ok(Self {
            config,
            did_manager,
            protocol_manager,
        })
    }
    
    /// Access the DID manager component
    pub fn did_manager(&self) -> &identity::DIDManager  -> Result<(), Box<dyn Error>> {
        &self.did_manager
    }
    
    /// Access the protocol manager component
    pub fn protocol_manager(&self) -> &protocols::ProtocolManager  -> Result<(), Box<dyn Error>> {
        &self.protocol_manager
    }
    
    /// Initialize the Web5 subsystem with default protocols
    pub fn initialize(&mut self) -> Web5Result<()>  -> Result<(), Box<dyn Error>> {
        // Register standard protocols
        let profile_handler = protocols::ProfileProtocolHandler::new();
        self.protocol_manager.register_protocol(Box::new(profile_handler))?;
        
        // Create default identity if none exists
        if self.config.use_local_storage && self.did_manager.get_default_did()?.is_none() {
            let did = self.did_manager.create_did()?;
            self.did_manager.set_default_did(&did.id)?;
        }
        
        Ok(())
    }
    
    /// Get the system status
    pub fn status(&self) -> Web5Result<Web5Status>  -> Result<(), Box<dyn Error>> {
        let did_count = self.did_manager.dids().len();
        let protocol_count = self.protocol_manager.get_all_protocols().len();
        
        Ok(Web5Status {
            enabled: self.config.enabled,
            did_count,
            protocol_count,
            dwn_connected: self.config.dwn_url.is_some(),
        })
    }

    /// Get metrics for the Web5 system
    pub fn get_metrics(&self) -> HashMap<String, serde_json::Value>  -> Result<(), Box<dyn Error>> {
        let mut metrics = HashMap::new();
        
        // Add basic metrics
        metrics.insert("did_count".to_string(), serde_json::json!(self.did_manager.dids().len()));
        metrics.insert("protocol_count".to_string(), serde_json::json!(self.protocol_manager.get_all_protocols().len()));
        metrics.insert("dwn_connected".to_string(), serde_json::json!(self.config.dwn_url.is_some()));
        
        metrics
    }
}

/// Web5 system status information
#[derive(Clone, Debug)]
pub struct Web5Status {
    /// Whether Web5 is enabled
    pub enabled: bool,
    /// Number of DIDs managed
    pub did_count: usize,
    /// Number of protocols registered
    pub protocol_count: usize,
    /// Whether connected to a DWN
    pub dwn_connected: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_web5_manager_creation()  -> Result<(), Box<dyn Error>> {
        let config = Web5Config::default();
        let manager = Web5Manager::new(config)?;
        
        assert!(manager.config.enabled);
        assert_eq!(manager.config.did_method, "ion");
    }
    
    #[test]
    fn test_web5_status()  -> Result<(), Box<dyn Error>> {
        let config = Web5Config::default();
        let manager = Web5Manager::new(config)?;
        
        let status = manager.status()?;
        assert!(status.enabled);
        assert_eq!(status.did_count, 0);
        assert_eq!(status.protocol_count, 0);
        assert!(!status.dwn_connected);
    }
}

