// Tokenomics rewards module for Anya Core

use crate::AnyaResult;
use crate::tokenomics::models::{StakingTier, EconomicEvent};

/// Rewards manager for the tokenomics system
pub struct RewardsManager {
    /// Staking tiers configuration
    tiers: Vec<StakingTier>,
    /// Base APY rate
    base_apy: f64,
    /// Current total staked
    total_staked: u64,
}

impl RewardsManager {
    /// Create a new rewards manager
    pub fn new(base_apy: f64) -> Self {
        let default_tiers = vec![
            StakingTier {
                name: "Bronze".to_string(),
                minimum_stake: 100,
                reward_multiplier: 1.0,
            },
            StakingTier {
                name: "Silver".to_string(),
                minimum_stake: 1000,
                reward_multiplier: 1.2,
            },
            StakingTier {
                name: "Gold".to_string(),
                minimum_stake: 10000,
                reward_multiplier: 1.5,
            },
            StakingTier {
                name: "Platinum".to_string(),
                minimum_stake: 100000,
                reward_multiplier: 2.0,
            },
        ];

        Self {
            tiers: default_tiers,
            base_apy,
            total_staked: 0,
        }
    }

    /// Calculate rewards for a given stake amount and duration
    pub fn calculate_rewards(&self, stake_amount: u64, days: u32) -> AnyaResult<f64> {
        // Find applicable tier
        let tier = self.tiers.iter()
            .filter(|t| stake_amount >= t.minimum_stake)
            .max_by_key(|t| t.minimum_stake)
            .unwrap_or(&self.tiers[0]);
        
        // Calculate base rewards
        let annual_rate = self.base_apy * tier.reward_multiplier;
        let daily_rate = annual_rate / 365.0;
        let rewards = (stake_amount as f64) * (daily_rate * days as f64);
        
        Ok(rewards)
    }
    
    /// Record a staking event
    pub fn record_stake(&mut self, account: &str, amount: u64) -> AnyaResult<EconomicEvent> {
        self.total_staked += amount;
        
        let event = EconomicEvent {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            event_type: crate::tokenomics::models::EconomicEventType::Stake,
            amount,
            account: account.to_string(),
        };
        
        Ok(event)
    }
    
    /// Get the total amount staked in the system
    pub fn get_total_staked(&self) -> u64 {
        self.total_staked
    }
}
