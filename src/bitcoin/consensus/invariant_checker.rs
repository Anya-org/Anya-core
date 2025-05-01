use bitcoin::{Transaction, Block, BlockHeader, Script, OutPoint};
use thiserror::Error;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use super::ConsensusError;

/// [CONSENSUS CRITICAL] A consensus invariant that must always be maintained
/// for Bitcoin consensus compatibility
#[derive(Debug, Clone)]
pub struct ConsensusInvariant {
    /// Unique identifier for the invariant
    pub id: String,
    
    /// Human-readable description
    pub description: String,
    
    /// Level of severity if violated
    pub severity: InvariantSeverity,
    
    /// BIP reference (if applicable)
    pub bip_reference: Option<String>,
    
    /// Code references where this invariant is enforced
    pub code_references: Vec<String>,
}

/// Severity level for consensus invariants
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InvariantSeverity {
    /// Critical - violation would cause immediate consensus fork
    Critical,
    
    /// High - violation would likely cause consensus issues
    High,
    
    /// Medium - violation could lead to consensus issues in some cases
    Medium,
    
    /// Low - violation unlikely to cause consensus issues but violates spec
    Low,
}

/// [CONSENSUS CRITICAL] Represents a violation of a consensus invariant
#[derive(Debug, Clone)]
pub struct InvariantViolation {
    /// The invariant that was violated
    pub invariant: ConsensusInvariant,
    
    /// Description of how it was violated
    pub violation_description: String,
    
    /// Transaction that violated the invariant (if applicable)
    pub transaction: Option<Transaction>,
    
    /// Block that violated the invariant (if applicable)
    pub block: Option<Block>,
    
    /// Timestamp when the violation was detected
    pub timestamp: u64,
    
    /// Additional context about the violation
    pub context: HashMap<String, String>,
}

/// [CONSENSUS CRITICAL] Interface for checking Bitcoin consensus invariants
pub trait ConsensusInvariantChecker {
    /// Check if a transaction violates any consensus invariants
    fn check_transaction(&self, tx: &Transaction) -> Result<(), InvariantViolation>;
    
    /// Check if a block violates any consensus invariants
    fn check_block(&self, block: &Block) -> Result<(), InvariantViolation>;
    
    /// Get all invariants monitored by this checker
    fn get_invariants(&self) -> Vec<ConsensusInvariant>;
}

/// Default implementation of ConsensusInvariantChecker that enforces
/// Bitcoin Core consensus rules
pub struct BitcoinCoreInvariantChecker {
    /// All invariants being checked
    invariants: Vec<ConsensusInvariant>,
}

impl Default for BitcoinCoreInvariantChecker {
    fn default() -> Self {
        Self {
            invariants: get_bitcoin_core_invariants(),
        }
    }
}

impl BitcoinCoreInvariantChecker {
    /// Create a new invariant checker
    pub fn new() -> Self {
        Self::default()
    }
}

impl ConsensusInvariantChecker for BitcoinCoreInvariantChecker {
    fn check_transaction(&self, tx: &Transaction) -> Result<(), InvariantViolation> {
        // Check transaction version
        if tx.version < 1 || tx.version > 2 {
            return Err(create_violation(
                "tx-version",
                format!("Transaction version {} is invalid, must be 1 or 2", tx.version),
                Some(tx.clone()),
                None,
                &self.invariants,
            ));
        }
        
        // Check transaction has inputs (non-coinbase)
        if tx.input.is_empty() {
            return Err(create_violation(
                "tx-inputs",
                "Transaction must have at least one input".into(),
                Some(tx.clone()),
                None,
                &self.invariants,
            ));
        }
        
        // Check transaction has outputs
        if tx.output.is_empty() {
            return Err(create_violation(
                "tx-outputs",
                "Transaction must have at least one output".into(),
                Some(tx.clone()),
                None,
                &self.invariants,
            ));
        }
        
        // Check for duplicate inputs
        let mut input_outpoints = HashSet::new();
        for input in &tx.input {
            if !input_outpoints.insert(input.previous_output) {
                return Err(create_violation(
                    "tx-duplicate-inputs",
                    format!("Transaction contains duplicate input: {}", input.previous_output),
                    Some(tx.clone()),
                    None,
                    &self.invariants,
                ));
            }
        }
        
        // Many more checks would be implemented in a complete version
        
        Ok(())
    }
    
