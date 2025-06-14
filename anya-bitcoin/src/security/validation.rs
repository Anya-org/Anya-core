//! Security validation utilities
//!
//! This module provides validation functions for security-critical operations
//! in the Bitcoin implementation.

/// Result of a validation operation
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    /// Whether the validation passed
    pub is_valid: bool,
    /// Optional error message if validation failed
    pub error_message: Option<String>,
    /// Additional validation metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl ValidationResult {
    /// Create a successful validation result
    pub fn success() -> Self {
        Self {
            is_valid: true,
            error_message: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a failed validation result with an error message
    pub fn failure(error: String) -> Self {
        Self {
            is_valid: false,
            error_message: Some(error),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Add metadata to the validation result
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::success()
    }
}

/// Validate a cryptographic signature
pub fn validate_signature(data: &[u8], signature: &[u8], public_key: &[u8]) -> ValidationResult {
    // Placeholder implementation - in production this would use proper crypto verification
    if data.is_empty() || signature.is_empty() || public_key.is_empty() {
        return ValidationResult::failure("Invalid input data".to_string());
    }

    // For now, return success - real implementation would verify the signature
    ValidationResult::success()
}

/// Validate transaction inputs
pub fn validate_transaction_inputs(inputs: &[Vec<u8>]) -> ValidationResult {
    if inputs.is_empty() {
        return ValidationResult::failure("Transaction must have at least one input".to_string());
    }

    for input in inputs {
        if input.is_empty() {
            return ValidationResult::failure("Transaction input cannot be empty".to_string());
        }
    }

    ValidationResult::success()
}

/// Validate transaction outputs
pub fn validate_transaction_outputs(outputs: &[Vec<u8>]) -> ValidationResult {
    if outputs.is_empty() {
        return ValidationResult::failure("Transaction must have at least one output".to_string());
    }

    for output in outputs {
        if output.is_empty() {
            return ValidationResult::failure("Transaction output cannot be empty".to_string());
        }
    }

    ValidationResult::success()
}
