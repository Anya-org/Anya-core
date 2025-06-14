// Layer 2 Protocol Factory
// This file contains factory implementations for Layer 2 protocols

use crate::core::error::{AnyaError, AnyaResult};
use crate::layer2::framework::{Layer2Protocol, ProtocolConfig};
use std::collections::HashMap;
use std::sync::RwLock;

/// Factory function type for creating Layer 2 protocols
pub type ProtocolFactoryFn =
    Box<dyn Fn(Box<dyn ProtocolConfig>) -> AnyaResult<Box<dyn Layer2Protocol>> + Send + Sync>;

/// Factory for creating Layer 2 protocols
pub struct Layer2Factory {
    /// Factory functions for creating protocols
    factory_functions: RwLock<HashMap<String, ProtocolFactoryFn>>,
}

impl Layer2Factory {
    /// Create a new Layer 2 factory
    pub fn new() -> Self {
        Self {
            factory_functions: RwLock::new(HashMap::new()),
        }
    }

    /// Register a protocol factory function
    pub fn register_protocol<F>(&self, protocol_type: &str, factory_fn: F)
    where
        F: Fn(Box<dyn ProtocolConfig>) -> AnyaResult<Box<dyn Layer2Protocol>>
            + Send
            + Sync
            + 'static,
    {
        let mut factory_functions = self.factory_functions.write().unwrap();
        factory_functions.insert(protocol_type.to_string(), Box::new(factory_fn));
    }

    /// Create a protocol instance
    pub fn create_protocol(
        &self,
        config: Box<dyn ProtocolConfig>,
    ) -> AnyaResult<Box<dyn Layer2Protocol>> {
        let protocol_type = config.protocol_name();

        let factory_functions = self.factory_functions.read().unwrap();

        if let Some(factory_fn) = factory_functions.get(protocol_type) {
            factory_fn(config)
        } else {
            Err(AnyaError::NotImplemented(format!(
                "Protocol type not supported: {}",
                protocol_type
            )))
        }
    }
}

// Default implementation for a factory that creates no-op protocols
impl Default for Layer2Factory {
    fn default() -> Self {
        Self::new()
    }
}
