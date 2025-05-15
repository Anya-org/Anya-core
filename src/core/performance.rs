use std::error::Error;
// Core performance module
// Implements performance metrics for Bitcoin operations
// as per Bitcoin Development Framework v2.5 requirements

use std::collections::HashMap;

/// Performance metrics
#[derive(Debug, Clone)]
pub struct Metrics {
    /// Transactions per second
    pub transactions_per_second: f64,
    /// Average block time in seconds
    pub block_time: f64,
    /// Number of active validators
    pub active_validators: u32,
    /// Network usage metrics
    pub network_usage: HashMap<String, f64>,
}

impl Metrics {
    /// Create new metrics
    pub fn new() -> Self {
        Self {
            transactions_per_second: 0.0,
            block_time: 0.0,
            active_validators: 0,
            network_usage: HashMap::new(),
        }
    }
    
    /// Add a network usage metric
    pub fn add_network_usage(&mut self, key: &str, value: f64)  -> Result<(), Box<dyn Error>> {
        self.network_usage.insert(key.to_string(), value);
    }
} 
