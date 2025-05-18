//! Machine Learning module
//!
//! This module provides machine learning capabilities for the Anya system,
//! including model management, training, prediction, and federated learning.

use std::error::Error;
use crate::AnyaError;
use crate::AnyaResult;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::path::Path;

mod service;
pub use service::MLService;

// ML agent system module
mod agent_system;
pub use agent_system::MLAgentSystem;

/// Configuration options for ML functionality
#[derive(Debug, Clone)]
pub struct MLConfig {
    /// Whether ML functionality is enabled
    pub enabled: bool,
    /// Path to model storage
    pub model_path: Option<String>,
    /// Whether to use GPU for ML
    pub use_gpu: bool,
    /// Whether to enable federated learning
    pub federated_learning: bool,
    /// Maximum model size in bytes
    pub max_model_size: usize,
}

impl Default for MLConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            model_path: Some("./data/models".to_string()),
            use_gpu: true,
            federated_learning: true,
            max_model_size: 100 * 1024 * 1024, // 100 MB
        }
    }
}

/// Core ML system implementation
pub struct MLSystem {
    config: MLConfig,
    service: MLService,
    models: HashMap<String, Arc<Mutex<dyn MLModel>>>,
}

impl MLSystem {
    /// Create a new MLSystem with the given configuration
    pub fn new(config: MLConfig) -> AnyaResult<Self> {
        if !config.enabled {
            return Ok(Self {
                config,
                service: MLService::new(),
                models: HashMap::new(),
            });
        }

        // Create model directory if it doesn't exist
        if let Some(path) = &config.model_path {
            if !Path::new(path).exists() {
                std::fs::create_dir_all(path).map_err(|e| {
                    AnyaError::ML(format!("Failed to create model directory: {}", e))
                })?;
            }
        }

        let ml_service = MLService::new();

        Ok(Self {
            config,
            service: ml_service,
            models: HashMap::new(),
        })
    }

    /// Get the ML service
    pub fn service(&self) -> &MLService {
        &self.service
    }

    /// Register a model with the ML system
    pub fn register_model<M: MLModel + 'static>(&mut self, name: &str, model: M) -> AnyaResult<()> {
        self.models.insert(name.to_string(), Arc::new(Mutex::new(model)));
        Ok(())
    }

    /// Get a model by name
    pub fn get_model(&self, name: &str) -> Option<Arc<Mutex<dyn MLModel>>> {
        self.models.get(name).cloned()
    }

    /// Get health metrics for the ML system
    pub fn get_health_metrics(&self) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        metrics.insert("model_count".to_string(), self.models.len() as f64);
        metrics.insert("enabled".to_string(), if self.config.enabled { 1.0 } else { 0.0 });
        metrics.insert("federated_learning".to_string(), if self.config.federated_learning { 1.0 } else { 0.0 });
        
        // Add more detailed metrics here if needed
        metrics
    }

    /// List all registered models
    pub fn list_models(&self) -> Vec<String> {
        self.models.keys().cloned().collect()
    }

    /// Get health metrics for all models
    pub fn get_model_health_metrics(&self) -> HashMap<String, HashMap<String, f64>> {
        let mut metrics = HashMap::new();
        
        // Add service metrics
        metrics.insert("service".to_string(), self.service.get_health_metrics());
        
        // Add model-specific metrics
        for (name, model) in &self.models {
            if let Ok(model_lock) = model.lock() {
                metrics.insert(name.clone(), model_lock.get_health_metrics());
            }
        }
        
        metrics
    }
}

/// Trait for ML models
pub trait MLModel: Send + Sync {
    /// Train the model with new data
    fn train(&mut self, features: &[f64], labels: &[f64]) -> AnyaResult<()>;
    
    /// Make predictions with the model
    fn predict(&self, features: &[f64]) -> AnyaResult<Vec<f64>>;
    
    /// Get health metrics for the model
    fn get_health_metrics(&self) -> HashMap<String, f64>;
}

/// ML model input
#[derive(Debug, Clone)]
pub struct MLInput {
    /// Features for the model
    pub features: Vec<f64>,
    /// Label for supervised learning
    pub label: f64,
    /// Additional metadata
    pub metadata: Option<HashMap<String, String>>,
}

/// ML model output
#[derive(Debug, Clone)]
pub struct MLOutput {
    /// Model prediction
    pub prediction: f64,
    /// Model confidence
    pub confidence: f64,
    /// Additional information
    pub additional_info: Option<HashMap<String, Vec<f64>>>,
}

/// Federated learning node
pub struct FederatedNode {
    /// Node identifier
    pub id: String,
    /// Node URL
    pub url: String,
    /// Public key for verification
    pub public_key: Vec<u8>,
}

/// Federated learning manager
pub struct FederatedLearningManager {
    /// Known nodes
    nodes: Vec<FederatedNode>,
    /// Aggregation method
    aggregation_method: String,
}

impl FederatedLearningManager {
    /// Create a new federated learning manager
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            aggregation_method: "average".to_string(),
        }
    }
    
    /// Add a node to the federation
    pub fn add_node(&mut self, node: FederatedNode) {
        self.nodes.push(node);
    }
    
    /// Remove a node from the federation
    pub fn remove_node(&mut self, node_id: &str)  -> Result<(), Box<dyn Error>> {
        self.nodes.retain(|n| n.id != node_id);
    }
    
    /// List all nodes in the federation
    pub fn list_nodes(&self) -> &[FederatedNode] {
        &self.nodes
    }
}

// AIP-002: ML Module Integration
// Exports ML-based agent checker functionality

// Agent checker module
pub mod agent_checker;

// Re-exports for convenience
pub use agent_checker::AgentChecker;
pub use agent_checker::SystemStage;
pub use agent_checker::ComponentStatus;
pub use agent_checker::SystemHealth;

// Development, Production, and Release thresholds
pub const DEV_THRESHOLD: f64 = 0.60;
pub const PROD_THRESHOLD: f64 = 0.90;
pub const RELEASE_THRESHOLD: f64 = 0.99;

/// Helper function to create an agent checker with default auto-save frequency (20)
pub fn create_agent_checker() -> AgentChecker {
    AgentChecker::new(20)
}

/// Helper function to determine if a system is ready for a given stage
pub fn is_ready_for_stage(health: f64, stage: SystemStage) -> bool {
    match stage {
        SystemStage::Development => health >= DEV_THRESHOLD,
        SystemStage::Production => health >= PROD_THRESHOLD,
        SystemStage::Release => health >= RELEASE_THRESHOLD,
        SystemStage::Unavailable => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stage_readiness() -> Result<(), Box<dyn Error>> {
        assert_eq!(is_ready_for_stage(0.55, SystemStage::Development), false);
        assert_eq!(is_ready_for_stage(0.65, SystemStage::Development), true);
        assert_eq!(is_ready_for_stage(0.85, SystemStage::Production), false);
        assert_eq!(is_ready_for_stage(0.95, SystemStage::Production), true);
        Ok(())
    }
}

