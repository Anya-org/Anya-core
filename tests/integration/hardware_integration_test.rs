use anya_core::{
    hardware_optimization::{
        HardwareOptimizationManager,
        OptimizableOperation,
        intel::IntelOptimizer,
        work_scheduling::DualCoreWorkScheduler
    },
    monitoring::integrated_system_metrics::IntegratedSystemMetrics,
    bitcoin::{
        validation::TransactionValidator,
        dlc::batch_verification::DLCOracleBatchVerifier
    },
    core::system_awareness::SystemCapabilityMonitor
};

use bitcoin::Transaction;

use crate::common::test_utilities::{
    TestTransactionFactory, TestEnvironmentFactory, MockFactory, TestAssertions
};
use tokio::time::{Duration, Instant};
use std::sync::Arc;

/// Test that the hardware optimization layer integrates correctly with system monitoring
#[tokio::test]
async fn test_hardware_system_integration() {
    // Initialize components
    let (auth_manager, ml_features, metrics, revenue_tracker, web5_integration) = 
        super::system::common::setup_test_system().await;
        
    // Create hardware optimization manager
    let hw_manager = Arc::new(HardwareOptimizationManager::new());
    
    // Create system capability monitor with hardware awareness
    let capability_monitor = SystemCapabilityMonitor::new()
        .with_hardware_optimization(hw_manager.clone());
    
    // Verify system detects hardware capabilities correctly
    let system_capabilities = capability_monitor.get_system_capabilities().await;
    assert!(system_capabilities.hardware_capabilities.is_some());
    
    // Get hardware metrics
    let hw_metrics = system_capabilities
        .hardware_capabilities
        .as_ref()
        .unwrap();
    
    // Verify core system principles alignment
    println!("Verifying hardware alignment with Bitcoin Core principles:");
    println!("- Decentralization: Hardware minimum spec i3-7020U support: {}", 
             hw_metrics.meets_minimum_requirements);
    println!("- Security: Hardware accelerated verification available: {}", 
             hw_metrics.supports_secure_verification);
    println!("- Immutability: Consensus compatible verification: {}", 
             hw_metrics.maintains_consensus);
    println!("- Privacy: Batch verification support: {}", 
             hw_metrics.supports_batch_operations);
    
    // Test integration with system metrics
    let metrics_with_hw = metrics.with_hardware_monitoring(hw_manager.clone());
    
    // Run a system-wide benchmark with hardware awareness
    let benchmark_result = metrics_with_hw
        .benchmark_system_operations()
        .await;
        
    // Verify alignment with minimum hardware requirements
    assert!(benchmark_result.verification_rate >= 1000.0, 
            "Transaction verification rate should meet minimum requirements");
            
    assert!(benchmark_result.batch_capacity >= 64, 
            "Batch verification capacity should meet minimum requirements");
            
    assert!(benchmark_result.memory_usage <= 500 * 1024 * 1024, 
            "Memory usage should be within acceptable limits for minimum hardware");
            
    // Run integrated workflow test with hardware optimization
    let workflow_result = test_optimized_workflow(&hw_manager, &web5_integration).await;
    assert!(workflow_result.is_ok(), "Optimized workflow should succeed");
    assert!(workflow_result.unwrap(), "Optimized workflow should return true");
}

/// Test that DLC operations use hardware optimization appropriately
#[tokio::test]
async fn test_dlc_hardware_integration() {
    // Get hardware optimization manager
    let hw_manager = Arc::new(HardwareOptimizationManager::new());
    
    // Create DLC verifier with hardware awareness
    let dlc_verifier = DLCOracleBatchVerifier::new()
        .with_hardware_optimization(hw_manager.clone());
    
    // Test DLC oracle batch verification with hardware optimization
    let (signatures, outcome_hashes) = create_test_dlc_data(100);
    
    // Verify with hardware optimization
    let start = Instant::now();
    let optimized_result = dlc_verifier
        .verify_batch(&signatures, &outcome_hashes)
        .await;
    let optimized_duration = start.elapsed();
    
    // Verify without hardware optimization
    let start = Instant::now();
    let standard_result = dlc_verifier
        .with_hardware_optimization(None)
        .verify_batch(&signatures, &outcome_hashes)
        .await;
    let standard_duration = start.elapsed();
    
    // Verify results match (consensus compliance)
    assert_eq!(optimized_result.is_ok(), standard_result.is_ok());
    if optimized_result.is_ok() && standard_result.is_ok() {
        assert_eq!(optimized_result.unwrap(), standard_result.unwrap());
    }
    
    // Hardware optimization should be faster
    println!("DLC verification time comparison:");
    println!("- Standard: {:?}", standard_duration);
    println!("- Optimized: {:?}", optimized_duration);
    println!("- Speedup: {:.2}x", standard_duration.as_secs_f64() / optimized_duration.as_secs_f64());
}

