// Tokenomics models for Anya Core
use serde::{Deserialize, Serialize};

/// Token distribution model
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenDistribution {
    /// Percentage allocated to staking rewards
    pub staking_allocation: f64,
    /// Percentage allocated to ecosystem growth
    pub ecosystem_allocation: f64,
    /// Percentage allocated to protocol treasury
    pub treasury_allocation: f64,
    /// Percentage allocated to team and early contributors
    pub team_allocation: f64,
}

impl Default for TokenDistribution {
    fn default() -> Self {
        Self {
            staking_allocation: 0.40,
            ecosystem_allocation: 0.30,
            treasury_allocation: 0.20,
            team_allocation: 0.10,
        }
    }
}

/// Staking tier information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StakingTier {
    /// Name of the tier
    pub name: String,
    /// Minimum stake amount required for this tier
    pub minimum_stake: u64,
    /// Reward multiplier for this tier
    pub reward_multiplier: f64,
}

/// Represents an economic event in the system
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EconomicEvent {
    /// Timestamp of the event
    pub timestamp: u64,
    /// Type of event
    pub event_type: EconomicEventType,
    /// Amount of tokens involved
    pub amount: u64,
    /// Associated account/address
    pub account: String,
}

/// Types of economic events in the system
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EconomicEventType {
    /// Staking tokens
    Stake,
    /// Unstaking tokens
    Unstake,
    /// Reward distribution
    Reward,
    /// Fee payment
    Fee,
    /// Burning tokens
    Burn,
    /// Minting new tokens
    Mint,
}
