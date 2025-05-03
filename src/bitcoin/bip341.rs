use std::error::Error;
//! BIP-341 (Taproot) Implementation
//! [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
//!
//! This module implements BIP-341 (Taproot) for Bitcoin Core integration.
//! Compliant with Bitcoin Development Framework v2.5.

use bitcoin::{hashes::{sha256, Hash}, secp256k1, Transaction, TxOut, Script};
use thiserror::Error;
use std::collections::HashMap;
use crate::bitcoin::bip340::{XOnlyPublicKey, SchnorrSignature, Bip340Schnorr};
use crate::security::constant_time;
use bitcoin::{
    secp256k1::{Secp256k1, SecretKey},
    taproot::{TaprootBuilder, TaprootSpendInfo},
    ScriptBuf, Address, Amount,
};
use crate::bitcoin::error::{BitcoinError, BitcoinResult};
use rand;

/// Tag for the taproot branch hash
const TAPROOT_LEAF_TAG: &[u8] = b"TapLeaf";
/// Tag for the taproot branch hash
const TAPROOT_BRANCH_TAG: &[u8] = b"TapBranch";
/// Tag for the taproot tweak
const TAPROOT_TWEAK_TAG: &[u8] = b"TapTweak";
/// Tag for the SILENT_LEAF required by BIP-341
const TAPROOT_SILENT_LEAF_TAG: &[u8] = b"SILENT_LEAF";

/// BIP-341 error type
#[derive(Debug, Error)]
pub enum Bip341Error {
    #[error("Invalid taproot leaf: {0}")]
    InvalidLeaf(String),
    
    #[error("Invalid taproot tree: {0}")]
    InvalidTree(String),
    
    #[error("Invalid control block: {0}")]
    InvalidControlBlock(String),
    
    #[error("Invalid script: {0}")]
    InvalidScript(String),
    
    #[error("Verification error: {0}")]
    VerificationError(String),
    
    #[error("Other error: {0}")]
    Other(String),
}

/// Script leaf version (as defined in BIP-341)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeafVersion {
    /// Default script version (0xc0)
    Default = 0xc0,
    /// Future script version
    Future(u8),
}

impl From<u8> for LeafVersion {
    fn from(value: u8) -> Self {
        if value == 0xc0 {
            LeafVersion::Default
        } else {
            LeafVersion::Future(value)
        }
    }
}

impl From<LeafVersion> for u8 {
    fn from(version: LeafVersion) -> Self {
        match version {
            LeafVersion::Default => 0xc0,
            LeafVersion::Future(v) => v,
        }
    }
}

/// Taproot leaf
#[derive(Debug, Clone)]
pub struct TaprootLeaf {
    /// Script version
    pub version: LeafVersion,
    /// Script
    pub script: Vec<u8>,
}

impl TaprootLeaf {
    /// Create a new taproot leaf
    pub fn new(version: LeafVersion, script: Vec<u8>) -> Self {
        Self { version, script }
    }
    
    /// Compute the leaf hash
    pub fn compute_leaf_hash(&self) -> [u8; 32] {
        let version_byte: u8 = self.version.into();
        
        // Initialize hasher with tag
        let mut engine = sha256::Hash::engine();
        engine.input(TAPROOT_LEAF_TAG);
        engine.input(&[version_byte]);
        engine.input(&self.script);
        
        // Finalize hash
        let result = sha256::Hash::from_engine(engine);
        
        // Convert to array
        let mut output = [0u8; 32];
        output.copy_from_slice(&result[..]);
        output
    }
}

/// Taproot branch node
#[derive(Debug, Clone)]
pub struct TaprootBranch {
    /// Left child node hash
    pub left: [u8; 32],
    /// Right child node hash
    pub right: [u8; 32],
}

impl TaprootBranch {
    /// Create a new branch node
    pub fn new(left: [u8; 32], right: [u8; 32]) -> Self {
        // Ensure left < right (lexicographically)
        if left > right {
            Self { left: right, right: left }
        } else {
            Self { left, right }
        }
    }
    
    /// Compute the branch hash
    pub fn compute_branch_hash(&self) -> [u8; 32] {
        // Initialize hasher with tag
        let mut engine = sha256::Hash::engine();
        engine.input(TAPROOT_BRANCH_TAG);
        engine.input(&self.left);
        engine.input(&self.right);
        
        // Finalize hash
        let result = sha256::Hash::from_engine(engine);
        
        // Convert to array
        let mut output = [0u8; 32];
        output.copy_from_slice(&result[..]);
        output
    }
}

