//! Universal Adaptive Hardware Optimization Framework for anya-core [AIR-3][AIS-3][BPC-3][PFM-3][RES-3]
//!
//! This module provides a framework for detecting hardware capabilities
//! and optimizing cryptographic operations based on the available hardware.
//! It supports various architectures including RISC-V, ARM, x86_64 (Intel/AMD),
//! and provides fallback implementations for unsupported hardware.
//!
//! # Design Principles
//!
//! 1. **Decentralization** - Support for diverse hardware platforms enhances network participation
//! 2. **Security** - Architecture-specific optimizations maintain Bitcoin's security model
//! 3. **Immutability** - All optimizations ensure identical consensus results
//! 4. **Privacy** - Leverage Taproot for enhanced privacy across all hardware platforms
//! 5. **Compatibility** - Full compatibility with Bitcoin Core and BIPs

use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// Re-export main modules for external use
pub mod detection;
pub mod hal;
pub mod fallback;
pub mod riscv;
pub mod arm;
pub mod intel;
pub mod amd;
pub mod gpu;
pub mod npu;
pub mod integration;
pub mod benchmark;

use crate::metrics::MetricsProvider;
use crate::security::SecurityVerification;
use crate::protocol::ProtocolValidation;

/// Hardware architecture types supported by the optimization framework
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Architecture {
    /// RISC-V architecture (64-bit)
    RISCV64,
    /// x86-64 architecture (AMD64)
    X86_64,
    /// ARM architecture (AArch64)
    AArch64,
    /// Generic fallback for unsupported architectures
    Generic,
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Architecture::RISCV64 => write!(f, "RISC-V 64-bit"),
            Architecture::X86_64 => write!(f, "x86-64"),
            Architecture::AArch64 => write!(f, "AArch64"),
            Architecture::Generic => write!(f, "Generic"),
        }
    }
}

/// CPU vendor identification for specific optimizations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Vendor {
    /// AMD processors (Zen architecture)
    AMD,
    /// Intel processors
    Intel,
    /// ARM processors
    ARM,
    /// RISC-V implementations
    RISCV,
    /// Other vendors
    Other,
}

impl fmt::Display for Vendor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Vendor::AMD => write!(f, "AMD"),
            Vendor::Intel => write!(f, "Intel"),
            Vendor::ARM => write!(f, "ARM"),
            Vendor::RISCV => write!(f, "RISC-V"),
            Vendor::Other => write!(f, "Other"),
        }
    }
}

/// Core operation types that can be hardware-accelerated
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operation {
    /// Schnorr signature verification (BIP-340)
    SchnorrVerification,
    /// ECDSA signature verification
    ECDSAVerification,
    /// SHA-256 hashing operations
    SHA256,
    /// SHA-512 hashing operations
    SHA512,
    /// Batch signature verification
    BatchVerification,
    /// Bitcoin script execution
    ScriptExecution,
    /// Merkle path verification
    MerkleVerification,
    /// Taproot verification (BIP-341)
    TaprootVerification,
    /// Tapscript execution (BIP-342)
    TapscriptExecution,
}

/// Hardware capabilities detected at runtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareCapabilities {
    /// Base architecture
    pub architecture: Architecture,
    /// CPU vendor
    pub vendor: Vendor,
    /// Model name
    pub model: String,
    /// CPU core count
    pub core_count: usize,
    /// Logical processor count
    pub thread_count: usize,
    /// Available vector extensions
    pub vector_extensions: Vec<String>,
    /// Available cryptographic extensions
    pub crypto_extensions: Vec<String>,
    /// L1/L2/L3 cache sizes
    pub cache_sizes: Vec<usize>,
    /// Detected NUMA nodes
    pub numa_nodes: usize,
    /// Core topology information (CCX for AMD, etc.)
    pub topology: Option<String>,
    /// GPU/NPU capabilities if available
    pub gpu_capabilities: Option<gpu::GpuCapabilities>,
}

