// Bitcoin Storage Adapter
//
// Implements storage adapters for Bitcoin data
// [AIR-3][AIS-3][BPC-3]

use anyhow::Result;
use bitcoin::{Transaction, Block, BlockHash, Txid};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::bitcoin::adapters::BitcoinStoragePort;

/// Bitcoin storage adapter using in-memory storage
pub struct BitcoinStorageAdapter {
    /// Transactions stored by txid
    transactions: Arc<Mutex<HashMap<Txid, Transaction>>>,
    /// Blocks stored by hash
    blocks: Arc<Mutex<HashMap<BlockHash, Block>>>,
}

impl BitcoinStorageAdapter {
    /// Create a new Bitcoin storage adapter
    pub fn new() -> Self {
        Self {
            transactions: Arc::new(Mutex::new(HashMap::new())),
            blocks: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Count stored transactions
    pub fn transaction_count(&self) -> usize {
        self.transactions.lock().unwrap().len()
    }
    
    /// Count stored blocks
    pub fn block_count(&self) -> usize {
        self.blocks.lock().unwrap().len()
    }
}

impl Default for BitcoinStorageAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl BitcoinStoragePort for BitcoinStorageAdapter {
    /// Store a transaction
    fn store_transaction(&self, tx: &Transaction) -> Result<()> {
        let mut transactions = self.transactions.lock().unwrap();
        transactions.insert(tx.txid(), tx.clone());
        Ok(())
    }
    
    /// Retrieve a transaction
    fn get_transaction(&self, txid: &Txid) -> Result<Option<Transaction>> {
        let transactions = self.transactions.lock().unwrap();
        Ok(transactions.get(txid).cloned())
    }
    
    /// Store a block
    fn store_block(&self, block: &Block) -> Result<()> {
        let mut blocks = self.blocks.lock().unwrap();
        blocks.insert(block.block_hash(), block.clone());
        Ok(())
    }
    
    /// Retrieve a block
    fn get_block(&self, hash: &BlockHash) -> Result<Option<Block>> {
        let blocks = self.blocks.lock().unwrap();
        Ok(blocks.get(hash).cloned())
    }
} 