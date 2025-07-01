//! Bitcoin module tests - Validates full alignment with Bitcoin Core principles
//! 
//! This module contains integration tests for Bitcoin functionality with specific
//! focus on ensuring full compliance with Bitcoin's core principles:
//! - Decentralization
//! - Security
//! - Immutability
//! - Privacy

mod validation_test;
mod vm_layer_tests;
mod cross_layer_tests;
mod riscv_tests;
mod riscv_vm_tests;
mod layer3_tests;
mod historical_compatibility_tests;
mod security_tests;  // Comprehensive security tests

// Protocol tests module
pub mod protocol;

// Re-export Bitcoin tests for main test runner
pub use validation_test::*;
pub use cross_layer_tests::*;
pub use historical_compatibility_tests::*;
pub use security_tests::*;  // Make security tests available to runner

// Tests that verify Bitcoin Core principles alignment
pub mod principles {
    //! Tests specifically focused on verifying alignment with Bitcoin Core principles
    
    pub use super::historical_compatibility_tests::test_full_bitcoin_principles_alignment;
    pub use super::historical_compatibility_tests::test_immutability_historical_compatibility;
    pub use super::historical_compatibility_tests::test_immutability_across_hardware_paths;
    
    // Security principle tests
    pub use super::security_tests::test_cve_2010_5139_value_overflow;
    pub use super::security_tests::test_cve_2018_17144_duplicate_inputs;
    pub use super::security_tests::test_differential_fuzzing;
    pub use super::security_tests::test_consensus_invariant_checker;
    pub use super::security_tests::test_hardware_optimizations_consensus;
    pub use super::security_tests::test_timing_side_channels;
    pub use super::security_tests::test_historical_consensus_bugs;
}
