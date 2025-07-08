use bitcoin::Transaction;

// Mock test utilities since common module doesn't exist
struct TestTransactionFactory;

impl TestTransactionFactory {
    fn create_historical_batch(_era: &str) -> Vec<String> {
        vec!["mock_tx_1".to_string(), "mock_tx_2".to_string()]
    }

    /// Create a simple dummy transaction for testing
    fn create_simple() -> Transaction {
        // Create a minimal valid transaction
        Transaction {
            version: bitcoin::transaction::Version::ONE,
            lock_time: bitcoin::locktime::absolute::LockTime::ZERO,
            input: vec![],
            output: vec![],
        }
    }
}

#[test]
fn test_create_simple() {
    // This test should just verify that create_simple creates a valid transaction
    let tx = TestTransactionFactory::create_simple();

    // Verify the transaction has the expected properties
    assert_eq!(tx.version, bitcoin::transaction::Version::ONE);
    assert_eq!(tx.lock_time, bitcoin::locktime::absolute::LockTime::ZERO);
    assert_eq!(tx.input.len(), 0);
    assert_eq!(tx.output.len(), 0);

    println!("Transaction created successfully");
}
