#![feature(edition2021)]
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hexagonal_validation() {
        // Add missing validation code
        let validator = HexValidator::new();
        assert!(validator.check_adapters());
        assert!(validator.check_hsm_integration());
    }
} 