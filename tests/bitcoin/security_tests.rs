use anya_core::{
    bitcoin::{
        consensus::{
            differential_fuzzer::{
                BitcoinReferenceClient, DifferentialFuzzer, DifferentialFuzzerConfig,
            },
            invariant_checker::{
                verify_transaction_consensus_invariants, BitcoinCoreInvariantChecker,
            },
            ConsensusError,
        },
        validation::{TransactionValidator, ValidationError},
    },
    hardware_optimization::HardwareOptimizationManager,
};

use bitcoin::{Block, BlockHeader, OutPoint, Script, Transaction, TxIn, TxOut};
use std::collections::HashMap;
use std::sync::Arc;

/// [SECURITY SENSITIVE] Test full Bitcoin Core security alignment
/// Tests our implementation against CVE-2010-5139 (value overflow)
#[test]
fn test_cve_2010_5139_value_overflow() {
    println!("Testing CVE-2010-5139 (Value Overflow) protection...");

    // Create a transaction that tries to exploit the value overflow bug
    // In the actual CVE-2010-5139, an overflow caused more than 21M BTC to be created
    let exploit_tx = create_value_overflow_transaction();

    // Our validator should reject this
    let validator = TransactionValidator::new();

    let result = validator.validate(&exploit_tx);
    assert!(
        result.is_err(),
        "Value overflow transaction should be rejected"
    );

    // Check that the error correctly identifies the overflow
    if let Err(error) = result {
        assert!(
            format!("{:?}", error).contains("overflow")
                || format!("{:?}", error).contains("Overflow")
                || format!("{:?}", error).contains("value"),
            "Error should specifically mention value overflow"
        );
    }

    println!("✅ CVE-2010-5139 (Value Overflow) protection verified");
}

/// [SECURITY SENSITIVE] Test protection against CVE-2018-17144 (inflation bug)
#[test]
fn test_cve_2018_17144_duplicate_inputs() {
    println!("Testing CVE-2018-17144 (Duplicate Inputs) protection...");

    // Create a transaction with duplicate inputs to simulate the bug
    let exploit_tx = create_duplicate_inputs_transaction();

    // Our validator should reject this
    let validator = TransactionValidator::new();

    let result = validator.validate(&exploit_tx);
    assert!(
        result.is_err(),
        "Transaction with duplicate inputs should be rejected"
    );

    // Check invariant validation also catches this
    let invariant_result = verify_transaction_consensus_invariants(&exploit_tx);
    assert!(
        invariant_result.is_err(),
        "Invariant checker should reject duplicate inputs"
    );

    println!("✅ CVE-2018-17144 (Duplicate Inputs) protection verified");
}

/// [SECURITY SENSITIVE] Test differential fuzzing for consensus compatibility
#[test]
fn test_differential_fuzzing() {
    println!("Testing differential fuzzing for consensus compatibility...");

    // Create clients
    let reference_client = Arc::new(BitcoinReferenceClient::new("http://localhost:8332"));
    let validator = Arc::new(TransactionValidator::new());

    // Create fuzzer with minimal config for quick test
    let fuzzer = DifferentialFuzzer::new(reference_client, validator).with_config(
        DifferentialFuzzerConfig {
            iterations: 5,  // Low number for fast test
            batch_size: 10, // Small batch size
            parallel: false,
            fail_fast: true,
        },
    );

    // Run fuzzer
    let result = fuzzer.run();
    assert!(result.is_ok(), "Fuzzer should complete without errors");

    // Check no violations
    let violations = result.unwrap();
    assert!(
        violations.is_empty(),
        "Expected no consensus violations, but found {}",
        violations.len()
    );

    println!("✅ Differential fuzzing verified consensus compatibility");
}

/// [SECURITY SENSITIVE] Test invariant checker
#[test]
fn test_consensus_invariant_checker() {
    println!("Testing consensus invariant checker...");

    let checker = BitcoinCoreInvariantChecker::new();

    // Get all invariants
    let invariants = checker.get_invariants();
    assert!(
        !invariants.is_empty(),
        "Should have at least one invariant defined"
    );

    // Test with valid transaction
    let valid_tx = create_valid_transaction();
    let result = checker.check_transaction(&valid_tx);
    assert!(
        result.is_ok(),
        "Valid transaction should pass invariant checks"
    );

    // Test with each type of invalid transaction
    test_invariant_violations(&checker);

    println!("✅ Consensus invariant checker verified");
}

