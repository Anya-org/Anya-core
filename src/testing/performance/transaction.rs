/// Transaction performance testing

use crate::testing::performance::{
    PerformanceTestable, TestConfig, TestResult, PerfTestError, Result, Timer, MetricType
};
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use std::time::Instant;
use chrono;

/// Transaction test configuration
#[derive(Debug, Clone)]
pub struct TransactionTestConfig {
    /// Number of transactions to test
    pub transaction_count: usize,
    
    /// Number of inputs per transaction
    pub inputs_per_tx: usize,
    
    /// Number of outputs per transaction
    pub outputs_per_tx: usize,
    
    /// Transaction value range (min, max)
    pub value_range: (u64, u64),
    
    /// Include signature verification
    pub verify_signatures: bool,
}

impl Default for TransactionTestConfig {
    fn default() -> Self {
        Self {
            transaction_count: 1000,
            inputs_per_tx: 2,
            outputs_per_tx: 2,
            value_range: (1000, 1000000),
            verify_signatures: true,
        }
    }
}

/// Mock transaction for testing
#[derive(Debug, Clone)]
pub struct MockTransaction {
    /// Transaction ID
    pub id: String,
    
    /// Input count
    pub inputs: Vec<MockInput>,
    
    /// Output count
    pub outputs: Vec<MockOutput>,
    
    /// Transaction fee
    pub fee: u64,
    
    /// Timestamp
    pub timestamp: u64,
}

/// Mock transaction input
#[derive(Debug, Clone)]
pub struct MockInput {
    /// Previous transaction ID
    pub prev_tx_id: String,
    
    /// Output index
    pub output_index: u32,
    
    /// Script signature
    pub script_sig: Vec<u8>,
}

/// Mock transaction output
#[derive(Debug, Clone)]
pub struct MockOutput {
    /// Output value
    pub value: u64,
    
    /// Script public key
    pub script_pubkey: Vec<u8>,
}

/// Transaction performance test
pub struct TransactionPerformanceTest {
    /// Test configuration
    config: TransactionTestConfig,
    
    /// Generated transactions
    transactions: Vec<MockTransaction>,
}

impl TransactionPerformanceTest {
    /// Create a new transaction performance test
    pub fn new(config: TransactionTestConfig) -> Self {
        let mut test = Self {
            config,
            transactions: Vec::new(),
        };
        test.generate_transactions();
        test
    }
    
    /// Generate test transactions
    fn generate_transactions(&mut self) {
        let mut rng = thread_rng();
        
        for i in 0..self.config.transaction_count {
            let inputs = (0..self.config.inputs_per_tx)
                .map(|j| MockInput {
                    prev_tx_id: format!("prev_tx_{}_{}", i, j),
                    output_index: rng.gen_range(0..10),
                    script_sig: vec![0u8; rng.gen_range(50..200)],
                })
                .collect();
            
            let outputs = (0..self.config.outputs_per_tx)
                .map(|_| MockOutput {
                    value: rng.gen_range(self.config.value_range.0..=self.config.value_range.1),
                    script_pubkey: vec![0u8; rng.gen_range(20..100)],
                })
                .collect();
            
            let transaction = MockTransaction {
                id: format!("tx_{}", i),
                inputs,
                outputs,
                fee: rng.gen_range(100..10000),
                timestamp: Instant::now().elapsed().as_secs(),
            };
            
            self.transactions.push(transaction);
        }
    }
    
    /// Validate a transaction (simplified validation)
    fn validate_transaction(&self, tx: &MockTransaction) -> bool {
        // Basic validation checks
        if tx.inputs.is_empty() || tx.outputs.is_empty() {
            return false;
        }
        
        // Check that total outputs don't exceed total inputs (simplified)
        let total_output: u64 = tx.outputs.iter().map(|o| o.value).sum();
        let _assumed_input_value = total_output + tx.fee; // Simplified assumption
        
        // Basic range checks
        if tx.fee > 100000 { // Max reasonable fee
            return false;
        }
        
        if self.config.verify_signatures {
            // Simplified signature verification (just check script size)
            for input in &tx.inputs {
                if input.script_sig.is_empty() {
                    return false;
                }
            }
        }
        
        true
    }
    
    /// Process transactions and measure performance
    fn process_transactions(&self) -> (usize, usize) {
        let mut valid_count = 0;
        let mut invalid_count = 0;
        
        for tx in &self.transactions {
            if self.validate_transaction(tx) {
                valid_count += 1;
            } else {
                invalid_count += 1;
            }
        }
        
        (valid_count, invalid_count)
    }
}

impl PerformanceTestable for TransactionPerformanceTest {
    fn run_test(&self, test_config: &TestConfig) -> Result<TestResult> {
        let mut timer = Timer::new();
        let mut metrics = HashMap::new();
        
        // Start timing
        timer.start();
        
        // Run the actual test
        let (valid_count, invalid_count) = self.process_transactions();
        
        // Stop timing
        timer.stop();
        let duration_ms = timer.elapsed_ms()?;
        let duration_secs = timer.elapsed_secs()?;
        
        // Calculate metrics
        let total_transactions = valid_count + invalid_count;
        let throughput = total_transactions as f64 / duration_secs;
        
        metrics.insert("total_transactions".to_string(), total_transactions as f64);
        metrics.insert("valid_transactions".to_string(), valid_count as f64);
        metrics.insert("invalid_transactions".to_string(), invalid_count as f64);
        metrics.insert("throughput_tx_per_sec".to_string(), throughput);
        metrics.insert("avg_processing_time_ms".to_string(), 
            (duration_ms as f64) / total_transactions as f64);
        
        Ok(TestResult {
            name: test_config.name.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            duration_ms,
            metrics,
            metric_types: {
                let mut metric_types = HashMap::new();
                metric_types.insert("throughput_tx_per_sec".to_string(), MetricType::TPS);
                metric_types.insert("avg_processing_time_ms".to_string(), MetricType::LatencyMs);
                metric_types
            },
            parameters: test_config.parameters.clone(),
        })
    }
    
    fn name(&self) -> &str {
        "Transaction Performance Test"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transaction_creation() {
        let config = TransactionTestConfig {
            transaction_count: 10,
            ..Default::default()
        };
        
        let test = TransactionPerformanceTest::new(config);
        assert_eq!(test.transactions.len(), 10);
    }
    
    #[test]
    fn test_transaction_validation() {
        let config = TransactionTestConfig::default();
        let test = TransactionPerformanceTest::new(config);
        
        // Test with a valid transaction
        if let Some(tx) = test.transactions.first() {
            assert!(test.validate_transaction(tx));
        }
    }
    
    #[test]
    fn test_performance_test_run() {
        let config = TransactionTestConfig {
            transaction_count: 100,
            ..Default::default()
        };
        let test = TransactionPerformanceTest::new(config);
        
        let test_config = TestConfig {
            name: "Test Transaction Performance".to_string(),
            iterations: 1,
            warmup_iterations: 0,
            duration_limit_secs: 60,
            parameters: HashMap::new(),
        };
        
        let result = test.run_test(&test_config);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        // Check that the test result has valid data
        assert!(!result.name.is_empty());
        assert!(result.metrics.contains_key("throughput_tx_per_sec"));
    }
}