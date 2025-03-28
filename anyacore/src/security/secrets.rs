#![feature(edition2021)]
#[derive(Debug, AIS3)]
pub struct SecretsManager {
    hsm: HsmClient,
    rng: rand_core::OsRng,
}

impl SecretsManager {
    #[bip341]
    pub fn new() -> Result<Self> {
        Ok(Self {
            hsm: HsmClient::connect()?,
            rng: rand_core::OsRng,
        })
    }

    #[aip3]
    pub fn generate_key(&mut self, path: &KeyPath) -> Result<SecretKey> {
        let entropy: [u8; 32] = self.rng.gen();
        self.hsm.generate_derived_key(entropy, path)
    }

    #[bpc3]
    pub fn sign(&self, message: &[u8], key: &SecretKey) -> Result<Signature> {
        let ctx = secp256k1::Secp256k1::new();
        let msg = Message::from_slice(message)?;
        Ok(ctx.sign_ecdsa_low_r(&msg, key))
    }

    #[ais3]
    pub fn constant_time_verify(&self, a: &[u8], b: &[u8]) -> bool {
        subtle::ConstantTimeEq::ct_eq(a, b).into()
    }
} 