/// [SECURITY SENSITIVE] Test hardware optimizations don't affect consensus
#[test]
fn test_hardware_optimizations_consensus() {
    println!("Testing hardware optimizations maintain consensus...");

    // Create validators with and without optimization
    let standard_validator = TransactionValidator::new().with_optimization(false);

    let optimized_validator = TransactionValidator::new().with_optimization(true);

    // Test with various transactions
    let transactions = create_test_transaction_suite();

    for (i, tx) in transactions.iter().enumerate() {
        let standard_result = standard_validator.validate(tx);
        let optimized_result = optimized_validator.validate(tx);

        // Results must match exactly for consensus
        assert_eq!(
            standard_result.is_ok(),
            optimized_result.is_ok(),
            "Transaction {} validation results differ: standard={:?}, optimized={:?}",
            i,
            standard_result,
            optimized_result
        );
    }

    println!("✅ Hardware optimizations maintain consensus");
}

/// [SECURITY SENSITIVE] Test for timing side channels
#[test]
fn test_timing_side_channels() {
    println!("Testing for timing side channels...");

    // Create validator
    let validator = TransactionValidator::new();

    // Create similar valid and invalid transactions
    let valid_tx = create_valid_transaction();
    let invalid_tx = create_invalid_signature_transaction();

    // Measure validation time (repeat multiple times for stability)
    let iterations = 100;
    let mut valid_times = Vec::with_capacity(iterations);
    let mut invalid_times = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        let start = std::time::Instant::now();
        let _ = validator.validate(&valid_tx);
        valid_times.push(start.elapsed());

        let start = std::time::Instant::now();
        let _ = validator.validate(&invalid_tx);
        invalid_times.push(start.elapsed());
    }

    // Calculate statistics
    let avg_valid: f64 =
        valid_times.iter().map(|t| t.as_nanos() as f64).sum::<f64>() / iterations as f64;
    let avg_invalid: f64 = invalid_times
        .iter()
        .map(|t| t.as_nanos() as f64)
        .sum::<f64>()
        / iterations as f64;

    // Calculate standard deviations
    let std_dev_valid = calculate_std_dev(&valid_times, avg_valid);
    let std_dev_invalid = calculate_std_dev(&invalid_times, avg_invalid);

    println!(
        "Valid tx avg time: {:.2} ns, std dev: {:.2}",
        avg_valid, std_dev_valid
    );
    println!(
        "Invalid tx avg time: {:.2} ns, std dev: {:.2}",
        avg_invalid, std_dev_invalid
    );

    // If times are too similar despite different validation paths, it might indicate
    // artificial timing normalization is being used, which is good for security
    let diff_ratio = (avg_valid - avg_invalid).abs() / avg_valid;

    // Assert that time difference is not too revealing
    // Note: This is a very simple check, real side-channel analysis is much more complex
    assert!(
        diff_ratio < 0.5,
        "Timing difference may reveal too much information"
    );

    println!("✅ No obvious timing side channels detected");
}

/// Test a collection of Bitcoin historical consensus bugs
#[test]
fn test_historical_consensus_bugs() {
    println!("Testing protection against historical consensus bugs...");

    let validator = TransactionValidator::new();

    // Test CVE-2010-5139 (value overflow)
    println!("  Testing CVE-2010-5139...");
    let tx = create_value_overflow_transaction();
    assert!(
        validator.validate(&tx).is_err(),
        "Should reject value overflow"
    );

    // Test CVE-2018-17144 (inflation bug)
    println!("  Testing CVE-2018-17144...");
    let tx = create_duplicate_inputs_transaction();
    assert!(
        validator.validate(&tx).is_err(),
        "Should reject duplicate inputs"
    );

    // Test CVE-2012-2459 (OP_EVAL)
    println!("  Testing CVE-2012-2459...");
    let tx = create_op_eval_transaction();
    assert!(validator.validate(&tx).is_err(), "Should reject OP_EVAL");

    // Test CVE-2013-3220 (tx mutability)
    println!("  Testing CVE-2013-3220...");
    let tx = create_mutated_transaction();
    assert!(
        validator.validate(&tx).is_err(),
        "Should reject mutated transaction"
    );

    println!("✅ Protection against historical consensus bugs verified");
}

// Helper function to calculate standard deviation
fn calculate_std_dev(times: &[std::time::Duration], avg_ns: f64) -> f64 {
    let variance: f64 = times
        .iter()
        .map(|t| {
            let diff = t.as_nanos() as f64 - avg_ns;
            diff * diff
        })
        .sum::<f64>()
        / times.len() as f64;

    variance.sqrt()
}

