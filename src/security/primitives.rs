use bitcoin::secp256k1::{Secp256k1, SecretKey};
use sha2::{Sha256, Digest};

pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

pub fn derive_taproot_key(seed: &[u8]) -> anyhow::Result<SecretKey> {
    let secp = Secp256k1::new();
    let mut hasher = Sha256::new();
    hasher.update(seed);
    hasher.update(b"TAPROOT-KEY");
    let secret_bytes = hasher.finalize();
    SecretKey::from_slice(&secret_bytes).map_err(|e| anyhow::anyhow!("Invalid key: {}", e))
}

// Additional security primitives...
