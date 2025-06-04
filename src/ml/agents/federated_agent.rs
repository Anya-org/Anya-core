// Federated Learning Agent
//
// This module provides agents that facilitate federated learning across the network,
// enabling privacy-preserving machine learning that respects Bitcoin principles of
// decentralization and individual sovereignty.

use std::sync::{Arc, RwLock};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::Utc;

use crate::ml::{service::MLModel, FederatedLearningManager};
use crate::AnyaError;

use super::{Agent, AgentId, Observation, Action, Feedback, AgentMetrics, AgentError};

/// Configuration for federated learning agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedAgentConfig {
    /// Agent identifier
    pub id: String,
    
    /// Maximum model size in bytes
    pub max_model_size: usize,
    
    /// Minimum number of participants required
    pub min_participants: usize,
    
    /// Maximum aggregation delay in seconds
    pub max_aggregation_delay: u64,
    
    /// Privacy budget (epsilon value for differential privacy)
    pub privacy_budget: f64,
    
    /// Whether to prioritize reading before aggregation (read-first principle)
    pub read_first: bool,
}

impl Default for FederatedAgentConfig {
    fn default() -> Self {
        Self {
            id: "federated-default".to_string(),
            max_model_size: 100 * 1024 * 1024, // 100MB
            min_participants: 3,
            max_aggregation_delay: 86400, // 24 hours
            privacy_budget: 1.0,
            read_first: true, // Always prioritize reading first by default
        }
    }
}

/// Agent for coordinating federated learning
pub struct FederatedAgent {
    /// Unique identifier
    id: AgentId,
    
    /// Federated learning manager
    federation_manager: Arc<RwLock<FederatedLearningManager>>,
    
    /// Configuration
    config: FederatedAgentConfig,
    
    /// Metrics for this agent
    metrics: RwLock<AgentMetrics>,
    
    /// Participant state cache
    participant_state: RwLock<HashMap<String, ParticipantState>>,
    
    /// Current round state
    current_round: RwLock<Option<FederationRound>>,
}

/// State of a federation participant
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ParticipantState {
    /// Participant identifier
    id: String,
    
    /// Last active timestamp
    last_active: u64,
    
    /// Model hash (if available)
    model_hash: Option<String>,
    
    /// Contribution weight
    weight: f64,
    
    /// Number of training examples
    training_examples: u64,
    
    /// Reported model performance
    performance: HashMap<String, f64>,
}

/// A federation round
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FederationRound {
    /// Round identifier
    id: String,
    
    /// Start timestamp
    start_time: u64,
    
    /// End timestamp (or None if still active)
    end_time: Option<u64>,
    
    /// Participating nodes
    participants: Vec<String>,
    
    /// Round status
    status: FederationRoundStatus,
    
    /// Aggregation parameters
    aggregation_params: HashMap<String, f64>,
}

/// Status of a federation round
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum FederationRoundStatus {
    /// Round is being initialized
    Initializing,
    
    /// Models are being collected from participants (read-first phase)
    ReadingModels,
    
    /// Models are being evaluated
    EvaluatingModels,
    
    /// Aggregating models
    AggregatingModels,
    
    /// Distributing the aggregated model
    DistributingModel,
    
    /// Round completed successfully
    Completed,
    
    /// Round failed
    Failed(String),
}

impl FederatedAgent {
    /// Create a new federated learning agent
    pub fn new(config: FederatedAgentConfig) -> Self {
        // Initialize a new federation manager
        let federation_manager = Arc::new(RwLock::new(FederatedLearningManager::new()));
        
        Self {
            id: AgentId(config.id.clone()),
            federation_manager,
            config,
            metrics: RwLock::new(AgentMetrics::default()),
            participant_state: RwLock::new(HashMap::new()),
            current_round: RwLock::new(None),
        }
    }
    
    /// Start a new federation round
    pub async fn start_federation_round(&self) -> Result<String, AgentError> {
        let mut current_round = self.current_round.write().map_err(|_| {
            AgentError::InternalError("Failed to acquire write lock on current round".to_string())
        })?;
        
        // Check if there's already an active round
        if let Some(round) = current_round.as_ref() {
            if round.end_time.is_none() {
                return Err(AgentError::ProcessingError(
                    "There's already an active federation round".to_string()
                ));
            }
        }
        
        // Get current participant state
        let participant_state = self.participant_state.read().map_err(|_| {
            AgentError::InternalError("Failed to acquire read lock on participant state".to_string())
        })?;
        
        // Check if we have enough participants
        if participant_state.len() < self.config.min_participants {
            return Err(AgentError::ProcessingError(format!(
                "Not enough participants: {} (minimum: {})",
                participant_state.len(),
                self.config.min_participants
            )));
        }
        
        // Create a new round
        let round_id = format!("round-{}", Utc::now().timestamp());
        let round = FederationRound {
            id: round_id.clone(),
            start_time: Utc::now().timestamp() as u64,
            end_time: None,
            participants: participant_state.keys().cloned().collect(),
            status: FederationRoundStatus::Initializing,
            aggregation_params: HashMap::new(),
        };
        
        *current_round = Some(round);
        
        Ok(round_id)
    }
    
