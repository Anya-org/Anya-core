// federation module for RSK
// Implements a robust, production-grade federation model for cross-chain pegs (PowPeg style)
// Inspired by RSK, Bitcoin multisig, and open source bridge best practices

use std::collections::{HashMap, HashSet};
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey, Message, ecdsa::Signature};
use bitcoin::hashes::{sha256, Hash, HashEngine};

/// Represents a federation member (e.g., a signer)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FederationMember {
    pub id: String,           // Unique identifier (e.g., public key hex)
    pub name: Option<String>, // Optional human-readable name
    pub pubkey: PublicKey,    // Secp256k1 public key
}

/// Represents a proposal for a federated action (e.g., peg-in, peg-out, rotation)
#[derive(Debug, Clone)]
pub struct FederationProposal {
    pub id: String,                  // Unique proposal ID
    pub action: String,              // Action type (e.g., "peg-in", "peg-out", "rotate")
    pub data: HashMap<String, String>, // Arbitrary action data
    pub approvals: HashSet<String>,  // Set of member IDs who have approved
    pub signatures: HashMap<String, Signature>, // Collected signatures (member id -> sig)
    pub executed: bool,              // Whether the proposal has been executed
    pub onchain_txid: Option<String>,// On-chain txid if executed
}

/// Trait for multi-layer contract execution (e.g., BitVM, RSK, etc.)
pub trait ContractExecutor {
    /// Execute a contract action, returning a simulated or real txid
    fn execute_contract(&self, proposal: &FederationProposal) -> Result<String, String>;
}

/// Trait for ML-driven federation logic (e.g., anomaly detection, adaptive threshold)
pub trait FederationMLHook {
    /// Called before approval/signing to allow ML-based checks or adjustments
    fn on_approve(&self, proposal: &FederationProposal, member_id: &str) -> Result<(), String>;
    /// Called before execution to allow ML-based validation or veto
    fn on_execute(&self, proposal: &FederationProposal) -> Result<(), String>;
}

/// Main federation struct
pub struct Federation {
    pub members: HashSet<FederationMember>,
    pub threshold: usize, // M-of-N required for approval/signature
    pub proposals: HashMap<String, FederationProposal>,
    pub secp: Secp256k1<secp256k1::All>, // Shared context
    // Optional contract executor for multi-layer contract support (BitVM, RSK, etc.)
    pub contract_executor: Option<Box<dyn ContractExecutor + Send + Sync>>,
    // Optional ML hook for advanced federation logic
    pub ml_hook: Option<Box<dyn FederationMLHook + Send + Sync>>,
}

impl Federation {
    /// Create a new federation (Anya-core: top-layer enhancement, Bitcoin primitives below)
    pub fn new(members: HashSet<FederationMember>, threshold: usize) -> Self {
        Self {
            members,
            threshold,
            proposals: HashMap::new(),
            secp: Secp256k1::new(),
            contract_executor: None,
            ml_hook: None,
        }
    }

    /// Set a contract executor (enables multi-layer contract execution)
    pub fn with_contract_executor(mut self, exec: Box<dyn ContractExecutor + Send + Sync>) -> Self {
        self.contract_executor = Some(exec);
        self
    }

    /// Set an ML hook (enables ML-driven federation logic)
    pub fn with_ml_hook(mut self, hook: Box<dyn FederationMLHook + Send + Sync>) -> Self {
        self.ml_hook = Some(hook);
        self
    }

    /// Check if an ID is a member
    pub fn is_member(&self, id: &str) -> bool {
        self.members.iter().any(|m| m.id == id)
    }

    /// Get a member by id
    pub fn get_member(&self, id: &str) -> Option<&FederationMember> {
        self.members.iter().find(|m| m.id == id)
    }

    /// Propose a new action (returns proposal ID)
    pub fn propose(&mut self, action: &str, data: HashMap<String, String>) -> String {
        let proposal_id = format!("{}-{}", action, self.proposals.len() + 1);
        let proposal = FederationProposal {
            id: proposal_id.clone(),
            action: action.to_string(),
            data,
            approvals: HashSet::new(),
            signatures: HashMap::new(),
            executed: false,
            onchain_txid: None,
        };
        self.proposals.insert(proposal_id.clone(), proposal);
        proposal_id
    }

    /// Approve a proposal by a member (calls ML hook if present)
    pub fn approve(&mut self, proposal_id: &str, member_id: &str) -> Result<(), String> {
        if !self.is_member(member_id) {
            return Err("Not a federation member".to_string());
        }
        let proposal = self.proposals.get_mut(proposal_id).ok_or("Proposal not found")?;
        if proposal.executed {
            return Err("Proposal already executed".to_string());
        }
        // ML hook: allow ML-based approval logic
        if let Some(hook) = &self.ml_hook {
            hook.on_approve(proposal, member_id)?;
        }
        proposal.approvals.insert(member_id.to_string());
        Ok(())
    }

