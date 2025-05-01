// Bitcoin Protocol Tests Module
// [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
//
// Module organization for Bitcoin protocol tests according to
// Bitcoin Development Framework v2.5 requirements

/// BIP-341 (Taproot) Compliance Tests
pub mod bip341_compliance;

/// Test runner for all protocol tests
pub mod test_runner;

/// Public API for running protocol tests
pub use test_runner::run_all_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    
    /// Run all Bitcoin protocol tests
    #[test]
    fn test_all_protocol_tests() -> Result<()> {
        let results = test_runner::run_all_tests()?;
        
        // Assert that all tests passed
        assert!(results.values().all(|&success| success), "Some protocol tests failed");
        
        Ok(())
    }
} 