    /// Register a participant with the federation
    pub async fn register_participant(
        &self,
        participant_id: &str,
        weight: f64,
        training_examples: u64
    ) -> Result<(), AgentError> {
        let mut participant_state = self.participant_state.write().map_err(|_| {
            AgentError::InternalError("Failed to acquire write lock on participant state".to_string())
        })?;
        
        participant_state.insert(participant_id.to_string(), ParticipantState {
            id: participant_id.to_string(),
            last_active: Utc::now().timestamp() as u64,
            model_hash: None,
            weight,
            training_examples,
            performance: HashMap::new(),
        });
        
        Ok(())
    }
    
    /// Process a model update from a participant
    pub async fn process_model_update(
        &self,
        participant_id: &str,
        model_hash: &str,
        model_data: &[u8],
        performance: HashMap<String, f64>
    ) -> Result<(), AgentError> {
        // Update participant state
        {
            let mut participant_state = self.participant_state.write().map_err(|_| {
                AgentError::InternalError("Failed to acquire write lock on participant state".to_string())
            })?;
            if let Some(state) = participant_state.get_mut(participant_id) {
                state.last_active = Utc::now().timestamp() as u64;
                state.model_hash = Some(model_hash.to_string());
                state.performance = performance;
            } else {
                return Err(AgentError::ProcessingError(format!(
                    "Participant {} not registered",
                    participant_id
                )));
            }
        }
        
        // If we're in a read-first mode and a round is active, update its status
        {
            let mut current_round = self.current_round.write().map_err(|_| {
                AgentError::InternalError("Failed to acquire write lock on current round".to_string())
            })?;
            
            if let Some(round) = current_round.as_mut() {
                if self.config.read_first && round.status == FederationRoundStatus::Initializing {
                    round.status = FederationRoundStatus::ReadingModels;
                }
            }
        }
        
        // Validate model data
        if model_data.len() > self.config.max_model_size {
            return Err(AgentError::ProcessingError(format!(
                "Model size exceeds maximum: {} > {}",
                model_data.len(),
                self.config.max_model_size
            )));
        }
        
        // TODO: Store the model data for aggregation
        
        Ok(())
    }
    
    /// Aggregate models from participants
    pub async fn aggregate_models(&self, round_id: &str) -> Result<Vec<u8>, AgentError> {
        let mut current_round = self.current_round.write().map_err(|_| {
            AgentError::InternalError("Failed to acquire write lock on current round".to_string())
        })?;
        
        let round = current_round.as_mut().ok_or_else(|| {
            AgentError::ProcessingError("No active federation round".to_string())
        })?;
        
        if round.id != round_id {
            return Err(AgentError::ProcessingError(format!(
                "Round ID mismatch: {} != {}",
                round.id,
                round_id
            )));
        }
        
        // Ensure we've completed the reading phase if read_first is enabled
        if self.config.read_first && round.status != FederationRoundStatus::ReadingModels {
            return Err(AgentError::ProcessingError(
                "Must complete reading phase before aggregation".to_string()
            ));
        }
        
        // Update round status
        round.status = FederationRoundStatus::AggregatingModels;
        
        // TODO: Actual model aggregation logic
        
        // For now, just return a dummy result
        let aggregated_model = vec![0u8; 100];
        
        // Update round status
        round.status = FederationRoundStatus::DistributingModel;
        
        Ok(aggregated_model)
    }
    
    /// Complete a federation round
    pub async fn complete_round(&self, round_id: &str) -> Result<(), AgentError> {
        let mut current_round = self.current_round.write().map_err(|_| {
            AgentError::InternalError("Failed to acquire write lock on current round".to_string())
        })?;
        
        let round = current_round.as_mut().ok_or_else(|| {
            AgentError::ProcessingError("No active federation round".to_string())
        })?;
        
        if round.id != round_id {
            return Err(AgentError::ProcessingError(format!(
                "Round ID mismatch: {} != {}",
                round.id,
                round_id
            )));
        }
        
        // Update round status
        round.status = FederationRoundStatus::Completed;
        round.end_time = Some(Utc::now().timestamp() as u64);
        
        Ok(())
    }
}

