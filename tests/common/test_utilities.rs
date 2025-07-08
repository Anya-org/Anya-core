//! Centralized test utilities for Anya Core
//!
//! This module provides common test utilities to eliminate duplicates across test files.

use bitcoin::{
    absolute::LockTime,               // For transaction lock times
    hashes::Hash as BitcoinHashTrait, // Trait for .hash() method, aliased to avoid conflict if Hash struct exists
    transaction::Version,             // For Transaction.version
    Amount,                           // For TxOut.value
    OutPoint,
    ScriptBuf, // ScriptBuf for script_pubkey, script_sig
    Sequence,  // For TxIn.sequence
    Transaction,
    TxIn,
    TxOut,
    Witness, // Core types
};
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use std::collections::HashMap;

/// Common test transaction factory
pub struct TestTransactionFactory;

impl TestTransactionFactory {
    /// Alias for create_dummy_transaction to match test expectations
    pub fn create_simple() -> Transaction {
        Self::create_dummy_transaction()
    }
    /// Create a dummy transaction for testing
    /// This replaces all duplicate `create_dummy_transaction()` functions
    pub fn create_dummy_transaction() -> Transaction {
        Transaction {
            version: Version(2),       // Corrected type
            lock_time: LockTime::ZERO, // Should resolve to absolute::LockTime::ZERO
            input: vec![],
            output: vec![],
        }
    }

    /// Create a batch of dummy transactions
    /// This replaces duplicate batch creation logic
    pub fn create_dummy_transaction_batch(size: usize) -> Vec<Transaction> {
        (0..size)
            .map(|_| Self::create_dummy_transaction())
            .collect()
    }

    /// Create a transaction with specific properties for testing
    #[allow(dead_code)]
    pub fn create_test_transaction_with_properties(
        version: i32,
        input_count: usize,
        output_count: usize,
    ) -> Transaction {
        let inputs = (0..input_count)
            .map(|_| TxIn {
                previous_output: OutPoint::null(),
                script_sig: ScriptBuf::new(), // Corrected type
                sequence: Sequence(0),        // Corrected type
                witness: Witness::new(),      // Corrected type for empty witness
            })
            .collect();

        let outputs = (0..output_count)
            .map(|i| TxOut {
                value: Amount::from_sat((i + 1) as u64 * 1000), // Corrected type
                script_pubkey: ScriptBuf::new(),                // Corrected type
            })
            .collect();

        Transaction {
            version: Version(version), // Corrected type
            lock_time: LockTime::ZERO, // Should resolve to absolute::LockTime::ZERO
            input: inputs,
            output: outputs,
        }
    }
}

/// Common test environment setup
pub struct TestEnvironmentFactory;

impl TestEnvironmentFactory {
    /// Alias for create_standard_environment to match test expectations
    #[allow(dead_code)]
    pub fn new_basic() -> TestEnvironment {
        Self::create_standard_environment()
    }
    /// Create a standard test environment
    /// This replaces duplicate TestEnvironment::new() patterns
    pub fn create_standard_environment() -> TestEnvironment {
        TestEnvironment {
            config: Default::default(),
            state: Default::default(),
        }
    }

    /// Create a test environment with specific configuration
    pub fn create_environment_with_config(config: TestConfig) -> TestEnvironment {
        TestEnvironment {
            config,
            state: Default::default(),
        }
    }
}

/// Common test configuration
#[derive(Debug, Clone, Default)]
pub struct TestConfig {
    pub enable_hardware_optimization: bool,
    pub enable_bitcoin_validation: bool,
    pub enable_dao_features: bool,
    pub batch_size: usize,
}

/// Common test environment state
#[derive(Debug, Default)]
pub struct TestEnvironment {
    pub config: TestConfig,
    pub state: HashMap<String, String>,
}

impl TestEnvironment {
    pub fn new() -> Self {
        TestEnvironmentFactory::create_standard_environment()
    }

    pub fn with_config(config: TestConfig) -> Self {
        TestEnvironmentFactory::create_environment_with_config(config)
    }
}

/// File-based test environment for tests requiring temporary files
#[derive(Debug)]
pub struct FileTestEnvironment {
    pub test_dir: std::path::PathBuf,
}

impl FileTestEnvironment {
    /// Create a new file-based test environment with a temporary directory
    pub fn new() -> Self {
        use std::fs;

        // Create a temporary directory for the test
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let test_dir = std::env::temp_dir().join(format!("anya_test_{}", timestamp));

        // Create the directory if it doesn't exist
        if !test_dir.exists() {
            fs::create_dir_all(&test_dir).expect("Failed to create test directory");
        }

        Self { test_dir }
    }

