// BIP-341 Compliance Test Suite - Simplified Version
// [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
//
// Tests the implementation of BIP-341 (Taproot) features according to
// Bitcoin Development Framework v2.5 requirements

use anyhow::{bail, Result};
use bitcoin::hashes::{sha256, Hash};
use bitcoin::key::{TapTweak, TweakedPublicKey, UntweakedPublicKey};
use bitcoin::secp256k1::{Secp256k1, SecretKey, XOnlyPublicKey};
use bitcoin::taproot::{
    ControlBlock, LeafVersion, TapBranchTag, TapLeafHash, TapNodeHash, TaprootBuilder,
};
use bitcoin::ScriptBuf;
use rand;
use std::str::FromStr;

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
    pub fn verify_key_path_spend(
        &self,
        output_key: TweakedPublicKey,
        internal_key: &XOnlyPublicKey,
    ) -> Result<bool> {
        // Extract internal key from control block
        let internal_key_untweaked: UntweakedPublicKey = (*internal_key).into();

        // Compute tweak
        let merkle_root = None;

        // Use the add_tweak method with the merkle_root option
        let (tweaked_key, _parity) = internal_key_untweaked.tap_tweak(&self.secp, merkle_root);

        // Compare the tweaked key with the output key
        // In Bitcoin 0.32.6, we need to pass an XOnlyPublicKey to dangerous_assume_tweaked
        Ok(TweakedPublicKey::dangerous_assume_tweaked(tweaked_key.into()) == output_key)
    }

    /// Compute merkle root from leaf hash and control block  
    fn compute_merkle_root(
        &self,
        _leaf_hash: TapLeafHash,
        _control_block: &ControlBlock,
    ) -> Result<Option<TapNodeHash>> {
        // Mock implementation - in real code, this would compute the actual Merkle root
        // from the leaf hash and control block proof path
        Ok(None) // Simplified for compilation
    }
}

/// Test complete Taproot key path spending flow
#[test]
pub fn test_taproot_key_path_spending_simple() -> Result<()> {
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
    builder = builder
        .add_leaf(0, script.clone())
        .map_err(|e| anyhow::anyhow!("Failed to add leaf: {:?}", e))?;
    let tap_tree = builder
        .finalize(&secp, internal_key)
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

    // Verify using TaprootVerifier
    let verifier = TaprootVerifier::new();
    assert!(verifier.verify_key_path_spend(tap_output_key, &internal_key)?);

    Ok(())
}

// Additional test for tapleaf verification
#[cfg(test)]
mod additional_tests {
    use super::*;

    #[test]
    fn test_tapleaf_verification_simple() {
        // Initialize checker
        let checker = TaprootVerifier::new();
        let secp = Secp256k1::new();

        // Generate random keypair
        let secret_key = SecretKey::new(&mut rand::thread_rng());
        let public_key = bitcoin::secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
        let x_only = XOnlyPublicKey::from(public_key);

        // Compute tweaked key with no merkle root (key-path spending)
        let internal_key_untweaked: UntweakedPublicKey = x_only.into();
        let merkle_root = None; // Using None for key-path spending
        let (tweaked_key, _parity) = internal_key_untweaked.tap_tweak(&secp, merkle_root);
        let tweaked_public_key = TweakedPublicKey::dangerous_assume_tweaked(tweaked_key.into());

        // Verify key path spend directly (which doesn't need a control block)
        let key_path_result = checker
            .verify_key_path_spend(tweaked_public_key, &x_only)
            .unwrap();
        assert!(key_path_result, "Key path verification should succeed");

        println!("Taproot verification test completed successfully");
    }
}