#[async_trait]
impl Agent for FederatedAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    
    fn agent_type(&self) -> &str {
        "federated"
    }
    
    async fn process(&self, observation: Observation) -> Result<Option<Action>, AgentError> {
        // Update metrics
        {
            let mut metrics = self.metrics.write().map_err(|_| {
                AgentError::InternalError("Failed to acquire write lock on metrics".to_string())
            })?;
            metrics.observations_processed += 1;
        }
        
        match observation {
            Observation::Custom(name, data) => {
                match name.as_str() {
                    "FederationRoundRequest" => {
                        // Request to start a new federation round
                        let round_id = self.start_federation_round().await?;
                        
                        let action = Action::Custom(
                            "FederationRoundCreated".to_string(),
                            serde_json::to_vec(&round_id).unwrap_or_default()
                        );
                        
                        // Update metrics
                        {
                            let mut metrics = self.metrics.write().map_err(|_| {
                                AgentError::InternalError("Failed to acquire write lock on metrics".to_string())
                            })?;
                            metrics.actions_taken += 1;
                        }
                        
                        Ok(Some(action))
                    },
                    "ModelUpdateSubmission" => {
                        // Model update from a participant
                        if let Ok(submission) = serde_json::from_slice::<ModelUpdateSubmission>(&data) {
                            self.process_model_update(
                                &submission.participant_id,
                                &submission.model_hash,
                                &submission.model_data,
                                submission.performance
                            ).await?;
                            
                            let action = Action::Custom(
                                "ModelUpdateAccepted".to_string(),
                                serde_json::to_vec(&submission.participant_id).unwrap_or_default()
                            );
                            
                            // Update metrics
                            {
                                let mut metrics = self.metrics.write().map_err(|_| {
                                    AgentError::InternalError("Failed to acquire write lock on metrics".to_string())
                                })?;
                                metrics.actions_taken += 1;
                            }
                            
                            Ok(Some(action))
                        } else {
                            Err(AgentError::InvalidObservation(
                                "Invalid model update submission format".to_string()
                            ))
                        }
                    },
                    "AggregationRequest" => {
                        // Request to aggregate models
                        if let Ok(round_id) = serde_json::from_slice::<String>(&data) {
                            let aggregated_model = self.aggregate_models(&round_id).await?;
                            
                            let action = Action::Custom(
                                "AggregatedModel".to_string(),
                                aggregated_model
                            );
                            
                            // Update metrics
                            {
                                let mut metrics = self.metrics.write().map_err(|_| {
                                    AgentError::InternalError("Failed to acquire write lock on metrics".to_string())
                                })?;
                                metrics.actions_taken += 1;
                            }
                            
                            Ok(Some(action))
                        } else {
                            Err(AgentError::InvalidObservation(
                                "Invalid aggregation request format".to_string()
                            ))
                        }
                    },
                    _ => Ok(None), // Ignore other custom observations
                }
            },
            _ => Ok(None), // Ignore other observation types
        }
    }
    
    async fn receive_feedback(&mut self, feedback: Feedback) -> Result<(), AgentError> {
        // Update metrics
        let mut metrics = self.metrics.write().map_err(|_| {
            AgentError::InternalError("Failed to acquire write lock on metrics".to_string())
        })?;
        
        // Calculate new average feedback
        let total_feedback = metrics.average_feedback * metrics.actions_taken as f32;
        metrics.average_feedback = (total_feedback + feedback.score) / (metrics.actions_taken as f32 + 1.0);
        
        Ok(())
    }
    
    fn metrics(&self) -> AgentMetrics {
        self.metrics.read().map(|m| m.clone()).unwrap_or_default()
    }
    
    fn ethical_compliance(&self) -> f32 {
        // Federated learning is inherently privacy-preserving
        // but we should still check proper implementation details
        let compliance_factors = [
            (self.config.privacy_budget > 0.0, 0.3), // Differential privacy enabled
            (self.config.min_participants >= 3, 0.3), // Minimum participants for anonymity
            (self.config.read_first, 0.4), // Read-first principle enforced
        ];
        
        compliance_factors.iter()
            .map(|(condition, weight)| if *condition { *weight } else { 0.0 })
            .sum()
    }
}

/// Model update submission from a participant
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ModelUpdateSubmission {
    /// Participant identifier
    participant_id: String,
    
    /// Model hash for integrity verification
    model_hash: String,
    
    /// Actual model data
    model_data: Vec<u8>,
    
    /// Performance metrics for the model
    performance: HashMap<String, f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_federated_agent_round_lifecycle() {
        // Test the full lifecycle of a federation round
    }
    
    #[tokio::test]
    async fn test_federated_agent_read_first_principle() {
        // Test that the read-first principle is enforced
    }
}

