// AIR-008: Core Module Integration
// Integrates all Priority 1 implementations with auto-save functionality

// Performance optimization module
pub mod performance_optimization;
// Configuration management module
pub mod config_management;

// Re-exports
pub use performance_optimization::PerformanceOptimizer;
pub use performance_optimization::ResourceType;
pub use performance_optimization::OptimizationStatus;

// Export configuration management
pub use config_management::ConfigManager;
pub use config_management::ConfigValue;
pub use config_management::ConfigSource;
pub use config_management::ValidationRule;
pub use config_management::CONFIG_MANAGER;

// ML agent checker module is in src/ml/agent_checker.rs
// Re-export from ml module
pub use crate::ml::agent_checker::AgentChecker;
pub use crate::ml::agent_checker::SystemStage;

// System hardening module is in src/security/system_hardening.rs
// Re-export from security module  
pub use crate::security::system_hardening::SystemHardening;
pub use crate::security::system_hardening::SecurityLevel;
pub use crate::security::system_hardening::ConfigStatus;

/// Core functionality with auto-save capabilities
pub struct CoreSystem {
    // Component managers with auto-save functionality
    agent_checker: AgentChecker,
    system_hardening: SystemHardening, 
    performance_optimizer: PerformanceOptimizer,
    // Configuration manager (global static instance)
    config_manager: &'static ConfigManager,
}

impl CoreSystem {
    /// Create a new core system with specified auto-save frequency for each component
    pub fn new(auto_save_frequency: usize) -> Self {
        Self {
            agent_checker: AgentChecker::new(auto_save_frequency),
            system_hardening: SystemHardening::new(auto_save_frequency),
            performance_optimizer: PerformanceOptimizer::new(auto_save_frequency),
            config_manager: &CONFIG_MANAGER,
        }
    }
    
    /// Get access to the agent checker
    pub fn agent_checker(&self) -> &AgentChecker {
        &self.agent_checker
    }
    
    /// Get access to the system hardening manager
    pub fn system_hardening(&self) -> &SystemHardening {
        &self.system_hardening
    }
    
    /// Get access to the performance optimizer
    pub fn performance_optimizer(&self) -> &PerformanceOptimizer {
        &self.performance_optimizer
    }
    
    /// Get access to the configuration manager
    pub fn config_manager(&self) -> &ConfigManager {
        self.config_manager
    }
    
    /// Process input across all components
    pub fn process_input(&self, input: &str) -> Result<(), String> {
        // Process with agent checker
        self.agent_checker.process_input(input)
            .map_err(|e| format!("Agent checker error: {}", e))?;
            
        // Process with system hardening
        self.system_hardening.process_input(input)
            .map_err(|e| format!("System hardening error: {}", e))?;
            
        // Process with performance optimizer
        self.performance_optimizer.process_input(input)
            .map_err(|e| format!("Performance optimizer error: {}", e))?;
            
        Ok(())
    }
    
    /// Get auto-save statistics for all components
    /// Returns (AgentChecker, SystemHardening, PerformanceOptimizer) auto-save counts
    pub fn get_auto_save_stats(&self) -> (usize, usize, usize) {
        (
            self.agent_checker.get_auto_save_count(),
            self.system_hardening.get_auto_save_count(),
            self.performance_optimizer.get_auto_save_count(),
        )
    }
    
    /// Initialize configuration with default values
    pub fn initialize_default_config(&self) -> Result<(), String> {
        // Set up default configuration values
        let config = self.config_manager();
        
        // System-wide configuration
        config.set_value(
            "system.auto_save_frequency", 
            ConfigValue::Integer(20), 
            ConfigSource::Default
        ).map_err(|e| format!("Config error: {}", e))?;
        
        config.set_value(
            "system.log_level", 
            ConfigValue::String("info".to_string()), 
            ConfigSource::Default
        ).map_err(|e| format!("Config error: {}", e))?;
        
        // Security configuration
        config.set_value(
            "security.default_level", 
            ConfigValue::String("Enhanced".to_string()), 
            ConfigSource::Default
        ).map_err(|e| format!("Config error: {}", e))?;
        
        // Performance configuration
        config.set_value(
            "performance.cpu_target", 
            ConfigValue::Integer(80), 
            ConfigSource::Default
        ).map_err(|e| format!("Config error: {}", e))?;
        
        config.set_value(
            "performance.memory_target", 
            ConfigValue::Integer(70), 
            ConfigSource::Default
        ).map_err(|e| format!("Config error: {}", e))?;
        
        // ML configuration
        config.set_value(
            "ml.default_stage", 
            ConfigValue::String("Development".to_string()), 
            ConfigSource::Default
        ).map_err(|e| format!("Config error: {}", e))?;
        
        // Add validation rules
        config.add_validation_rule("system.auto_save_frequency", ValidationRule::MinValue(1.0));
        config.add_validation_rule("system.auto_save_frequency", ValidationRule::MaxValue(100.0));
        
        config.add_validation_rule("performance.cpu_target", ValidationRule::MinValue(0.0));
        config.add_validation_rule("performance.cpu_target", ValidationRule::MaxValue(100.0));
        
        config.add_validation_rule("performance.memory_target", ValidationRule::MinValue(0.0));
        config.add_validation_rule("performance.memory_target", ValidationRule::MaxValue(100.0));
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_core_system_integration() {
        let core = CoreSystem::new(20);
        
        // Test agent checker access
        assert_eq!(core.agent_checker().get_stage(), SystemStage::Development);
        
        // Test system hardening access
        assert_eq!(core.system_hardening().get_security_level(), SecurityLevel::Basic);
        
        // Test performance optimizer access
        assert_eq!(core.performance_optimizer().get_status(), OptimizationStatus::Inactive);
        
        // Test configuration management
        assert!(core.config_manager().set_value(
            "test.key", 
            ConfigValue::String("test value".to_string()), 
            ConfigSource::Default
        ).is_ok());
        
        assert_eq!(
            core.config_manager().get_string("test.key").unwrap(),
            "test value"
        );
    }
    
    #[test]
    fn test_default_configuration() {
        let core = CoreSystem::new(20);
        core.initialize_default_config().unwrap();
        
        let config = core.config_manager();
        
        // Test default values
        assert_eq!(config.get_integer("system.auto_save_frequency").unwrap(), 20);
        assert_eq!(config.get_string("system.log_level").unwrap(), "info");
        assert_eq!(config.get_string("security.default_level").unwrap(), "Enhanced");
        assert_eq!(config.get_integer("performance.cpu_target").unwrap(), 80);
        assert_eq!(config.get_integer("performance.memory_target").unwrap(), 70);
        assert_eq!(config.get_string("ml.default_stage").unwrap(), "Development");
        
        // Test validation
        assert!(config.set_value(
            "performance.cpu_target", 
            ConfigValue::Integer(110), 
            ConfigSource::UserInput
        ).is_err());
        
        assert!(config.set_value(
            "performance.cpu_target", 
            ConfigValue::Integer(90), 
            ConfigSource::UserInput
        ).is_ok());
    }
} 