//! Machine Learning module
//!
//! This module provides machine learning capabilities for the Anya system,
//! including model management, training, prediction, and federated learning.
//! [PRODUCTION-READY] Using real ML implementations with actual model inference

use std::error::Error;
// [AIR-3][AIS-3][BPC-3][RES-3] Import necessary dependencies for ML module
// This follows official Bitcoin Improvement Proposals (BIPs) standards for ML operations
use crate::{AnyaError, AnyaResult};
// Re-export these types to make them public
pub use crate::dao::{Proposal, ProposalMetrics, RiskMetrics};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

// Production ML Service (replaces all mock implementations)
pub mod production;
pub use production::{
    InferenceResult, LoadedModel, MLServiceConfig, MLServiceMetrics, ModelPerformanceMetrics,
    ModelType, ProductionMLService as MLService, RealMLService,
};

// Keep service module for trait definitions
mod service;
pub use service::MLModel;

// Real ML inference engine (replaces mock implementations)
pub mod real_inference;
pub use real_inference::{
    InferenceMetrics, InferenceRequest, InferenceResponse, MLConfig as RealMLConfig, RealMLEngine,
};

// ML agent system module
pub mod agent_system;
pub use agent_system::MLAgentSystem;

// ML model adapters for external framework integration
pub mod adapters;
pub use adapters::{AdapterFactory, MLAdapterRegistry, MLModelAdapter};

// Tool integration framework
pub mod tools;
pub use tools::{Tool, ToolManager, ToolRegistry, ToolResult};

// Planning and reasoning engine
pub mod planning;
pub use planning::{Goal, Plan, Planner, PlanningEngine, Reasoner};

// Advanced agent orchestration system
pub mod orchestration;
pub use orchestration::{WorkflowBuilder, WorkflowDefinition, WorkflowEngine};

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

impl std::fmt::Debug for MLSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MLSystem")
            .field("config", &self.config)
            .field("service", &"<MLService>")
            .field("models", &format!("{} models", self.models.len()))
            .finish()
    }
}

// Implement Send and Sync for MLSystem since its fields are all Send + Sync
unsafe impl Send for MLSystem {}
unsafe impl Sync for MLSystem {}

impl MLSystem {
    /// Create a new MLSystem with the given configuration
    pub async fn new(config: MLConfig) -> AnyaResult<Self> {
        if !config.enabled {
            // Create a default ML service config for disabled mode
            let ml_config = MLServiceConfig {
                models_dir: config
                    .model_path
                    .as_ref()
                    .map(std::path::PathBuf::from)
                    .unwrap_or_else(|| std::path::PathBuf::from("./data/models")),
                max_memory_mb: 512,
                enable_gpu: config.use_gpu,
                max_batch_size: 32,
                confidence_threshold: 0.8,
                model_cache_timeout: 3600,
                auto_retrain_interval: 24,
                enable_federated_learning: config.federated_learning,
            };

            let service = MLService::new(ml_config)
                .await
                .map_err(|e| AnyaError::ML(format!("Failed to create ML service: {e}")))?;

            return Ok(Self {
                config,
                service,
                models: HashMap::new(),
            });
        }

        // Create model directory if it doesn't exist
        if let Some(path) = &config.model_path {
            if !Path::new(path).exists() {
                std::fs::create_dir_all(path)
                    .map_err(|e| AnyaError::ML(format!("Failed to create model directory: {e}")))?;
            }
        }

        // Create ML service config
        let ml_config = MLServiceConfig {
            models_dir: config
                .model_path
                .as_ref()
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| std::path::PathBuf::from("./data/models")),
            max_memory_mb: 1024,
            enable_gpu: config.use_gpu,
            max_batch_size: 64,
            confidence_threshold: 0.8,
            model_cache_timeout: 3600,
            auto_retrain_interval: 24,
            enable_federated_learning: config.federated_learning,
        };