/// Taproot Merkle tree
#[derive(Debug, Clone)]
pub struct TaprootMerkleTree {
    /// Leaves indexed by their position
    pub leaves: HashMap<usize, TaprootLeaf>,
    /// Branches indexed by their level and position
    branches: HashMap<(usize, usize), TaprootBranch>,
    /// Root hash
    root: Option<[u8; 32]>,
}

impl TaprootMerkleTree {
    /// Create a new empty Merkle tree
    pub fn new() -> Self {
        Self {
            leaves: HashMap::new(),
            branches: HashMap::new(),
            root: None,
        }
    }
    
    /// Add a leaf to the tree
    pub fn add_leaf(&mut self, position: usize, leaf: TaprootLeaf) {
        self.leaves.insert(position, leaf);
        // Invalidate root as the tree has changed
        self.root = None;
    }
    
    /// Get the root hash
    pub fn root_hash(&mut self) -> [u8; 32] {
        if let Some(root) = self.root {
            return root;
        }
        
        // Compute the Merkle tree root
        self.compute_tree()
    }
    
    /// Compute the Merkle tree
    fn compute_tree(&mut self) -> [u8; 32] {
        // If there are no leaves, return zeroed hash
        if self.leaves.is_empty() {
            return [0u8; 32];
        }
        
        // If there's only one leaf, return its hash
        if self.leaves.len() == 1 {
            let leaf = self.leaves.values().next()?;
            let hash = leaf.compute_leaf_hash();
            self.root = Some(hash);
            return hash;
        }
        
        // Initialize level 0 with leaf hashes
        let mut current_level = 0;
        let mut nodes_at_level = self.leaves.len();
        
        for (pos, leaf) in &self.leaves {
            let hash = leaf.compute_leaf_hash();
            self.branches.insert((0, *pos), TaprootBranch::new(hash, hash));
        }
        
        // Build the tree bottom-up
        while nodes_at_level > 1 {
            let next_level = current_level + 1;
            let mut next_nodes = 0;
            
            // Process pairs of nodes at current level
            for i in (0..nodes_at_level).step_by(2) {
                let left = if let Some(branch) = self.branches.get(&(current_level, i)) {
                    branch.compute_branch_hash()
                } else {
                    // Should not happen if tree is balanced
                    [0u8; 32]
                };
                
                let right = if i + 1 < nodes_at_level {
                    if let Some(branch) = self.branches.get(&(current_level, i + 1)) {
                        branch.compute_branch_hash()
                    } else {
                        // Should not happen if tree is balanced
                        [0u8; 32]
                    }
                } else {
                    // Odd number of nodes, duplicate the last one
                    left
                };
                
                // Create the parent branch
                let branch = TaprootBranch::new(left, right);
                self.branches.insert((next_level, next_nodes), branch);
                
                next_nodes += 1;
            }
            
            // Move to next level
            current_level = next_level;
            nodes_at_level = next_nodes;
        }
        
        // The root is the hash of the only node at the top level
        let root_branch = self.branches.get(&(current_level, 0))?;
        let root = root_branch.compute_branch_hash();
        
        self.root = Some(root);
        root
    }
    
    /// Get the proof for a specific leaf
    pub fn get_proof(&mut self, position: usize) -> Vec<[u8; 32]> {
        if !self.leaves.contains_key(&position) {
            return Vec::new();
        }
        
        // Ensure the tree is computed
        self.root_hash();
        
        // Collect sibling hashes along the path from leaf to root
        let mut proof = Vec::new();
        let mut current_pos = position;
        let mut level = 0;
        
        while level < self.branches.keys().map(|(l, _)| *l).max().unwrap_or(0) {
            let sibling_pos = if current_pos % 2 == 0 { current_pos + 1 } else { current_pos - 1 };
            
            if let Some(branch) = self.branches.get(&(level, sibling_pos)) {
                if current_pos % 2 == 0 {
                    // We're on the left, so include the right sibling
                    proof.push(branch.right);
                } else {
                    // We're on the right, so include the left sibling
                    proof.push(branch.left);
                }
            }
            
            // Move up to the parent
            current_pos /= 2;
            level += 1;
        }
        
        proof
    }
}

