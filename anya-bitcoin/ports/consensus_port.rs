//! Consensus port for Bitcoin components
//! 
//! This port defines the consensus interface for Bitcoin components in the hexagonal architecture.
//! It provides methods for consensus validation and rule enforcement.

use bitcoin::{Block, BlockHash, Transaction, TxOut, Script, Network};
use std::collections::HashMap;

/// Error types for consensus operations
#[derive(Debug, thiserror::Error)]
pub enum ConsensusError {
    #[error("Block consensus error: {0}")]
    Block(String),
    
    #[error("Transaction consensus error: {0}")]
    Transaction(String),
    
    #[error("Script consensus error: {0}")]
    Script(String),
    
    #[error("Chain consensus error: {0}")]
    Chain(String),
    
    #[error("Difficulty adjustment error: {0}")]
    Difficulty(String),
    
    #[error("Witness validation error: {0}")]
    Witness(String),
}

/// Consensus result type
pub type ConsensusResult<T> = Result<T, ConsensusError>;

/// Consensus parameters
#[derive(Clone)]
pub struct ConsensusParams {
    /// Network type
    pub network: Network,
    
    /// Maximum block size in bytes
    pub max_block_size: usize,
    
    /// Maximum block weight
    pub max_block_weight: usize,
    
    /// Minimum required difficulty bits
    pub min_difficulty_bits: u32,
    
    /// Flag to check if SegWit is active
    pub segwit_active: bool,
    
    /// Flag to check if Taproot is active
    pub taproot_active: bool,
}

impl Default for ConsensusParams {
    fn default() -> Self {
        Self {
            network: Network::Bitcoin,
            max_block_size: 1_000_000,
            max_block_weight: 4_000_000,
            min_difficulty_bits: 0x1d00ffff,
            segwit_active: true,
            taproot_active: true,
        }
    }
}

/// Consensus port interface
pub trait ConsensusPort {
    /// Get the consensus parameters
    fn get_consensus_params(&self) -> &ConsensusParams;
    
    /// Verify block according to consensus rules
    fn verify_block(&self, block: &Block, height: u32) -> ConsensusResult<()>;
    
    /// Verify transaction according to consensus rules
    fn verify_transaction(&self, tx: &Transaction, utxos: &HashMap<TxOut, u32>) -> ConsensusResult<()>;
    
    /// Verify block header
    fn verify_header(&self, block: &Block, prev_hash: &BlockHash) -> ConsensusResult<()>;
    
    /// Verify difficulty adjustment
    fn verify_difficulty(&self, block: &Block, prev_timestamp: u32, prev_bits: u32) -> ConsensusResult<()>;
    
    /// Verify witness data (for SegWit)
    fn verify_witness(&self, tx: &Transaction, input_index: usize, value: u64, script_pubkey: &Script) -> ConsensusResult<()>;
    
    /// Verify Taproot execution (for BIP-341)
    fn verify_taproot(&self, tx: &Transaction, input_index: usize, value: u64, script_pubkey: &Script) -> ConsensusResult<()>;
}

/// Extended consensus methods for BIP-341 (Taproot)
pub trait TaprootConsensusPort: ConsensusPort {
    /// Verify script path spend
    fn verify_script_path_spend(&self, 
                               tx: &Transaction, 
                               input_index: usize,
                               script: &Script,
                               leaf_version: u8,
                               control_block: &[u8]) -> ConsensusResult<()>;
    
    /// Verify key path spend
    fn verify_key_path_spend(&self,
                            tx: &Transaction,
                            input_index: usize,
                            internal_key: &[u8]) -> ConsensusResult<()>;
    
    /// Verify annex usage
    fn verify_annex(&self, tx: &Transaction, input_index: usize, annex: &[u8]) -> ConsensusResult<()>;
}

/// Extended consensus methods for testing and validation
pub trait TestConsensusPort: ConsensusPort {
    /// Create test block that passes consensus
    fn create_valid_block(&self, prev_hash: &BlockHash, transactions: Vec<Transaction>) -> ConsensusResult<Block>;
    
    /// Override consensus parameters for testing
    fn with_params(&self, params: ConsensusParams) -> Box<dyn TestConsensusPort>;
    
    /// Validate chain tip consensus
    fn validate_chain_tip(&self, tip_hash: &BlockHash, height: u32) -> ConsensusResult<()>;
}