    /// Clean up the test environment
    pub fn cleanup(&self) {
        use std::fs;
        if self.test_dir.exists() {
            fs::remove_dir_all(&self.test_dir).expect("Failed to clean up test directory");
        }
    }
}

/// Common mock factory for tests
/// Common mock factory for tests
pub struct MockFactory;

impl MockFactory {
    /// Create mock secp256k1 context for testing
    pub fn create_mock_secp_context() -> Secp256k1<secp256k1::All> {
        Secp256k1::new()
    }

    /// Simulate a Bitcoin transaction ID for testing
    pub fn simulate_bitcoin_txid() -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        format!("txid{:x}", timestamp)
    }

    /// Simulate a Bitcoin address for testing
    pub fn simulate_bitcoin_address() -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        format!("bc1q{:x}qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq", timestamp % 10000)
    }

    /// Simulate a DID (Decentralized Identifier) for testing
    pub fn simulate_did() -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        format!("did:key:z{:x}", timestamp)
    }

    /// Simulate an RGB asset ID for testing
    pub fn simulate_rgb_asset_id() -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        format!("rgb1{:x}", timestamp)
    }

    /// Create mock key pair for testing
    pub fn create_mock_keypair() -> (SecretKey, PublicKey) {
        let secp = Self::create_mock_secp_context();
        let secret_key = SecretKey::from_slice(&[0x42; 32]).expect("Valid key");
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        (secret_key, public_key)
    }

    /// Create mock Oracle data for DLC testing
    pub fn create_mock_oracle_data(
        count: usize,
    ) -> Vec<(String, secp256k1::ecdsa::Signature, PublicKey)> {
        let secp = Self::create_mock_secp_context();
        let (secret_key, public_key) = Self::create_mock_keypair();

        (0..count)
            .map(|i| {
                let outcome = format!("outcome-{}", i);
                // Use the imported BitcoinHashTrait for the .hash() method
                let outcome_hash = bitcoin::hashes::sha256::Hash::hash(outcome.as_bytes());
                let message = Message::from_digest_slice(&outcome_hash[..]).expect("Valid message");
                let signature = secp.sign_ecdsa(&message, &secret_key);
                (outcome, signature, public_key)
            })
            .collect()
    }
}

/// Common assertion helpers
pub struct TestAssertions;

impl TestAssertions {
    /// Assert that a transaction is valid (placeholder for real logic)
    pub fn assert_transaction_valid(tx: &Transaction) -> bool {
        // In real tests, add actual validation logic
        !tx.input.is_empty() || !tx.output.is_empty() || tx.version == Version(2)
    }
    /// Assert that two transaction validation results match (for consensus compliance)
    pub fn assert_consensus_compliance<T, E>(
        standard_result: Result<T, E>,
        optimized_result: Result<T, E>,
        context: &str,
    ) where
        E: std::fmt::Debug,
    {
        assert_eq!(
            standard_result.is_ok(),
            optimized_result.is_ok(),
            "Consensus compliance failed for {}: standard={:?}, optimized={:?}",
            context,
            standard_result.is_err(),
            optimized_result.is_err()
        );
    }

    /// Assert performance improvement while maintaining correctness
    pub fn assert_performance_improvement(
        baseline_duration: std::time::Duration,
        optimized_duration: std::time::Duration,
        min_improvement_percent: f64,
        context: &str,
    ) {
        let improvement = (baseline_duration.as_secs_f64() - optimized_duration.as_secs_f64())
            / baseline_duration.as_secs_f64()
            * 100.0;

        assert!(
            improvement >= min_improvement_percent,
            "Performance improvement insufficient for {}: got {:.2}%, expected >= {:.2}%",
            context,
            improvement,
            min_improvement_percent
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_factory() {
        let tx = TestTransactionFactory::create_dummy_transaction();
        assert_eq!(tx.version, Version(2)); // Corrected type
        assert_eq!(tx.input.len(), 0);
        assert_eq!(tx.output.len(), 0);
    }

    #[test]
    fn test_batch_creation() {
        let batch = TestTransactionFactory::create_dummy_transaction_batch(5);
        assert_eq!(batch.len(), 5);
    }

    #[test]
    fn test_environment_factory() {
        let env = TestEnvironment::new();
        assert!(!env.config.enable_hardware_optimization);

        let config = TestConfig {
            enable_hardware_optimization: true,
            batch_size: 256,
            ..Default::default()
        };
        let env_with_config = TestEnvironment::with_config(config);
        assert!(env_with_config.config.enable_hardware_optimization);
        assert_eq!(env_with_config.config.batch_size, 256);
    }
}
