// Web5 Module for Anya Core
// Provides Web5 functionality including DIDs, protocols, and web components

// Re-export modules
pub mod identity;
pub mod protocols;
pub mod dwn;  // Decentralized Web Node
pub mod vc;   // Verifiable Credentials

// Re-export important types for easy access
pub use identity::{Web5Error, Web5Result, DIDManager, DID, DIDDocument, VerificationMethod};
pub use protocols::{ProtocolHandler, ProtocolManager, ProtocolDefinition};

/// Web5 configuration
pub struct Web5Config {
    /// Whether Web5 functionality is enabled
    pub enabled: bool,
    /// Default DID method to use
    pub did_method: String,
    /// DWN endpoint URL
    pub dwn_url: Option<String>,
    /// Whether to automatically sync DIDs
    pub auto_sync: bool,
}

impl Default for Web5Config {
    fn default() -> Self {
        Self {
            enabled: true,
            did_method: "ion".to_string(),
            dwn_url: None,
            auto_sync: true,
        }
    }
}

/// Web5 manager that coordinates all Web5 functionality
pub struct Web5Manager {
    /// Configuration
    config: Web5Config,
    /// DID manager
    did_manager: identity::DIDManager,
    /// Protocol manager
    protocol_manager: protocols::ProtocolManager,
}

impl Web5Manager {
    /// Create a new Web5 manager
    pub fn new(config: &Web5Config) -> Web5Result<Self> {
        let did_manager = identity::DIDManager::new(&config.did_method);
        let protocol_manager = protocols::ProtocolManager::new();
        
        Ok(Self {
            config: config.clone(),
            did_manager,
            protocol_manager,
        })
    }
    
    /// Get the DID manager
    pub fn did_manager(&self) -> &identity::DIDManager {
        &self.did_manager
    }
    
    /// Get the protocol manager
    pub fn protocol_manager(&self) -> &protocols::ProtocolManager {
        &self.protocol_manager
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
