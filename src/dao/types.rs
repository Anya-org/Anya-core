//! DAO Types
//!
//! This module defines types for the DAO module, including proposals,
//! votes, and metrics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Proposal status enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalStatus {
    /// Proposal is being drafted
    Draft,
    /// Proposal is active and can be voted on
    Active,
    /// Proposal has passed but not executed
    Passed,
    /// Proposal was executed
    Executed,
    /// Proposal was rejected
    Rejected,
    /// Proposal was canceled
    Canceled,
}

/// A governance proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    /// Proposal ID
    pub id: String,
    /// Proposal title
    pub title: String,
    /// Proposal description
    pub description: String,
    /// Proposer address
    pub proposer: String,
    /// Proposed amount
    pub amount: u64,
    /// Votes for the proposal
    pub votes_for: u64,
    /// Votes against the proposal
    pub votes_against: u64,
    /// Proposal status
    pub status: ProposalStatus,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Execution timestamp (if executed)
    pub execution_time: Option<DateTime<Utc>>,
}

/// Metrics for a proposal from ML analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalMetrics {
    /// Sentiment score for the proposal
    pub sentiment_score: f64,
    /// Risk assessment for the proposal
    pub risk_assessment: RiskMetrics,
    /// ML predictions for the proposal
    pub ml_predictions: HashMap<String, f64>,
    /// Consensus from federated learning
    pub federated_consensus: HashMap<String, f64>,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

/// Risk metrics for a proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMetrics {
    /// Overall risk score
    pub risk_score: f64,
    /// Individual risk factors
    pub risk_factors: Vec<(String, f64)>,
    /// Suggestions for risk mitigation
    pub mitigation_suggestions: Vec<String>,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

/// A vote on a proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    /// Voter address
    pub voter: String,
    /// Proposal ID
    pub proposal_id: String,
    /// Vote amount
    pub amount: u64,
    /// Whether the vote is for or against
    pub vote_for: bool,
    /// Voting timestamp
    pub timestamp: DateTime<Utc>,
}

/// Token distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenDistribution {
    /// Token holders and their balances
    pub holders: HashMap<String, u64>,
    /// Total supply
    pub total_supply: u64,
    /// Circulating supply
    pub circulating_supply: u64,
    /// Distribution timestamp
    pub timestamp: DateTime<Utc>,
}

/// Governance parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceParams {
    /// Proposal threshold
    pub proposal_threshold: u64,
    /// Minimum quorum for proposals
    pub quorum: u64,
    /// Voting period in blocks
    pub voting_period_blocks: u32,
    /// Time lock period in blocks
    pub time_lock_blocks: u32,
    /// Whether to use quadratic voting
    pub quadratic_voting: bool,
    /// Whether to require time-locked execution
    pub time_locked_execution: bool,
}

impl Default for GovernanceParams {
    fn default() -> Self {
        Self {
            proposal_threshold: 100_000_000,  // 1 token with 8 decimals
            quorum: 1_000_000_000,            // 10 tokens with 8 decimals
            voting_period_blocks: 1008,       // ~1 week
            time_lock_blocks: 144,            // ~1 day
            quadratic_voting: true,
            time_locked_execution: true,
        }
    }
}

impl Default for Proposal {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            description: String::new(),
            proposer: String::new(),
            amount: 0,
            votes_for: 0,
            votes_against: 0,
            status: ProposalStatus::Draft,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            execution_time: None,
        }
    }
}

impl Proposal {
    pub fn new(title: String, description: String) -> Self {
        Self {
            id: String::new(),
            title,
            description,
            proposer: String::new(),
            amount: 0,
            votes_for: 0,
            votes_against: 0,
            status: ProposalStatus::Draft,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            execution_time: None,
        }
    }
}

impl Vote {
    pub fn new(voter: String, proposal_id: String, amount: u64, vote_for: bool) -> Self {
        Self {
            voter,
            proposal_id,
            amount,
            vote_for,
            timestamp: Utc::now(),
        }
    }
}

impl Default for ProposalMetrics {
    fn default() -> Self {
        Self {
            sentiment_score: 0.0,
            risk_assessment: RiskMetrics::default(),
            ml_predictions: HashMap::new(),
            federated_consensus: HashMap::new(),
            last_updated: Utc::now(),
        }
    }
}

impl Default for RiskMetrics {
    fn default() -> Self {
        Self {
            risk_score: 0.0,
            risk_factors: Vec::new(),
            mitigation_suggestions: Vec::new(),
            last_updated: Utc::now(),
        }
    }
} 