/// Hardware optimization status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStatus {
    /// Hardware detected
    pub hardware: HardwareCapabilities,
    /// Active optimizations
    pub active_optimizations: Vec<String>,
    /// Disabled optimizations
    pub disabled_optimizations: Vec<String>,
    /// Performance metrics
    pub performance_metrics: Option<PerformanceMetrics>,
    /// Last updated timestamp
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Performance metrics for hardware optimizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Signature verifications per second
    pub sig_verifications_per_second: f64,
    /// Transactions validated per second
    pub transactions_per_second: f64,
    /// Script operations per second
    pub script_ops_per_second: f64,
    /// Hash calculations per second
    pub hashes_per_second: f64,
    /// CPU utilization percentage
    pub cpu_utilization: f64,
    /// Memory utilization in MB
    pub memory_usage_mb: f64,
}

/// Core hardware optimization trait
#[async_trait::async_trait]
pub trait HardwareOptimization: Send + Sync {
    /// Detect hardware capabilities
    async fn detect_capabilities(&self) -> HardwareCapabilities;
    
    /// Create optimized execution path for a specific operation
    async fn optimize_operation(&self, operation: Operation) -> Box<dyn ExecutionPath>;
    
    /// Tune optimization parameters for current workload
    async fn tune_for_workload(&mut self, workload: WorkloadProfile) -> Result<(), OptimizationError>;
    
    /// Collect performance metrics
    async fn collect_metrics(&self) -> Result<PerformanceMetrics, OptimizationError>;
    
    /// Verify optimization correctness against reference implementation
    async fn verify_correctness(&self, operation: Operation) -> Result<(), OptimizationError>;
}

/// Execution path for hardware-optimized operations
#[async_trait::async_trait]
pub trait ExecutionPath: Send + Sync {
    /// Get the operation type
    fn operation(&self) -> Operation;
    
    /// Get the hardware architecture
    fn architecture(&self) -> Architecture;
    
    /// Execute the optimized operation with provided data
    async fn execute(&self, data: &[u8]) -> Result<Vec<u8>, ExecutionError>;
    
    /// Verify that the execution produces identical results to reference implementation
    async fn verify_correctness(&self, test_vectors: &[(&[u8], &[u8])]) -> Result<(), ExecutionError>;
    
    /// Get performance metrics for this execution path
    async fn benchmark(&self, iterations: usize) -> Result<PerformanceMetrics, ExecutionError>;
}

/// Workload profile for optimization tuning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadProfile {
    /// Transaction volume
    pub transaction_volume: usize,
    /// Block validation priority
    pub block_validation_priority: Priority,
    /// Memory usage target
    pub memory_target: MemoryTarget,
    /// Power efficiency target
    pub power_target: PowerTarget,
    /// Custom optimization parameters
    pub custom_parameters: std::collections::HashMap<String, f64>,
}

/// Priority levels for operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Priority {
    /// Highest priority (time-critical)
    Critical,
    /// High priority
    High,
    /// Normal priority
    Normal,
    /// Low priority (background)
    Low,
}

/// Memory usage targets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryTarget {
    /// Minimize memory usage
    Minimal,
    /// Balanced memory usage
    Balanced,
    /// Maximum performance (higher memory usage)
    Performance,
}

/// Power efficiency targets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PowerTarget {
    /// Maximum power efficiency
    Efficient,
    /// Balanced power usage
    Balanced,
    /// Maximum performance (higher power usage)
    Performance,
}

/// Hardware optimization errors
#[derive(thiserror::Error, Debug)]
pub enum OptimizationError {
    #[error("Hardware detection failed: {0}")]
    DetectionFailed(String),
    
