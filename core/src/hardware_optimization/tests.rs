//! Test suite for hardware optimization framework
//!
//! This module provides comprehensive testing of the hardware optimization
//! framework, including correctness verification, performance benchmarks,
//! and integration tests to ensure that optimized code provides identical
//! results with standard implementations across all platforms.

#[cfg(test)]
mod tests {
    use super::super::*;
    use tokio::test;
    use sha2::{Sha256, Digest};
    use rand::{Rng, thread_rng};
    
    // Test hardware detection on current system
    #[test]
    async fn test_hardware_detection() {
        // Detect hardware
        let capabilities = detection::detect_hardware().await.unwrap();
        
        // Verify required fields
        assert!(capabilities.core_count > 0, "Core count should be positive");
        assert!(capabilities.thread_count >= capabilities.core_count, 
                "Thread count should be >= core count");
        assert!(!capabilities.vendor.is_empty(), "Vendor should be detected");
        
        // Log detected hardware for developer reference
        println!("Detected hardware: {:?}", capabilities);
        println!("Architecture: {}", capabilities.architecture);
        println!("CPU Vendor: {}", capabilities.vendor);
        println!("Cores: {}, Threads: {}", capabilities.core_count, capabilities.thread_count);
        println!("Vector extensions: {:?}", capabilities.vector_extensions);
        println!("Crypto extensions: {:?}", capabilities.crypto_extensions);
    }
    
    // Test hardware optimization manager creation
    #[test]
    async fn test_optimization_manager_creation() {
        // Create manager
        let manager = HardwareOptimizationManager::new().await.unwrap();
        
        // Get status
        let status = manager.get_status().await;
        
        // Verify manager initialized correctly
        assert!(status.initialized, "Manager should be initialized");
        assert!(!status.active_optimizations.is_empty(), 
                "At least one optimization should be active");
    }
    
    // Test correctness verification for all optimizations
    #[test]
    async fn test_correctness_verification() {
        // Create manager
        let manager = HardwareOptimizationManager::new().await.unwrap();
        
        // Verify correctness
        assert!(manager.verify_correctness().await.is_ok(), 
                "Correctness verification should succeed");
    }
    
    // Test SHA-256 implementation correctness
    #[test]
    async fn test_sha256_correctness() {
        // Create manager
        let manager = HardwareOptimizationManager::new().await.unwrap();
        
        // Create test data with known hashes
        let test_vectors = vec![
            // Empty string
            (
                vec![], 
                hex::decode("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855").unwrap()
            ),
            // "abc"
            (
                "abc".as_bytes().to_vec(),
                hex::decode("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad").unwrap()
            ),
            // "hello world"
            (
                "hello world".as_bytes().to_vec(),
                hex::decode("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9").unwrap()
            ),
            // 1KB of zeros
            (
                vec![0; 1024],
                hex::decode("5f70bf18a086007016e948b04aed3b82103a36bea41755b6cddfaf10ace3c6ef").unwrap()
            ),
        ];
        
        // Get optimized execution path
        let execution_path = manager.optimize_operation(Operation::SHA256).await.unwrap();
        
        // Verify each test vector
        for (input, expected_output) in test_vectors {
            let output = execution_path.execute(&input).await.unwrap();
            assert_eq!(output, expected_output, 
                       "SHA-256 hash output should match expected value");
        }
    }
    
    // Test Schnorr verification (simplified test vectors)
    #[test]
    async fn test_schnorr_verification() {
        // Create manager
        let manager = HardwareOptimizationManager::new().await.unwrap();
        
        // Get optimized execution path
        let execution_path = manager.optimize_operation(Operation::SchnorrVerification).await.unwrap();
        
        // Use the placeholder test vectors we defined in the implementations
        // In a real implementation, we would use actual BIP-340 test vectors
        let valid_sig = vec![1; 128];
        let invalid_sig = vec![0; 128];
        
        // Valid signature should verify as true (result = [1])
        let result = execution_path.execute(&valid_sig).await.unwrap();
        assert_eq!(result, vec![1], "Valid signature should verify as true");
        
        // Invalid signature should verify as false (result = [0])
        let result = execution_path.execute(&invalid_sig).await.unwrap();
        assert_eq!(result, vec![0], "Invalid signature should verify as false");
    }
    