    fn check_block(&self, block: &Block) -> Result<(), InvariantViolation> {
        // Check block version
        if block.header.version < 1 {
            return Err(create_violation(
                "block-version",
                format!("Block version {} is invalid, must be ≥ 1", block.header.version),
                None,
                Some(block.clone()),
                &self.invariants,
            ));
        }
        
        // Check block has transactions
        if block.txdata.is_empty() {
            return Err(create_violation(
                "block-tx-count",
                "Block must contain at least one transaction (coinbase)".into(),
                None,
                Some(block.clone()),
                &self.invariants,
            ));
        }
        
        // Check first transaction is coinbase
        if !block.txdata[0].is_coin_base() {
            return Err(create_violation(
                "block-coinbase",
                "First transaction in block must be coinbase".into(),
                None,
                Some(block.clone()),
                &self.invariants,
            ));
        }
        
        // Check other transactions are not coinbase
        for (i, tx) in block.txdata.iter().enumerate().skip(1) {
            if tx.is_coin_base() {
                return Err(create_violation(
                    "block-multiple-coinbase",
                    format!("Block contains multiple coinbase transactions (at index {})", i),
                    None,
                    Some(block.clone()),
                    &self.invariants,
                ));
            }
            
            // Check each transaction
            self.check_transaction(tx)?;
        }
        
        // Many more checks would be implemented in a complete version
        
        Ok(())
    }
    
    fn get_invariants(&self) -> Vec<ConsensusInvariant> {
        self.invariants.clone()
    }
}

/// Create an invariant violation from an invariant ID and description
fn create_violation(
    invariant_id: &str,
    description: String,
    tx: Option<Transaction>,
    block: Option<Block>,
    invariants: &[ConsensusInvariant],
) -> InvariantViolation {
    // Find the invariant
    let invariant = invariants
        .iter()
        .find(|i| i.id == invariant_id)
        .cloned()
        .unwrap_or_else(|| {
            // Create a default invariant if not found
            ConsensusInvariant {
                id: invariant_id.to_string(),
                description: "Unknown invariant".to_string(),
                severity: InvariantSeverity::High,
                bip_reference: None,
                code_references: vec![],
            }
        });
    
    // Create the violation
    InvariantViolation {
        invariant,
        violation_description: description,
        transaction: tx,
        block,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        context: HashMap::new(),
    }
}

