// Web5 Adapter Implementation
// Implements adapter pattern for Web5 functionality in the Anya Core system

use std::sync::Arc;
// [AIR-3][AIS-3][BPC-3][RES-3] Removed unused imports: Web5Error, Web5Result
// [AIR-3][AIS-3][BPC-3][RES-3] Removed unused Web5Error and Web5Result imports
use crate::web5::{Web5Config, Web5Manager};
use crate::{AnyaResult, AnyaError};

/// Web5 adapter for the Anya Core system
pub struct Web5Adapter {
    /// Internal Web5 manager instance
    manager: Arc<Web5Manager>,
}

impl Web5Adapter {
    /// Build a new Web5Adapter with the provided configuration
    pub async fn build(config: Web5Config) -> AnyaResult<Self> {
        // Create a new Web5Manager
        let manager = Web5Manager::new(config)
            .map_err(|e| AnyaError::Web5(format!("Failed to create Web5Manager: {}", e)))?;
        
        // Initialize the manager
        let mut manager_instance = manager;
        manager_instance.initialize()
            .map_err(|e| AnyaError::Web5(format!("Failed to initialize Web5Manager: {}", e)))?;
            
        Ok(Self {
            manager: Arc::new(manager_instance),
        })
    }
    
    /// Get the internal Web5Manager
    pub fn manager(&self) -> Arc<Web5Manager> {
        self.manager.clone()
    }
    
    /// Create a new DID
    pub async fn create_did(&self) -> AnyaResult<String> {
        let did = self.manager.did_manager().create_did()
            .map_err(|e| AnyaError::Web5(format!("Failed to create DID: {}", e)))?;
        
        Ok(did.id)
    }
    
    /// Get the system status
    pub async fn status(&self) -> AnyaResult<bool> {
        let status = self.manager.status()
            .map_err(|e| AnyaError::Web5(format!("Failed to get Web5 status: {}", e)))?;
            
        Ok(status.enabled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_web5_adapter_build() -> AnyaResult<()> {
        let config = Web5Config::default();
        let adapter = Web5Adapter::build(config).await?;
        
        let status = adapter.status().await?;
        assert!(status);
        
        Ok(())
    }
}
