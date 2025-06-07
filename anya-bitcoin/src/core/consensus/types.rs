//! Consensus types and structures
//!
//! This module provides common types used in consensus validation.

use bitflags::bitflags;

/// Script verification flags
bitflags! {
    /// Flags controlling script verification behavior
    #[derive(Copy, Clone)]
    pub struct VerifyFlags: u32 {
        /// No flags set
        const NONE = 0;
        /// Verify P2SH scripts
        const P2SH = 1 << 0;
        /// Verify SegWit scripts
        const WITNESS = 1 << 1;
        /// Verify Taproot scripts
        const TAPROOT = 1 << 2;
        /// Strictly verify DER signatures
        const STRICTENC = 1 << 3;
        /// Verify minimal data pushes
        const MINIMALDATA = 1 << 4;
        /// Discourage upgradable witness programs
        const DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM = 1 << 5;
        /// Verify witness pubkey type
        const WITNESS_PUBKEYTYPE = 1 << 6;
        /// Verify const script code
        const CONST_SCRIPTCODE = 1 << 7;
    }
}

impl Default for VerifyFlags {
    fn default() -> Self {
        VerifyFlags::NONE
    }
}

/// Transaction ID type alias
pub type TransactionId = bitcoin::Txid;

/// Proof structure for consensus validation
#[derive(Debug, Clone)]
pub struct Proof {
    /// Merkle proof data
    pub merkle_proof: Vec<u8>,
    /// Block header data
    pub block_header: Vec<u8>,
    /// Additional proof metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl Proof {
    /// Create a new proof
    pub fn new() -> Self {
        Self {
            merkle_proof: Vec::new(),
            block_header: Vec::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Validate the proof
    pub fn validate(&self) -> bool {
        // Placeholder implementation
        // In production, this would validate the merkle proof
        !self.merkle_proof.is_empty() && !self.block_header.is_empty()
    }
}

impl Default for Proof {
    fn default() -> Self {
        Self::new()
    }
}