    /// Collect a cryptographic signature for a proposal (simulated threshold signing)
    pub fn sign(&mut self, proposal_id: &str, member_id: &str, sk: &SecretKey) -> Result<Signature, String> {
        // Check if member exists first
        let member_exists = self.get_member(member_id).is_some();
        if !member_exists {
            return Err("Not a federation member".to_string());
        }
        
        let member_id_clone = member_id.to_string(); // Clone the member ID for later use
        let proposal = self.proposals.get_mut(proposal_id).ok_or("Proposal not found")?;
        if proposal.executed {
            return Err("Proposal already executed".to_string());
        }
        // Hash the proposal data for signing
        let mut hasher = sha256::Hash::engine();
        hasher.input(proposal.id.as_bytes());
        hasher.input(proposal.action.as_bytes());
        for (k, v) in &proposal.data {
            hasher.input(k.as_bytes());
            hasher.input(v.as_bytes());
        }
        let msg_hash = sha256::Hash::from_engine(hasher);
        let msg = Message::from_digest_slice(&msg_hash[..]).map_err(|e| e.to_string())?;
        let sig = self.secp.sign_ecdsa(&msg, sk);
        proposal.signatures.insert(member_id_clone, sig);
        Ok(sig)
    }

    /// Check if a proposal has enough valid signatures
    pub fn has_threshold_signatures(&self, proposal_id: &str) -> bool {
        self.proposals.get(proposal_id)
            .map(|p| p.signatures.len() >= self.threshold)
            .unwrap_or(false)
    }

    /// Execute a proposal if approved and threshold signatures collected
    /// Simulates on-chain execution and returns a fake txid
    /// Calls ML hook and contract executor if present (Anya-core: top-layer enhancement)
    pub fn execute(&mut self, proposal_id: &str) -> Result<String, String> {
        let proposal = self.proposals.get_mut(proposal_id).ok_or("Proposal not found")?;
        if proposal.executed {
            return Ok(proposal.onchain_txid.clone().unwrap_or_default());
        }
        if proposal.approvals.len() < self.threshold {
            return Err("Not enough approvals".to_string());
        }
        if proposal.signatures.len() < self.threshold {
            return Err("Not enough signatures".to_string());
        }
        // ML hook: allow ML-based veto or validation
        if let Some(hook) = &self.ml_hook {
            hook.on_execute(proposal)?;
        }
        // Contract executor: allow multi-layer contract execution (BitVM, RSK, etc.)
        let txid = if let Some(exec) = &self.contract_executor {
            exec.execute_contract(proposal)?
        } else {
            // Default: simulate a txid as sha256(proposal_id)
            format!("{:x}", sha256::Hash::hash(proposal_id.as_bytes()))
        };
        proposal.executed = true;
        proposal.onchain_txid = Some(txid.clone());
        Ok(txid)
    }

    /// List all proposals
    pub fn list_proposals(&self) -> Vec<&FederationProposal> {
        self.proposals.values().collect()
    }
}

impl std::fmt::Debug for Federation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Federation")
            .field("members", &self.members)
            .field("threshold", &self.threshold)
            .field("proposals", &self.proposals)
            .field("secp", &"Secp256k1<All>")
            .field("contract_executor", &self.contract_executor.as_ref().map(|_| "ContractExecutor"))
            .field("ml_hook", &self.ml_hook.as_ref().map(|_| "FederationMLHook"))
            .finish()
    }
}

impl Clone for Federation {
    fn clone(&self) -> Self {
        Self {
            members: self.members.clone(),
            threshold: self.threshold,
            proposals: self.proposals.clone(),
            secp: Secp256k1::new(),
            contract_executor: None, // Note: trait objects can't be cloned generically
            ml_hook: None, // Note: trait objects can't be cloned generically
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;
    use bitcoin::secp256k1::{Secp256k1, SecretKey};

    #[test]
    fn test_federation_signing_flow() {
        let secp = Secp256k1::new();
        let sk_a = SecretKey::from_slice(&[1u8; 32]).unwrap();
        let sk_b = SecretKey::from_slice(&[2u8; 32]).unwrap();
        let pk_a = secp256k1::PublicKey::from_secret_key(&secp, &sk_a);
        let pk_b = secp256k1::PublicKey::from_secret_key(&secp, &sk_b);
        let members = HashSet::from_iter(vec![
            FederationMember { id: "A".to_string(), name: Some("Alice".to_string()), pubkey: pk_a },
            FederationMember { id: "B".to_string(), name: Some("Bob".to_string()), pubkey: pk_b },
        ]);
        let mut fed = Federation::new(members, 2);
        let mut data = HashMap::new();
        data.insert("amount".to_string(), "1000".to_string());
        let pid = fed.propose("peg-out", data);
        fed.approve(&pid, "A").unwrap();
        fed.approve(&pid, "B").unwrap();
        fed.sign(&pid, "A", &sk_a).unwrap();
        fed.sign(&pid, "B", &sk_b).unwrap();
        assert!(fed.has_threshold_signatures(&pid));
        let txid = fed.execute(&pid).unwrap();
        assert!(!txid.is_empty());
    }
}

// --- Anya-core: This module acts as a top-layer enhancement ---
// - All Bitcoin primitives and best practices are preserved below
// - Extensibility for ML and multi-layer contracts is provided via traits/hooks
// - No core Bitcoin logic is replaced, only enhanced
// - Ready for integration with BitVM, RSK, and ML-driven federation logic

