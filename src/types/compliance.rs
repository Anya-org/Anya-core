// Missing types for BIP compliance and protocol validation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolCompliance {
    pub support_level: BipSupportLevel,
    pub verification_status: VerificationStatus,
    pub compliance_score: f64,
    pub supported_bips: Vec<String>,
    pub missing_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BipSupportLevel {
    Full,
    Partial,
    Minimal,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationStatus {
    Passed,
    Failed,
    Pending,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MilestoneStatus {
    Completed,
    InProgress,
    Failed,
    Pending,
}

impl Default for ProtocolCompliance {
    fn default() -> Self {
        Self {
            support_level: BipSupportLevel::Full,
            verification_status: VerificationStatus::Passed,
            compliance_score: 1.0,
            supported_bips: vec![
                "BIP-174".to_string(),
                "BIP-340".to_string(),
                "BIP-341".to_string(),
                "BIP-342".to_string(),
                "BIP-370".to_string(),
            ],
            missing_features: Vec::new(),
        }
    }
}

impl ProtocolCompliance {
    pub fn new(support_level: BipSupportLevel, verification_status: VerificationStatus) -> Self {
        Self {
            support_level,
            verification_status,
            compliance_score: 1.0,
            supported_bips: Vec::new(),
            missing_features: Vec::new(),
        }
    }

    pub fn is_compliant(&self) -> bool {
        matches!(self.verification_status, VerificationStatus::Passed)
            && matches!(
                self.support_level,
                BipSupportLevel::Full | BipSupportLevel::Partial
            )
    }
}

/// Enhanced DWN options for advanced message processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedDwnOptions {
    /// Whether to use Bitcoin anchoring
    pub use_bitcoin_anchoring: bool,
    /// Minimum confirmations for anchored records
    pub min_confirmations: u32,
    /// Custom metadata for the operation
    pub metadata: HashMap<String, String>,
    /// Whether to enable cross-layer verification
    pub cross_layer_verification: bool,
    /// Timeout for operations in seconds
    pub timeout_seconds: u64,
    /// Priority level for processing
    pub priority: DwnPriority,
}

impl Default for EnhancedDwnOptions {
    fn default() -> Self {
        Self {
            use_bitcoin_anchoring: false,
            min_confirmations: 1,
            metadata: HashMap::new(),
            cross_layer_verification: false,
            timeout_seconds: 30,
            priority: DwnPriority::Normal,
        }
    }
}

/// DWN operation priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub enum DwnPriority {
    Low,
    #[default]
    Normal,
    High,
    Critical,
}


/// Core system structure for main.rs
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct CoreSystem {
    /// System configuration
    pub config: SystemConfig,
    /// System status
    pub status: SystemStatus,
    /// Performance metrics
    pub metrics: HashMap<String, f64>,
}


/// System configuration
#[derive(Debug, Clone)]
pub struct SystemConfig {
    /// Whether the system is enabled
    pub enabled: bool,
    /// System name
    pub name: String,
    /// Configuration version
    pub version: String,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            name: "anya-core".to_string(),
            version: "1.1.0".to_string(),
        }
    }
}

/// System status information
#[derive(Debug, Clone)]
pub struct SystemStatus {
    /// Whether the system is operational
    pub operational: bool,
    /// Status message
    pub message: String,
    /// Last updated timestamp
    pub last_updated: u64,
}

impl Default for SystemStatus {
    fn default() -> Self {
        Self {
            operational: true,
            message: "System running".to_string(),
            last_updated: 0,
        }
    }
}

/// Security level enumeration for main.rs
#[derive(Debug, Clone)]
pub enum SecurityLevel {
    Basic,
    Enhanced,
    Strict,
    Maximum,
}

/// Resource type enumeration for main.rs
#[derive(Debug, Clone)]
pub enum ResourceType {
    CPU,
    Memory,
    Network,
    Storage,
    Custom(i32),
}

// BIP Compliance Report structures that are expected by the installer
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BipComplianceReport {
    pub bip341: ComplianceStatus,
    pub bip342: ComplianceStatus,
    pub bip174: ComplianceStatus,
    pub bip370: ComplianceStatus,
    pub overall_status: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ComplianceStatus {
    #[default]
    Full,
    Partial,
    Missing,
}
