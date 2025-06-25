//! Reward Proposal Generator (skeleton)
//! Generates DAO proposals for developer rewards.

use super::contribution_analyzer::ContributionReport;

#[derive(Debug, Clone)]
pub struct RewardProposal {
    pub proposal_id: String,
    pub time_period: (u64, u64),
    pub contributors: Vec<(String, f64, u64)>, // (github_username, contribution_weight, proposed_amount)
    pub total_distribution_amount: u64,
    pub metrics_report_uri: String,
    pub categories: Vec<String>,
}

pub struct RewardProposalGenerator;

impl RewardProposalGenerator {
    pub fn generate(
        reports: Vec<ContributionReport>,
        total_distribution_amount: u64,
        time_period: (u64, u64),
    ) -> RewardProposal {
        // Placeholder: distribute equally for now
        let n = reports.len() as u64;
        let per_contributor = if n > 0 { total_distribution_amount / n } else { 0 };
        let contributors = reports
            .into_iter()
            .map(|r| (r.github_username, r.calculated_weight, per_contributor))
            .collect();
        RewardProposal {
            proposal_id: "test-proposal-001".to_string(),
            time_period,
            contributors,
            total_distribution_amount,
            metrics_report_uri: "ipfs://placeholder".to_string(),
            categories: vec!["Core Development".to_string()],
        }
    }
}
