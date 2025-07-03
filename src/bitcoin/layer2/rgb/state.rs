// RGB State implementation
// This file provides state functionality for RGB assets

use std::collections::HashMap;

/// RGB State Transfer
#[derive(Debug, Clone)]
pub struct StateTransfer {
    pub asset_id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub transition_id: String,
}

/// RGB State Validator
#[derive(Debug)]
pub struct StateValidator {
    transfers: HashMap<String, StateTransfer>, // transition_id -> transfer
}

/// RGB State Transition
#[derive(Debug, Clone)]
pub struct StateTransition {
    pub id: String,
    pub asset_id: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<(String, u64)>, // (address, amount)
    pub metadata: HashMap<String, String>,
}

impl StateValidator {
    /// Create a new state validator
    pub fn new() -> Self {
        Self {
            transfers: HashMap::new(),
        }
    }
    
    /// Register a state transfer
    pub fn register_transfer(&mut self, transfer: StateTransfer) -> Result<(), &'static str> {
        self.transfers.insert(transfer.transition_id.clone(), transfer);
        Ok(())
    }
    
    /// Validate a state transfer
    pub fn validate_transfer(&self, transition_id: &str) -> Result<bool, &'static str> {
        if let Some(_transfer) = self.transfers.get(transition_id) {
            // In a real implementation, we would validate the transfer
            // For now, just return true if the transfer exists
            Ok(true)
        } else {
            Err("Transfer not found")
        }
    }
}

impl StateTransfer {
    /// Create a new state transfer
    pub fn new(asset_id: &str, from: &str, to: &str, amount: u64) -> Self {
        // Generate a random transition ID
        let transition_id = format!("transition:{:x}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());
            
        Self {
            asset_id: asset_id.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            amount,
            transition_id,
        }
    }
}

impl StateTransition {
    /// Create a new state transition
    pub fn new(asset_id: &str) -> Self {
        // Generate a random transition ID
        let id = format!("transition:{:x}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());
            
        Self {
            id,
            asset_id: asset_id.to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    
    /// Add input to transition
    pub fn add_input(&mut self, input: &str) {
        self.inputs.push(input.to_string());
    }
    
    /// Add output to transition
    pub fn add_output(&mut self, address: &str, amount: u64) {
        self.outputs.push((address.to_string(), amount));
    }
    
    /// Add metadata to transition
    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }
}
