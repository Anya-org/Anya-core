//! Common test utilities module
//!
//! This module provides centralized test utilities to eliminate duplicates

pub mod test_utilities;

// Re-export the most commonly used items
pub use test_utilities::{
    MockFactory, TestAssertions, TestConfig, TestEnvironment, TestTransactionFactory,
};

// The following sub-modules are commented out because their corresponding files are missing
// pub mod mock_ledger; // Corresponding file mock_ledger.rs seems to be missing
// pub mod network_mocks; // Corresponding file network_mocks.rs seems to be missing

// Re-export key components for easier access in tests
// pub use mock_ledger::*; // Corresponding file mock_ledger.rs seems to be missing
// pub use network_mocks::*; // Corresponding file network_mocks.rs seems to be missing
