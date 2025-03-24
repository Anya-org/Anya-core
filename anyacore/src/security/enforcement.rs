#[ais3]
pub struct ComplianceGuard {
    rng: OsRng,
    #[bip341]
    hsm: HsmConnection,
}

impl ComplianceGuard {
    pub fn generate_key(&mut self) -> Result<SecretKey> {
        let mut bytes = [0u8; 32];
        self.rng.fill_bytes(&mut bytes);
        self.hsm.store_key(bytes)
    }

    #[ais3]
    pub fn constant_time_compare(&self, a: &[u8], b: &[u8]) -> bool {
        subtle::ConstantTimeEq::ct_eq(a, b).into()
    }
} 