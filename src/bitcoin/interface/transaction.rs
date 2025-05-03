// Bitcoin Transaction Interface Types
// [AIR-3][AIS-3][BPC-3]
//
// Transaction-related interface types for Bitcoin operations

use serde::{Serialize, Deserialize};

/// Transaction info interface type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    /// Transaction ID
    pub txid: String,
    /// Transaction witness ID (if available)
    pub wtxid: Option<String>,
    /// Confirmed in block hash
    pub blockhash: Option<String>,
    /// Confirmed at block height
    pub height: Option<u32>,
    /// Transaction size in bytes
    pub size: usize,
    /// Transaction weight
    pub weight: usize,
    /// Transaction fee in satoshis
    pub fee: u64,
} 