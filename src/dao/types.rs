use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub creator: String,
    pub start_block: u64,
    pub end_block: u64,
    pub ml_metrics: ProposalMetrics,
    pub agent_feedback: AgentFeedback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalMetrics {
    pub sentiment_score: f64,
    pub risk_assessment: RiskMetrics,
    pub ml_predictions: HashMap<String, f64>,
    pub federated_consensus: FederatedConsensus,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFeedback {
    pub recommendations: Vec<String>,
    pub confidence_level: f64,
    pub system_metrics: SystemMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub support: bool,
    pub power: u64,
    pub ml_confidence: f64,
    pub agent_suggestion: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProposalStatus {
    Pending,
    Active,
    Succeeded,
    Defeated,
    Executed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMetrics {
    pub financial_risk: f64,
    pub operational_risk: f64,
    pub security_risk: f64,
    pub compliance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedConsensus {
    pub participant_count: u32,
    pub agreement_ratio: f64,
    pub confidence_score: f64,
    pub training_rounds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub performance_metrics: HashMap<String, f64>,
    pub resource_utilization: ResourceMetrics,
    pub health_status: SystemHealth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub network_latency: f64,
    pub storage_usage: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SystemHealth {
    Optimal,
    Healthy,
    Degraded,
    Critical,
}

impl Proposal {
    pub fn new(title: String, description: String) -> Self {
        Self {
            title,
            description,
            creator: String::new(),
            start_block: 0,
            end_block: 0,
            ml_metrics: ProposalMetrics::default(),
            agent_feedback: AgentFeedback::default(),
        }
    }
}

impl Vote {
    pub fn new(support: bool, power: u64) -> Self {
        Self {
            support,
            power,
            ml_confidence: 0.0,
            agent_suggestion: None,
        }
    }

    pub fn with_ml_analysis(support: bool, power: u64, confidence: f64, suggestion: Option<String>) -> Self {
        Self {
            support,
            power,
            ml_confidence: confidence,
            agent_suggestion: suggestion,
        }
    }
}

impl Default for ProposalMetrics {
    fn default() -> Self {
        Self {
            sentiment_score: 0.0,
            risk_assessment: RiskMetrics::default(),
            ml_predictions: HashMap::new(),
            federated_consensus: FederatedConsensus::default(),
            last_updated: Utc::now(),
        }
    }
}

impl Default for RiskMetrics {
    fn default() -> Self {
        Self {
            financial_risk: 0.0,
            operational_risk: 0.0,
            security_risk: 0.0,
            compliance_score: 0.0,
        }
    }
}

impl Default for FederatedConsensus {
    fn default() -> Self {
        Self {
            participant_count: 0,
            agreement_ratio: 0.0,
            confidence_score: 0.0,
            training_rounds: 0,
        }
    }
}

impl Default for AgentFeedback {
    fn default() -> Self {
        Self {
            recommendations: Vec::new(),
            confidence_level: 0.0,
            system_metrics: SystemMetrics::default(),
        }
    }
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            performance_metrics: HashMap::new(),
            resource_utilization: ResourceMetrics::default(),
            health_status: SystemHealth::Healthy,
        }
    }
}

impl Default for ResourceMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            network_latency: 0.0,
            storage_usage: 0.0,
        }
    }
}