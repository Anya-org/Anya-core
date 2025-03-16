//! DAO module
//! 
//! This module provides decentralized autonomous organization functionality,
//! including governance, voting, and proposal management.

use crate::AnyaResult;
use crate::AnyaError;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde_json;

pub mod types;
pub use types::{Proposal, ProposalMetrics, RiskMetrics};

/// Configuration options for DAO functionality
#[derive(Debug, Clone)]
pub struct DAOConfig {
    /// Whether DAO functionality is enabled
    pub enabled: bool,
    /// Governance contract address
    pub contract_address: Option<String>,
    /// Proposal threshold (minimum token amount)
    pub proposal_threshold: u64,
    /// Voting period in blocks
    pub voting_period_blocks: u32,
    /// Time lock period in blocks
    pub time_lock_blocks: u32,
}

impl Default for DAOConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            contract_address: None,
            proposal_threshold: 100_000_000,  // 1 token with 8 decimals
            voting_period_blocks: 1008,       // ~1 week
            time_lock_blocks: 144,            // ~1 day
        }
    }
}

/// Core DAO implementation
pub struct DAOManager {
    config: DAOConfig,
    proposals: HashMap<String, Proposal>,
}

impl DAOManager {
    /// Create a new DAOManager with the given configuration
    pub fn new(config: DAOConfig) -> AnyaResult<Self> {
        if !config.enabled {
            return Ok(Self {
                config,
                proposals: HashMap::new(),
            });
        }

        Ok(Self {
            config,
            proposals: HashMap::new(),
        })
    }

    /// Create a new proposal
    pub fn create_proposal(&mut self, title: &str, description: &str, amount: u64) -> AnyaResult<Proposal> {
        if amount < self.config.proposal_threshold {
            return Err(AnyaError::DAO(format!(
                "Proposal amount ({}) is below the threshold ({})",
                amount, self.config.proposal_threshold
            )));
        }

        let proposal_id = format!("proposal:{:x}", rand::random::<u64>());
        
        let proposal = Proposal {
            id: proposal_id.clone(),
            title: title.to_string(),
            description: description.to_string(),
            proposer: "unknown".to_string(), // Would be set from context in real implementation
            amount,
            votes_for: 0,
            votes_against: 0,
            status: types::ProposalStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            execution_time: None,
        };
        
        self.proposals.insert(proposal_id.clone(), proposal.clone());
        
        Ok(proposal)
    }

    /// Vote on a proposal
    pub fn vote(&mut self, proposal_id: &str, vote_for: bool, amount: u64) -> AnyaResult<()> {
        let proposal = self.proposals.get_mut(proposal_id)
            .ok_or_else(|| AnyaError::DAO(format!("Proposal not found: {}", proposal_id)))?;
        
        if proposal.status != types::ProposalStatus::Active {
            return Err(AnyaError::DAO(format!(
                "Proposal is not active: {:?}", proposal.status
            )));
        }
        
        if vote_for {
            proposal.votes_for += amount;
        } else {
            proposal.votes_against += amount;
        }
        
        proposal.updated_at = Utc::now();
        
        Ok(())
    }

    /// Get a proposal by ID
    pub fn get_proposal(&self, proposal_id: &str) -> AnyaResult<Proposal> {
        self.proposals.get(proposal_id)
            .cloned()
            .ok_or_else(|| AnyaError::DAO(format!("Proposal not found: {}", proposal_id)))
    }

    /// List all proposals
    pub fn list_proposals(&self) -> Vec<Proposal> {
        self.proposals.values().cloned().collect()
    }

    /// Execute a proposal
    pub fn execute_proposal(&mut self, proposal_id: &str) -> AnyaResult<()> {
        let proposal = self.proposals.get_mut(proposal_id)
            .ok_or_else(|| AnyaError::DAO(format!("Proposal not found: {}", proposal_id)))?;
        
        if proposal.status != types::ProposalStatus::Active {
            return Err(AnyaError::DAO(format!(
                "Proposal is not active: {:?}", proposal.status
            )));
        }
        
        if proposal.votes_for <= proposal.votes_against {
            return Err(AnyaError::DAO(format!(
                "Proposal does not have enough votes: {} vs {}",
                proposal.votes_for, proposal.votes_against
            )));
        }
        
        // In a real implementation, this would execute the proposal
        proposal.status = types::ProposalStatus::Executed;
        proposal.execution_time = Some(Utc::now());
        proposal.updated_at = Utc::now();
        
        Ok(())
    }

    /// Get the system status
    pub fn get_status(&self) -> (bool, u8) {
        let operational = self.config.enabled;
        let health = if operational {
            // Basic health check based on configuration
            if self.config.contract_address.is_some() {
                100
            } else {
                70 // Configured but no contract address
            }
        } else {
            0
        };
        
        (operational, health)
    }
    
    /// Get system metrics
    pub fn get_metrics(&self) -> HashMap<String, serde_json::Value> {
        let mut metrics = HashMap::new();
        
        // Add basic metrics
        metrics.insert("enabled".to_string(), serde_json::json!(self.config.enabled));
        metrics.insert("proposal_count".to_string(), serde_json::json!(self.proposals.len()));
        metrics.insert("proposal_threshold".to_string(), serde_json::json!(self.config.proposal_threshold));
        metrics.insert("voting_period_blocks".to_string(), serde_json::json!(self.config.voting_period_blocks));
        
        // Add contract address if available
        if let Some(address) = &self.config.contract_address {
            metrics.insert("contract_address".to_string(), serde_json::json!(address));
        }
        
        // Add proposal status counts
        let active_proposals = self.proposals.values()
            .filter(|p| p.status == types::ProposalStatus::Active)
            .count();
        let passed_proposals = self.proposals.values()
            .filter(|p| p.status == types::ProposalStatus::Passed)
            .count();
        let executed_proposals = self.proposals.values()
            .filter(|p| p.status == types::ProposalStatus::Executed)
            .count();
        
        metrics.insert("active_proposals".to_string(), serde_json::json!(active_proposals));
        metrics.insert("passed_proposals".to_string(), serde_json::json!(passed_proposals));
        metrics.insert("executed_proposals".to_string(), serde_json::json!(executed_proposals));
        
        metrics
    }
}

pub mod governance;
pub mod legal;
pub mod voting;

pub use governance::{DaoGovernance, DaoLevel, GovernanceError}; 