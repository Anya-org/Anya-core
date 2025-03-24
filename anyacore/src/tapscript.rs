#[bip342]
pub struct TapscriptValidator {
    #[ais3]
    hsm: HsmConnection,
}

impl TapscriptValidator {
    pub fn validate_script(&self, script: &Script) -> Result<()> {
        let ctx = secp256k1::Secp256k1::new();
        let control_block = script.control_block().ok_or(Error::InvalidTapscript)?;
        let (output_key, _) = control_block.recover_public_key(&ctx, script)?;
        self.hsm.verify_taproot_commitment(output_key)
    }
} 