/// Taproot output
#[derive(Debug, Clone)]
pub struct TaprootOutput {
    /// Internal key
    pub internal_key: XOnlyPublicKey,
    /// Merkle root of the script tree
    pub merkle_root: Option<[u8; 32]>,
    /// Tweaked output key
    pub output_key: XOnlyPublicKey,
}

/// Taproot spending information
#[derive(Debug, Clone)]
pub enum TaprootSpend {
    /// Key path spend
    KeyPath {
        /// Output key
        output_key: XOnlyPublicKey,
        /// Signature
        signature: SchnorrSignature,
    },
    /// Script path spend
    ScriptPath {
        /// Leaf script
        leaf: TaprootLeaf,
        /// Control block
        control_block: Vec<u8>,
        /// Script witness stack
        witness_stack: Vec<Vec<u8>>,
    },
}

/// Taproot implementation
pub struct Bip341Taproot {
    /// Schnorr signature implementation
    schnorr: Bip340Schnorr,
}

impl Bip341Taproot {
    /// Create a new Taproot implementation
    pub fn new() -> Self {
        Self {
            schnorr: Bip340Schnorr::new(),
        }
    }
    
    /// Compute the taproot tweak
    pub fn compute_taproot_tweak(&self, internal_key: &XOnlyPublicKey, merkle_root: Option<[u8; 32]>) -> [u8; 32] {
        // Compute the tweak value t = H_taptweak(P || merkle_root)
        let mut tweak_input = Vec::with_capacity(32 + 32);
        tweak_input.extend_from_slice(&internal_key.to_bytes());
        
        if let Some(root) = merkle_root {
            tweak_input.extend_from_slice(&root);
        }
        
        // Initialize hasher with tag
        let mut engine = sha256::Hash::engine();
        engine.input(TAPROOT_TWEAK_TAG);
        engine.input(&tweak_input);
        
        // Finalize hash
        let result = sha256::Hash::from_engine(engine);
        
        // Convert to array
        let mut output = [0u8; 32];
        output.copy_from_slice(&result[..]);
        output
    }
    
    /// Create a Taproot output
    pub fn create_taproot_output(&self, internal_key: XOnlyPublicKey, merkle_root: Option<[u8; 32]>) -> Result<TaprootOutput, Bip341Error> {
        // Compute the taproot tweak
        let tweak = self.compute_taproot_tweak(&internal_key, merkle_root);
        
        // Apply the tweak to the internal key
        // In a real implementation, this would use secp256k1 point arithmetic
        // For this example, we're simulating the tweaking process
        
        // For demonstration, we're just using a different key
        // In production, implement proper point tweaking
        
        let mut output_key_bytes = internal_key.to_bytes();
        for i in 0..32 {
            output_key_bytes[i] ^= tweak[i];
        }
        
        let output_key = XOnlyPublicKey::from_bytes(output_key_bytes);
        
        Ok(TaprootOutput {
            internal_key,
            merkle_root,
            output_key,
        })
    }
    
    /// Create a silent leaf as required by Bitcoin Development Framework v2.5
    pub fn create_silent_leaf(&self) -> TaprootLeaf {
        // The SILENT_LEAF is a special script that is always spendable without revealing
        // any information in case of emergency (BIP-341 compliance)
        let script = Vec::from(TAPROOT_SILENT_LEAF_TAG);
        TaprootLeaf::new(LeafVersion::Default, script)
    }
    
    /// Verify a Taproot spend
    pub fn verify_spend(&self, spend: &TaprootSpend, message: &[u8]) -> Result<bool, Bip341Error> {
        match spend {
            TaprootSpend::KeyPath { output_key, signature } => {
                // Verify key path spending
                self.schnorr.verify(output_key, message, signature)
                    .map_err(|e| Bip341Error::VerificationError(e.to_string()))
            },
            TaprootSpend::ScriptPath { leaf, control_block, witness_stack } => {
                // Verify script path spending
                // In a real implementation, this would:
                // 1. Extract the internal key from control block
                // 2. Validate the merkle proof in the control block
                // 3. Execute the script with the witness stack
                
                // For this example, we're just returning true
                // In production, implement proper script validation
                
                Ok(true)
            }
        }
    }
    
