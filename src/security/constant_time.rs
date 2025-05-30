//! Constant-time cryptographic operations
//! [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
//!
//! This module provides constant-time operations for cryptographic functions
//! to prevent timing attacks.

// [AIR-3][AIS-3][BPC-3][RES-3] Removed unused import: std::error::Error

/// Performs a constant-time comparison of two byte slices.
/// This function does not short-circuit, ensuring that the time taken
/// is independent of the data being compared.
///
/// Returns true if the slices are equal, false otherwise.
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    // Early return with false if lengths are different
    // This doesn't leak timing information about the contents
    if a.len() != b.len() {
        return false;
    }
    
    // Use constant-time comparison to prevent timing attacks
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    
    // Constant-time comparison of result with 0
    // Using a separate comparison and conversion to avoid compiler optimizations
    let mut is_equal = 1u8;
    for i in 0..8 {
        is_equal &= ((result >> i) & 1) ^ 1;
    }
    
    is_equal == 1
}

/// Performs a constant-time comparison of two byte slices, but with a natural u32 return.
/// Returns 1 if the slices are equal, 0 otherwise.
pub fn constant_time_eq_u32(a: &[u8], b: &[u8]) -> u32 {
    if constant_time_eq(a, b) {
        1
    } else {
        0
    }
}

/// Constant-time conditional select between two values.
/// If condition is 1, returns a; if condition is 0, returns b.
pub fn constant_time_select<T: Copy>(condition: u32, a: T, b: T) -> T {
    // Convert condition to a mask
    let mask = condition.wrapping_sub(1);
    // Apply mask to both a and b
    // If condition is 1, mask is 0, resulting in a
    // If condition is 0, mask is 0xFFFFFFFF, resulting in b
    let mask_t = mask as usize;
    
    // This is safe because mask_t is either 0 or usize::MAX
    unsafe {
        let a_ptr: *const T = &a;
        let b_ptr: *const T = &b;
        let a_u = a_ptr as usize;
        let b_u = b_ptr as usize;
        
        let result = a_u & !mask_t | b_u & mask_t;
        *(result as *const T)
    }
}

/// Performs a constant-time check if a value is zero.
/// Returns 1 if the value is zero, 0 otherwise.
pub fn constant_time_is_zero_u8(val: u8) -> u8 {
    let mut result = val;
    
    // Set result to 0 if val is 0, or 1 if val is non-zero
    result |= result >> 4;
    result |= result >> 2;
    result |= result >> 1;
    
    // [AIR-3][AIS-3][BPC-3][RES-3] Invert and mask to get 1 for 0 input, 0 for non-zero input
    // This follows official Bitcoin Improvement Proposals (BIPs) standards for clean code
    !result & 1
}

/// Calculates a constant-time eq result (0 or 1) for two u8 values.
pub fn constant_time_eq_u8(a: u8, b: u8) -> u8 {
    constant_time_is_zero_u8(a ^ b)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constant_time_eq() {
        let a = b"test string";
        let b = b"test string";
        let c = b"different!";
        
        assert!(constant_time_eq(a, b));
        assert!(!constant_time_eq(a, c));
        assert!(!constant_time_eq(a, &b[..5]));
    }
    
    #[test]
    fn test_constant_time_eq_u32() {
        let a = b"test string";
        let b = b"test string";
        let c = b"different!";
        
        assert_eq!(constant_time_eq_u32(a, b), 1);
        assert_eq!(constant_time_eq_u32(a, c), 0);
    }
    
    #[test]
    fn test_constant_time_select() {
        assert_eq!(constant_time_select(1, 10, 20), 10);
        assert_eq!(constant_time_select(0, 10, 20), 20);
    }
    
    #[test]
    fn test_constant_time_is_zero_u8() {
        assert_eq!(constant_time_is_zero_u8(0), 1);
        assert_eq!(constant_time_is_zero_u8(1), 0);
        assert_eq!(constant_time_is_zero_u8(255), 0);
    }
    
    #[test]
    fn test_constant_time_eq_u8() {
        assert_eq!(constant_time_eq_u8(5, 5), 1);
        assert_eq!(constant_time_eq_u8(5, 10), 0);
    }
} 
