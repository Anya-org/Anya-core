//! Secure Random Number Generator Implementation
//! [AIR-1][AIS-1][BPC-1][AIT-1][RES-1]
//!
//! This module provides cryptographically secure random number generation
//! to replace insecure Math.random() or similar implementations.

use rand::{RngCore, SeedableRng, Rng};
use rand_chacha::ChaCha20Rng;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

/// Global secure random number generator
lazy_static! {
    static ref SECURE_RNG: Arc<Mutex<SecureRng>> = Arc::new(Mutex::new(SecureRng::new()));
}

/// Secure random number generator
///
/// This uses ChaCha20 CSPRNG for cryptographically secure random numbers.
/// Seeded from the system's entropy source.
#[derive(Debug)]
pub struct SecureRng {
    rng: ChaCha20Rng,
}

impl SecureRng {
    /// Create a new secure RNG
    pub fn new() -> Self {
        // Create a cryptographically secure RNG seeded from the system entropy source
        let mut seed = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut seed);
        let rng = ChaCha20Rng::from_seed(seed);
        
        Self { rng }
    }
    
    /// Generate random bytes
    pub fn random_bytes(&mut self, len: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; len];
        self.rng.fill_bytes(&mut bytes);
        bytes
    }
    
    /// Generate a random u64
    pub fn random_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }
    
    /// Generate a random u32
    pub fn random_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }
    
    /// Generate a random usize
    pub fn random_usize(&mut self) -> usize {
        self.rng.gen::<usize>()
    }
    
    /// Generate a random f64 between 0.0 and 1.0
    pub fn random_f64(&mut self) -> f64 {
        self.rng.gen::<f64>()
    }
    
    /// Generate a random integer in range [min, max)
    pub fn random_in_range(&mut self, min: i64, max: i64) -> i64 {
        self.rng.gen_range(min..max)
    }
    
    /// Generate a random boolean
    pub fn random_bool(&mut self) -> bool {
        self.rng.gen::<bool>()
    }
    
    /// Shuffle a slice
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        use rand::seq::SliceRandom;
        slice.shuffle(&mut self.rng);
    }
    
    /// Reseed the RNG
    pub fn reseed(&mut self) {
        let mut seed = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut seed);
        self.rng = ChaCha20Rng::from_seed(seed);
    }
}

/// Generate random bytes
pub fn random_bytes(len: usize) -> Vec<u8> {
    match SECURE_RNG.lock() {
        Ok(mut rng) => rng.random_bytes(len),
        Err(_) => vec![0u8; len], // Fallback
    }
}

/// Generate a random u64
pub fn random_u64() -> u64 {
    match SECURE_RNG.lock() {
        Ok(mut rng) => rng.random_u64(),
        Err(_) => 0, // Fallback
    }
}

/// Generate a random u32
pub fn random_u32() -> u32 {
    match SECURE_RNG.lock() {
        Ok(mut rng) => rng.random_u32(),
        Err(_) => 0, // Fallback
    }
}

/// Generate a random usize
pub fn random_usize() -> usize {
    match SECURE_RNG.lock() {
        Ok(mut rng) => rng.random_usize(),
        Err(_) => 0, // Fallback
    }
}

/// Generate a random f64 between 0.0 and 1.0
pub fn random_f64() -> f64 {
    match SECURE_RNG.lock() {
        Ok(mut rng) => rng.random_f64(),
        Err(_) => 0.0, // Fallback
    }
}

/// Generate a random integer in range [min, max)
pub fn random_in_range(min: i64, max: i64) -> i64 {
    match SECURE_RNG.lock() {
        Ok(mut rng) => rng.random_in_range(min, max),
        Err(_) => min, // Fallback
    }
}

/// Generate a random boolean
pub fn random_bool() -> bool {
    match SECURE_RNG.lock() {
        Ok(mut rng) => rng.random_bool(),
        Err(_) => false, // Fallback
    }
}

/// Shuffle a slice
pub fn shuffle<T>(slice: &mut [T]) {
    if let Ok(mut rng) = SECURE_RNG.lock() {
        rng.shuffle(slice);
    }
}

/// Reseed the global RNG
pub fn reseed() {
    if let Ok(mut rng) = SECURE_RNG.lock() {
        rng.reseed();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_random_bytes() {
        let bytes1 = random_bytes(32);
        let bytes2 = random_bytes(32);
        
        // Verify we got the right length
        assert_eq!(bytes1.len(), 32);
        assert_eq!(bytes2.len(), 32);
        
        // Verify the two sets of bytes are different
        assert_ne!(bytes1, bytes2);
    }
    
    #[test]
    fn test_random_in_range() {
        let min = 10;
        let max = 100;
        
        for _ in 0..100 {
            let value = random_in_range(min, max);
            assert!(value >= min);
            assert!(value < max);
        }
    }
    
    #[test]
    fn test_shuffle() {
        let original = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut shuffled = original.clone();
        
        shuffle(&mut shuffled);
        
        // There's a very small chance this could fail if the shuffle
        // happens to produce the original order, but it's extremely unlikely
        // for a length-10 array
        assert_ne!(original, shuffled);
        
        // Verify all elements are still present
        let mut sorted = shuffled.clone();
        sorted.sort();
        assert_eq!(sorted, original);
    }
}