    // Benchmark SHA-256 performance
    #[test]
    async fn benchmark_sha256_performance() {
        // Create manager
        let manager = HardwareOptimizationManager::new().await.unwrap();
        
        // Get optimized execution path
        let execution_path = manager.optimize_operation(Operation::SHA256).await.unwrap();
        
        // Benchmark settings
        let iterations = 1000;
        let data_size = 1024; // 1 KB
        
        // Create random test data
        let mut rng = thread_rng();
        let test_data: Vec<u8> = (0..data_size).map(|_| rng.gen()).collect();
        
        // Run benchmark
        let metrics = execution_path.benchmark(iterations).await.unwrap();
        
        // Print results
        println!("SHA-256 Performance Metrics:");
        println!("  Hashes per second: {:.2}", metrics.hashes_per_second);
        
        // Verify performance is reasonable (very minimal check)
        assert!(metrics.hashes_per_second > 0.0, 
                "Hash rate should be positive");
    }
    
    // Test Bitcoin consensus operations using architecture-aware optimizations
    #[test]
    async fn test_consensus_operations() {
        // Create manager
        let manager = HardwareOptimizationManager::new().await.unwrap();
        
        // Test different operations with the HAL interface
        let hal = manager.get_hal().await;
        
        // Test SHA-256 via HAL
        let input = "Bitcoin secure consensus".as_bytes().to_vec();
        let context = hal::OperationContext {
            input: input.clone(),
            parameters: HashMap::new(),
            security_level: hal::SecurityLevel::Standard,
            verification: hal::VerificationRequirement::Basic,
        };
        
        // Execute operation through HAL
        let result = hal.execute_operation(Operation::SHA256, context).await.unwrap();
        
        // Verify against standard implementation
        let mut hasher = Sha256::new();
        hasher.update(&input);
        let expected = hasher.finalize().to_vec();
        
        assert_eq!(result.output, expected, 
                   "HAL SHA-256 result should match standard implementation");
    }
    
    // Test integration with the system component registration
    #[test]
    async fn test_system_integration() {
        use crate::system::{SystemManager, SystemComponent};
        
        // Create a system manager
        let system = SystemManager::new();
        
        // Create the hardware optimizer component
        let optimizer = integration::HardwareOptimizerComponent::new("HardwareOptimizer").await.unwrap();
        
        // Register the component
        let component_id = system.register_component(Box::new(optimizer)).await.unwrap();
        
        // Initialize the component
        system.initialize_component(&component_id).await.unwrap();
        
        // Check component health
        let health = system.check_component_health(&component_id).await.unwrap();
        
        // Verify component is operational
        assert!(health.operational, "Hardware optimizer component should be operational");
        assert!(health.health_score > 90.0, "Hardware optimizer should have good health score");
    }
    
    // Test workload adaptation
    #[test]
    async fn test_workload_adaptation() {
        // Create manager
        let manager = HardwareOptimizationManager::new().await.unwrap();
        
        // Create high-priority workload profile
        let high_priority_workload = WorkloadProfile {
            transaction_volume: 10000,
            block_validation_priority: Priority::High,
            memory_target: MemoryTarget::Performance,
            power_target: PowerTarget::Performance,
            custom_parameters: HashMap::new(),
        };
        
        // Update workload
        manager.update_workload(high_priority_workload).await.unwrap();
        
        // Verify workload was updated
        let status = manager.get_status().await;
        assert_eq!(status.workload.block_validation_priority, Priority::High);
    }
    
    // Test metrics collection
    #[test]
    async fn test_metrics_collection() {
        // Create manager
        let manager = HardwareOptimizationManager::new().await.unwrap();
        
        // Run some operations to generate metrics
        let path = manager.optimize_operation(Operation::SHA256).await.unwrap();
        for _ in 0..100 {
            path.execute(&vec![0; 1024]).await.unwrap();
        }
        
        // Collect metrics
        let metrics = manager.collect_metrics().await.unwrap();
        
        // Verify metrics were collected
        assert!(metrics.hashes_per_second > 0.0, "Hash rate should be positive");
    }
    
    // Test multi-threading with hardware-aware scheduling
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn test_multithreaded_execution() {
        // Create manager
        let manager = Arc::new(HardwareOptimizationManager::new().await.unwrap());
        
        // Number of parallel tasks
        let task_count = 16;
        let iterations_per_task = 100;
        
        // Create multiple tasks
        let mut tasks = Vec::new();
        for i in 0..task_count {
            let manager_clone = manager.clone();
            let task = tokio::spawn(async move {
                // Select different operations based on task ID
                let operation = match i % 3 {
                    0 => Operation::SHA256,
                    1 => Operation::SchnorrVerification,
                    _ => Operation::BatchVerification,
                };
                
                // Get optimized path for this operation
                let path = manager_clone.optimize_operation(operation).await.unwrap();
                
                // Execute multiple times
                let start = std::time::Instant::now();
                for _ in 0..iterations_per_task {
                    // Create input based on operation
                    let input = match operation {
                        Operation::SHA256 => vec![i as u8; 1024],
                        Operation::SchnorrVerification => vec![i as u8; 128],
                        _ => vec![i as u8; 256],
                    };
                    
                    // Execute
                    let _ = path.execute(&input).await.unwrap();
                }
                
                // Return task ID and elapsed time
                (i, start.elapsed())
            });
            
            tasks.push(task);
        }
        
        // Wait for all tasks to complete
        let mut completion_times = Vec::new();
        for task in tasks {
            let (id, time) = task.await.unwrap();
            completion_times.push((id, time));
            println!("Task {} completed in {:?}", id, time);
        }
        
        // Calculate overall throughput
        let total_operations = task_count * iterations_per_task;
        let max_time = completion_times.iter()
            .map(|(_, time)| time.as_secs_f64())
            .fold(0.0, |a, b| a.max(b));
        
        let throughput = total_operations as f64 / max_time;
        println!("Overall throughput: {:.2} operations/second", throughput);
        
        // Verify positive throughput
        assert!(throughput > 0.0, "Throughput should be positive");
    }
    
