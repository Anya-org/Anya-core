#[ais3]
pub trait HsmStandard {
    fn sign_psbt(&self, psbt: &mut Psbt) -> Result<()>;
    fn verify_taproot_commitment(&self, output_key: XOnlyPublicKey) -> Result<()>;
    #[ais3]
    fn generate_key(&mut self) -> Result<SecretKey>;
}

// Implement for all HSM types
impl HsmStandard for YubiHsm {
    // ... platform-specific implementations ...
} 