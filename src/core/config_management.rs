#![feature(edition2021)]
/// AIR-012: Unified Configuration Management System
/// 
/// This module provides a centralized configuration management system
/// with support for multiple configuration sources, validation, and
/// both automated and user inputs.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use thiserror::Error;
use std::io;

/// Configuration source types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigSource {
    /// File-based configuration
    File(PathBuf),
    /// Environment variables
    Environment,
    /// Command-line arguments
    CommandLine,
    /// User input via UI/CLI
    UserInput,
    /// Programmatic API
    Api,
    /// Default values
    Default,
}

/// Configuration value types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigValue {
    /// String value
    String(String),
    /// Integer value
    Integer(i64),
    /// Float value
    Float(f64),
    /// Boolean value
    Boolean(bool),
    /// Array of values
    Array(Vec<ConfigValue>),
    /// Map of values
    Map(HashMap<String, ConfigValue>),
    /// Null value
    Null,
}

/// Configuration entry with metadata
#[derive(Debug, Clone)]
pub struct ConfigEntry {
    /// Configuration key
    pub key: String,
    /// Configuration value
    pub value: ConfigValue,
    /// Source of the configuration value
    pub source: ConfigSource,
    /// Timestamp when the configuration was last updated
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Whether the configuration value is sensitive
    pub sensitive: bool,
    /// Whether the configuration value is read-only
    pub read_only: bool,
}

/// Configuration validation rule
#[derive(Clone)]
pub enum ValidationRule {
    /// Required field
    Required,
    /// Minimum value for numbers
    MinValue(f64),
    /// Maximum value for numbers
    MaxValue(f64),
    /// Minimum length for strings
    MinLength(usize),
    /// Maximum length for strings
    MaxLength(usize),
    /// Regular expression pattern for strings
    Pattern(String),
    /// Enumeration of allowed values
    Enum(Vec<ConfigValue>),
    /// Custom validation function
    Custom(Arc<dyn Fn(&ConfigValue) -> Result<(), ValidationError> + Send + Sync>),
}

impl std::fmt::Debug for ValidationRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationRule::Required => write!(f, "ValidationRule::Required"),
            ValidationRule::MinValue(val) => write!(f, "ValidationRule::MinValue({})", val),
            ValidationRule::MaxValue(val) => write!(f, "ValidationRule::MaxValue({})", val),
            ValidationRule::MinLength(len) => write!(f, "ValidationRule::MinLength({})", len),
            ValidationRule::MaxLength(len) => write!(f, "ValidationRule::MaxLength({})", len),
            ValidationRule::Pattern(pattern) => write!(f, "ValidationRule::Pattern({})", pattern),
            ValidationRule::Enum(values) => write!(f, "ValidationRule::Enum({:?})", values),
            ValidationRule::Custom(_) => write!(f, "ValidationRule::Custom(<function>)"),
        }
    }
}

/// Configuration change event
#[derive(Debug, Clone)]
pub struct ConfigChangeEvent {
    /// Configuration key
    pub key: String,
    /// Old configuration value
    pub old_value: Option<ConfigValue>,
    /// New configuration value
    pub new_value: ConfigValue,
    /// Source of the change
    pub source: ConfigSource,
    /// Timestamp when the change occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Configuration error types
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Configuration key not found
    #[error("Configuration key not found: {0}")]
    KeyNotFound(String),
    
    /// Configuration validation error
    #[error("Configuration validation error: {0}")]
    ValidationError(#[from] ValidationError),
    
    /// Configuration loading error
    #[error("Configuration loading error: {0}")]
    LoadError(String),
    
    /// Configuration saving error
    #[error("Configuration saving error: {0}")]
    SaveError(String),
    
    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    /// Deserialization error
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}

/// Configuration validation error
#[derive(Debug, Error)]
pub enum ValidationError {
    /// Required field is missing
    #[error("Required field is missing: {0}")]
    RequiredField(String),
    
    /// Value is less than minimum
    #[error("Value is less than minimum: {0} < {1}")]
    LessThanMinimum(f64, f64),
    
