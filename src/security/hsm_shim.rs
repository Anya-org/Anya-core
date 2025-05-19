//! HSM Shim Implementation
//!
//! This module provides a minimal compatible implementation of HSM interfaces
//! when the full HSM feature is not enabled. This allows the rest of the system
//! to compile and operate without requiring the HSM functionality.

use std::fmt;
use std::sync::Arc;
use std::collections::HashMap;

/// Stub error type for HSM operations when the feature is disabled
#[derive(Debug)]
pub struct HsmStubError {
    pub message: String,
}

impl fmt::Display for HsmStubError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HSM functionality not available: {}", self.message)
    }
}

impl std::error::Error for HsmStubError {}

/// Create an HSM stub error
pub fn hsm_stub_error(msg: &str) -> HsmStubError {
    HsmStubError {
        message: format!("HSM support disabled: {}", msg)
    }
}

/// Stub HSM Manager for when HSM functionality is disabled
#[derive(Debug)]
pub struct HsmManager;

impl HsmManager {
    /// Create a new stub HSM manager
    pub fn new(_config: HashMap<String, String>) -> Result<Self, HsmStubError> {
        Err(hsm_stub_error("Cannot create HsmManager when HSM feature is disabled"))
    }
    
    /// Initialize the HSM (not available in stub)
    pub async fn initialize(&self) -> Result<(), HsmStubError> {
        Err(hsm_stub_error("HSM functionality is disabled"))
    }
    
    /// Get the status of the HSM (always returns an error in stub)
    pub async fn get_status(&self) -> Result<HsmStatus, HsmStubError> {
        Err(hsm_stub_error("HSM functionality is disabled"))
    }
}

/// Stub HSM Status
#[derive(Debug)]
pub struct HsmStatus {
    pub provider_name: String,
    pub available: bool,
}

/// Minimal stub for HsmKeyType
#[derive(Debug, Clone)]
pub enum KeyType {
    Rsa,
    Ec,
    Aes,
    Hmac,
}

/// Minimal stub for SigningAlgorithm
#[derive(Debug, Clone)]
pub enum SigningAlgorithm {
    RsaSha256,
    EcdsaP256,
}

/// Stub trait for HsmProvider
pub trait HsmProvider: Send + Sync {}

/// Stub for BitcoinHsmProvider
#[derive(Debug, Clone, Default)]
pub struct BitcoinHsmProvider;

impl BitcoinHsmProvider {
    pub fn default() -> Self {
        BitcoinHsmProvider
    }
}

/// Stub for HsmConfig
#[derive(Debug, Clone, Default)]
pub struct HsmConfig;
