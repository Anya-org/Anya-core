use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Bitcoin-Inspired AGT Governance Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AGTGovernanceProtocol {
    /// Total supply fixed at 21 million
    pub total_supply: u64,
    /// Initial block reward
    pub initial_block_reward: u64,
    /// Halving interval (same as Bitcoin)
    pub halving_interval: u64,
    /// Current block reward
    pub current_block_reward: u64,
    /// Blocks mined
    pub blocks_mined: u64,
}

impl AGTGovernanceProtocol {
    /// Create new governance protocol with Bitcoin-like supply
    pub fn new() -> Self {
        Self {
            total_supply: 21_000_000 * 100_000_000, // 21M with 8 decimal places
            initial_block_reward: 50 * 100_000_000, // 50 coins
            halving_interval: 210_000, // Bitcoin halving cycle
            current_block_reward: 50 * 100_000_000,
            blocks_mined: 0,
        }
    }

    /// Calculate total mined supply
    pub fn calculate_total_mined_supply(&self) -> u64 {
        let mut total_mined = 0;
        let mut current_reward = self.initial_block_reward;
        let mut blocks_processed = 0;

        while blocks_processed < self.blocks_mined && total_mined < self.total_supply {
            let cycle_blocks = std::cmp::min(
                self.halving_interval, 
                self.blocks_mined - blocks_processed
            );

            let cycle_supply = current_reward * cycle_blocks;
            total_mined += cycle_supply;
            
            // Halve reward every 210,000 blocks
            if blocks_processed % self.halving_interval == 0 {
                current_reward /= 2;
            }

            blocks_processed += cycle_blocks;
        }

        total_mined
    }

    /// Verify if minting would exceed max supply
    pub fn can_mint(&self, amount: u64) -> bool {
        self.calculate_total_mined_supply() + amount <= self.total_supply
    }
}

/// Governance Voting Mechanism
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DAOGovernance {
    /// Voting parameters
    pub voting_threshold: f64,
    pub proposal_threshold: u64,
    pub quorum_percentage: f64,

    /// Proposal tracking
    pub active_proposals: RwLock<Vec<Proposal>>,

    /// Governance token protocol
    pub token_protocol: Arc<Mutex<AGTGovernanceProtocol>>,

    /// Delegation mapping (voter -> delegate)
    pub delegations: RwLock<HashMap<String, String>>,
}

impl DAOGovernance {
    pub fn new() -> Self {
        Self {
            voting_threshold: 0.6, // 60% majority
            proposal_threshold: 100, // 100 AGT to propose
            quorum_percentage: 0.3, // 30% participation required
            active_proposals: RwLock::new(Vec::new()),
            token_protocol: Arc::new(Mutex::new(AGTGovernanceProtocol::new())),
            delegations: RwLock::new(HashMap::new()),
        }
    }

    /// Submit a new proposal
    pub async fn submit_proposal(&self, proposal: Proposal) -> Result<(), String> {
        let mut proposals = self.active_proposals.write().await;
        
        // Check proposer's token balance meets threshold
        if proposal.proposer_token_balance < self.proposal_threshold {
            return Err("Insufficient tokens to submit proposal".to_string());
        }

        proposals.push(proposal);
        Ok(())
    }

    /// Calculate the cost of votes based on quadratic voting
    pub fn calculate_vote_cost(&self, num_votes: u64) -> u64 {
        num_votes * num_votes // Quadratic cost
    }

    /// Calculate the weight of a vote based on its timestamp
    pub fn calculate_time_weighted_vote(&self, vote: &Vote, voting_start: DateTime<Utc>, voting_end: DateTime<Utc>) -> f64 {
        let total_duration = (voting_end - voting_start).num_seconds() as f64;
        let time_elapsed = (vote.timestamp - voting_start).num_seconds() as f64;
        let time_weight = 1.0 - (time_elapsed / total_duration);
        vote.voting_power as f64 * time_weight
    }

    /// Cast a time-weighted vote on a proposal
    pub async fn cast_time_weighted_vote(&self, proposal_id: u64, voter: String, decision: VoteDecision, num_votes: u64, timestamp: DateTime<Utc>) -> Result<(), String> {
        let cost = self.calculate_vote_cost(num_votes);

        // Check if voter has enough voting power
        if cost > 100 { // Assume 100 is the voter's available voting power
            return Err("Insufficient voting power".to_string());
        }

        let mut proposals = self.active_proposals.write().await;

        if let Some(proposal) = proposals.iter_mut().find(|p| p.id == proposal_id) {
            proposal.votes.push(Vote {
                voter,
                decision,
                voting_power: cost,
                num_votes,
                timestamp,
            });
            Ok(())
        } else {
            Err("Proposal not found".to_string())
        }
    }

