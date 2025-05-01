//! Integration module for hardware optimization framework
//!
//! This module provides the entry point for the hardware optimization framework,
//! automatically detecting the underlying hardware and creating the appropriate
//! optimizer. It follows Bitcoin's principles of decentralization, security,
//! and immutability while providing significant performance improvements.

use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;

use super::{
    Architecture, HardwareCapabilities, Operation, OptimizationError,
    ExecutionError, WorkloadProfile, PerformanceMetrics, HardwareOptimization,
    ExecutionPath, HardwareOptimizationManager
};

use crate::metrics::MetricsProvider;
use crate::hardware_optimization::detection;
use crate::hardware_optimization::riscv::RISCVOptimizer;
use crate::hardware_optimization::arm::ARMOptimizer;
use crate::hardware_optimization::fallback::GenericOptimizer;

/// Create the optimal hardware optimizer for the current system
pub async fn create_optimal_optimizer() -> Result<HardwareOptimizationManager, OptimizationError> {
    // Detect hardware capabilities
    let capabilities = detection::detect_hardware().await?;
    
    // Create appropriate optimizer based on detected architecture
    HardwareOptimizationManager::new().await
}

/// Create the optimal hardware optimizer with consideration for GPU/NPU
pub async fn create_accelerated_optimizer() -> Result<HardwareOptimizationManager, OptimizationError> {
    // Detect hardware capabilities including GPU/NPU
    let capabilities = detection::detect_hardware().await?;
    
    // First check for NPU acceleration
    if let Some(ref gpu_caps) = capabilities.gpu_capabilities {
        if gpu_caps.npu_available && gpu_caps.npu_type.is_some() {
            println!("Detected NPU: {:?}", gpu_caps.npu_type);
            return HardwareOptimizationManager::with_npu().await;
        }
    }
    
    // Then check for GPU acceleration
    if let Some(ref gpu_caps) = capabilities.gpu_capabilities {
        if gpu_caps.gpu_available {
            println!("Detected GPU: {} with {} backends", 
                     gpu_caps.model, 
                     gpu_caps.backends.len());
            return HardwareOptimizationManager::with_gpu().await;
        }
    }
    
    // Fall back to CPU-only optimization
    println!("No GPU/NPU detected, using CPU-only optimizations");
    HardwareOptimizationManager::new().await
}

/// Universal Hardware Optimizer Component
/// This component can be registered with the system manager
pub struct HardwareOptimizerComponent {
    /// The hardware optimization manager
    manager: Arc<HardwareOptimizationManager>,
    
    /// Component name
    name: String,
    
    /// Component status
    status: Arc<RwLock<ComponentStatus>>,
    
    /// Last updated timestamp
    last_updated: Arc<RwLock<chrono::DateTime<chrono::Utc>>>,
}

/// Component status
#[derive(Debug, Clone)]
pub struct ComponentStatus {
    /// Whether the component is operational
    pub operational: bool,
    
    /// Health score (0-100)
    pub health_score: f64,
    
    /// Error count
    pub error_count: usize,
    
    /// Warning count
    pub warning_count: usize,
    
    /// Selected architecture
    pub selected_architecture: Architecture,
    
    /// Active optimizations
    pub active_optimizations: Vec<String>,
}

impl HardwareOptimizerComponent {
    /// Create a new hardware optimizer component
    pub async fn new(name: &str) -> Result<Self, OptimizationError> {
        // Create hardware optimization manager
        let manager = HardwareOptimizationManager::new().await?;
        
        // Get capabilities
        let capabilities = manager.get_capabilities().await;
        
        // Create component status
        let status = ComponentStatus {
            operational: true,
            health_score: 100.0,
            error_count: 0,
            warning_count: 0,
            selected_architecture: capabilities.architecture,
            active_optimizations: Vec::new(),
        };
        
        Ok(Self {
            manager: Arc::new(manager),
            name: name.to_string(),
            status: Arc::new(RwLock::new(status)),
            last_updated: Arc::new(RwLock::new(chrono::Utc::now())),
        })
    }
    
    /// Get the component name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Get the component status
    pub async fn status(&self) -> ComponentStatus {
        self.status.read().await.clone()
    }
    
    /// Get the hardware optimization manager
    pub fn manager(&self) -> Arc<HardwareOptimizationManager> {
        self.manager.clone()
    }
    
    /// Update the component status
    pub async fn update_status(&self) -> Result<(), OptimizationError> {
        // Get latest performance metrics
        let metrics = self.manager.collect_metrics().await?;
        
        // Get optimization status
        let opt_status = self.manager.get_status().await;
        
        // Update status
        let mut status = self.status.write().await;
        status.operational = true;
        status.active_optimizations = opt_status.active_optimizations.clone();
        
        // Update timestamp
        let mut last_updated = self.last_updated.write().await;
        *last_updated = chrono::Utc::now();
        
        Ok(())
    }
    
