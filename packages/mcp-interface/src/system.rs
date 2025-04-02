// SystemComponent interfaces [AIR-3][BPC-3]
//
// This module defines the interfaces for the system's component architecture
// following hexagonal design principles from the Bitcoin Development Framework

use std::sync::{Arc, Mutex};

/// Component status information structure
#[derive(Debug, Clone)]
pub struct ComponentStatus {
    /// Component name
    pub name: String,
    /// Health status (starting, running, error, etc.)
    pub health: String,
    /// Component metrics
    pub metrics: Vec<Metric>,
}

/// Metric for component monitoring [AIM-3]
#[derive(Debug, Clone)]
pub struct Metric {
    /// Metric name
    pub name: String,
    /// Metric value
    pub value: i64,
}

impl Metric {
    /// Create a new metric
    pub fn new(name: &str, value: i64) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }
}

/// System index for component discovery [AIR-3]
#[derive(Debug, Default)]
pub struct SystemIndex {
    /// Registered components
    components: Vec<RegisteredComponent>,
}

/// Registered component information
#[derive(Debug)]
struct RegisteredComponent {
    /// Component name
    name: String,
    /// Health status
    health: Arc<Mutex<String>>,
    /// Metrics
    metrics: Vec<(String, i64)>,
}

impl SystemIndex {
    /// Create a new system index
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    
    /// Register a component with the system index
    pub fn register_component(
        &mut self,
        name: &str,
        health: Arc<Mutex<String>>, 
        metrics: Vec<(String, i64)>,
    ) {
        self.components.push(RegisteredComponent {
            name: name.to_string(),
            health,
            metrics,
        });
    }
    
    /// Get a component by name
    pub fn get_component(&self, name: &str) -> Option<Arc<Mutex<String>>> {
        self.components.iter()
            .find(|c| c.name == name)
            .map(|c| c.health.clone())
    }
    
    /// Get implementation details for a specific BIP
    pub fn get_implementation(&self, bip: &str) -> Option<String> {
        // In a real implementation, this would return actual implementation details
        // For now, just return a placeholder value
        match bip {
            "BIP-341" => Some("Taproot implementation v1.2.3".to_string()),
            "BIP-342" => Some("Tapscript implementation v1.1.0".to_string()),
            "BIP-174" => Some("PSBT implementation v2.0.1".to_string()),
            _ => None,
        }
    }
}

/// System component interface [AIR-3]
pub trait SystemComponent {
    /// Register with the system index
    fn register_with_index(&self, index: &mut SystemIndex);
    
    /// Get component status
    fn get_status(&self) -> ComponentStatus;
}
