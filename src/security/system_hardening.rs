use std::error::Error;
// AIE-001: System Hardening Implementation
// Priority: HIGH - Security configurations with in-memory state

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// System hardening configuration status
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigStatus {
    NotApplied,
    Pending,
    Applied,
    Failed,
}

/// Security policy levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecurityLevel {
    Basic,
    Enhanced,
    Strict,
    Custom,
}

/// Hardening configuration for a component
#[derive(Debug, Clone)]
pub struct HardeningConfig {
    name: String,
    status: ConfigStatus,
    level: SecurityLevel,
    settings: HashMap<String, String>,
    last_modified: Instant,
    auto_save_enabled: bool,
}

/// System hardening manager
pub struct SystemHardening {
    configs: Arc<Mutex<HashMap<String, HardeningConfig>>>,
    input_counter: Arc<Mutex<usize>>,
    auto_save_frequency: usize,
}

impl SystemHardening {
    /// Create a new system hardening manager
    pub fn new(auto_save_frequency: usize) -> Self {
        Self {
            configs: Arc::new(Mutex::new(HashMap::new())),
            input_counter: Arc::new(Mutex::new(0)),
            auto_save_frequency,
        }
    }
    
    /// Add or update a component configuration
    pub fn configure_component(&self, 
                              name: &str, 
                              level: SecurityLevel, 
                              settings: HashMap<String, String>,
                              auto_save: bool) -> Result<(), String> {
        let mut configs = self.configs.lock().map_err(|e| format!("Mutex lock error: {}", e))?;
        
        let config = HardeningConfig {
            name: name.to_string(),
            status: ConfigStatus::NotApplied,
            level,
            settings,
            last_modified: Instant::now(),
            auto_save_enabled: auto_save,
        };
        
        configs.insert(name.to_string(), config);
        
        // Update input counter and check for auto-save
        self.record_input_and_check_save();
        
        Ok(())
    }
    
    /// Record an input and check if auto-save is needed
    fn record_input_and_check_save(&self) -> Result<(), String> {
        let mut counter = self.input_counter.lock().map_err(|e| format!("Mutex lock error: {}", e))?;
        *counter += 1;
        
        // Auto-save every Nth input (e.g., every 20th input)
        if *counter % self.auto_save_frequency == 0 {
            match self.save_state_to_memory() {
                Ok(_) => println!("Auto-saved security configuration after {} changes", *counter),
                Err(e) => eprintln!("Failed to auto-save security configuration: {}", e),
            }
        }
        
        Ok(())
    }
    
    /// Save the current state to memory (no file writing)
    fn save_state_to_memory(&self) -> Result<(), String> {
        // In a real implementation, this would create a backup of security configurations
        // For this implementation, we're just keeping everything in memory
        let configs = self.configs.lock().map_err(|e| format!("Mutex lock error: {}", e))?;
        println!("In-memory security configuration snapshot created: {} components", configs.len());
        
        // Here you would normally serialize the state and store it
        Ok(())
    }
    
    /// Apply security hardening configuration for a component
    pub fn apply_hardening(&self, component_name: &str) -> Result<ConfigStatus, String> {
        let mut configs = self.configs.lock().map_err(|e| format!("Mutex lock error: {}", e))?;
        
        let config = match configs.get_mut(component_name) {
            Some(config) => config,
            None => return Err(format!("No configuration found for component {}", component_name)),
        };
        
        // For demonstration purposes, we're just simulating the application
        // In a real implementation, this would apply actual security settings
        println!("Applying security configuration for {}: {:?}", component_name, config.level);
        
        // Update status
        config.status = ConfigStatus::Applied;
        config.last_modified = Instant::now();
        
        // Record this input and potentially auto-save
        self.record_input_and_check_save();
        
        Ok(config.status.clone())
    }
    
    /// Set a specific security setting
    pub fn set_security_setting(&self, component_name: &str, key: &str, value: &str) -> Result<(), String> {
        let mut configs = self.configs.lock().map_err(|e| format!("Mutex lock error: {}", e))?;
        
        let config = match configs.get_mut(component_name) {
            Some(config) => config,
            None => return Err(format!("No configuration found for component {}", component_name)),
        };
        
        // Update the setting
        config.settings.insert(key.to_string(), value.to_string());
        config.status = ConfigStatus::Pending;  // Changed but not applied
        config.last_modified = Instant::now();
        
        // Auto-save if needed
        self.record_input_and_check_save();
        
        Ok(())
    }
    
    /// Get the configuration for a component
    pub fn get_component_config(&self, component_name: &str) -> Option<HardeningConfig> {
        let configs = self.configs.lock().map_err(|e| format!("Mutex lock error: {}", e))?;
        configs.get(component_name).cloned()
    }
    
    /// Get all component configurations
    pub fn get_all_configs(&self) -> Vec<HardeningConfig> {
        let configs = self.configs.lock().map_err(|e| format!("Mutex lock error: {}", e))?;
        configs.values().cloned().collect()
    }
    
    /// Get number of changes and configs
    pub fn get_stats(&self) -> (usize, usize) {
        let counter = self.input_counter.lock().map_err(|e| format!("Mutex lock error: {}", e))?;
        let configs = self.configs.lock().map_err(|e| format!("Mutex lock error: {}", e))?;
        
        (*counter, configs.len())
    }
    
    /// Apply all pending configurations
    pub fn apply_all_pending(&self) -> Vec<(String, Result<ConfigStatus, String>)> {
        let pending_components = match self.configs.lock() {
            Ok(configs) => {
                let components: Vec<String> = configs
                    .iter()
                    .filter(|(_, config)| config.status == ConfigStatus::Pending)
                    .map(|(name, _)| name.clone())
                    .collect();
                drop(configs); // Release the lock
                components
            },
            Err(e) => {
                // Return an empty vec with the error if we can't get the lock
                let error_msg = format!("Mutex lock error: {}", e);
                let mut results = Vec::new();
                results.push(("general".to_string(), Err(error_msg)));
                return results;
            }
        };
        
        // Apply each pending config
        let mut results = Vec::new();
        for component_name in pending_components {
            results.push((component_name.clone(), self.apply_hardening(&component_name)));
        }
        
        results
    }
}

// Tests for the SystemHardening
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_configuration_and_auto_save() {
        let hardening = SystemHardening::new(20); // Auto-save every 20th change
        
        // Create 25 configurations to trigger auto-save
        for i in 0..25 {
            let mut settings = HashMap::new();
            settings.insert("firewall".to_string(), "enabled".to_string());
            settings.insert("port_scanning".to_string(), "block".to_string());
            
            hardening.configure_component(
                &format!("component_{}", i),
                SecurityLevel::Enhanced,
                settings,
                true
            )?;
        }
        
        // Check stats
        let (changes, configs) = hardening.get_stats();
        assert_eq!(changes, 25);
        assert_eq!(configs, 25);
    }
    
    #[test]
    fn test_apply_hardening() {
        let hardening = SystemHardening::new(10);
        
        // Create a configuration
        let mut settings = HashMap::new();
        settings.insert("firewall".to_string(), "enabled".to_string());
        hardening.configure_component("network", SecurityLevel::Strict, settings, true)?;
        
        // Apply the hardening
        let result = hardening.apply_hardening("network");
        assert!(result.is_ok());
        assert_eq!(result?, ConfigStatus::Applied);
        
        // Verify the status
        let config = hardening.get_component_config("network")?;
        assert_eq!(config.status, ConfigStatus::Applied);
    }
} 