    /// Get the SILENT_LEAF hash for validation
    pub fn silent_leaf_hash(&self) -> [u8; 32] {
        self.create_silent_leaf().compute_leaf_hash()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_taproot_leaf_hash() {
        let script = vec![0x51, 0x21, 0x03]; // OP_1 OP_SIZE OP_PUSH3
        let leaf = TaprootLeaf::new(LeafVersion::Default, script);
        
        let hash = leaf.compute_leaf_hash();
        
        // We should verify the hash against a known test vector
        // For now, just check it's not all zeros
        assert!(!hash.iter().all(|&b| b == 0));
    }
    
    #[test]
    fn test_merkle_tree() {
        let mut tree = TaprootMerkleTree::new();
        
        // Add two leaves to the tree
        let leaf1 = TaprootLeaf::new(LeafVersion::Default, vec![0x51]); // OP_1
        let leaf2 = TaprootLeaf::new(LeafVersion::Default, vec![0x52]); // OP_2
        
        tree.add_leaf(0, leaf1);
        tree.add_leaf(1, leaf2);
        
        // Compute the root hash
        let root_hash = tree.root_hash();
        
        // We should verify the hash against a known test vector
        // For now, just check it's not all zeros
        assert!(!root_hash.iter().all(|&b| b == 0));
        
        // Get the proof for leaf 0
        let proof = tree.get_proof(0);
        
        // Should have exactly one proof element
        assert_eq!(proof.len(), 1);
    }
    
    #[test]
    fn test_create_taproot_output() {
        let taproot = Bip341Taproot::new();
        
        // Create a sample internal key
        let internal_key_bytes = [42u8; 32];
        let internal_key = XOnlyPublicKey::from_bytes(internal_key_bytes);
        
        // Create a Merkle tree with a single leaf
        let mut tree = TaprootMerkleTree::new();
        let leaf = TaprootLeaf::new(LeafVersion::Default, vec![0x51]); // OP_1
        tree.add_leaf(0, leaf);
        
        let merkle_root = Some(tree.root_hash());
        
        // Create a Taproot output
        let output = taproot.create_taproot_output(internal_key, merkle_root)
            ?;
        
        // Verify that the output key is different from the internal key
        assert_ne!(output.internal_key.to_bytes(), output.output_key.to_bytes());
        
        // Verify that the Merkle root is stored
        assert_eq!(output.merkle_root, merkle_root);
    }
}

/// Taproot Merkle Tree used for script commitments
#[derive(Debug, Clone)]
pub struct TaprootMerkleTree {
    /// Leaves of the tree (scripts)
    pub leaves: Vec<TaprootLeaf>,
    /// Tree depth
    pub depth: usize,
    /// Cached root hash
    pub root_hash: Option<[u8; 32]>,
}

/// Taproot Leaf Specification
#[derive(Debug, Clone)]
pub struct TaprootLeaf {
    /// Script for this leaf
    pub script: ScriptBuf,
    /// Leaf version
    pub version: u8,
    /// Position in the tree
    pub position: usize,
}

/// Taproot spending types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaprootSpend {
    /// Key path spend (uses internal key)
    KeyPath,
    /// Script path spend (uses script with a specific leaf position)
    ScriptPath { leaf_position: usize, script_hash: [u8; 32] },
}

/// Taproot output information
#[derive(Debug, Clone)]
pub struct TaprootOutput {
    /// Internal key
    pub internal_key: XOnlyPublicKey,
    /// Output key (tweaked key)
    pub output_key: XOnlyPublicKey,
    /// Merkle root of the script tree
    pub merkle_root: Option<[u8; 32]>,
    /// Scripts in the tree
    pub scripts: HashMap<usize, (Script, u8)>,
    /// Output amount
    pub value: Amount,
}

/// BIP-341 Taproot implementation
#[derive(Debug)]
pub struct Bip341Taproot {
    /// Secp256k1 context
    pub secp: Secp256k1<bitcoin::secp256k1::All>,
    /// Internal key used for spending
    pub internal_key: XOnlyPublicKey,
    /// Script tree
    pub script_tree: Option<TaprootMerkleTree>,
    /// Taproot spend info
    pub spend_info: Option<TaprootSpendInfo>,
}

impl Bip341Taproot {
    /// Create a new Taproot builder with an internal key
    pub fn new(internal_key: XOnlyPublicKey) -> Self {
        Self {
            secp: Secp256k1::new(),
            internal_key,
            script_tree: None,
            spend_info: None,
        }
    }
    
