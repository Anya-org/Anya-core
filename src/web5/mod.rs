//! Web5 Implementation Core [AIR-3][AIS-3][BPC-3][RES-3]

// Re-export modules
pub mod dwn; // Decentralized Web Node
pub mod identity;
pub mod protocols;
pub mod vc; // Verifiable Credentials

// Re-export important types for easy access
// Legacy Web5Adapter removed. Use the canonical HTTP client adapter from src/web/web5_adapter.rs
pub use identity::{DIDDocument, DIDManager, IdentityManager, Web5Error, Web5Result, DID};
pub use protocols::{ProtocolDefinition, ProtocolHandler, ProtocolManager};

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
    pub fn did_manager(&self) -> &identity::DIDManager {
        &self.did_manager
    }

    /// Access the protocol manager component
    pub fn protocol_manager(&self) -> &protocols::ProtocolManager {
        &self.protocol_manager
    }

    /// Initialize the Web5 subsystem with default protocols
    pub fn initialize(&mut self) -> Web5Result<()> {
        // Register standard protocols
        let profile_handler = protocols::ProfileProtocolHandler::new();
        self.protocol_manager
            .register_protocol(Box::new(profile_handler))?;

        // Create default identity if none exists
        if self.config.use_local_storage && self.did_manager.get_default_did()?.is_none() {
            let did = self.did_manager.create_did()?;
            self.did_manager.set_default_did(&did.id)?
        }

        Ok(())
    }

    /// Get the system status
    pub fn status(&self) -> Web5Result<Web5Status> {
        let did_count = self.did_manager.dids()?.len();
        let protocol_count = self.protocol_manager.get_all_protocols().len();

        Ok(Web5Status {
            enabled: self.config.enabled,
            did_count,
            protocol_count,
            dwn_connected: self.config.dwn_url.is_some(),
        })
    }

    /// Get metrics for the Web5 system
    pub fn get_metrics(&self) -> Web5Result<HashMap<String, String>> {
        let mut metrics = HashMap::new();
        metrics.insert(
            "dids".to_string(),
            self.did_manager.dids()?.len().to_string(),
        );
        metrics.insert(
            "protocols".to_string(),
            self.protocol_manager.get_all_protocols().len().to_string(),
        );
        metrics.insert(
            "dwn_connected".to_string(),
            self.config.dwn_url.is_some().to_string(),
        );

        Ok(metrics)
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
    // [AIR-3][AIS-3][BPC-3][RES-3] Proper error handling organization
    use super::*;

    #[test]
    fn test_web5_manager_creation() -> Result<(), Box<dyn std::error::Error>> {
        let config = Web5Config::default();
        let manager = Web5Manager::new(config)?;

        assert!(manager.config.enabled);
        assert_eq!(manager.config.did_method, "ion");
        Ok(())
    }

    #[test]
    fn test_web5_status() -> Result<(), Box<dyn std::error::Error>> {
        let config = Web5Config::default();
        let manager = Web5Manager::new(config)?;

        let status = manager.status()?;
        assert!(status.enabled);
        assert_eq!(status.did_count, 0);
        assert_eq!(status.protocol_count, 0);
        assert!(!status.dwn_connected);
        Ok(())
    }
}
