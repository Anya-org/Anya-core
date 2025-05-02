//! Blockchain port for Bitcoin components
//! 
//! This port defines the blockchain interface for Bitcoin components in the hexagonal architecture.
//! It provides methods for interacting with the Bitcoin blockchain.

use bitcoin::{Block, BlockHash, BlockHeader, Transaction, TxOut, OutPoint, Txid};
use std::collections::HashMap;

/// Error types for blockchain operations
#[derive(Debug, thiserror::Error)]
pub enum BlockchainError {
    #[error("Block not found: {0}")]
    BlockNotFound(String),
    
    #[error("Transaction not found: {0}")]
    TxNotFound(String),
    
    #[error("Invalid block data: {0}")]
    InvalidBlockData(String),
    
    #[error("Chain reorganization: {0}")]
    Reorg(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
}

/// Blockchain result type
pub type BlockchainResult<T> = Result<T, BlockchainError>;

/// Block with metadata
#[derive(Clone)]
pub struct BlockWithMetadata {
    /// The block
    pub block: Block,
    
    /// Block height
    pub height: u32,
    
    /// Hash of next block (if available)
    pub next_block_hash: Option<BlockHash>,
    
    /// Confirmation count
    pub confirmations: u32,
    
    /// Size in bytes
    pub size: usize,
    
    /// Block weight
    pub weight: usize,
}

/// Transaction with metadata
#[derive(Clone)]
pub struct TransactionWithMetadata {
    /// The transaction
    pub tx: Transaction,
    
    /// Block hash (if confirmed)
    pub block_hash: Option<BlockHash>,
    
    /// Block height (if confirmed)
    pub block_height: Option<u32>,
    
    /// Confirmation count
    pub confirmations: u32,
    
    /// Transaction timestamp
    pub timestamp: u32,
}

/// UTXO set entry
#[derive(Clone)]
pub struct UTXOEntry {
    /// Transaction output
    pub tx_out: TxOut,
    
    /// Height at which this output was created
    pub height: u32,
    
    /// Whether this output is coinbase
    pub is_coinbase: bool,
}

/// Blockchain port interface
pub trait BlockchainPort {
    /// Get block by hash
    fn get_block(&self, hash: &BlockHash) -> BlockchainResult<Block>;
    
    /// Get block with metadata
    fn get_block_with_metadata(&self, hash: &BlockHash) -> BlockchainResult<BlockWithMetadata>;
    
    /// Get block by height
    fn get_block_by_height(&self, height: u32) -> BlockchainResult<Block>;
    
    /// Get transaction by hash
    fn get_transaction(&self, txid: &Txid) -> BlockchainResult<Transaction>;
    
    /// Get transaction with metadata
    fn get_transaction_with_metadata(&self, txid: &Txid) -> BlockchainResult<TransactionWithMetadata>;
    
    /// Get current blockchain height
    fn get_blockchain_height(&self) -> BlockchainResult<u32>;
    
    /// Get block hash for given height
    fn get_block_hash(&self, height: u32) -> BlockchainResult<BlockHash>;
    
    /// Get multiple block headers
    fn get_block_headers(&self, start_height: u32, count: u32) -> BlockchainResult<Vec<BlockHeader>>;
    
    /// Get UTXO by outpoint
    fn get_utxo(&self, outpoint: &OutPoint) -> BlockchainResult<Option<UTXOEntry>>;
    
    /// Get multiple UTXOs by outpoints
    fn get_utxos(&self, outpoints: &[OutPoint]) -> BlockchainResult<HashMap<OutPoint, UTXOEntry>>;
    
    /// Broadcast transaction to the network
    fn broadcast_transaction(&self, transaction: &Transaction) -> BlockchainResult<Txid>;
    
    /// Check if transaction is in mempool
    fn is_in_mempool(&self, txid: &Txid) -> BlockchainResult<bool>;
}

/// Extended blockchain methods for testnet
pub trait TestnetBlockchainPort: BlockchainPort {
    /// Generate a new block with the given transactions
    fn generate_block(&self, transactions: Vec<Transaction>) -> BlockchainResult<Block>;
    
    /// Generate multiple blocks
    fn generate_blocks(&self, count: u32) -> BlockchainResult<Vec<BlockHash>>;
    
    /// Mine a transaction into a block
    fn mine_transaction(&self, transaction: &Transaction) -> BlockchainResult<BlockHash>;
    
    /// Reset the chain to a specific height (for testing)
    fn reset_chain(&self, height: u32) -> BlockchainResult<()>;
}

/// Events that blockchain observers can subscribe to
pub enum BlockchainEvent {
    /// New block has been added to the chain
    NewBlock(BlockWithMetadata),
    
    /// Block has been removed due to reorg
    RemovedBlock(BlockHash),
    
    /// New transaction has been added to mempool
    NewTransaction(Transaction),
    
    /// Transaction has been confirmed in a block
    ConfirmedTransaction { txid: Txid, block_hash: BlockHash },
    
    /// Chain reorganization event
    ChainReorg { old_tip: BlockHash, new_tip: BlockHash, common_ancestor: BlockHash },
}

/// Observer interface for blockchain events
pub trait BlockchainObserver: Send + Sync {
    /// Handle blockchain events
    fn on_event(&self, event: BlockchainEvent);
}

/// Extended blockchain port with observer support
pub trait ObservableBlockchainPort: BlockchainPort {
    /// Register an observer for blockchain events
    fn register_observer(&self, observer: Box<dyn BlockchainObserver>);
    
    /// Unregister an observer
    fn unregister_observer(&self, id: usize);
}