    /// Finalize proposal based on voting rules
    pub async fn finalize_proposal(&self, proposal_id: u64) -> Result<ProposalStatus, String> {
        let mut proposals = self.active_proposals.write().await;
        
        if let Some(proposal) = proposals.iter_mut().find(|p| p.id == proposal_id) {
            let total_votes: u64 = proposal.votes.iter().map(|v| v.voting_power).sum();
            let for_votes: u64 = proposal.votes.iter()
                .filter(|v| v.decision == VoteDecision::For)
                .map(|v| v.voting_power)
                .sum();

            let voting_percentage = (for_votes as f64) / (total_votes as f64);

            let status = if voting_percentage >= self.voting_threshold {
                ProposalStatus::Passed
            } else {
                ProposalStatus::Failed
            };

            proposal.status = status;
            Ok(status)
        } else {
            Err("Proposal not found".to_string())
        }
    }

    /// Adjust the voting threshold based on historical participation data
    pub async fn adjust_voting_threshold(&self) -> Result<(), String> {
        let proposals = self.active_proposals.read().await;
        let total_proposals = proposals.len() as f64;
        if total_proposals == 0.0 {
            return Ok(()); // No proposals to analyze
        }

        // Calculate average participation rate
        let total_votes: u64 = proposals.iter().flat_map(|p| p.votes.iter()).map(|v| v.voting_power).sum();
        let average_participation = total_votes as f64 / total_proposals;

        // Adjust voting threshold based on average participation
        // Example: Increase threshold if participation is high, decrease if low
        if average_participation > self.quorum_percentage * 1.5 {
            self.voting_threshold = (self.voting_threshold + 0.05).min(0.8); // Cap at 80%
        } else if average_participation < self.quorum_percentage * 0.5 {
            self.voting_threshold = (self.voting_threshold - 0.05).max(0.5); // Floor at 50%
        }

        Ok(())
    }

    /// Perform sentiment analysis on a proposal's description
    pub fn analyze_sentiment(&self, description: &str) -> f64 {
        // Placeholder for sentiment analysis integration
        // In a real implementation, this would involve calling a sentiment analysis API or library
        // For now, we'll use a simple heuristic: positive words increase score, negative words decrease it
        let positive_words = vec!["good", "excellent", "positive", "beneficial", "advantageous"];
        let negative_words = vec!["bad", "poor", "negative", "detrimental", "disadvantageous"];

        let mut score = 0.0;
        for word in description.split_whitespace() {
            if positive_words.contains(&word.to_lowercase().as_str()) {
                score += 1.0;
            } else if negative_words.contains(&word.to_lowercase().as_str()) {
                score -= 1.0;
            }
        }

        score
    }

    /// Use sentiment analysis in proposal scoring
    pub async fn score_proposals_with_sentiment(&self) -> Result<(), String> {
        let proposals = self.active_proposals.read().await;

        for proposal in proposals.iter() {
            let sentiment_score = self.analyze_sentiment(&proposal.description);
            println!("Sentiment score for proposal '{}': {}", proposal.title, sentiment_score);
        }

        Ok(())
    }

    /// Transition a proposal to the next stage
    pub async fn transition_proposal_stage(&self, proposal_id: u64) -> Result<(), String> {
        let mut proposals = self.active_proposals.write().await;

        if let Some(proposal) = proposals.iter_mut().find(|p| p.id == proposal_id) {
            match proposal.status {
                ProposalStatus::Draft => proposal.status = ProposalStatus::Review,
                ProposalStatus::Review => proposal.status = ProposalStatus::Voting,
                ProposalStatus::Voting => proposal.status = ProposalStatus::Executed,
                _ => return Err("Proposal cannot be transitioned from its current state".to_string()),
            }
            Ok(())
        } else {
            Err("Proposal not found".to_string())
        }
    }

    /// Delegate voting power to another user
    pub async fn delegate_vote(&self, voter: String, delegate: String) -> Result<(), String> {
        let mut delegations = self.delegations.write().await;
        delegations.insert(voter, delegate);
        Ok(())
    }

    /// Revoke delegation of voting power
    pub async fn revoke_delegation(&self, voter: String) -> Result<(), String> {
        let mut delegations = self.delegations.write().await;
        delegations.remove(&voter);
        Ok(())
    }

    /// Get the effective voter (considering delegation)
    pub async fn get_effective_voter(&self, voter: &str) -> String {
        let delegations = self.delegations.read().await;
        if let Some(delegate) = delegations.get(voter) {
            delegate.clone()
        } else {
            voter.to_string()
        }
    }
}

/// Proposal data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub proposer_token_balance: u64,
    pub votes: Vec<Vote>,
    pub status: ProposalStatus,
    /// Categories for the proposal
    pub categories: Vec<String>,
    /// Tags for the proposal
    pub tags: Vec<String>,
}

/// Vote representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub voter: String,
    pub decision: VoteDecision,
    pub voting_power: u64,
    /// Number of votes cast (for quadratic voting)
    pub num_votes: u64,
    /// Timestamp of the vote
    pub timestamp: DateTime<Utc>,
}

/// Proposal status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalStatus {
    Draft,
    Review,
    Voting,
    Executed,
    Passed,
    Failed,
}

/// Vote decision enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoteDecision {
    For,
    Against,
    Abstain,
}
