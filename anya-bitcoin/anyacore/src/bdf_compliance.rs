impl TaprootVerifier {
    pub fn verify_psbt_structure(&self, psbt: &Psbt) -> Result<()> {
        // BIP-341 Validation
        psbt.inputs.iter().try_for_each(|input| {
            input.tap_key_origins.iter().try_for_each(|(_, (leaf_hashes, origins))| {
                origins.iter().try_for_each(|origin| {
                    verify_merkle_proof(origin, leaf_hashes)
                })
            })
        })?;
        Ok(())
    }
} 