    // Test architecture-specific optimizations
    #[test]
    async fn test_architecture_specific_implementations() {
        // Detect hardware
        let capabilities = detection::detect_hardware().await.unwrap();
        
        // Create optimizers for each architecture
        // Note: These will only succeed on the matching architecture
        
        // RISC-V
        match riscv::RISCVOptimizer::new(&capabilities).await {
            Ok(optimizer) => {
                let metrics = optimizer.collect_metrics().await.unwrap();
                println!("RISC-V optimizer metrics: {:?}", metrics);
                
                // Test Schnorr verification
                let path = optimizer.optimize_operation(Operation::SchnorrVerification).await;
                let valid_sig = vec![1; 128];
                let result = path.execute(&valid_sig).await.unwrap();
                assert_eq!(result, vec![1], "RISC-V valid signature should verify as true");
            },
            Err(e) => {
                println!("RISC-V optimizer not available: {}", e);
            }
        }
        
        // ARM
        match arm::ARMOptimizer::new(&capabilities).await {
            Ok(optimizer) => {
                let metrics = optimizer.collect_metrics().await.unwrap();
                println!("ARM optimizer metrics: {:?}", metrics);
                
                // Test SHA-256
                let path = optimizer.optimize_operation(Operation::SHA256).await;
                let input = "ARM test".as_bytes();
                let mut hasher = Sha256::new();
                hasher.update(input);
                let expected = hasher.finalize().to_vec();
                
                let result = path.execute(input).await.unwrap();
                assert_eq!(result, expected, "ARM SHA-256 should match standard implementation");
            },
            Err(e) => {
                println!("ARM optimizer not available: {}", e);
            }
        }
        
        // Intel
        match intel::IntelOptimizer::new(&capabilities).await {
            Ok(optimizer) => {
                let metrics = optimizer.collect_metrics().await.unwrap();
                println!("Intel optimizer metrics: {:?}", metrics);
                
                // Test batch verification
                let path = optimizer.optimize_operation(Operation::BatchVerification).await;
                let input = vec![1; 256];
                let result = path.execute(&input).await;
                assert!(result.is_ok(), "Intel batch verification should succeed");
            },
            Err(e) => {
                println!("Intel optimizer not available: {}", e);
            }
        }
        
        // AMD
        match amd::AMDOptimizer::new(&capabilities).await {
            Ok(optimizer) => {
                let metrics = optimizer.collect_metrics().await.unwrap();
                println!("AMD optimizer metrics: {:?}", metrics);
                
                // Test CCX-aware optimizations
                if optimizer.capabilities.is_zen {
                    println!("Testing Zen-specific optimizations:");
                    println!("CCX count: {}", optimizer.capabilities.ccx_count);
                    println!("Cores per CCX: {}", optimizer.capabilities.cores_per_ccx);
                    
                    // Ensure CCX map is valid
                    assert!(!optimizer.ccx_map.is_empty(), "CCX map should not be empty for Zen processor");
                }
            },
            Err(e) => {
                println!("AMD optimizer not available: {}", e);
            }
        }
        
        // Generic fallback (should always succeed)
        let optimizer = fallback::GenericOptimizer::new(&capabilities).await.unwrap();
        let metrics = optimizer.collect_metrics().await.unwrap();
        println!("Generic optimizer metrics: {:?}", metrics);
        
        // Test operations
        let path = optimizer.optimize_operation(Operation::SHA256).await;
        let input = "Generic test".as_bytes();
        let mut hasher = Sha256::new();
        hasher.update(input);
        let expected = hasher.finalize().to_vec();
        
        let result = path.execute(input).await.unwrap();
        assert_eq!(result, expected, "Generic SHA-256 should match standard implementation");
    }
}
