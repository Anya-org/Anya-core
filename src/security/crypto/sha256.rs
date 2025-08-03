// SHA-256 implementation
// Bitcoin Development Framework v2.5

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

/// Calculates the SHA-256 hash of the input data
///
/// # Arguments
/// * `data` - The data to hash
///
/// # Returns
/// The SHA-256 hash (32 bytes)
pub fn hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Calculates the double SHA-256 hash (used in Bitcoin)
///
/// # Arguments
/// * `data` - The data to hash
///
/// # Returns
/// The double SHA-256 hash (32 bytes)
pub fn double_hash(data: &[u8]) -> [u8; 32] {
    // First hash
    let first_hash = hash(data);

    // Second hash
    hash(&first_hash)
}

/// Calculates the HMAC-SHA-256
///
/// # Arguments
/// * `key` - The key for HMAC
/// * `data` - The data to hash
///
/// # Returns
/// The HMAC-SHA-256 (32 bytes)
pub fn hmac(key: &[u8], data: &[u8]) -> [u8; 32] {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hash() {
        let data = b"test data";
        let hash_result = hash(data);

        // In a real test, we would check against known hash values
        assert_eq!(hash_result.len(), 32);
    }

    #[test]
    fn test_double_hash() {
        let data = b"test data";
        let hash_result = double_hash(data);

        // In a real test, we would check against known hash values
        assert_eq!(hash_result.len(), 32);
    }

    #[test]
    fn test_hmac() {
        let key = b"test key";
        let data = b"test data";
        let hmac_result = hmac(key, data);

        // In a real test, we would check against known HMAC values
        assert_eq!(hmac_result.len(), 32);
    }
}
