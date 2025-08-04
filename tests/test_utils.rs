// This file is deprecated and should not be used directly.
// All test utilities have been moved to common/test_utilities.rs
// This file only re-exports from there for backward compatibility.

#[path = "common/mod.rs"] // Added path attribute for the common module
mod common;

// Re-export everything from centralized utilities to eliminate duplicates
pub use common::test_utilities::*;

// Backward compatibility re-exports for existing tests
pub use common::test_utilities::FileTestEnvironment as TestEnvironment;
pub use common::test_utilities::MockFactory;

// Re-export simulation functions for backward compatibility
#[allow(dead_code)] // Provided for test utility compatibility
pub fn simulate_bitcoin_txid() -> String {
    MockFactory::simulate_bitcoin_txid()
}

#[allow(dead_code)] // Provided for test utility compatibility
pub fn simulate_bitcoin_address() -> String {
    MockFactory::simulate_bitcoin_address()
}

#[allow(dead_code)] // Provided for test utility compatibility
pub fn simulate_did() -> String {
    MockFactory::simulate_did()
}

#[allow(dead_code)] // Provided for test utility compatibility
pub fn simulate_rgb_asset_id() -> String {
    MockFactory::simulate_rgb_asset_id()
}

// Set up a file-based test environment for backward compatibility
pub fn setup_test_environment() -> TestEnvironment {
    FileTestEnvironment::new()
}

// Clean up a test environment for backward compatibility
#[allow(dead_code)] // Provided for test utility compatibility
pub fn cleanup_test_environment(env: &TestEnvironment) {
    env.cleanup();
}

// Re-export transaction creation for backward compatibility
#[allow(dead_code)] // Provided for test utility compatibility
pub fn create_test_transaction() -> bitcoin::Transaction {
    TestTransactionFactory::create_dummy_transaction()
}

#[allow(dead_code)] // Provided for test utility compatibility
pub fn create_test_transaction_batch(size: usize) -> Vec<bitcoin::Transaction> {
    TestTransactionFactory::create_dummy_transaction_batch(size)
}
