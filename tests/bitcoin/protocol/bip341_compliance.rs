// BIP-341 Compliance Test Suite
// [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
//
// Tests the implementation of BIP-341 (Taproot) features according to
// Bitcoin Development Framework v2.5 requirements

use anyhow::{bail, Result};
use bitcoin::secp256k1::{Secp256k1, XOnlyPublicKey};
use bitcoin::taproot::{LeafVersion, TaprootBuilder, ControlBlock, TapLeaf, TapNodeHash};
use bitcoin::{ScriptBuf, TapTweakHash};
use bitcoin::taproot::{LeafVersion, TaprootBuilder, ControlBlock, TapLeaf, TapNodeHash};
use std::str::FromStr;
use bitcoin::{ScriptBuf, TapLeafHash, TapNodeHash, TapTweakHash};
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
    pub fn verify_key_path_spend(&self, output_key: bitcoin::taproot::TweakedPublicKey, internal_key: &XOnlyPublicKey) -> Result<bool> {
        // Extract internal key from control block
        let internal_key_untweaked: bitcoin::taproot::UntweakedPublicKey = (*internal_key).into();
        
        // Compute tweak
        let tweak = TapTweakHash::from_key_and_tweak(
            internal_key_untweaked,
            None,
        );
        
        // Verify the tweak matches the output key
        let tweaked_key = internal_key_untweaked.tap_tweak(&self.secp, tweak).to_inner();
        
        Ok(tweaked_key == output_key.to_inner())
    }
    
    /// Verify a script path spend for a Taproot output
    pub fn verify_script_path_spend(
        &self,
        output_key: bitcoin::taproot::TweakedPublicKey,
        script: &ScriptBuf,
        control_block: &ControlBlock,
        leaf_version: LeafVersion,
    ) -> Result<bool> {
        // Extract internal key from control block
        let internal_key = control_block.internal_key;
        let internal_key_untweaked: bitcoin::taproot::UntweakedPublicKey = internal_key.into();
        
        // Create leaf from script
        let leaf = TapLeaf::new(leaf_version, script.clone());
        let leaf_hash = leaf.tap_leaf_hash();
        
        // Verify the TapPath in the control block
        let merkle_root = self.compute_merkle_root(leaf_hash, control_block)?;
        
        // Compute the tweak
        let tweak = TapTweakHash::from_key_and_tweak(
            internal_key_untweaked,
            merkle_root,
        );
        
        // Verify the tweak matches the output key
        let tweaked_key = internal_key_untweaked.tap_tweak(&self.secp, tweak).to_inner();
        
        Ok(tweaked_key == output_key.to_inner())
    }
    
    /// Compute taproot output key from internal key and tweak
    pub fn compute_taproot_output_key(
        &self,
        internal_key: &XOnlyPublicKey,
        tweak: &[u8],
    ) -> Result<(XOnlyPublicKey, bool)> {
        // Convert tweak to scalar
        let tweak = bitcoin::secp256k1::Scalar::from_be_bytes(tweak.try_into()?)?;
        
        // Convert internal_key to UntweakedPublicKey for the new API
        let internal_key_untweaked: bitcoin::taproot::UntweakedPublicKey = (*internal_key).into();
        
        // Compute tweaked key
        let tweaked_key = internal_key_untweaked.tap_tweak(&self.secp, tweak);
        
        // Return the XOnlyPublicKey and parity
        Ok((tweaked_key.to_inner(), tweaked_key.parity()))
    }
    
    /// Compute merkle root from leaf hash and control block
    fn compute_merkle_root(&self, leaf_hash: TapLeafHash, control_block: &ControlBlock) -> Result<Option<TapNodeHash>> {
        let mut current = TapNodeHash::from_inner(leaf_hash.to_byte_array());
        
        // Iterate through path and compute merkle root
        for (i, hash) in control_block.merkle_branch().iter().enumerate() {
            // Combine the current hash with the path element
            let first = if control_block.merkle_branch_position(i)? {
                hash.as_ref()
            } else {
                current.as_ref()
            };
            
            let second = if control_block.merkle_branch_position(i)? {
                current.as_ref()
            } else {
                hash.as_ref()
            };
            
            // Concatenate and hash
            let mut concat = [0u8; 64];
            concat[0..32].copy_from_slice(first);
            concat[32..].copy_from_slice(second);
            
            // Update current with the branch hash
            current = TapNodeHash::from_inner(bitcoin::hashes::sha256::Hash::hash(&concat).into_inner());
        }
        
        Ok(Some(current))
    }
    
    /// Verify a Taproot address derivation
    pub fn verify_address_derivation(
        &self,
        internal_key: &XOnlyPublicKey,
        scripts: &[ScriptBuf],
        expected_address: &str,
    ) -> Result<bool> {
        // Create Taproot tree
        let mut builder = TaprootBuilder::new();
        
        // Convert internal_key to UntweakedPublicKey for the new API
        let internal_key_untweaked: bitcoin::taproot::UntweakedPublicKey = (*internal_key).into();
        
        // Add scripts to tree
        for (i, script) in scripts.iter().enumerate() {
            builder = builder.add_leaf(i as u8, script.clone())
                .map_err(|e| anyhow::anyhow!("Failed to add leaf: {:?}", e))?;
        }
        
        // Finalize tree
        let tap_tree = builder.finalize(&self.secp, internal_key_untweaked)
            .map_err(|e| anyhow::anyhow!("Failed to finalize taproot builder: {:?}", e))?;
        
        // Get output key
        let output_key = tap_tree.output_key();
        
        // Create Taproot output address
        let address = bitcoin::Address::p2tr(&self.secp, output_key, None, bitcoin::Network::Bitcoin);
        
        // Verify address matches expected
        Ok(address.to_string() == expected_address)
    }
}