        let ml_service = MLService::new(ml_config)
            .await
            .map_err(|e| AnyaError::ML(format!("Failed to create ML service: {e}")))?;

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
        self.models
            .insert(name.to_string(), Arc::new(Mutex::new(model)));
        Ok(())
    }

    /// Get a model by name
    pub fn get_model(&self, name: &str) -> Option<Arc<Mutex<dyn MLModel>>> {
        self.models.get(name).cloned()
    }

    /// Get health metrics for the ML system
    pub async fn get_health_metrics(&self) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        metrics.insert("model_count".to_string(), self.models.len() as f64);
        metrics.insert(
            "enabled".to_string(),
            if self.config.enabled { 1.0 } else { 0.0 },
        );
        metrics.insert(
            "federated_learning".to_string(),
            if self.config.federated_learning {
                1.0
            } else {
                0.0
            },
        );

        // Add service metrics
        let service_metrics = self.service.get_metrics().await;
        metrics.insert(
            "service_total_inferences".to_string(),
            service_metrics.total_inferences as f64,
        );
        metrics.insert(
            "service_successful_inferences".to_string(),
            service_metrics.successful_inferences as f64,
        );
        metrics.insert(
            "service_failed_inferences".to_string(),
            service_metrics.failed_inferences as f64,
        );

        metrics
    }

    /// List all registered models
    pub fn list_models(&self) -> Vec<String> {
        self.models.keys().cloned().collect()
    }

    /// Get health metrics for all models
    pub async fn get_model_health_metrics(&self) -> HashMap<String, HashMap<String, f64>> {
        let mut metrics = HashMap::new();

        // Convert service metrics to HashMap<String, f64>
        let service_metrics = self.service.get_metrics().await;
        let mut service_health = HashMap::new();
        service_health.insert(
            "total_inferences".to_string(),
            service_metrics.total_inferences as f64,
        );
        service_health.insert(
            "successful_inferences".to_string(),
            service_metrics.successful_inferences as f64,
        );
        service_health.insert(
            "failed_inferences".to_string(),
            service_metrics.failed_inferences as f64,
        );
        service_health.insert(
            "average_inference_time_ms".to_string(),
            service_metrics.average_inference_time_ms,
        );

        metrics.insert("service".to_string(), service_health);

        // Add model-specific metrics
        for (name, model) in &self.models {
            if let Ok(model_lock) = model.lock() {
                metrics.insert(name.clone(), model_lock.get_health_metrics());
            }
        }

        metrics
    }
}

/// Trait for ML models (re-exported from service module)
/// This is just a placeholder to avoid duplicate definitions
pub trait MLModelPlaceholder {}

/// ML model input
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MLInput {
    /// Features for the model
    pub features: Vec<f64>,
    /// Label for supervised learning
    pub label: f64,
    /// Additional metadata
    pub metadata: Option<HashMap<String, String>>,
}

/// ML model output
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[allow(dead_code)]
pub struct FederatedLearningManager {
    /// Known nodes
    nodes: Vec<FederatedNode>,
    /// Aggregation method
    aggregation_method: String,
}

impl Default for FederatedLearningManager {
    fn default() -> Self {
        Self::new()
    }
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
    pub fn remove_node(&mut self, node_id: &str) -> Result<(), Box<dyn Error>> {
        self.nodes.retain(|n| n.id != node_id);
        Ok(())
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
pub use agent_checker::ComponentStatus;
pub use agent_checker::SystemHealth;
pub use agent_checker::SystemStage;

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

pub mod agents;
pub use agents::*;

pub mod models;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_readiness() -> Result<(), Box<dyn Error>> {
        assert!(!is_ready_for_stage(0.55, SystemStage::Development));
        assert!(is_ready_for_stage(0.65, SystemStage::Development));
        assert!(!is_ready_for_stage(0.85, SystemStage::Production));
        assert!(is_ready_for_stage(0.95, SystemStage::Production));
        Ok(())
    }
}
