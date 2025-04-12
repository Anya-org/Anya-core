const SILENT_LEAF: [u8; 32] = hex!("1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9");

fn verify_silent_leaf(proof: &MerkleProof) -> Result<(), TaprootError> {
    // BIP-341 §4.3 implementation
    verify_merkle_path(proof, SILENT_LEAF)?;
    verify_commitment(proof.tapleaf_hash)?;
    Ok(())
} 