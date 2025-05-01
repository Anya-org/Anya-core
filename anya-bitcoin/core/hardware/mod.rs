//! Hardware acceleration module for Anya Bitcoin
//!
//! This module provides hardware-accelerated implementations of cryptographic operations
//! used in Bitcoin, with a focus on optimizing Taproot and batch verification operations.
//! 
//! It implements adaptive hardware selection based on available resources and provides
//! optimized implementations for different hardware configurations.

use bitcoin::secp256k1::{self, Secp256k1, XOnlyPublicKey, Message, Verification};
use bitcoin::hashes::{sha256, Hash};
use bitcoin::secp256k1::schnorr::Signature as SchnorrSignature;
use std::sync::Arc;
use std::error::Error;
use std::fmt;

/// Hardware acceleration error type
#[derive(Debug)]
pub enum HardwareError {
    /// Initialization failure
    InitError(String),
    /// Operation failure
    OperationError(String),
    /// Validation error
    ValidationError(String),
    /// Unsupported hardware
    UnsupportedHardware(String),
    /// Internal error
    InternalError(String),
}

impl fmt::Display for HardwareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HardwareError::InitError(msg) => write!(f, "Hardware initialization error: {}", msg),
            HardwareError::OperationError(msg) => write!(f, "Hardware operation error: {}", msg),
            HardwareError::ValidationError(msg) => write!(f, "Hardware validation error: {}", msg),
            HardwareError::UnsupportedHardware(msg) => write!(f, "Unsupported hardware: {}", msg),
            HardwareError::InternalError(msg) => write!(f, "Internal hardware error: {}", msg),
        }
    }
}

impl Error for HardwareError {}

/// Hardware acceleration trait
pub trait HardwareAccelerator: Send + Sync {
    /// Initialize hardware acceleration
    fn initialize(&mut self) -> Result<(), HardwareError>;
    
    /// Check if hardware acceleration is available
    fn is_available(&self) -> bool;
    
    /// Verify a single Schnorr signature
    fn verify_schnorr_signature(
        &self, 
        message: &[u8],
        signature: &SchnorrSignature,
        public_key: &XOnlyPublicKey,
    ) -> Result<bool, HardwareError>;
    
    /// Batch verify multiple Schnorr signatures
    fn verify_schnorr_batch(
        &self,
        messages: &[&[u8]],
        signatures: &[SchnorrSignature],
        public_keys: &[XOnlyPublicKey],
    ) -> Result<bool, HardwareError>;
    
    /// Compute SHA256 hash with hardware acceleration if available
    fn compute_sha256(&self, data: &[u8]) -> Result<[u8; 32], HardwareError>;
    
    /// Compute transaction ID with hardware acceleration if available
    fn compute_txid(&self, tx_data: &[u8]) -> Result<[u8; 32], HardwareError>;
    
    /// Get hardware type
    fn hardware_type(&self) -> HardwareType;
    
    /// Get performance metrics
    fn get_metrics(&self) -> HardwareMetrics;
}

/// Available hardware acceleration types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareType {
    /// CPU with SIMD instructions
    CpuSimd,
    /// GPU with CUDA
    GpuCuda,
    /// GPU with OpenCL
    GpuOpenCL,
    /// Neural Processing Unit
    Npu,
    /// Software fallback (no hardware acceleration)
    Software,
}

/// Hardware performance metrics
#[derive(Debug, Clone)]
pub struct HardwareMetrics {
    /// Hardware type
    pub hardware_type: HardwareType,
    /// Operations per second
    pub ops_per_second: u64,
    /// Average latency in microseconds
    pub avg_latency_us: u64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Batch size
    pub batch_size: u32,
}

/// Factory for creating hardware accelerators
pub struct HardwareAcceleratorFactory;

