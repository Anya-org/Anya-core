#[ais3]
pub struct DataRedactor {
    #[bip341]
    silent_leaf: [u8; 32],
}

impl DataRedactor {
    pub fn redact_personal_data(&self, data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.update(&self.silent_leaf);
        hex::encode(hasher.finalize())
    }
} 