/// Test complete Taproot key path spending flow
#[test]
pub fn test_taproot_key_path_spending() -> Result<()> {
    // Initialize secp context
    let secp = Secp256k1::new();

    // Generate internal key
    let internal_key = XOnlyPublicKey::from_str(
        "93c7378d96518a75448821c4f7c8f4bae7ce60f804d03d1f0628dd5dd0f5de51",
    )?;

    // Create spending conditions
    let script = ScriptBuf::from_hex(
        "5121030681b3e0d62e8455f48c657bf8b2556e1c6c89be232f57f1f53a88b0a9986cc751ae",
    )?;
    
    // Create Taproot tree
    let mut builder = TaprootBuilder::new();
    builder = builder.add_leaf(0, script.clone())
        .map_err(|e| anyhow::anyhow!("Failed to add leaf: {:?}", e))?;
    let tap_tree = builder.finalize(&secp, internal_key)
        .map_err(|e| anyhow::anyhow!("Failed to finalize taproot builder: {:?}", e))?;

    // Get Taproot output key
    let tap_output_key = tap_tree.output_key();

    // Verify tap output key matches expected value
    let expected_key = XOnlyPublicKey::from_str(
        "ee4fe085983462a184015d1f782d6a5f8b9c2b60130aff050ce221aff7cc6b47",
    )?;

    // Convert tap_output_key to XOnlyPublicKey for comparison
    let tap_output_xonly = tap_output_key.to_x_only_public_key();

    if tap_output_xonly != expected_key {
        bail!("Taproot output key doesn't match expected value");
    }

    // Verify TaprootVerifier can validate this spending path
    let verifier = TaprootVerifier::new();
    assert!(verifier.verify_key_path_spend(tap_output_key, &internal_key)?);

    Ok(())
}

/// Test Taproot script path spending flow
#[test]
pub fn test_taproot_script_path_spending() -> Result<()> {
    // Initialize secp context
    let secp = Secp256k1::new();

    // Generate internal key
    let internal_key = XOnlyPublicKey::from_str(
        "93c7378d96518a75448821c4f7c8f4bae7ce60f804d03d1f0628dd5dd0f5de51",
    )?;

    // Create multiple scripts for testing
    let script1 = ScriptBuf::from_hex(
        "5121030681b3e0d62e8455f48c657bf8b2556e1c6c89be232f57f1f53a88b0a9986cc751ae",
    )?;
    let script2 = ScriptBuf::from_hex(
        "5121036e34cc5ee5558b925045f968e834316d8c90c8d0dd850cc3f990d56755abfa0751ae",
    )?;

    // Build complex Taproot tree with multiple spending paths
    let mut builder = TaprootBuilder::new();
    builder = builder.add_leaf(0, script1.clone())
        .map_err(|e| anyhow::anyhow!("Failed to add first leaf: {:?}", e))?;
    builder = builder.add_leaf(1, script2.clone())
        .map_err(|e| anyhow::anyhow!("Failed to add second leaf: {:?}", e))?;
    let tree = builder.finalize(&secp, internal_key)
        .map_err(|e| anyhow::anyhow!("Failed to finalize taproot builder: {:?}", e))?;

    // Get Taproot output key
    let tap_output_key = tree.output_key();

    // Get control block and verify it
    let control_block = tree.control_block(&(script1.clone(), LeafVersion::TapScript))
        .ok_or(anyhow::anyhow!("Failed to get control block"))?;

    // Verify control block is valid
    let verifier = TaprootVerifier::new();
    assert!(verifier.verify_script_path_spend(
        tap_output_key,
        &script1,
        &control_block,
        LeafVersion::TapScript
    )?);

    Ok(())
}