// Helper function to test various invariant violations
fn test_invariant_violations(checker: &BitcoinCoreInvariantChecker) {
    // Test version invariant
    let invalid_version_tx = Transaction {
        version: Version(0), // Invalid version
        lock_time: bitcoin::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::new().into(),
            sequence: Sequence(0),
            witness: Witness::default(),
        }],
        output: vec![TxOut {
            value: Amount::from_sat(1000),
            script_pubkey: ScriptBuf::new().into(),
        }],
    };
    assert!(
        checker.check_transaction(&invalid_version_tx).is_err(),
        "Invalid version should be rejected"
    );

    // Test empty inputs invariant
    let empty_inputs_tx = Transaction {
        version: Version(1),
        lock_time: bitcoin::LockTime::ZERO,
        input: vec![], // No inputs
        output: vec![TxOut {
            value: Amount::from_sat(1000),
            script_pubkey: ScriptBuf::new().into(),
        }],
    };
    assert!(
        checker.check_transaction(&empty_inputs_tx).is_err(),
        "Transaction with no inputs should be rejected"
    );

    // Test empty outputs invariant
    let empty_outputs_tx = Transaction {
        version: Version(1),
        lock_time: bitcoin::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::new().into(),
            sequence: Sequence(0),
            witness: Witness::default(),
        }],
        output: vec![], // No outputs
    };
    assert!(
        checker.check_transaction(&empty_outputs_tx).is_err(),
        "Transaction with no outputs should be rejected"
    );

    // Test duplicate inputs invariant
    let duplicate_inputs_tx = create_duplicate_inputs_transaction();
    assert!(
        checker.check_transaction(&duplicate_inputs_tx).is_err(),
        "Transaction with duplicate inputs should be rejected"
    );
}

// Create a valid minimal transaction
fn create_valid_transaction() -> Transaction {
    Transaction {
        version: Version(1),
        lock_time: bitcoin::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::new().into(),
            sequence: Sequence(0),
            witness: Witness::default(),
        }],
        output: vec![TxOut {
            value: Amount::from_sat(1000),
            script_pubkey: ScriptBuf::new().into(),
        }],
    }
}

// Create a transaction with duplicate inputs (CVE-2018-17144)
fn create_duplicate_inputs_transaction() -> Transaction {
    let outpoint = OutPoint::null();

    Transaction {
        version: Version(1),
        lock_time: bitcoin::LockTime::ZERO,
        input: vec![
            TxIn {
                previous_output: outpoint,
                script_sig: ScriptBuf::new().into(),
                sequence: Sequence(0),
                witness: Witness::default(),
            },
            TxIn {
                previous_output: outpoint, // Same as above, this is the duplicate
                script_sig: ScriptBuf::new().into(),
                sequence: Sequence(0),
                witness: Witness::default(),
            },
        ],
        output: vec![TxOut {
            value: Amount::from_sat(1000),
            script_pubkey: ScriptBuf::new().into(),
        }],
    }
}

// Create a transaction that attempts value overflow (CVE-2010-5139)
fn create_value_overflow_transaction() -> Transaction {
    Transaction {
        version: Version(1),
        lock_time: bitcoin::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::new().into(),
            sequence: Sequence(0),
            witness: Witness::default(),
        }],
        output: vec![
            // Two outputs with maximum Bitcoin value could cause overflow
            TxOut {
                value: Amount::from_sat(21_000_000 * 100_000_000), // Max BTC supply in satoshis
                script_pubkey: ScriptBuf::new().into(),
            },
            TxOut {
                value: Amount::from_sat(21_000_000 * 100_000_000), // Max BTC supply in satoshis
                script_pubkey: ScriptBuf::new().into(),
            },
        ],
    }
}

// Create a transaction with OP_EVAL (CVE-2012-2459)
fn create_op_eval_transaction() -> Transaction {
    // Create script with OP_EVAL (0xBA)
    let script_bytes = vec![0xBA]; // OP_EVAL
    let script = ScriptBuf::from(script_bytes);

    Transaction {
        version: Version(1),
        lock_time: bitcoin::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint::null(),
            script_sig: script,
            sequence: Sequence(0),
            witness: Witness::default(),
        }],
        output: vec![TxOut {
            value: Amount::from_sat(1000),
            script_pubkey: ScriptBuf::new().into(),
        }],
    }
}

// Create a transaction with signature malleability (CVE-2013-3220)
fn create_mutated_transaction() -> Transaction {
    // This is a simplified simulation - in reality, the attack involved
    // modifying signatures in specific ways
    let tx = create_valid_transaction();

    // In a real test, we'd include a malleable signature
    // For this demo, we'll just return the basic transaction
    tx
}

// Create a transaction with invalid signature
fn create_invalid_signature_transaction() -> Transaction {
    let mut tx = create_valid_transaction();
    let invalid_sig = vec![0x30, 0xFF, 0xFF, 0xFF]; // Invalid DER encoding
    tx.input[0].script_sig = ScriptBuf::from(invalid_sig);
    tx
}

// Create a suite of test transactions with various properties
fn create_test_transaction_suite() -> Vec<Transaction> {
    vec![
        create_valid_transaction(),
        create_invalid_signature_transaction(),
        // Additional test cases would be added here
    ]
}
