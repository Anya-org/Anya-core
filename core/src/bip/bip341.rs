// BIP-341 (Taproot) Implementation
// [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
//
// Implements BIP-341 (Taproot) verification logic according to
// Bitcoin Development Framework v2.5 requirements

use bitcoin::secp256k1::{Secp256k1, XOnlyPublicKey};
use bitcoin::{ScriptBuf, TapLeafHash, TapNodeHash, TapTweakHash};
use bitcoin::taproot::{TapBranchHash, TapLeaf, TapTree, LeafVersion, TaprootBuilder, ControlBlock};
use std::str::FromStr;
use anyhow::{Result, bail, Context};

/// TaprootVerifier provides methods for verifying Taproot constructions
/// according to BIP-341 specification
pub struct TaprootVerifier {
    secp: Secp256k1<bitcoin::secp256k1::All>,
}

impl TaprootVerifier {
    /// Create a new TaprootVerifier with a secp256k1 context
    pub fn new() -> Self {
        Self {
            secp: Secp256k1::new(),
        }
    }
    
    /// Verify a key path spend for a Taproot output
    pub fn verify_key_path_spend(&self, output_key: XOnlyPublicKey, internal_key: &XOnlyPublicKey) -> Result<bool> {
        // For key path spending, we need to verify that the output key is derived from the internal key
        // This involves checking that output_key = internal_key + H(internal_key || merkle_root)
        
        // Since we don't have the merkle root directly in this case (key path only),
        // we need to compute all possible tweaks and see if any matches
        
        // Simplest case: no scripts, just key path spending
        let tweak = TapTweakHash::from_key_and_tweak(*internal_key, []);
        let (tweaked_key, _parity) = internal_key.tap_tweak(&self.secp, tweak);
        
        if tweaked_key == output_key {
            return Ok(true);
        }
        
        // For more complex scenarios, we'd need additional information about the scripts
        // to reconstruct the merkle root
        
        // This is a simplified version; production code would need more checks
        Ok(false)
    }
    
    /// Verify a script path spend for a Taproot output
    pub fn verify_script_path_spend(
        &self,
        output_key: XOnlyPublicKey,
        script: &ScriptBuf,
        control_block: &ControlBlock,
        leaf_version: LeafVersion,
    ) -> Result<bool> {
        // Extract internal key from control block
        let internal_key = control_block.internal_key;
        
        // Create leaf from script
        let leaf = TapLeaf::new(leaf_version, script.clone());
        let leaf_hash = leaf.tap_leaf_hash();
        
        // Verify the TapPath in the control block
        let merkle_root = self.compute_merkle_root(leaf_hash, control_block)?;
        
        // Compute the tweak
        let tweak = TapTweakHash::from_key_and_merkle_root(internal_key, merkle_root);
        
        // Compute the expected output key
        let (expected_output_key, _parity) = internal_key.tap_tweak(&self.secp, tweak);
        
        // Verify output key matches
        if expected_output_key != output_key {
            bail!("Output key verification failed for script path spend");
        }
        
        Ok(true)
    }
    
    /// Compute taproot output key from internal key and tweak
    pub fn compute_taproot_output_key(
        &self,
        internal_key: &XOnlyPublicKey,
        tweak: &[u8],
    ) -> Result<(XOnlyPublicKey, bool)> {
        // Convert tweak to scalar
        let tweak = bitcoin::secp256k1::Scalar::from_be_bytes(tweak.try_into()?)?;
        
        // Compute tweaked key
        Ok(internal_key.tap_tweak(&self.secp, tweak))
    }
    
    /// Compute merkle root from leaf hash and control block
    fn compute_merkle_root(&self, leaf_hash: TapLeafHash, control_block: &ControlBlock) -> Result<TapNodeHash> {
        let mut current = TapNodeHash::from_inner(leaf_hash.into_inner());
        
        // Traverse the path and compute the root
        for node in &control_block.merkle_branch {
            // In a Taproot tree, if current < node, then hash(current || node), otherwise hash(node || current)
            let (first, second) = if current.as_ref() < node.as_ref() {
                (current.as_ref(), node.as_ref())
            } else {
                (node.as_ref(), current.as_ref())
            };
            
            // Concatenate and hash
            let mut concat = [0u8; 64];
            concat[..32].copy_from_slice(first);
            concat[32..].copy_from_slice(second);
            
            // Update current with the branch hash
            current = TapNodeHash::from_inner(bitcoin::hashes::sha256::Hash::hash(&concat).into_inner());
        }
        
        Ok(current)
    }
    
    /// Verify a Taproot address derivation
    pub fn verify_address_derivation(
        &self,
        internal_key: &XOnlyPublicKey,
        scripts: &[ScriptBuf],
        expected_address: &str,
    ) -> Result<bool> {
        // Create Taproot tree
        let builder = TaprootBuilder::new();
        let mut builder = builder;
        
        // Add scripts to tree
        for (i, script) in scripts.iter().enumerate() {
            let leaf = TapLeaf::new(LeafVersion::TapScript, script.clone());
            builder = builder.add_leaf(i as u8, leaf)?;
        }
        
        // Finalize tree
        let tap_tree = builder.finalize(&self.secp, *internal_key)?;
        
        // Get output key
        let output_key = tap_tree.output_key();
        
        // Create Taproot output address
        let address = bitcoin::Address::p2tr(&self.secp, output_key, None, bitcoin::Network::Bitcoin);
        
        // Verify address matches expected
        Ok(address.to_string() == expected_address)
    }
} 