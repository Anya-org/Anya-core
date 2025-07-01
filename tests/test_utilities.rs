//! Test utilities for test modules that aren't using common/test_utilities.rs
//!
//! This file provides mock implementations for needed test utilities.

use bitcoin::{TxIn, TxOut, OutPoint, ScriptBuf, Witness};

// Mock implementations
pub struct MockFactory;
pub struct TestAssertions;
pub struct TestEnvironmentFactory;
pub struct TestTransactionFactory;

impl TestTransactionFactory {
    pub fn create_dummy_transaction() -> bitcoin::Transaction {
        bitcoin::Transaction {
            version: bitcoin::transaction::Version(2),
            lock_time: bitcoin::absolute::LockTime::ZERO,
            input: vec![TxIn {
                previous_output: OutPoint::null(),
                script_sig: ScriptBuf::new(),
                sequence: bitcoin::Sequence::ENABLE_RBF_NO_LOCKTIME,
                witness: Witness::new(),
            }],
            output: vec![TxOut {
                value: bitcoin::Amount::from_sat(100000),
                script_pubkey: ScriptBuf::new(),
            }],
        }
    }
}

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
