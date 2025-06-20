//! Developer Identity Registry
//! Maps GitHub usernames, GPG key hashes, and payment addresses.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DeveloperIdentity {
    pub github_username: String,
    pub gpg_key_hash: String,
    pub payment_address: String, // Could be Stacks, Bitcoin, or DID
    pub web5_did: Option<String>,
    pub reputation_score: u64,
    pub contribution_history: Vec<ContributionRecord>,
}

#[derive(Debug, Clone)]
pub struct ContributionRecord {
    pub commit_hash: String,
    pub timestamp: u64,
    pub weight: u64,
}

#[derive(Default)]
pub struct IdentityRegistry {
    pub identities: HashMap<String, DeveloperIdentity>, // github_username -> identity
}

impl IdentityRegistry {
    pub fn register_identity(&mut self, identity: DeveloperIdentity) {
        self.identities.insert(identity.github_username.clone(), identity);
    }

    pub fn get_identity(&self, github_username: &str) -> Option<&DeveloperIdentity> {
        self.identities.get(github_username)
    }
}
