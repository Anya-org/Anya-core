use std::error::Error;
// Security validation module
// Implements security validation for Bitcoin operations
// as per official Bitcoin Improvement Proposals (BIPs) requirements

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
    pub fn new(is_valid: bool, message: String) -> Self {
        Self {
            is_valid,
            message,
        }
    }
    
    /// Create a valid validation result
    pub fn valid(message: String) -> Self {
        Self {
            is_valid: true,
            message,
        }
    }
    
    /// Create an invalid validation result
    pub fn invalid(message: String) -> Self {
        Self {
            is_valid: false,
            message,
        }
    }
    
    // Methods already defined above
} 
