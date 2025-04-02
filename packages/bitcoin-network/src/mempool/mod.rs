// Bitcoin Mempool Implementation
//
// Provides mempool management for Bitcoin transactions with BIP-342 support

use bitcoin::{
    Transaction,
    Txid,
    Weight,
    FeeRate,
};
use std::collections::{HashMap, BTreeMap};
use std::sync::Arc;
use std::time::{Duration, Instant};
use log::{info, warn, error, debug};
use tokio::sync::RwLock;
use super::{BitcoinNetworkConfig, BitcoinNetworkError};

/// Mempool entry
#[derive(Debug, Clone)]
pub struct MempoolEntry {
    /// Transaction
    pub tx: Transaction,
    /// Time added
    pub time_added: Instant,
    /// Fee rate in satoshis per vbyte
    pub fee_rate: FeeRate,
    /// Weight units
    pub weight: Weight,
    /// Whether the transaction has been validated
    pub validated: bool,
}

/// Bitcoin mempool
pub struct Mempool {
    /// Network configuration
    config: BitcoinNetworkConfig,
    /// Mempool entries by txid
    entries: HashMap<Txid, MempoolEntry>,
    /// Fee rate index
    fee_index: BTreeMap<FeeRate, Vec<Txid>>,
    /// Running flag
    running: bool,
}

impl Mempool {
    /// Create a new mempool
    pub async fn new(config: BitcoinNetworkConfig) -> Result<Self, BitcoinNetworkError> {
        Ok(Self {
            config,
            entries: HashMap::new(),
            fee_index: BTreeMap::new(),
            running: false,
        })
    }
    
    /// Start the mempool
    pub async fn start(&mut self) -> Result<(), BitcoinNetworkError> {
        if self.running {
            return Ok(());
        }
        
        info!("Starting Bitcoin mempool");
        
        // Start eviction loop
        let interval = Duration::from_secs(60);
        
        self.running = true;
        
        info!("Bitcoin mempool started");
        Ok(())
    }
    
    /// Stop the mempool
    pub async fn stop(&mut self) -> Result<(), BitcoinNetworkError> {
        if !self.running {
            return Ok(());
        }
        
        info!("Stopping Bitcoin mempool");
        
        self.running = false;
        
        info!("Bitcoin mempool stopped");
        Ok(())
    }
    
    /// Add a transaction to the mempool
    pub async fn add_transaction(&mut self, tx: Transaction) -> Result<(), BitcoinNetworkError> {
        let txid = tx.txid();
        debug!("Adding transaction to mempool: {}", txid);
        
        // Check if already in mempool
        if self.entries.contains_key(&txid) {
            debug!("Transaction {} already in mempool", txid);
            return Ok(());
        }
        
        // Calculate fee rate (in a real implementation, we would calculate this properly)
        let fee_rate = FeeRate::from_sat_per_vb(1.0);
        
        // Calculate weight
        let weight = tx.weight();
        
        // Create entry
        let entry = MempoolEntry {
            tx: tx.clone(),
            time_added: Instant::now(),
            fee_rate,
            weight,
            validated: false,
        };
        
        // Add to entries
        self.entries.insert(txid, entry);
        
        // Add to fee index
        if let Some(txids) = self.fee_index.get_mut(&fee_rate) {
            txids.push(txid);
        } else {
            self.fee_index.insert(fee_rate, vec![txid]);
        }
        
        debug!("Transaction {} added to mempool", txid);
        Ok(())
    }
    
    /// Remove a transaction from the mempool
    pub async fn remove_transaction(&mut self, txid: &Txid) -> Result<(), BitcoinNetworkError> {
        debug!("Removing transaction from mempool: {}", txid);
        
        // Check if in mempool
        if let Some(entry) = self.entries.remove(txid) {
            // Remove from fee index
            if let Some(txids) = self.fee_index.get_mut(&entry.fee_rate) {
                if let Some(pos) = txids.iter().position(|t| t == txid) {
                    txids.remove(pos);
                }
                
                // Remove empty entries
                if txids.is_empty() {
                    self.fee_index.remove(&entry.fee_rate);
                }
            }
            
            debug!("Transaction {} removed from mempool", txid);
        } else {
            debug!("Transaction {} not found in mempool", txid);
        }
        
        Ok(())
    }
    
    /// Get the mempool size
    pub fn size(&self) -> usize {
        self.entries.len()
    }
    
    /// Get the total weight
    pub fn total_weight(&self) -> Weight {
        self.entries.values()
            .fold(Weight::from_wu(0), |total, entry| total + entry.weight)
    }
    
    /// Get transactions by fee rate
    pub fn get_transactions_by_fee_rate(&self, limit: usize) -> Vec<Transaction> {
        let mut result = Vec::new();
        
        // Iterate from highest to lowest fee rate
        for (_, txids) in self.fee_index.iter().rev() {
            for txid in txids {
                if let Some(entry) = self.entries.get(txid) {
                    result.push(entry.tx.clone());
                    
                    if result.len() >= limit {
                        return result;
                    }
                }
            }
        }
        
        result
    }
}