/// Test Taproot multisig with Schnorr signatures
#[test]
pub fn test_taproot_multisig_schnorr() -> Result<()> {
    // Initialize secp context
    let secp = Secp256k1::new();

    // Create multisig script with 2-of-3 signers using Schnorr
    let key1 = XOnlyPublicKey::from_str(
        "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
    )?;
    let key2 = XOnlyPublicKey::from_str(
        "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
    )?;
    let key3 = XOnlyPublicKey::from_str(
        "e493dbf1c10d80f3581e4904930b1404cc6c13900ee0758474fa94abe8c4cd13",
    )?;

    // Create 2-of-3 multisig script
    let script = ScriptBuf::from_hex(
        "202079be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798ac\
         20c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5ac\
         20e493dbf1c10d80f3581e4904930b1404cc6c13900ee0758474fa94abe8c4cd13ac\
         53ae"
    )?;

    // Create internal key for key path spending
    let internal_key = XOnlyPublicKey::from_str(
        "93c7378d96518a75448821c4f7c8f4bae7ce60f804d03d1f0628dd5dd0f5de51",
    )?;

    // Create Taproot tree
    let mut builder = TaprootBuilder::new();
    builder = builder.add_leaf(0, script.clone())
        .map_err(|e| anyhow::anyhow!("Failed to add leaf: {:?}", e))?;
    let tap_tree = builder.finalize(&secp, internal_key)
        .map_err(|e| anyhow::anyhow!("Failed to finalize taproot builder: {:?}", e))?;

    // Get output key
    let output_key = tap_tree.output_key();

    // Verify with TaprootVerifier
    let verifier = TaprootVerifier::new();

    // Get control block
    let control_block = tap_tree.control_block(&(script.clone(), LeafVersion::TapScript))
        .ok_or(anyhow::anyhow!("Failed to get control block"))?;

    // Verify script path
    assert!(verifier.verify_script_path_spend(
        output_key,
        &script,
        &control_block,
        LeafVersion::TapScript
    )?);

    Ok(())
}

/// Test BIP-341 compliance under edge cases
#[test]
pub fn test_taproot_edge_cases() -> Result<()> {
    // Initialize secp context
    let secp = Secp256k1::new();

    // Test empty script
    let empty_script = ScriptBuf::new();
    // Create internal key
    let internal_key = XOnlyPublicKey::from_str(
        "93c7378d96518a75448821c4f7c8f4bae7ce60f804d03d1f0628dd5dd0f5de51",
    )?;

    // Verify we can still create taproot output with empty script
    let mut builder = TaprootBuilder::new();
    builder = builder.add_leaf(0, empty_script.clone())
        .map_err(|e| anyhow::anyhow!("Failed to add empty script leaf: {:?}", e))?;
    let tap_tree = builder.finalize(&secp, internal_key)
        .map_err(|e| anyhow::anyhow!("Failed to finalize taproot builder: {:?}", e))?;
    let output_key = tap_tree.output_key();

    // Verify with TaprootVerifier
    let verifier = TaprootVerifier::new();
    let control_block = tap_tree.control_block(&(empty_script.clone(), LeafVersion::TapScript))
        .ok_or(anyhow::anyhow!("Failed to get control block"))?;

    // Should still verify with empty script
    assert!(verifier.verify_script_path_spend(
        output_key,
        &empty_script,
        &control_block,
        LeafVersion::TapScript
    )?);

    // Test maximum allowed script size
    let max_script = ScriptBuf::from(vec![0x51; 520]); // Just under the limit
    
    // Verify we can create taproot output with max size script
    let mut builder_max = TaprootBuilder::new();
    builder_max = builder_max.add_leaf(0, max_script.clone())
        .map_err(|e| anyhow::anyhow!("Failed to add max script leaf: {:?}", e))?;
    let tap_tree_max = builder_max.finalize(&secp, internal_key)
        .map_err(|e| anyhow::anyhow!("Failed to finalize taproot builder: {:?}", e))?;
    let output_key_max = tap_tree_max.output_key();

    // Get control block
    let control_block_max = tap_tree_max.control_block(&(max_script.clone(), LeafVersion::TapScript))
        .ok_or(anyhow::anyhow!("Failed to get control block"))?;

    // Should verify with max script
    assert!(verifier.verify_script_path_spend(
        output_key_max,
        &max_script,
        &control_block_max,
        LeafVersion::TapScript
    )?);

    Ok(())
}

/// Test Taproot compliance with BIP-341 vectors
#[test]
pub fn test_taproot_compliance_vectors() -> Result<()> {
    // Test vector from BIP-341
    // Test Vector 1: Output key derivation
    let internal_pubkey = XOnlyPublicKey::from_str(
        "d6889cb081036e0faefa3a35157ad71086b123b2b144b649798b494c300a961d",
    )?;
    let merkle_root = TapNodeHash::from_str(
        "53a1f6e454df1aa2776a2814a721372d6258050de330b3c6d10ee8f4e0dda343",
    )?;

    // Compute taptweak using API as of Bitcoin 0.32
    let tweak = TapTweakHash::from_key_and_tweak(
        internal_pubkey.into(),
        Some(merkle_root),
    );

    // Expected output key after tweaking
    let expected_output_key = XOnlyPublicKey::from_str(
        "53a1f6e454df1aa2776a2814a721372d6258050de330b3c6d10ee8f4e0dda343",
    )?;

    // Build verifier and check
    let verifier = TaprootVerifier::new();
    let (output_key, parity) =
        verifier.compute_taproot_output_key(&internal_pubkey, tweak.as_ref())?;

    assert_eq!(
        output_key, expected_output_key,
        "Taproot output key doesn't match expected BIP-341 test vector"
    );

    Ok(())
}