/// Define all Bitcoin Core consensus invariants
fn get_bitcoin_core_invariants() -> Vec<ConsensusInvariant> {
    vec![
        // Transaction invariants
        ConsensusInvariant {
            id: "tx-version".to_string(),
            description: "Transaction version must be 1 or 2".to_string(),
            severity: InvariantSeverity::Critical,
            bip_reference: None,
            code_references: vec!["src/bitcoin/validation.rs".to_string()],
        },
        ConsensusInvariant {
            id: "tx-inputs".to_string(),
            description: "Transaction must have at least one input".to_string(),
            severity: InvariantSeverity::Critical,
            bip_reference: None,
            code_references: vec!["src/bitcoin/validation.rs".to_string()],
        },
        ConsensusInvariant {
            id: "tx-outputs".to_string(),
            description: "Transaction must have at least one output".to_string(),
            severity: InvariantSeverity::Critical,
            bip_reference: None,
            code_references: vec!["src/bitcoin/validation.rs".to_string()],
        },
        ConsensusInvariant {
            id: "tx-duplicate-inputs".to_string(),
            description: "Transaction must not have duplicate inputs".to_string(),
            severity: InvariantSeverity::Critical,
            bip_reference: Some("CVE-2018-17144".to_string()),
            code_references: vec!["src/bitcoin/validation.rs".to_string()],
        },
        
        // Block invariants
        ConsensusInvariant {
            id: "block-version".to_string(),
            description: "Block version must be ≥ 1".to_string(),
            severity: InvariantSeverity::Critical,
            bip_reference: None,
            code_references: vec!["src/bitcoin/validation.rs".to_string()],
        },
        ConsensusInvariant {
            id: "block-tx-count".to_string(),
            description: "Block must contain at least one transaction".to_string(),
            severity: InvariantSeverity::Critical,
            bip_reference: None,
            code_references: vec!["src/bitcoin/validation.rs".to_string()],
        },
        ConsensusInvariant {
            id: "block-coinbase".to_string(),
            description: "First transaction in block must be coinbase".to_string(),
            severity: InvariantSeverity::Critical,
            bip_reference: None,
            code_references: vec!["src/bitcoin/validation.rs".to_string()],
        },
        ConsensusInvariant {
            id: "block-multiple-coinbase".to_string(),
            description: "Block must not contain multiple coinbase transactions".to_string(),
            severity: InvariantSeverity::Critical,
            bip_reference: None,
            code_references: vec!["src/bitcoin/validation.rs".to_string()],
        },
        
        // Signature validation invariants
        ConsensusInvariant {
            id: "sig-der-encoding".to_string(),
            description: "ECDSA signatures must use strict DER encoding".to_string(),
            severity: InvariantSeverity::Critical,
            bip_reference: Some("BIP-66".to_string()),
            code_references: vec!["src/bitcoin/validation.rs".to_string()],
        },
        ConsensusInvariant {
            id: "sig-low-s-value".to_string(),
            description: "ECDSA signatures S value must be low".to_string(),
            severity: InvariantSeverity::Critical,
            bip_reference: Some("BIP-62".to_string()),
            code_references: vec!["src/bitcoin/validation.rs".to_string()],
        },
        
        // Taproot invariants
        ConsensusInvariant {
            id: "taproot-signature".to_string(),
            description: "Taproot signatures must follow Schnorr signature specs".to_string(),
            severity: InvariantSeverity::Critical,
            bip_reference: Some("BIP-340, BIP-341".to_string()),
            code_references: vec!["src/bitcoin/taproot.rs".to_string()],
        },
        
        // Script execution invariants
        ConsensusInvariant {
            id: "script-op-limit".to_string(),
            description: "Script must not exceed 201 non-push operations".to_string(),
            severity: InvariantSeverity::Critical,
            bip_reference: None,
            code_references: vec!["src/bitcoin/script.rs".to_string()],
        },
        ConsensusInvariant {
            id: "script-size-limit".to_string(),
            description: "Script size must not exceed 10,000 bytes".to_string(),
            severity: InvariantSeverity::Critical,
            bip_reference: None,
            code_references: vec!["src/bitcoin/script.rs".to_string()],
        },
        ConsensusInvariant {
            id: "script-stack-size".to_string(),
            description: "Script stack size must not exceed 1,000 items".to_string(),
            severity: InvariantSeverity::Critical,
            bip_reference: None,
            code_references: vec!["src/bitcoin/script.rs".to_string()],
        },
    ]
}

/// [CONSENSUS CRITICAL] Check transaction consensus invariants
pub fn verify_transaction_consensus_invariants(
    tx: &Transaction
) -> Result<(), ConsensusError> {
    let checker = BitcoinCoreInvariantChecker::new();
    
    match checker.check_transaction(tx) {
        Ok(()) => Ok(()),
        Err(violation) => {
            Err(ConsensusError::InvariantViolation(format!(
                "Transaction violates consensus invariant {}: {}",
                violation.invariant.id,
                violation.violation_description
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::{Transaction, TxIn, TxOut};
    
    /// [SECURITY SENSITIVE] Test basic invariant violations
    #[test]
    fn test_basic_invariant_violations() {
        let checker = BitcoinCoreInvariantChecker::new();
        
        // Create invalid transaction (empty)
        let invalid_tx = Transaction {
            version: 1,
            lock_time: bitcoin::LockTime::ZERO,
            input: vec![],
            output: vec![],
        };
        
        // Check should fail
        let result = checker.check_transaction(&invalid_tx);
        assert!(result.is_err(), "Empty transaction should violate invariants");
        
        // Check the specific violation
        if let Err(violation) = result {
            assert_eq!(violation.invariant.id, "tx-inputs");
        }
        
        // Create valid minimal transaction
        let minimal_tx = Transaction {
            version: 1,
            lock_time: bitcoin::LockTime::ZERO,
            input: vec![TxIn {
                previous_output: OutPoint::null(),
                script_sig: Script::new(),
                sequence: 0,
                witness: vec![],
            }],
            output: vec![TxOut {
                value: 1000,
                script_pubkey: Script::new(),
            }],
        };
        
        // Check should pass
        let result = checker.check_transaction(&minimal_tx);
        assert!(result.is_ok(), "Minimal valid transaction should pass invariant check");
    }
}