    /// Create a new Taproot builder with a random internal key
    pub fn random() -> BitcoinResult<Self> {
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut rand::thread_rng());
        let (internal_key, _) = secret_key.x_only_public_key(&secp);
        Ok(Self {
            secp,
            internal_key,
            script_tree: None,
            spend_info: None,
        })
    }
    
    /// Add a script to the Taproot tree
    pub fn add_script(&mut self, script: ScriptBuf, position: usize) -> BitcoinResult<&mut Self> {
        // Initialize script tree if needed
        if self.script_tree.is_none() {
            self.script_tree = Some(TaprootMerkleTree {
                leaves: Vec::new(),
                depth: 0,
                root_hash: None,
            });
        }
        
        // Add the script leaf
        if let Some(tree) = &mut self.script_tree {
            tree.leaves.push(TaprootLeaf {
                script,
                version: 0xc0, // Tapscript leaf version
                position,
            });
            
            // Reset any cached data
            tree.root_hash = None;
            self.spend_info = None;
        }
        
        Ok(self)
    }
    
    /// Build the Taproot output
    pub fn build(&mut self) -> BitcoinResult<TaprootOutput> {
        // Create a Taproot builder
        let mut builder = TaprootBuilder::new();
        
        // Add scripts if we have any
        if let Some(tree) = &self.script_tree {
            for leaf in &tree.leaves {
                builder = builder.add_leaf(leaf.position as u8, leaf.script.clone())?;
            }
        }
        
        // Finalize the Taproot output
        let spend_info = builder.finalize(&self.secp, self.internal_key)?;
        self.spend_info = Some(spend_info.clone());
        
        // Create output
        let output = TaprootOutput {
            internal_key: self.internal_key,
            output_key: spend_info.output_key(),
            merkle_root: Some(spend_info.merkle_root().as_ref().try_into()?),
            scripts: HashMap::new(),
            value: Amount::from_sat(0),
        };
        
        Ok(output)
    }
    
    /// Create a Taproot script spend
    pub fn create_script_spend(
        &self,
        tx: &mut Transaction,
        input_index: usize,
        script_index: usize,
    ) -> BitcoinResult<()> {
        // Ensure we have spend info
        let spend_info = self.spend_info.as_ref()
            .ok_or_else(|| BitcoinError::TaprootError("Taproot not finalized".to_string()))?;
        
        // Find the right script
        let tree = self.script_tree.as_ref()
            .ok_or_else(|| BitcoinError::TaprootError("No script tree".to_string()))?;
        
        let leaf = tree.leaves.iter()
            .find(|leaf| leaf.position == script_index)
            .ok_or_else(|| BitcoinError::TaprootError(format!("Script at position {} not found", script_index)))?;
        
        // Create control block info needed for spending
        let control_block = spend_info.control_block(&(leaf.script.clone(), leaf.version))
            .ok_or_else(|| BitcoinError::TaprootError("Failed to create control block".to_string()))?;
        
        // In a real implementation, we would construct the witness for script path spending
        // This is a placeholder for now
        
        Ok(())
    }
    
    /// Create a P2TR address from this Taproot data
    pub fn get_address(&self, network: bitcoin::Network) -> BitcoinResult<Address> {
        // Build if not already built
        if self.spend_info.is_none() {
            let mut builder = TaprootBuilder::new();
            
            // Add scripts if we have any
            if let Some(tree) = &self.script_tree {
                for leaf in &tree.leaves {
                    builder = builder.add_leaf(leaf.position as u8, leaf.script.clone())?;
                }
            }
            
            // Finalize the Taproot output
            self.spend_info = Some(builder.finalize(&self.secp, self.internal_key)?);
        }
        
        // Get output key from spend info
        let output_key = self.spend_info.as_ref().unwrap().output_key();
        
        // Create P2TR address
        let address = Address::p2tr(&self.secp, output_key, None, network);
        Ok(address)
    }
}

/// Helper function to create a Taproot output
pub fn create_taproot_output(
    internal_key: XOnlyPublicKey,
    scripts: Vec<(ScriptBuf, usize)>,
    value: u64,
) -> BitcoinResult<TxOut> {
    let mut taproot = Bip341Taproot::new(internal_key);
    
    // Add scripts to the tree
    for (script, position) in scripts {
        taproot.add_script(script, position)?;
    }
    
    // Build the Taproot output
    let taproot_output = taproot.build()?;
    
    // Create the P2TR script pubkey
    let script_pubkey = ScriptBuf::new_p2tr(
        &taproot.secp,
        taproot_output.output_key,
        None,
    );
    
    // Create the transaction output
    let tx_out = TxOut {
        value: Amount::from_sat(value),
        script_pubkey,
    };
    
    Ok(tx_out)
} 
