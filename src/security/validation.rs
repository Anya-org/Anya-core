use std::error::Error;
// Security validation module
// Implements security validation for Bitcoin operations
// as per Bitcoin Development Framework v2.5 requirements

/// Validation result for security operations
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the validation succeeded
    pub is_valid: bool,
    /// Validation message
    pub message: String,
}

impl ValidationResult {
    /// Create a new validation result
    pub fn new(is_valid: bool, message: String) -> Self  -> Result<(), Box<dyn Error>> {
        Self {
            is_valid,
            message,
        }
    }
    
    /// Create a valid result
    pub fn valid(message: String) -> Self  -> Result<(), Box<dyn Error>> {
        Self::new(true, message)
    }
    
    /// Create an invalid result
    pub fn invalid(message: String) -> Self  -> Result<(), Box<dyn Error>> {
        Self::new(false, message)
    }
} 
