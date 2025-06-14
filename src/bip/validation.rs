// [AIR-3][AIS-3][BPC-3][AIT-3] BIP Validation Implementation

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

/// Result type for BIP validation
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Compliance status for BIP validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    /// BIP is enabled and compliant
    Compliant,
    /// BIP is present but not fully compliant
    Partial,
    /// BIP is not present or not implemented
    Missing,
}

impl From<bool> for ComplianceStatus {
    fn from(status: bool) -> Self {
        if status {
            ComplianceStatus::Compliant
        } else {
            ComplianceStatus::Missing
        }
    }
}

impl fmt::Display for ComplianceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComplianceStatus::Compliant => write!(f, "Compliant"),
            ComplianceStatus::Partial => write!(f, "Partial"),
            ComplianceStatus::Missing => write!(f, "Missing"),
        }
    }
}

/// Trait for validating BIP implementations
pub trait BipValidator {
    /// Validate a BIP implementation
    fn validate_bip(&self, bip: &str) -> Result<ComplianceStatus>;
}

/// Configuration for Bitcoin functionality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinConfig {
    pub taproot_enabled: bool,
    pub psbt_version: u8,
    pub tapscript_enabled: bool,
    pub bip353_enabled: bool,
    pub bip353_status: super::bip353::Bip353Status,
}

impl Default for BitcoinConfig {
    fn default() -> Self {
        Self {
            taproot_enabled: true,
            psbt_version: 2,
            tapscript_enabled: true,
            bip353_enabled: false,
            bip353_status: super::bip353::Bip353Status::Disabled,
        }
    }
}

impl BipValidator for BitcoinConfig {
    fn validate_bip(&self, bip: &str) -> Result<ComplianceStatus> {
        match bip {
            "BIP-341" => Ok(self.taproot_enabled.into()),
            "BIP-174" => Ok((self.psbt_version >= 2).into()),
            "BIP-342" => Ok(self.tapscript_enabled.into()),
            "BIP-370" => Ok((self.psbt_version >= 2).into()),
            "BIP-353" => match self.bip353_status {
                super::bip353::Bip353Status::Disabled => Ok(ComplianceStatus::Missing),
                super::bip353::Bip353Status::Stable => Ok(ComplianceStatus::Compliant),
                super::bip353::Bip353Status::Beta => Ok(ComplianceStatus::Partial),
            },
            _ => Ok(ComplianceStatus::Missing),
        }
    }
}
