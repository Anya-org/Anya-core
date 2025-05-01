// Bitcoin Interface Module
// [AIR-3][AIS-3][BPC-3]
//
// Interface types for Bitcoin operations that abstract away
// implementation details.

use bitcoin::block::Header as BitcoinBlockHeader;
use bitcoin::BlockHash;
use serde::{Serialize, Deserialize};

/// Block header interface type
/// 
/// This is a compatibility layer to avoid breaking changes from
/// different bitcoin library versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Block version
    pub version: i32,
    /// Previous block hash
    pub prev_blockhash: String,
    /// Merkle root hash
    pub merkle_root: String,
    /// Block timestamp
    pub time: u32,
    /// Block difficulty bits
    pub bits: u32,
    /// Block nonce
    pub nonce: u32,
}

impl From<BitcoinBlockHeader> for BlockHeader {
    fn from(header: BitcoinBlockHeader) -> Self {
        Self {
            version: header.version,
            prev_blockhash: header.prev_blockhash.to_string(),
            merkle_root: header.merkle_root.to_string(),
            time: header.time,
            bits: header.bits,
            nonce: header.nonce,
        }
    }
}

/// Block info interface type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    /// Block hash
    pub hash: String,
    /// Block height
    pub height: u32,
    /// Block header
    pub header: BlockHeader,
    /// Number of transactions
    pub tx_count: usize,
    /// Block size in bytes
    pub size: usize,
    /// Block weight
    pub weight: usize,
}

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