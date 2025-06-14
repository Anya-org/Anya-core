// Shared Layer 2 types for all protocols
// This file centralizes ProtocolState, AssetParams, AssetTransfer, TransferResult, Proof, VerificationResult, ValidationResult
// so all Layer 2 modules can import from one place.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProtocolState {
    pub initialized: bool,
    pub connected: bool,
    pub last_block_height: Option<u64>,
    pub last_sync_time: Option<u64>,
    pub data: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetParams {
    pub name: String,
    pub symbol: String,
    pub supply: u64,
    pub precision: u8,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetTransfer {
    pub asset_id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferResult {
    pub tx_id: String,
    pub asset_id: String,
    pub status: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Proof {
    pub id: String,
    pub proof_type: String,
    pub data: Vec<u8>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationResult {
    Valid,
    Invalid(String),
    Pending,
}

impl Default for VerificationResult {
    fn default() -> Self {
        Self::Valid
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationResult {
    Valid,
    Invalid(String),
    Pending,
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::Valid
    }
}