impl HardwareAcceleratorFactory {
    /// Create the best available hardware accelerator
    pub fn create_best_available() -> Arc<dyn HardwareAccelerator> {
        // Try to create hardware accelerators in order of performance
        if let Ok(npu) = Self::create_npu() {
            return Arc::new(npu);
        }
        
        if let Ok(cuda) = Self::create_cuda() {
            return Arc::new(cuda);
        }
        
        if let Ok(opencl) = Self::create_opencl() {
            return Arc::new(opencl);
        }
        
        if let Ok(simd) = Self::create_cpu_simd() {
            return Arc::new(simd);
        }
        
        // Fallback to software implementation
        Arc::new(SoftwareAccelerator::new())
    }
    
    /// Create NPU accelerator
    pub fn create_npu() -> Result<NpuAccelerator, HardwareError> {
        if NpuAccelerator::is_supported() {
            let mut accelerator = NpuAccelerator::new();
            accelerator.initialize()?;
            Ok(accelerator)
        } else {
            Err(HardwareError::UnsupportedHardware("NPU not available".into()))
        }
    }
    
    /// Create CUDA GPU accelerator
    pub fn create_cuda() -> Result<CudaAccelerator, HardwareError> {
        if CudaAccelerator::is_supported() {
            let mut accelerator = CudaAccelerator::new();
            accelerator.initialize()?;
            Ok(accelerator)
        } else {
            Err(HardwareError::UnsupportedHardware("CUDA not available".into()))
        }
    }
    
    /// Create OpenCL GPU accelerator
    pub fn create_opencl() -> Result<OpenCLAccelerator, HardwareError> {
        if OpenCLAccelerator::is_supported() {
            let mut accelerator = OpenCLAccelerator::new();
            accelerator.initialize()?;
            Ok(accelerator)
        } else {
            Err(HardwareError::UnsupportedHardware("OpenCL not available".into()))
        }
    }
    
    /// Create CPU SIMD accelerator
    pub fn create_cpu_simd() -> Result<CpuSimdAccelerator, HardwareError> {
        if CpuSimdAccelerator::is_supported() {
            let mut accelerator = CpuSimdAccelerator::new();
            accelerator.initialize()?;
            Ok(accelerator)
        } else {
            Err(HardwareError::UnsupportedHardware("CPU SIMD not available".into()))
        }
    }
}

// Import specific implementations
mod cpu;
mod cuda;
mod opencl;
mod npu;
mod software;

// Re-export implementations
pub use cpu::CpuSimdAccelerator;
pub use cuda::CudaAccelerator;
pub use opencl::OpenCLAccelerator;
pub use npu::NpuAccelerator;
pub use software::SoftwareAccelerator;

// Convenience functions

/// Get the best available hardware accelerator
pub fn get_best_accelerator() -> Arc<dyn HardwareAccelerator> {
    HardwareAcceleratorFactory::create_best_available()
}

/// Verify a Schnorr signature using the best available hardware
pub fn verify_schnorr(
    message: &[u8],
    signature: &SchnorrSignature,
    public_key: &XOnlyPublicKey,
) -> Result<bool, HardwareError> {
    let accelerator = get_best_accelerator();
    accelerator.verify_schnorr_signature(message, signature, public_key)
}

/// Batch verify Schnorr signatures using the best available hardware
pub fn verify_schnorr_batch(
    messages: &[&[u8]],
    signatures: &[SchnorrSignature],
    public_keys: &[XOnlyPublicKey],
) -> Result<bool, HardwareError> {
    let accelerator = get_best_accelerator();
    accelerator.verify_schnorr_batch(messages, signatures, public_keys)
}

/// Compute SHA256 hash using the best available hardware
pub fn compute_sha256(data: &[u8]) -> Result<[u8; 32], HardwareError> {
    let accelerator = get_best_accelerator();
    accelerator.compute_sha256(data)
}

/// Compute transaction ID using the best available hardware
pub fn compute_txid(tx_data: &[u8]) -> Result<[u8; 32], HardwareError> {
    let accelerator = get_best_accelerator();
    accelerator.compute_txid(tx_data)
}
