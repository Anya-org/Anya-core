// Layer 2 Framework for Bitcoin implementations
// 
// Modular framework for implementing future Layer 2 solutions
// This module follows hexagonal architecture principles

pub mod adapters;
pub mod config;
pub mod factory;

use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::fmt;
use async_trait::async_trait;
use crate::core::error::{AnyaResult, AnyaError};

pub use crate::layer2::types::{
    ProtocolState, AssetParams, AssetTransfer, TransferResult, Proof, VerificationResult, ValidationResult
};

/// Transaction status for Layer 2 protocols
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    /// Transaction is pending
    Pending,
    /// Transaction is confirmed
    Confirmed,
    /// Transaction failed
    Failed,
    /// Transaction is in an unknown state
    Unknown,
}

/// Protocol configuration trait
pub trait ProtocolConfig: Send + Sync + std::fmt::Debug {
    /// Get protocol name
    fn protocol_name(&self) -> &str;
    
    /// Get network type
    fn network_type(&self) -> &str;
    
    /// Clone configuration
    fn clone_box(&self) -> Box<dyn ProtocolConfig>;
}

/// Layer 2 protocol interface
#[async_trait]
pub trait Layer2Protocol: Send + Sync + fmt::Debug {
    /// Get the protocol name
    fn name(&self) -> &str;
    
    /// Get the protocol version
    fn version(&self) -> &str;
    
    /// Initialize the protocol
    async fn init(&self) -> AnyaResult<()>;
    
    /// Start the protocol
    async fn start(&self) -> AnyaResult<()>;
    
    /// Stop the protocol
    async fn stop(&self) -> AnyaResult<()>;
    
    /// Check if the protocol is running
    async fn is_running(&self) -> bool;
    
    /// Execute a protocol-specific command
    async fn execute_command(&self, command: &str, args: &[&str]) -> AnyaResult<String>;
}

// Re-export Layer2Factory from factory module
pub use factory::Layer2Factory;

/// Registry for Layer 2 protocols
pub struct Layer2Registry {
    factory: Arc<Layer2Factory>,
    protocols: RwLock<HashMap<String, Arc<dyn Layer2Protocol>>>,
}

impl Layer2Registry {
    /// Create a new Layer 2 registry
    pub fn new(factory: Arc<Layer2Factory>) -> Self {
        Self {
            factory,
            protocols: RwLock::new(HashMap::new()),
        }
    }
    
    /// Register a protocol
    pub fn register(&self, protocol_type: &str) -> AnyaResult<Arc<dyn Layer2Protocol>> {
        let protocol = self.factory.create_protocol(protocol_type)?;
        let mut protocols = self.protocols.write().unwrap();
        protocols.insert(protocol_type.to_string(), protocol.clone());
        Ok(protocol)
    }
    
    /// Get a protocol
    pub fn get_protocol(&self, protocol_type: &str) -> AnyaResult<Arc<dyn Layer2Protocol>> {
        // Check if protocol is already registered
        {
            let protocols = self.protocols.read().unwrap();
            if let Some(protocol) = protocols.get(protocol_type) {
                return Ok(protocol.clone());
            }
        }
        
        // If not, register it
        self.register(protocol_type)
    }
    
    /// Get all registered protocols
    pub fn get_all_protocols(&self) -> Vec<Arc<dyn Layer2Protocol>> {
        let protocols = self.protocols.read().unwrap();
        protocols.values().cloned().collect()
    }
}

/// No-op Layer 2 protocol implementation for testing
#[derive(Debug)]
pub struct NoopLayer2Protocol {
    name: String,
    version: String,
    running: bool,
}

impl NoopLayer2Protocol {
    /// Create a new no-op Layer 2 protocol
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            running: false,
        }
    }
}

#[async_trait]
impl Layer2Protocol for NoopLayer2Protocol {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    async fn init(&self) -> AnyaResult<()> {
        Ok(())
    }
    
    async fn start(&self) -> AnyaResult<()> {
        // In a real implementation, this would actually start the protocol
        Ok(())
    }
    
    async fn stop(&self) -> AnyaResult<()> {
        // In a real implementation, this would actually stop the protocol
        Ok(())
    }
    
    async fn is_running(&self) -> bool {
        self.running
    }
    
    async fn execute_command(&self, command: &str, _args: &[&str]) -> AnyaResult<String> {
        Ok(format!("Executed command '{}' on protocol '{}'", command, self.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(Debug, Clone)]
    struct TestConfig {
        name: String,
        network: String,
    }
    
    impl ProtocolConfig for TestConfig {
        fn protocol_name(&self) -> &str {
            &self.name
        }
        
        fn network_type(&self) -> &str {
            &self.network
        }
        
        fn clone_box(&self) -> Box<dyn ProtocolConfig> {
            Box::new(self.clone())
        }
    }
    
    #[derive(Debug)]
    struct TestProtocol {
        name: String,
        version: String,
        running: bool,
    }
    
    #[async_trait]
    impl Layer2Protocol for TestProtocol {
        fn name(&self) -> &str {
            &self.name
        }
        
        fn version(&self) -> &str {
            &self.version
        }
        
        async fn init(&self) -> AnyaResult<()> {
            Ok(())
        }
        
        async fn start(&self) -> AnyaResult<()> {
            Ok(())
        }
        
        async fn stop(&self) -> AnyaResult<()> {
            Ok(())
        }
        
        async fn is_running(&self) -> bool {
            self.running
        }
        
        async fn execute_command(&self, command: &str, _args: &[&str]) -> AnyaResult<String> {
            Ok(format!("Test protocol executed: {}", command))
        }
    }
    
    #[test]
    fn test_layer2_factory() {
        let factory = factory::Layer2Factory::new();
        
        factory.register_protocol("test", |config| {
            let test_config = config.protocol_name();
            match test_config {
                "test" => Ok(Box::new(TestProtocol {
                    name: "test".to_string(),
                    version: "0.1.0".to_string(),
                    running: false,
                })),
                _ => Err(AnyaError::NotImplemented("Invalid protocol".to_string())),
            }
        });
        
        let config = Box::new(TestConfig {
            name: "test".to_string(),
            network: "testnet".to_string(),
        });
        
        let protocol = factory.create_protocol(config).unwrap();
        
        // We'll just check that we got a protocol with the right name
        assert_eq!(protocol.name(), "test");
    }
}
