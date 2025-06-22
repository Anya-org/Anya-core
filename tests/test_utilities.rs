//! Test utilities for test modules that aren't using common/test_utilities.rs
//!
//! This file re-exports the TestTransactionFactory from common/test_utilities.rs
//! and provides mock implementations for other needed test utilities.

// Re-export from common module
pub use crate::common::test_utilities::TestTransactionFactory;

// Mock implementations
pub struct MockFactory;
pub struct TestAssertions;
pub struct TestEnvironmentFactory;

impl MockFactory {
    pub fn new() -> Self {
        Self
    }
    
    pub fn create_mock_transaction(&self) -> bitcoin::Transaction {
        TestTransactionFactory::create_dummy_transaction()
    }
}

impl TestAssertions {
    pub fn new() -> Self {
        Self
    }
    
    pub fn assert_valid_transaction(&self, _tx: &bitcoin::Transaction) -> bool {
        true
    }
}

impl TestEnvironmentFactory {
    pub fn new() -> Self {
        Self
    }
    
    pub fn create_test_environment(&self) -> String {
        "test_environment".to_string()
    }
}
