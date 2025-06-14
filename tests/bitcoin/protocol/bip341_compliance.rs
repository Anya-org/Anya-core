// BIP-341 Compliance Test Suite
// [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
//
// Tests the implementation of BIP-341 (Taproot) features according to
// Bitcoin Development Framework v2.5 requirements

use anyhow::{bail, Context, Result};
use bitcoin::ecdsa::Signature;
use bitcoin::schnorr::SchnorrSignature;
use bitcoin::secp256k1::{Secp256k1, XOnlyPublicKey};
use bitcoin::taproot::{LeafVersion, TapBranchHash, TapLeaf, TapTree, TaprootBuilder};
use bitcoin::{PrivateKey, PublicKey, ScriptBuf, TapLeafHash, TapNodeHash, TapTweakHash};
use core::src::bip::bip341::TaprootVerifier;
use std::str::FromStr;

/// Test complete Taproot key path spending flow
#[test]
fn test_taproot_key_path_spending() -> Result<()> {
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
    let leaf = TapLeaf::new(LeafVersion::TapScript, script);

    // Create Taproot tree
    let tap_tree = TapTree::builder()
        .add_leaf(0, leaf.clone())
        .finalize(&secp, internal_key)?;

    // Get Taproot output key
    let tap_output_key = tap_tree.output_key();

    // Verify tap output key matches expected value
    let expected_key = XOnlyPublicKey::from_str(
        "ee4fe085983462a184015d1f782d6a5f8b9c2b60130aff050ce221aff7cc6b47",
    )?;

    if tap_output_key != expected_key {
        bail!("Taproot output key doesn't match expected value");
    }

    // Verify TaprootVerifier can validate this spending path
    let verifier = TaprootVerifier::new();
    assert!(verifier.verify_key_path_spend(tap_output_key, &internal_key)?);

    Ok(())
}

/// Test Taproot script path spending flow
#[test]
fn test_taproot_script_path_spending() -> Result<()> {
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

    let leaf1 = TapLeaf::new(LeafVersion::TapScript, script1);
    let leaf2 = TapLeaf::new(LeafVersion::TapScript, script2);

    // Build complex Taproot tree with multiple spending paths
    let builder = TaprootBuilder::new();
    let tree = builder
        .add_leaf(0, leaf1.clone())?
        .add_leaf(1, leaf2.clone())?
        .finalize(&secp, internal_key)?;

    // Get Taproot output key
    let tap_output_key = tree.output_key();

    // Get control block and verify it
    let control_block = tree.control_block(&leaf1)?;

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
fn test_taproot_multisig_schnorr() -> Result<()> {
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
        "202079be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798ac".to_string()
            + "20c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5ac"
            + "20e493dbf1c10d80f3581e4904930b1404cc6c13900ee0758474fa94abe8c4cd13ac"
            + "53ae",
    )?;

    let leaf = TapLeaf::new(LeafVersion::TapScript, script);

    // Create internal key for key path spending
    let internal_key = XOnlyPublicKey::from_str(
        "93c7378d96518a75448821c4f7c8f4bae7ce60f804d03d1f0628dd5dd0f5de51",
    )?;

    // Create Taproot tree
    let tap_tree = TapTree::builder()
        .add_leaf(0, leaf.clone())
        .finalize(&secp, internal_key)?;

    // Get output key
    let output_key = tap_tree.output_key();

    // Verify with TaprootVerifier
    let verifier = TaprootVerifier::new();

    // Get control block
    let control_block = tap_tree.control_block(&leaf)?;

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
fn test_taproot_edge_cases() -> Result<()> {
    // Initialize secp context
    let secp = Secp256k1::new();

    // Test empty script
    let empty_script = ScriptBuf::new();
    let leaf = TapLeaf::new(LeafVersion::TapScript, empty_script);

    // Create internal key
    let internal_key = XOnlyPublicKey::from_str(
        "93c7378d96518a75448821c4f7c8f4bae7ce60f804d03d1f0628dd5dd0f5de51",
    )?;

    // Verify we can still create taproot output with empty script
    let tap_tree = TapTree::builder()
        .add_leaf(0, leaf.clone())
        .finalize(&secp, internal_key)?;
    let output_key = tap_tree.output_key();

    // Verify with TaprootVerifier
    let verifier = TaprootVerifier::new();
    let control_block = tap_tree.control_block(&leaf)?;

    // Should still verify with empty script
    assert!(verifier.verify_script_path_spend(
        output_key,
        &empty_script,
        &control_block,
        LeafVersion::TapScript
    )?);

    // Test maximum allowed script size
    let max_script = ScriptBuf::from(vec![0x51; 520]); // Just under the limit
    let max_leaf = TapLeaf::new(LeafVersion::TapScript, max_script.clone());

    // Verify we can create taproot output with max size script
    let tap_tree_max = TapTree::builder()
        .add_leaf(0, max_leaf.clone())
        .finalize(&secp, internal_key)?;
    let output_key_max = tap_tree_max.output_key();

    // Get control block
    let control_block_max = tap_tree_max.control_block(&max_leaf)?;

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
fn test_taproot_compliance_vectors() -> Result<()> {
    // Test vector from BIP-341
    // Test Vector 1: Output key derivation
    let internal_pubkey = XOnlyPublicKey::from_str(
        "d6889cb081036e0faefa3a35157ad71086b123b2b144b649798b494c300a961d",
    )?;
    let merkle_root = TapBranchHash::from_str(
        "53a1f6e454df1aa2776a2814a721372d6258050de330b3c6d10ee8f4e0dda343",
    )?;

    // Compute taptweak
    let tweak = TapTweakHash::from_merkle_root_and_internal(
        merkle_root.as_ref().try_into().expect("Invalid length"),
        internal_pubkey.serialize(),
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
