use rand::{Rng, RngCore, distributions::Distribution, seq::SliceRandom};
use rand::rngs::OsRng;
use rand::distributions::Standard;
use std::fmt;

/// Error type for random number generation
#[derive(Debug)]
pub enum RandomError {
    Generation(String),
    Range(String),
}

impl fmt::Display for RandomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RandomError::Generation(msg) => write!(f, "Random generation error: {}", msg),
            RandomError::Range(msg) => write!(f, "Random range error: {}", msg),
        }
    }
}

impl std::error::Error for RandomError {}

type Result<T> = std::result::Result<T, RandomError>;

/// Generate random bytes
pub fn random_bytes(size: usize) -> Vec<u8> {
    let mut rng = OsRng;
    let mut bytes = vec![0u8; size];
    rng.fill_bytes(&mut bytes);
    bytes
}

/// Generate a random u64
pub fn random_u64() -> u64 {
    let mut rng = OsRng;
    rng.gen()
}

/// Generate a random u32
pub fn random_u32() -> u32 {
    let mut rng = OsRng;
    rng.gen()
}

/// Generate a random usize
pub fn random_usize() -> usize {
    let mut rng = OsRng;
    rng.gen()
}

/// Generate a random f64
pub fn random_f64() -> f64 {
    let mut rng = OsRng;
    rng.gen()
}

/// Generate a random number in the given range
pub fn random_in_range<T>(min: T, max: T) -> T 
where
    T: PartialOrd + Copy,
    Standard: Distribution<T>,
{
    let mut rng = OsRng;
    loop {
        let val: T = rng.gen();
        if val >= min && val <= max {
            return val;
        }
    }
}

/// Generate a random boolean
pub fn random_bool() -> bool {
    let mut rng = OsRng;
    rng.gen()
}

/// Shuffle a slice in place
pub fn shuffle<T>(slice: &mut [T]) {
    let mut rng = OsRng;
    slice.shuffle(&mut rng);
}

/// Reseed the random number generator (no-op for OsRng)
pub fn reseed() {
    // OsRng doesn't need reseeding as it uses the OS entropy source
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_bytes() {
        let bytes1 = random_bytes(32);
        let bytes2 = random_bytes(32);
        assert_eq!(bytes1.len(), 32);
        assert_eq!(bytes2.len(), 32);
        assert_ne!(bytes1, bytes2); // Should be different
    }

    #[test]
    fn test_random_numbers() {
        let num1 = random_u64();
        let num2 = random_u64();
        assert_ne!(num1, num2); // Very unlikely to be equal
        
        let num3 = random_u32();
        let num4 = random_u32();
        assert_ne!(num3, num4);
    }

    #[test]
    fn test_random_bool() {
        // Just test it doesn't panic
        let _b = random_bool();
    }

    #[test]
    fn test_shuffle() {
        let mut data = vec![1, 2, 3, 4, 5];
        let original = data.clone();
        shuffle(&mut data);
        assert_eq!(data.len(), 5);
        // Contains same elements (may be in same order by chance)
        data.sort();
        assert_eq!(data, original);
    }
}