    /// Value is greater than maximum
    #[error("Value is greater than maximum: {0} > {1}")]
    GreaterThanMaximum(f64, f64),
    
    /// String is shorter than minimum length
    #[error("String is shorter than minimum length: {0} < {1}")]
    ShorterThanMinLength(usize, usize),
    
    /// String is longer than maximum length
    #[error("String is longer than maximum length: {0} > {1}")]
    LongerThanMaxLength(usize, usize),
    
    /// Pattern does not match
    #[error("Pattern does not match: {0}")]
    PatternMismatch(String),
    
    /// Value is not in allowed enumeration
    #[error("Value is not in allowed enumeration")]
    NotInEnum,
    
    /// Custom validation error
    #[error("Custom validation error: {0}")]
    Custom(String),
}

/// Configuration change listener
pub type ConfigChangeListener = Arc<dyn Fn(&ConfigChangeEvent) -> () + Send + Sync>;

/// Configuration manager
pub struct ConfigManager {
    /// Configuration entries
    entries: RwLock<HashMap<String, ConfigEntry>>,
    /// Configuration validation rules
    validation_rules: RwLock<HashMap<String, Vec<ValidationRule>>>,
    /// Configuration change listeners
    listeners: RwLock<Vec<ConfigChangeListener>>,
    /// Configuration change history
    history: RwLock<Vec<ConfigChangeEvent>>,
    /// Maximum history size
    max_history_size: usize,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            validation_rules: RwLock::new(HashMap::new()),
            listeners: RwLock::new(Vec::new()),
            history: RwLock::new(Vec::new()),
            max_history_size: 100,
        }
    }
    
    /// Load configuration from a file
    pub fn load_from_file(&self, path: &PathBuf) -> Result<(), ConfigError> {
        let config_str = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::LoadError(format!("Failed to read file: {}", e)))?;
            
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
            
        let config_map: HashMap<String, ConfigValue> = match extension {
            "json" => serde_json::from_str(&config_str)
                .map_err(|e| ConfigError::DeserializationError(format!("JSON error: {}", e)))?,
            "yaml" | "yml" => serde_yaml::from_str(&config_str)
                .map_err(|e| ConfigError::DeserializationError(format!("YAML error: {}", e)))?,
            "toml" => toml::from_str(&config_str)
                .map_err(|e| ConfigError::DeserializationError(format!("TOML error: {}", e)))?,
            _ => return Err(ConfigError::LoadError(format!("Unsupported file extension: {}", extension))),
        };
        
        self.set_values_from_map(config_map, ConfigSource::File(path.clone()))
    }
    
    /// Load configuration from environment variables
    pub fn load_from_env(&self, prefix: &str) -> Result<(), ConfigError> {
        let mut config_map = HashMap::new();
        
        for (key, value) in std::env::vars() {
            if key.starts_with(prefix) {
                let config_key = key[prefix.len()..].to_lowercase();
                config_map.insert(config_key, ConfigValue::String(value));
            }
        }
        
        self.set_values_from_map(config_map, ConfigSource::Environment)
    }
    
    /// Set multiple configuration values from a map
    pub fn set_values_from_map(
        &self, 
        values: HashMap<String, ConfigValue>, 
        source: ConfigSource
    ) -> Result<(), ConfigError> {
        for (key, value) in values {
            self.set_value(&key, value, source.clone())?;
        }
        
        Ok(())
    }
    
    /// Set a configuration value
    pub fn set_value(
        &self, 
        key: &str, 
        value: ConfigValue, 
        source: ConfigSource
    ) -> Result<(), ConfigError> {
        // Validate the configuration value
        self.validate_value(key, &value)?;
        
        let mut entries = self.entries.write().unwrap();
        
        // Get the old value for event notification
        let old_value = entries.get(key).map(|entry| entry.value.clone());
        
        // Create the new configuration entry
        let entry = ConfigEntry {
            key: key.to_string(),
            value: value.clone(),
            source: source.clone(),
            updated_at: chrono::Utc::now(),
            sensitive: false, // Default to non-sensitive
            read_only: false, // Default to writable
        };
        
        // Update the configuration entry
        entries.insert(key.to_string(), entry);
        
        // Create and record the change event
        let event = ConfigChangeEvent {
            key: key.to_string(),
            old_value,
            new_value: value,
            source,
            timestamp: chrono::Utc::now(),
        };
        
        // Add to history
        self.add_to_history(event.clone());
        
        // Notify listeners
        self.notify_listeners(&event);
        
        Ok(())
    }
    
    /// Add a change event to history
    fn add_to_history(&self, event: ConfigChangeEvent) {
        let mut history = self.history.write().unwrap();
        
        // Add the event to history
        history.push(event);
        
        // Trim history if it exceeds the maximum size
        if history.len() > self.max_history_size {
            let drain_count = history.len() - self.max_history_size;
            history.drain(0..drain_count);
        }
    }
    
    /// Notify listeners of a configuration change
    fn notify_listeners(&self, event: &ConfigChangeEvent) {
        let listeners = self.listeners.read().unwrap();
        
        for listener in listeners.iter() {
            listener(event);
        }
    }
    
    /// Get a configuration value
    pub fn get_value(&self, key: &str) -> Result<ConfigValue, ConfigError> {
        let entries = self.entries.read().unwrap();
        
        entries.get(key)
            .map(|entry| entry.value.clone())
            .ok_or(ConfigError::KeyNotFound(key.to_string()))
    }
    
    /// Check if a configuration key exists
    pub fn has_key(&self, key: &str) -> bool {
        let entries = self.entries.read().unwrap();
        entries.contains_key(key)
    }
    
    /// Get a string configuration value
    pub fn get_string(&self, key: &str) -> Result<String, ConfigError> {
        match self.get_value(key)? {
            ConfigValue::String(value) => Ok(value),
            _ => Err(ConfigError::ValidationError(ValidationError::Custom(
                format!("Configuration value is not a string: {}", key)
            ))),
        }
    }
    
    /// Get an integer configuration value
    pub fn get_integer(&self, key: &str) -> Result<i64, ConfigError> {
        match self.get_value(key)? {
            ConfigValue::Integer(value) => Ok(value),
            _ => Err(ConfigError::ValidationError(ValidationError::Custom(
                format!("Configuration value is not an integer: {}", key)
            ))),
        }
    }
    
    /// Get a float configuration value
    pub fn get_float(&self, key: &str) -> Result<f64, ConfigError> {
        match self.get_value(key)? {
            ConfigValue::Float(value) => Ok(value),
            ConfigValue::Integer(value) => Ok(value as f64),
            _ => Err(ConfigError::ValidationError(ValidationError::Custom(
                format!("Configuration value is not a float: {}", key)
            ))),
        }
    }
    
    /// Get a boolean configuration value
    pub fn get_boolean(&self, key: &str) -> Result<bool, ConfigError> {
        match self.get_value(key)? {
            ConfigValue::Boolean(value) => Ok(value),
            _ => Err(ConfigError::ValidationError(ValidationError::Custom(
                format!("Configuration value is not a boolean: {}", key)
            ))),
        }
    }
    
    /// Add a validation rule for a configuration key
    pub fn add_validation_rule(&self, key: &str, rule: ValidationRule) {
        let mut rules = self.validation_rules.write().unwrap();
        
        let key_rules = rules.entry(key.to_string()).or_insert_with(Vec::new);
        key_rules.push(rule);
    }
    
    /// Validate a configuration value against the rules
    fn validate_value(&self, key: &str, value: &ConfigValue) -> Result<(), ConfigError> {
        let rules = self.validation_rules.read().unwrap();
        
        if let Some(key_rules) = rules.get(key) {
            for rule in key_rules {
                match rule {
                    ValidationRule::Required => {
                        if let ConfigValue::Null = value {
                            return Err(ConfigError::ValidationError(ValidationError::RequiredField(
                                key.to_string()
                            )));
                        }
                    },
                    ValidationRule::MinValue(min) => {
                        let value_f64 = match value {
                            ConfigValue::Integer(i) => *i as f64,
                            ConfigValue::Float(f) => *f,
                            _ => continue,
                        };
                        
                        if value_f64 < *min {
                            return Err(ConfigError::ValidationError(ValidationError::LessThanMinimum(
                                value_f64, *min
                            )));
                        }
                    },
                    ValidationRule::MaxValue(max) => {
                        let value_f64 = match value {
                            ConfigValue::Integer(i) => *i as f64,
                            ConfigValue::Float(f) => *f,
                            _ => continue,
                        };
                        
                        if value_f64 > *max {
                            return Err(ConfigError::ValidationError(ValidationError::GreaterThanMaximum(
                                value_f64, *max
                            )));
                        }
                    },
                    ValidationRule::MinLength(min) => {
                        if let ConfigValue::String(s) = value {
                            if s.len() < *min {
                                return Err(ConfigError::ValidationError(ValidationError::ShorterThanMinLength(
                                    s.len(), *min
                                )));
                            }
                        }
                    },
                    ValidationRule::MaxLength(max) => {
                        if let ConfigValue::String(s) = value {
                            if s.len() > *max {
                                return Err(ConfigError::ValidationError(ValidationError::LongerThanMaxLength(
                                    s.len(), *max
                                )));
                            }
                        }
                    },
                    ValidationRule::Pattern(pattern) => {
                        if let ConfigValue::String(s) = value {
                            let regex = regex::Regex::new(pattern)
                                .map_err(|e| ConfigError::ValidationError(ValidationError::Custom(
                                    format!("Invalid regex pattern: {}", e)
                                )))?;
                                
                            if !regex.is_match(s) {
                                return Err(ConfigError::ValidationError(ValidationError::PatternMismatch(
                                    s.clone()
                                )));
                            }
                        }
                    },
                    ValidationRule::Enum(allowed_values) => {
                        if !allowed_values.contains(value) {
                            return Err(ConfigError::ValidationError(ValidationError::NotInEnum));
                        }
                    },
                    ValidationRule::Custom(validator) => {
                        if let Err(e) = validator(value) {
                            return Err(ConfigError::ValidationError(e));
                        }
                    },
                }
            }
        }
        
        Ok(())
    }
    
    /// Add a configuration change listener
    pub fn add_listener(&self, listener: ConfigChangeListener) {
        let mut listeners = self.listeners.write().unwrap();
        listeners.push(listener);
    }
    
    /// Get the configuration history
    pub fn get_history(&self) -> Vec<ConfigChangeEvent> {
        let history = self.history.read().unwrap();
        history.clone()
    }
    
    /// Save the configuration to a file
    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), ConfigError> {
        let entries = self.entries.read().unwrap();
        
        // Convert entries to a map
        let mut config_map = HashMap::new();
        for (key, entry) in entries.iter() {
            if !entry.sensitive {
                config_map.insert(key.clone(), entry.value.clone());
            }
        }
        
        // Serialize the configuration
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
            
        let config_str = match extension {
            "json" => serde_json::to_string_pretty(&config_map)
                .map_err(|e| ConfigError::SerializationError(format!("JSON error: {}", e)))?,
            "yaml" | "yml" => serde_yaml::to_string(&config_map)
                .map_err(|e| ConfigError::SerializationError(format!("YAML error: {}", e)))?,
            "toml" => toml::to_string(&config_map)
                .map_err(|e| ConfigError::SerializationError(format!("TOML error: {}", e)))?,
            _ => return Err(ConfigError::SaveError(format!("Unsupported file extension: {}", extension))),
        };
        
        // Write the configuration to the file
        std::fs::write(path, config_str)
            .map_err(|e| ConfigError::SaveError(format!("Failed to write file: {}", e)))?;
            
        Ok(())
    }
    
    /// Mark a configuration key as sensitive
    pub fn mark_as_sensitive(&self, key: &str, sensitive: bool) -> Result<(), ConfigError> {
        let mut entries = self.entries.write().unwrap();
        
        let entry = entries.get_mut(key)
            .ok_or(ConfigError::KeyNotFound(key.to_string()))?;
            
        entry.sensitive = sensitive;
        
        Ok(())
    }
    
    /// Mark a configuration key as read-only
    pub fn mark_as_read_only(&self, key: &str, read_only: bool) -> Result<(), ConfigError> {
        let mut entries = self.entries.write().unwrap();
        
        let entry = entries.get_mut(key)
            .ok_or(ConfigError::KeyNotFound(key.to_string()))?;
            
        entry.read_only = read_only;
        
        Ok(())
    }
    
    /// Get all configuration keys
    pub fn get_keys(&self) -> Vec<String> {
        let entries = self.entries.read().unwrap();
        entries.keys().cloned().collect()
    }
    
    /// Reset a configuration key to its default value
    pub fn reset_to_default(&self, key: &str) -> Result<(), ConfigError> {
        let entries = self.entries.read().unwrap();
        
        // Find a default value
        let default_entry = entries.values()
            .find(|entry| entry.key == key && entry.source == ConfigSource::Default);
            
        if let Some(default_entry) = default_entry {
            // Clone the default value
            let default_value = default_entry.value.clone();
            
            // Release the read lock
            drop(entries);
            
            // Set the value to the default
            self.set_value(key, default_value, ConfigSource::Default)?;
            
            Ok(())
        } else {
            Err(ConfigError::KeyNotFound(format!("No default value for key: {}", key)))
        }
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global configuration manager instance
pub static CONFIG_MANAGER: once_cell::sync::Lazy<ConfigManager> = once_cell::sync::Lazy::new(|| {
    ConfigManager::new()
});

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_config_operations() {
        let config = ConfigManager::new();
        
        // Set a value
        config.set_value(
            "test.key", 
            ConfigValue::String("test value".to_string()), 
            ConfigSource::Default
        ).unwrap();
        
        // Get the value
        let value = config.get_value("test.key").unwrap();
        assert!(matches!(value, ConfigValue::String(s) if s == "test value"));
        
        // Check if key exists
        assert!(config.has_key("test.key"));
        assert!(!config.has_key("non.existent.key"));
        
        // Get typed value
        let string_value = config.get_string("test.key").unwrap();
        assert_eq!(string_value, "test value");
    }
    
    #[test]
    fn test_validation_rules() {
        let config = ConfigManager::new();
        
        // Add validation rules
        config.add_validation_rule("min.value", ValidationRule::MinValue(10.0));
        config.add_validation_rule("max.length", ValidationRule::MaxLength(5));
        
        // Test min value validation
        assert!(config.set_value(
            "min.value",
            ConfigValue::Integer(5),
            ConfigSource::Default
        ).is_err());
        
        assert!(config.set_value(
            "min.value",
            ConfigValue::Integer(15),
            ConfigSource::Default
        ).is_ok());
        
        // Test max length validation
        assert!(config.set_value(
            "max.length",
            ConfigValue::String("123456".to_string()),
            ConfigSource::Default
        ).is_err());
        
        assert!(config.set_value(
            "max.length",
            ConfigValue::String("12345".to_string()),
            ConfigSource::Default
        ).is_ok());
    }
    
    #[test]
    fn test_configuration_history() {
        let config = ConfigManager::new();
        
        // Set some values
        config.set_value(
            "history.test",
            ConfigValue::Integer(1),
            ConfigSource::Default
        ).unwrap();
        
        config.set_value(
            "history.test",
            ConfigValue::Integer(2),
            ConfigSource::Default
        ).unwrap();
        
        config.set_value(
            "history.test",
            ConfigValue::Integer(3),
            ConfigSource::Default
        ).unwrap();
        
        // Check history
        let history = config.get_history();
        assert_eq!(history.len(), 3);
        assert_eq!(history[0].key, "history.test");
        assert_eq!(history[0].old_value, None);
        assert!(matches!(history[0].new_value, ConfigValue::Integer(1)));
        
        assert_eq!(history[1].key, "history.test");
        assert!(matches!(history[1].old_value, Some(ConfigValue::Integer(1))));
        assert!(matches!(history[1].new_value, ConfigValue::Integer(2)));
        
        assert_eq!(history[2].key, "history.test");
        assert!(matches!(history[2].old_value, Some(ConfigValue::Integer(2))));
        assert!(matches!(history[2].new_value, ConfigValue::Integer(3)));
    }
} 