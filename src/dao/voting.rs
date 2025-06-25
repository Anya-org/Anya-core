use serde::{Deserialize, Serialize};
/// Quadratic Voting Implementation [AIS-3][BPC-3][DAO-3]
// [AIR-3][AIS-3][BPC-3][RES-3] Removed unused import: std::error::Error
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VotingError {
    #[error("Invalid vote: {0}")]
    InvalidVote(String),

    #[error("Voting not allowed: {0}")]
    VotingNotAllowed(String),

    #[error("Quadratic calculation error: {0}")]
    QuadraticError(String),
}

/// Vote on a proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    /// Proposal being voted on
    pub proposal_id: String,

    /// Voter identifier
    pub voter: String,

    /// Vote power (squared for quadratic voting)
    pub power: u64,

    /// Direction of vote
    pub direction: VoteDirection,

    /// Bitcoin signature (BPC-3 compliance)
    pub bitcoin_signature: Option<String>,
}

/// Direction of a vote
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteDirection {
    /// Vote in favor
    For,

    /// Vote against
    Against,

    /// Abstain from voting
    Abstain,
}

/// Quadratic voting system for DAO governance
pub struct QuadraticVoting {
    /// Votes by proposal ID and voter
    votes: HashMap<String, HashMap<String, Vote>>,
}

impl Default for QuadraticVoting {
    fn default() -> Self {
        Self::new()
    }
}

impl QuadraticVoting {
    /// Create a new quadratic voting system
    pub fn new() -> Self {
        Self {
            votes: HashMap::new(),
        }
    }

    /// Cast a vote using quadratic voting rules
    pub fn cast_vote(&mut self, vote: Vote) -> Result<(), VotingError> {
        // Validate vote
        if vote.power == 0 {
            return Err(VotingError::InvalidVote(
                "Vote power cannot be zero".to_string(),
            ));
        }

        // For DAO-4, require Bitcoin signature
        if vote.bitcoin_signature.is_none() {
            return Err(VotingError::InvalidVote(
                "Bitcoin signature required for BPC-3 compliance".to_string(),
            ));
        }

        // Get or create proposal votes
        let proposal_votes = self
            .votes
            .entry(vote.proposal_id.clone())
            .or_default();

        // Store the vote
        proposal_votes.insert(vote.voter.clone(), vote);

        Ok(())
    }

    /// Calculate quadratic voting results for a proposal
    pub fn tally_votes(&self, proposal_id: &str) -> Result<VoteTally, VotingError> {
        let proposal_votes = self.votes.get(proposal_id).ok_or_else(|| {
            VotingError::InvalidVote(format!("No votes found for proposal {}", proposal_id))
        })?;

        let mut for_votes = 0.0;
        let mut against_votes = 0.0;
        let mut abstain_votes = 0.0;

        for vote in proposal_votes.values() {
            // Calculate quadratic vote power (square root of power)
            let quadratic_power = (vote.power as f64).sqrt();

            match vote.direction {
                VoteDirection::For => for_votes += quadratic_power,
                VoteDirection::Against => against_votes += quadratic_power,
                VoteDirection::Abstain => abstain_votes += quadratic_power,
            }
        }

        Ok(VoteTally {
            proposal_id: proposal_id.to_string(),
            for_votes,
            against_votes,
            abstain_votes,
            total_votes: proposal_votes.len(),
        })
    }
}

/// Results of a vote tally
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteTally {
    /// Proposal that was voted on
    pub proposal_id: String,

    /// Quadratic votes in favor
    pub for_votes: f64,

    /// Quadratic votes against
    pub against_votes: f64,

    /// Quadratic abstentions
    pub abstain_votes: f64,

    /// Total number of voters
    pub total_votes: usize,
}
