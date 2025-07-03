//! Hardware optimization compatibility module for tests

use std::sync::Arc;
use anya_core::hardware_optimization::{HardwareOptimizationManager, intel::{IntelOptimizer, CpuCapabilities}};

// Re-export main types
pub use anya_core::hardware_optimization::{HardwareOptimizationManager, intel::*};

// Additional compatibility types for tests
#[derive(Debug, Clone)]
pub enum HardwareType {
    Intel,
    AMD,
    ARM,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum OptimizableOperation {
    SchnorrVerification,
    BatchVerification,
    TaprootValidation,
    MerkleProof,
}

#[derive(Debug, Clone)]
pub struct WorkItem {
    pub operation: OptimizableOperation,
    pub data: Vec<u8>,
    pub priority: u32,
}

#[derive(Debug, Clone)]
pub enum WorkStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

// Mock work scheduling module
pub mod work_scheduling {
    use super::*;
    
    #[derive(Debug)]
    pub struct DualCoreWorkScheduler {
        queue: Vec<WorkItem>,
    }
    
    impl DualCoreWorkScheduler {
        pub fn new() -> Self {
            Self {
                queue: Vec::new(),
            }
        }
        
        pub fn schedule_work(&mut self, work: WorkItem) -> Result<(), String> {
            self.queue.push(work);
            Ok(())
        }
        
        pub fn get_status(&self, _id: u64) -> WorkStatus {
            WorkStatus::Completed
        }
    }
    
    pub use super::{WorkItem, WorkStatus};
}

// Extended capabilities for testing
impl CpuCapabilities {
    pub fn meets_min_requirements(&self) -> bool {
        // For testing, assume hardware meets minimum requirements
        true
    }
}

// Extended HardwareOptimizationManager for testing
impl HardwareOptimizationManager {
    pub fn detected_hardware_type(&self) -> HardwareType {
        HardwareType::Intel // Default for testing
    }
    
    pub fn detected_architecture(&self) -> String {
        "x86_64".to_string()
    }
    
    pub fn capabilities(&self) -> CpuCapabilities {
        CpuCapabilities::default()
    }
    
    pub fn optimize_operation(&self, _operation: OptimizableOperation, _data: &[u8]) -> Result<Vec<u8>, String> {
        // Mock optimization result
        Ok(vec![1, 2, 3, 4])
    }
}

// Extended IntelOptimizer for testing
impl IntelOptimizer {
    pub fn calculate_optimal_batch_size(&self) -> usize {
        256 // Default batch size for testing
    }
}
