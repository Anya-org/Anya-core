// BIP Implementation Module
// [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
//
// Implements Bitcoin Improvement Proposals (BIPs) according to
// Bitcoin Improvement Proposals

pub mod bip341;
pub mod bip342;

/// Export key types for BIP implementations
pub use bitcoin::secp256k1::{Secp256k1, XOnlyPublicKey};
pub use bitcoin::taproot::{
    ControlBlock, LeafVersion, TapBranchHash, TapLeaf, TapTree, TaprootBuilder,
};
pub use bitcoin::{PrivateKey, PublicKey, ScriptBuf, TapLeafHash, TapNodeHash, TapTweakHash};

/// BIP implementation status for compliance checks
#[derive(Debug, Clone, PartialEq)]
pub enum BIPStatus {
    Complete,
    Partial,
    Planned,
    NotSupported,
}

/// BIP implementation registry
#[derive(Debug)]
pub struct BIPRegistry {
    registry: std::collections::HashMap<String, BIPStatus>,
}

impl BIPRegistry {
    /// Create a new BIP registry
    pub fn new() -> Self {
        let mut registry = Self {
            registry: std::collections::HashMap::new(),
        };

        // Initialize with supported BIPs
        registry.register("BIP-341", BIPStatus::Complete); // Taproot
        registry.register("BIP-342", BIPStatus::Complete); // Tapscript
        registry.register("BIP-174", BIPStatus::Complete); // PSBT
        registry.register("BIP-370", BIPStatus::Complete); // PSBT v2

        registry
    }

    /// Register a BIP with its implementation status
    pub fn register(&mut self, bip: &str, status: BIPStatus) {
        self.registry.insert(bip.to_string(), status);
    }

    /// Get the implementation status of a BIP
    pub fn status(&self, bip: &str) -> BIPStatus {
        self.registry
            .get(bip)
            .cloned()
            .unwrap_or(BIPStatus::NotSupported)
    }

    /// Check if a BIP is supported (Complete or Partial)
    pub fn is_supported(&self, bip: &str) -> bool {
        match self.status(bip) {
            BIPStatus::Complete | BIPStatus::Partial => true,
            _ => false,
        }
    }

    /// Get all supported BIPs
    pub fn supported_bips(&self) -> Vec<String> {
        self.registry
            .iter()
            .filter(|(_, status)| matches!(status, BIPStatus::Complete | BIPStatus::Partial))
            .map(|(bip, _)| bip.clone())
            .collect()
    }

    /// Get report of all BIPs and their status
    pub fn status_report(&self) -> Vec<(String, BIPStatus)> {
        self.registry
            .iter()
            .map(|(bip, status)| (bip.clone(), status.clone()))
            .collect()
    }
}