/// Test that transaction validation system uses hardware optimization
#[tokio::test]
async fn test_transaction_validation_hardware_integration() {
    // Get hardware optimization manager
    let hw_manager = Arc::new(HardwareOptimizationManager::new());
    
    // Create validators with and without optimization
    let optimized_validator = TransactionValidator::new()
        .with_hardware_optimization(hw_manager.clone());
        
    let standard_validator = TransactionValidator::new()
        .with_hardware_optimization(None);
    
    // Create test transactions
    let transactions = create_test_transactions(1000);
    
    // Verify with hardware optimization
    let start = Instant::now();
    let optimized_result = verify_transactions(&optimized_validator, &transactions);
    let optimized_duration = start.elapsed();
    
    // Verify without hardware optimization
    let start = Instant::now();
    let standard_result = verify_transactions(&standard_validator, &transactions);
    let standard_duration = start.elapsed();
    
    // Verify results match (consensus compliance)
    assert_eq!(optimized_result, standard_result);
    
    // Hardware optimization should be faster
    let speedup = standard_duration.as_secs_f64() / optimized_duration.as_secs_f64();
    println!("Transaction validation time comparison:");
    println!("- Standard: {:?}", standard_duration);
    println!("- Optimized: {:?}", optimized_duration);
    println!("- Speedup: {:.2}x", speedup);
    
    // Ensure minimum performance improvement
    assert!(speedup >= 1.5, "Hardware optimization should provide at least 1.5x speedup");
}

/// Test system resource usage with hardware optimization
#[tokio::test]
async fn test_system_resource_usage() {
    // Get hardware optimization manager
    let hw_manager = Arc::new(HardwareOptimizationManager::new());
    
    // Create system metrics with hardware monitoring
    let metrics = IntegratedSystemMetrics::new()
        .with_hardware_monitoring(hw_manager.clone());
        
    // Run system load test
    let resource_usage = metrics
        .monitor_resource_usage(|| {
            // Simulate system load
            let scheduler = DualCoreWorkScheduler::new();
            
            // Create mixed workload
            for i in 0..1000 {
                let operation = match i % 4 {
                    0 => OptimizableOperation::SchnorrVerification,
                    1 => OptimizableOperation::BatchVerification,
                    2 => OptimizableOperation::SHA256Hashing,
                    _ => OptimizableOperation::TaprootVerification,
                };
                
                scheduler.submit(operation, vec![i as u8; 32], (i % 10) as u8);
            }
            
            // Wait for completion
            std::thread::sleep(Duration::from_secs(2));
        })
        .await;
        
    // Verify resource usage is within acceptable limits for minimum hardware
    assert!(resource_usage.cpu_usage <= 100.0 * 2.0, "CPU usage should be within dual-core limit");
    assert!(resource_usage.memory_usage <= 300 * 1024 * 1024, "Memory usage should be within minimum spec limit");
    
    // Verify hardware optimization is active
    assert!(resource_usage.optimizations_applied > 0, 
           "Hardware optimizations should be applied during workload");
}

// Helper functions

/// Test an optimized workflow that spans multiple components
async fn test_optimized_workflow(
    hw_manager: &Arc<HardwareOptimizationManager>,
    web5_integration: &anya_core::web5::advanced_integration::AdvancedWeb5Integration,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Test data
    let did = create_test_did();
    let message = b"Test message for hardware-optimized verification".to_vec();
    let signature = create_test_signature(&message);
    
    // Verify DID signature with hardware optimization
    let verification_result = web5_integration
        .verify_signature_with_hardware_optimization(
            &did, 
            &message, 
            &signature, 
            hw_manager.clone()
        )
        .await?;
        
    // Process transactions with hardware optimization
    let transactions = create_test_transactions(100);
    let validation_result = validate_transactions_with_hardware(
        &transactions, 
        hw_manager.clone()
    )?;
    
    Ok(verification_result && validation_result)
}

/// Create test DID for testing
fn create_test_did() -> String {
    "did:web5:test:hardware:optimization:12345".to_string()
}

/// Create test signature
fn create_test_signature(message: &[u8]) -> Vec<u8> {
    // Dummy signature for testing
    vec![1, 2, 3, 4, 5]
}

/// Create test transactions
fn create_test_transactions(count: usize) -> Vec<Transaction> {
    (0..count)
        .map(|_| Transaction {
            version: 2,
            lock_time: bitcoin::LockTime::ZERO,
            input: vec![],
            output: vec![],
        })
        .collect()
}

/// Create test DLC data
fn create_test_dlc_data(count: usize) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let signatures = (0..count)
        .map(|i| vec![i as u8; 64])
        .collect();
        
    let outcome_hashes = (0..count)
        .map(|i| vec![(i * 2) as u8; 32])
        .collect();
        
    (signatures, outcome_hashes)
}

/// Verify transactions
fn verify_transactions(
    validator: &TransactionValidator,
    transactions: &[Transaction],
) -> usize {
    let mut valid_count = 0;
    
    for tx in transactions {
        if validator.validate(tx).is_ok() {
            valid_count += 1;
        }
    }
    
    valid_count
}

/// Validate transactions with hardware optimization
fn validate_transactions_with_hardware(
    transactions: &[Transaction],
    hw_manager: Arc<HardwareOptimizationManager>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let validator = TransactionValidator::new()
        .with_hardware_optimization(hw_manager);
        
    let valid_count = verify_transactions(&validator, transactions);
    
    Ok(valid_count == transactions.len())
}