    /// Execute an operation with the optimal hardware acceleration
    pub async fn execute_operation(
        &self,
        operation: Operation,
        data: &[u8],
    ) -> Result<Vec<u8>, ExecutionError> {
        // Create operation context
        let context = super::hal::OperationContext {
            input: data.to_vec(),
            parameters: std::collections::HashMap::new(),
            security_level: super::hal::SecurityLevel::Standard,
            verification: super::hal::VerificationRequirement::Basic,
        };
        
        // Get hardware abstraction layer
        let hal = self.manager.get_hal().await;
        
        // Execute operation
        let result = hal.execute_operation(operation, context).await?;
        
        Ok(result.output)
    }
    
    /// Tune the hardware optimizer for the current workload
    pub async fn tune_for_workload(&self, workload: WorkloadProfile) -> Result<(), OptimizationError> {
        self.manager.update_workload(workload).await
    }
    
    /// Verify that all operations produce correct results
    pub async fn verify_correctness(&self) -> Result<(), OptimizationError> {
        self.manager.verify_correctness().await
    }
}

// Implement component traits for integration with the system

#[async_trait]
impl crate::system::SystemComponent for HardwareOptimizerComponent {
    async fn initialize(&self) -> Result<(), crate::system::SystemError> {
        // Verify correctness of all operations
        self.verify_correctness().await
            .map_err(|e| crate::system::SystemError::ComponentError(e.to_string()))?;
        
        // Update status
        self.update_status().await
            .map_err(|e| crate::system::SystemError::ComponentError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn shutdown(&self) -> Result<(), crate::system::SystemError> {
        // Nothing to clean up
        Ok(())
    }
    
    async fn health_check(&self) -> Result<crate::system::ComponentHealth, crate::system::SystemError> {
        // Get status
        let status = self.status().await;
        
        Ok(crate::system::ComponentHealth {
            operational: status.operational,
            health_score: status.health_score,
            last_incident: None,
            error_count: status.error_count,
            warning_count: status.warning_count,
        })
    }
}

#[async_trait]
impl MetricsProvider for HardwareOptimizerComponent {
    async fn collect_metrics(&self) -> Result<crate::metrics::UnifiedMetrics, crate::metrics::MetricsError> {
        // Get hardware optimization metrics
        let opt_metrics = self.manager.collect_metrics().await
            .map_err(|e| crate::metrics::MetricsError::CollectionError(e.to_string()))?;
        
        // Create unified metrics
        let metrics = crate::metrics::UnifiedMetrics {
            system: crate::metrics::SystemMetrics {
                cpu_usage: 0.0,
                memory_usage: 0.0,
                disk_usage: 0.0,
                network_traffic: 0.0,
                error_rate: 0.0,
                ops_total: 0,
                ops_success: 0,
                ops_failed: 0,
                ops_latency: opt_metrics.sig_verifications_per_second,
                health_score: 100.0,
                last_check: chrono::Utc::now(),
            },
            ml: None,
            security: None,
            protocol: None,
            enterprise: None,
            validation: None,
            custom: {
                let mut map = std::collections::HashMap::new();
                map.insert("sig_verifications_per_second".to_string(), opt_metrics.sig_verifications_per_second);
                map.insert("transactions_per_second".to_string(), opt_metrics.transactions_per_second);
                map.insert("script_ops_per_second".to_string(), opt_metrics.script_ops_per_second);
                map.insert("hashes_per_second".to_string(), opt_metrics.hashes_per_second);
                map
            },
            timestamp: chrono::Utc::now(),
        };
        
        Ok(metrics)
    }
    
    async fn update_metrics(&self, metrics: crate::metrics::UnifiedMetrics) -> Result<(), crate::metrics::MetricsError> {
        // This component doesn't accept metrics updates from outside
        Ok(())
    }
    
    async fn get_health(&self) -> Result<crate::metrics::ComponentHealth, crate::metrics::MetricsError> {
        // Get health check
        let health = self.health_check().await
            .map_err(|e| crate::metrics::MetricsError::CollectionError(e.to_string()))?;
        
        // Convert to metrics component health
        Ok(crate::metrics::ComponentHealth {
            operational: health.operational,
            health_score: health.health_score,
            last_incident: health.last_incident,
            error_count: health.error_count,
            warning_count: health.warning_count,
        })
    }
}

// Create a helper function to benchmark all operations
pub async fn benchmark_all_operations() -> Result<std::collections::HashMap<Operation, PerformanceMetrics>, OptimizationError> {
    // Create hardware optimization manager
    let manager = HardwareOptimizationManager::new().await?;
    
    // Operations to benchmark
    let operations = vec![
        Operation::SchnorrVerification,
        Operation::ECDSAVerification,
        Operation::SHA256,
        Operation::SHA512,
        Operation::BatchVerification,
        Operation::ScriptExecution,
        Operation::MerkleVerification,
        Operation::TaprootVerification,
        Operation::TapscriptExecution,
    ];
    
    // Benchmark each operation
    let mut results = std::collections::HashMap::new();
    let hal = manager.get_hal().await;
    
    for operation in operations {
        // Get execution path
        let path = hal.create_optimized_path(operation).await?;
        
        // Run benchmark
        let metrics = path.benchmark(1000).await
            .map_err(|e| OptimizationError::TuningError(e.to_string()))?;
        
        // Add to results
        results.insert(operation, metrics);
    }
    
    Ok(results)
}
