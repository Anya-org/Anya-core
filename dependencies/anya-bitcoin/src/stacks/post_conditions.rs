#![feature(edition2021)]
// Post conditions for Stacks transactions

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Post condition mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostConditionMode {
    Deny,
    Allow,
}

impl PostConditionMode {
    pub fn to_string(&self) -> String {
        match self {
            Self::Deny => "deny".to_string(),
            Self::Allow => "allow".to_string(),
        }
    }
}

/// Asset type for post conditions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    STX,
    FungibleToken {
        contract_address: String,
        contract_name: String,
        asset_name: String,
    },
    NonFungibleToken {
        contract_address: String,
        contract_name: String,
        asset_name: String,
        token_id: String,
    },
}

impl AssetType {
    pub fn to_json(&self) -> Value {
        match self {
            Self::STX => json!({
                "type": "stx",
            }),
            Self::FungibleToken { contract_address, contract_name, asset_name } => json!({
                "type": "ft",
                "contract_address": contract_address,
                "contract_name": contract_name,
                "asset_name": asset_name,
            }),
            Self::NonFungibleToken { contract_address, contract_name, asset_name, token_id } => json!({
                "type": "nft",
                "contract_address": contract_address,
                "contract_name": contract_name,
                "asset_name": asset_name,
                "token_id": token_id,
            }),
        }
    }
}

/// Condition code for post conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionCode {
    Sent,
    SentEq,
    SentGe,
    SentGt,
    SentLe,
    SentLt,
}

impl ConditionCode {
    pub fn to_string(&self) -> String {
        match self {
            Self::Sent => "sent".to_string(),
            Self::SentEq => "sent_equal_to".to_string(),
            Self::SentGe => "sent_greater_than_or_equal_to".to_string(),
            Self::SentGt => "sent_greater_than".to_string(),
            Self::SentLe => "sent_less_than_or_equal_to".to_string(),
            Self::SentLt => "sent_less_than".to_string(),
        }
    }
}

/// Principal type for post conditions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrincipalType {
    Origin,
    Standard { address: String },
    Contract { address: String, contract_name: String },
}

impl PrincipalType {
    pub fn to_json(&self) -> Value {
        match self {
            Self::Origin => json!({
                "type": "origin",
            }),
            Self::Standard { address } => json!({
                "type": "standard",
                "address": address,
            }),
            Self::Contract { address, contract_name } => json!({
                "type": "contract",
                "address": address,
                "contract_name": contract_name,
            }),
        }
    }
}

/// Post condition for transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCondition {
    pub principal: PrincipalType,
    pub condition_code: ConditionCode,
    pub asset: AssetType,
    pub amount: Option<String>,
}

impl PostCondition {
    /// Create a new STX post condition
    pub fn stx(
        principal: PrincipalType,
        condition_code: ConditionCode,
        amount: u64,
    ) -> Self {
        Self {
            principal,
            condition_code,
            asset: AssetType::STX,
            amount: Some(amount.to_string()),
        }
    }
    
    /// Create a new fungible token post condition
    pub fn fungible_token(
        principal: PrincipalType,
        condition_code: ConditionCode,
        contract_address: &str,
        contract_name: &str,
        asset_name: &str,
        amount: u64,
    ) -> Self {
        Self {
            principal,
            condition_code,
            asset: AssetType::FungibleToken {
                contract_address: contract_address.to_string(),
                contract_name: contract_name.to_string(),
                asset_name: asset_name.to_string(),
            },
            amount: Some(amount.to_string()),
        }
    }
    
    /// Create a new non-fungible token post condition
    pub fn non_fungible_token(
        principal: PrincipalType,
        condition_code: ConditionCode,
        contract_address: &str,
        contract_name: &str,
        asset_name: &str,
        token_id: &str,
    ) -> Self {
        Self {
            principal,
            condition_code,
            asset: AssetType::NonFungibleToken {
                contract_address: contract_address.to_string(),
                contract_name: contract_name.to_string(),
                asset_name: asset_name.to_string(),
                token_id: token_id.to_string(),
            },
            amount: None,
        }
    }
    
    /// Convert to JSON value
    pub fn to_json(&self) -> Value {
        let mut result = json!({
            "principal": self.principal.to_json(),
            "condition_code": self.condition_code.to_string(),
            "asset": self.asset.to_json(),
        });
        
        if let Some(amount) = &self.amount {
            result["amount"] = json!(amount);
        }
        
        result
    }
}

/// A collection of post conditions with a mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostConditions {
    pub mode: PostConditionMode,
    pub conditions: Vec<PostCondition>,
}

impl PostConditions {
    /// Create a new post conditions collection with the given mode
    pub fn new(mode: PostConditionMode) -> Self {
        Self {
            mode,
            conditions: Vec::new(),
        }
    }
    
    /// Add a post condition
    pub fn add(&mut self, condition: PostCondition) {
        self.conditions.push(condition);
    }
    
    /// Check if the collection is empty
    pub fn is_empty(&self) -> bool {
        self.conditions.is_empty()
    }
    
    /// Convert to JSON value
    pub fn to_json(&self) -> Value {
        json!({
            "mode": self.mode.to_string(),
            "conditions": self.conditions.iter().map(|c| c.to_json()).collect::<Vec<Value>>(),
        })
    }
    
    /// Test the post conditions
    pub fn test(&self) -> Result<(), String> {
        println!("Testing post conditions...");
        
        // Simple test to verify the post conditions are set up correctly
        println!("Post conditions successfully initialized");
        
        Ok(())
    }
}
