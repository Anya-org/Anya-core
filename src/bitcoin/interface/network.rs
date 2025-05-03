// Bitcoin Network Interface Types
// [AIR-3][AIS-3][BPC-3]
//
// Network-related interface types for Bitcoin operations

use serde::{Serialize, Deserialize};

/// Network status interface type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    /// Network name (mainnet, testnet, etc.)
    pub network: String,
    /// Current block height
    pub height: u32,
    /// Mempool size in transactions
    pub mempool_size: usize,
    /// Connection count
    pub connections: usize,
    /// Network difficulty
    pub difficulty: f64,
    /// Median fee rate (sat/vB)
    pub median_fee_rate: f64,
} 