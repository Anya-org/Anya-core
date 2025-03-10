// AIE-001: Security Module Integration
// Exports system hardening functionality

// System hardening module
pub mod system_hardening;

// Re-exports for convenience
pub use system_hardening::SystemHardening;
pub use system_hardening::SecurityLevel;
pub use system_hardening::ConfigStatus;
pub use system_hardening::HardeningConfig;

/// Helper function to create a system hardening manager with default auto-save frequency (20)
pub fn create_system_hardening() -> SystemHardening {
    SystemHardening::new(20)
}

/// Helper function to create a basic security configuration for a component
pub fn create_basic_security_config(component_name: &str) -> std::collections::HashMap<String, String> {
    let mut settings = std::collections::HashMap::new();
    
    // Basic security settings
    settings.insert("firewall".to_string(), "enabled".to_string());
    settings.insert("encryption".to_string(), "enabled".to_string());
    settings.insert("access_control".to_string(), "strict".to_string());
    settings.insert("audit_logging".to_string(), "enabled".to_string());
    settings.insert("intrusion_detection".to_string(), "enabled".to_string());
    
    // Component-specific settings
    match component_name {
        "network" => {
            settings.insert("port_scanning_protection".to_string(), "enabled".to_string());
            settings.insert("ddos_protection".to_string(), "enabled".to_string());
        },
        "database" => {
            settings.insert("query_sanitization".to_string(), "strict".to_string());
            settings.insert("data_encryption".to_string(), "aes-256".to_string());
        },
        "api" => {
            settings.insert("rate_limiting".to_string(), "enabled".to_string());
            settings.insert("input_validation".to_string(), "strict".to_string());
        },
        _ => {
            // Generic settings for other components
            settings.insert("default_deny".to_string(), "enabled".to_string());
        }
    }
    
    settings
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_security_config() {
        let network_config = create_basic_security_config("network");
        let db_config = create_basic_security_config("database");
        
        // Check common settings
        assert_eq!(network_config.get("firewall"), Some(&"enabled".to_string()));
        assert_eq!(db_config.get("firewall"), Some(&"enabled".to_string()));
        
        // Check component-specific settings
        assert_eq!(network_config.get("ddos_protection"), Some(&"enabled".to_string()));
        assert_eq!(db_config.get("data_encryption"), Some(&"aes-256".to_string()));
    }
}

// Security module
// Implements security features for Bitcoin operations
// as per Bitcoin Development Framework v2.5 requirements

pub mod validation;

// Re-export key types
pub use validation::ValidationResult; 