/// AIM-004: Framework for Layer 2 Solutions
/// 
/// Modular framework for implementing future Layer 2 solutions
/// This module follows hexagonal architecture principles
///
/// Related to: AIR-342, AIT-367

mod adapters;
mod config;
mod factory;

use super::ports::{Layer2Protocol, TransactionStatus};
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Protocol configuration trait
pub trait ProtocolConfig: Send + Sync + std::fmt::Debug {
    /// Get protocol name
    fn protocol_name(&self) -> &str;
    
    /// Get network type
    fn network_type(&self) -> &str;
    
    /// Clone configuration
    fn clone_box(&self) -> Box<dyn ProtocolConfig>;
}

/// Layer 2 protocol factory
pub struct Layer2Factory {
    /// Registered protocol factories
    factories: HashMap<String, Box<dyn Fn(Box<dyn ProtocolConfig>) -> Result<Box<dyn Layer2Protocol>>>>,
}

impl Layer2Factory {
    /// Create new Layer 2 factory
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }
    
    /// Register protocol factory
    pub fn register_protocol<F>(&mut self, protocol_name: &str, factory: F)
    where
        F: Fn(Box<dyn ProtocolConfig>) -> Result<Box<dyn Layer2Protocol>> + 'static,
    {
        self.factories.insert(protocol_name.to_string(), Box::new(factory));
    }
    
    /// Create protocol instance
    pub fn create_protocol(&self, config: Box<dyn ProtocolConfig>) -> Result<Box<dyn Layer2Protocol>> {
        let protocol_name = config.protocol_name().to_string();
        
        match self.factories.get(&protocol_name) {
            Some(factory) => factory(config),
            None => Err(anyhow!("Unsupported protocol: {}", protocol_name)),
        }
    }
}

/// Registry for Layer 2 protocols
pub struct Layer2Registry {
    /// Registered protocols
    protocols: RwLock<HashMap<String, Arc<Box<dyn Layer2Protocol>>>>,
    /// Protocol factory
    factory: Arc<Layer2Factory>,
}

impl Layer2Registry {
    /// Create new Layer 2 registry
    pub fn new(factory: Arc<Layer2Factory>) -> Self {
        Self {
            protocols: RwLock::new(HashMap::new()),
            factory,
        }
    }
    
    /// Register protocol instance
    pub fn register_protocol(&self, name: &str, protocol: Box<dyn Layer2Protocol>) -> Result<()> {
        let mut protocols = self.protocols.write().unwrap();
        protocols.insert(name.to_string(), Arc::new(protocol));
        Ok(())
    }
    
    /// Create and register protocol instance
    pub fn create_and_register(&self, name: &str, config: Box<dyn ProtocolConfig>) -> Result<()> {
        let protocol = self.factory.create_protocol(config)?;
        self.register_protocol(name, protocol)
    }
    
    /// Get protocol instance
    pub fn get_protocol(&self, name: &str) -> Option<Arc<Box<dyn Layer2Protocol>>> {
        let protocols = self.protocols.read().unwrap();
        protocols.get(name).cloned()
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
    
    struct TestProtocol {
        config: TestConfig,
    }
    
    impl Layer2Protocol for TestProtocol {
        fn initialize(&self) -> Result<()> {
            Ok(())
        }
        
        fn connect(&self) -> Result<()> {
            Ok(())
        }
        
        fn submit_transaction(&self, _transaction: &[u8]) -> Result<String> {
            Ok("test_tx".to_string())
        }
        
        fn get_transaction_status(&self, _tx_id: &str) -> Result<TransactionStatus> {
            Ok(TransactionStatus::Confirmed)
        }
    }
    
    #[test]
    fn test_layer2_factory() {
        let mut factory = Layer2Factory::new();
        
        factory.register_protocol("test", |config| {
            let test_config = match config.protocol_name() {
                "test" => TestConfig {
                    name: "test".to_string(),
                    network: config.network_type().to_string(),
                },
                _ => return Err(anyhow!("Invalid protocol")),
            };
            
            Ok(Box::new(TestProtocol { config: test_config }))
        });
        
        let config = Box::new(TestConfig {
            name: "test".to_string(),
            network: "testnet".to_string(),
        });
        
        let protocol = factory.create_protocol(config).unwrap();
        assert!(protocol.initialize().is_ok());
    }
} 