    #[error("Optimization not available: {0}")]
    NotAvailable(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Correctness verification failed: {0}")]
    CorrectnessError(String),
    
    #[error("Tuning error: {0}")]
    TuningError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Execution path errors
#[derive(thiserror::Error, Debug)]
pub enum ExecutionError {
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
    
    #[error("Benchmark error: {0}")]
    BenchmarkError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Central manager for hardware optimizations
pub struct HardwareOptimizationManager {
    /// Current hardware capabilities
    capabilities: Arc<RwLock<HardwareCapabilities>>,
    
    /// Active optimization engine
    engine: Arc<RwLock<Box<dyn HardwareOptimization>>>,
    
    /// Optimization status
    status: Arc<RwLock<OptimizationStatus>>,
    
    /// Current workload profile
    workload: Arc<RwLock<WorkloadProfile>>,
}

impl HardwareOptimizationManager {
    /// Create a new hardware optimization manager with CPU-only optimizations
    pub async fn new() -> Result<Self, OptimizationError> {
        // Detect hardware capabilities
        let capabilities = detection::detect_hardware().await?;
        
        // Check if GPU/NPU acceleration is available and should be prioritized
        if let Some(gpu_caps) = &capabilities.gpu_capabilities {
            if gpu_caps.gpu_available || gpu_caps.npu_available {
                return Self::with_hardware_acceleration().await;
            }
        }

        // Create optimization engine based on architecture
        let engine: Box<dyn HardwareOptimization> = match capabilities.architecture {
            Architecture::RISCV64 => {
                match riscv::RISCVOptimizer::new(&capabilities).await {
                    Ok(optimizer) => Box::new(optimizer),
                    Err(_) => Box::new(fallback::GenericOptimizer::new(&capabilities).await?),
                }
            },
            Architecture::X86_64 => {
                if capabilities.vendor == Vendor::AMD {
                    // Use AMD-specific optimizations for AMD processors
                    match amd::AMDOptimizer::new(&capabilities).await {
                        Ok(optimizer) => Box::new(optimizer),
                        Err(_) => Box::new(fallback::GenericOptimizer::new(&capabilities).await?),
                    }
                } else if capabilities.vendor == Vendor::Intel {
                    // Use Intel-specific optimizations for Intel processors
                    match intel::IntelOptimizer::new(&capabilities).await {
                        Ok(optimizer) => Box::new(optimizer),
                        Err(_) => Box::new(fallback::GenericOptimizer::new(&capabilities).await?),
                    }
                } else {
                    // Fallback for other x86_64 vendors
                    Box::new(fallback::GenericOptimizer::new(&capabilities).await?)
                }
            },
            Architecture::AArch64 => {
                match arm::ARMOptimizer::new(&capabilities).await {
                    Ok(optimizer) => Box::new(optimizer),
                    Err(_) => Box::new(fallback::GenericOptimizer::new(&capabilities).await?),
                }
            },
            _ => {
                // Fallback for unsupported architectures
                Box::new(fallback::GenericOptimizer::new(&capabilities).await?)
            }
        };
        
        // Create HAL
        let hal = hal::HardwareAbstractionLayer::new(engine.clone());
        
        // Create metrics
        let metrics = PerformanceMetrics {
            sig_verifications_per_second: 0.0,
            transactions_per_second: 0.0,
            script_ops_per_second: 0.0,
            hashes_per_second: 0.0,
            cpu_utilization: 0.0,
            memory_usage_mb: 0.0,
        };
        
        // Create workload with default values
        let workload = WorkloadProfile {
            transaction_volume: 1000,
            block_validation_priority: Priority::Normal,
            memory_target: MemoryTarget::Balanced,
            power_target: PowerTarget::Balanced,
            custom_parameters: std::collections::HashMap::new(),
        };
        
        // Create status
        let status = OptimizationStatus {
            initialized: true,
            hardware: capabilities.clone(),
            active_optimizations: vec![format!("{}::{}", capabilities.architecture, capabilities.vendor)],
            performance_metrics: Some(metrics.clone()),
            last_updated: chrono::Utc::now(),
            workload: workload.clone(),
        };
        
        Ok(Self {
            capabilities: Arc::new(RwLock::new(capabilities)),
            engine: Arc::new(RwLock::new(engine)),
            hal: Arc::new(hal),
            metrics: Arc::new(RwLock::new(metrics)),
            status: Arc::new(RwLock::new(status)),
            workload: Arc::new(RwLock::new(workload)),
        })
    }
    
    /// Create a new hardware optimization manager with GPU acceleration
    pub async fn with_gpu() -> Result<Self, OptimizationError> {
        // Detect hardware capabilities
        let mut capabilities = detection::detect_hardware().await?;
        
        // Verify GPU is available
        if let Some(ref gpu_caps) = capabilities.gpu_capabilities {
            if !gpu_caps.gpu_available {
                return Self::new().await;
            }
            
            // Create GPU optimizer
            let gpu_optimizer = match gpu::GpuOptimizer::new(&capabilities).await {
                Ok(optimizer) => optimizer,
                Err(_) => return Self::new().await, // Fall back to CPU
            };
            
            // Create HAL with GPU optimizer
            let engine: Box<dyn HardwareOptimization> = Box::new(gpu_optimizer);
            let hal = hal::HardwareAbstractionLayer::new(engine.clone());
            
            // Create metrics
            let metrics = PerformanceMetrics {
                sig_verifications_per_second: 0.0,
                transactions_per_second: 0.0,
                script_ops_per_second: 0.0,
                hashes_per_second: 0.0,
                cpu_utilization: 0.0,
                memory_usage_mb: 0.0,
            };
            
            // Create workload with default values
            let workload = WorkloadProfile {
                transaction_volume: 1000,
                block_validation_priority: Priority::Normal,
                memory_target: MemoryTarget::Balanced,
                power_target: PowerTarget::Balanced,
                custom_parameters: std::collections::HashMap::new(),
            };
            
            // Create status with GPU info
            let status = OptimizationStatus {
                initialized: true,
                hardware: capabilities.clone(),
                active_optimizations: vec![
                    format!("GPU::{}", gpu_caps.vendor),
                    format!("Backends::{:?}", gpu_caps.backends),
                ],
                performance_metrics: Some(metrics.clone()),
                last_updated: chrono::Utc::now(),
                workload: workload.clone(),
            };
            
            return Ok(Self {
                capabilities: Arc::new(RwLock::new(capabilities)),
                engine: Arc::new(RwLock::new(engine)),
                hal: Arc::new(hal),
                metrics: Arc::new(RwLock::new(metrics)),
                status: Arc::new(RwLock::new(status)),
                workload: Arc::new(RwLock::new(workload)),
            });
        }
        
        // Fall back to CPU if no GPU
        Self::new().await
    }
    
    /// Create a new hardware optimization manager with NPU acceleration
    pub async fn with_npu() -> Result<Self, OptimizationError> {
        // Detect hardware capabilities
        let capabilities = detection::detect_hardware().await?;
        
        // Verify NPU is available
        if let Some(ref gpu_caps) = capabilities.gpu_capabilities {
            if !gpu_caps.npu_available || gpu_caps.npu_type.is_none() {
                return Self::new().await;
            }
            
            // Create NPU optimizer
            let npu_optimizer = match npu::create_npu_optimizer(&capabilities).await {
                Ok(optimizer) => optimizer,
                Err(_) => return Self::new().await, // Fall back to CPU
            };
            
            // Create HAL with NPU optimizer
            let engine = npu_optimizer;
            let hal = hal::HardwareAbstractionLayer::new(engine.clone());
            
            // Create metrics
            let metrics = PerformanceMetrics {
                sig_verifications_per_second: 0.0,
                transactions_per_second: 0.0,
                script_ops_per_second: 0.0,
                hashes_per_second: 0.0,
                cpu_utilization: 0.0,
                memory_usage_mb: 0.0,
            };
            
            // Create workload with default values
            let workload = WorkloadProfile {
                transaction_volume: 1000,
                block_validation_priority: Priority::High, // NPUs benefit from high priority
                memory_target: MemoryTarget::Performance,  // Focus on performance
                power_target: PowerTarget::Balanced,       // Balance power usage
                custom_parameters: std::collections::HashMap::new(),
            };
            
            // Create status with NPU info
            let status = OptimizationStatus {
                initialized: true,
                hardware: capabilities.clone(),
                active_optimizations: vec![
                    format!("NPU::{:?}", gpu_caps.npu_type.unwrap()),
                ],
                performance_metrics: Some(metrics.clone()),
                last_updated: chrono::Utc::now(),
                workload: workload.clone(),
            };
            
            return Ok(Self {
                capabilities: Arc::new(RwLock::new(capabilities)),
                engine: Arc::new(RwLock::new(engine)),
                hal: Arc::new(hal),
                metrics: Arc::new(RwLock::new(metrics)),
                status: Arc::new(RwLock::new(status)),
                workload: Arc::new(RwLock::new(workload)),
            });
        }
        
        // Fall back to CPU if no NPU
        Self::new().await
    }
    
    /// Get current hardware capabilities
    pub async fn get_capabilities(&self) -> HardwareCapabilities {
        self.capabilities.read().await.clone()
    }
    
    /// Get current optimization status
    pub async fn get_status(&self) -> OptimizationStatus {
        self.status.read().await.clone()
    }
    
    /// Update workload profile
    pub async fn update_workload(&self, workload: WorkloadProfile) -> Result<(), OptimizationError> {
        // Update workload
        {
            let mut current_workload = self.workload.write().await;
            *current_workload = workload;
        }
        
        // Tune optimizations for new workload
        let mut engine = self.engine.write().await;
        engine.tune_for_workload(self.workload.read().await.clone()).await?;
        
        // Update status
        {
            let mut status = self.status.write().await;
            status.last_updated = chrono::Utc::now();
        }
        
        Ok(())
    }
    
    /// Create optimized execution path for operation
    pub async fn optimize_operation(&self, operation: Operation) -> Result<Box<dyn ExecutionPath>, OptimizationError> {
        let engine = self.engine.read().await;
        Ok(engine.optimize_operation(operation).await)
    }
    
    /// Collect performance metrics
    pub async fn collect_metrics(&self) -> Result<PerformanceMetrics, OptimizationError> {
        let engine = self.engine.read().await;
        let metrics = engine.collect_metrics().await?;
        
        // Update status with metrics
        {
            let mut status = self.status.write().await;
            status.performance_metrics = Some(metrics.clone());
            status.last_updated = chrono::Utc::now();
        }
        
        Ok(metrics)
    }
    
    /// Verify correctness of all optimizations
    pub async fn verify_correctness(&self) -> Result<(), OptimizationError> {
        let engine = self.engine.read().await;
        
        // Verify each operation type
        for operation in &[
            Operation::SchnorrVerification,
            Operation::ECDSAVerification,
            Operation::SHA256,
            Operation::SHA512,
            Operation::BatchVerification,
            Operation::ScriptExecution,
            Operation::MerkleVerification,
            Operation::TaprootVerification,
            Operation::TapscriptExecution,
        ] {
            engine.verify_correctness(*operation).await?;
        }
        
        Ok(())
    }
}

/// Hardware detection module (placeholder - actual implementation in detection.rs)
pub mod detection {
    use super::*;
    
    /// Detect hardware capabilities
    pub async fn detect_hardware() -> Result<HardwareCapabilities, OptimizationError> {
        // Actual implementation will be in detection.rs
        // This is a placeholder that will be replaced
        
        Ok(HardwareCapabilities {
            architecture: Architecture::Generic,
            vendor: Vendor::Other,
            model: "Unknown".to_string(),
            core_count: 1,
            thread_count: 1,
            vector_extensions: Vec::new(),
            crypto_extensions: Vec::new(),
            cache_sizes: vec![0, 0, 0],
            numa_nodes: 1,
            topology: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_hardware_detection() {
        let capabilities = detection::detect_hardware().await.unwrap();
        assert!(capabilities.core_count > 0);
        assert!(capabilities.thread_count >= capabilities.core_count);
    }
    
    #[tokio::test]
    async fn test_optimization_manager_creation() {
        let manager = HardwareOptimizationManager::new().await.unwrap();
        let status = manager.get_status().await;
        assert_eq!(status.hardware.architecture, Architecture::Generic);
    }
    
    // Additional tests will be implemented for specific optimizations
}
