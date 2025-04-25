use std::error::Error;
// src/bitcoin/cross_chain/atomic_swaps.rs

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::bitcoin::error::BitcoinError;
use crate::AnyaResult;
use super::Asset;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwapStatus {
    Created,
    Initialized,
    Funded,
    Redeemed,
    Refunded,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DLCStatus {
    Created,
    Initialized,
    Funded,
    Executed,
    Refunded,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone)]
pub enum Network {
    Bitcoin,
    Lightning,
    Liquid,
    RGB,
    RSK,
    Stacks,
}

#[derive(Debug, Clone)]
pub struct AtomicSwap {
    pub id: String,
    pub asset_send: Asset,
    pub asset_receive: Asset,
    pub counterparty: String,
    pub timelock: u32,
    pub secret_hash: [u8; 32],
    pub status: SwapStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub transactions: HashMap<String, String>, // Chain -> TxID
}

#[derive(Debug, Clone)]
pub struct CrossChainDLC {
    pub id: String,
    pub network_a: Network,
    pub network_b: Network,
    pub terms: crate::bitcoin::dlc::contracts::ContractTerms,
    pub oracles: Vec<crate::bitcoin::dlc::oracle::Oracle>,
    pub status: DLCStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub contracts: HashMap<String, String>, // Chain -> Contract ID
}

impl AtomicSwap {
    pub fn initialize(&mut self, initiator_tx: &str) -> AnyaResult<()>  -> Result<(), Box<dyn Error>> {
        if self.status != SwapStatus::Created {
            return Err(BitcoinError::CrossChain(
                format!("Cannot initialize swap in state {:?}", self.status)
            ).into());
        }
        
        self.status = SwapStatus::Initialized;
        self.transactions.insert("initiator".to_string(), initiator_tx.to_string());
        
        Ok(())
    }
    
    pub fn fund(&mut self, counterparty_tx: &str) -> AnyaResult<()>  -> Result<(), Box<dyn Error>> {
        if self.status != SwapStatus::Initialized {
            return Err(BitcoinError::CrossChain(
                format!("Cannot fund swap in state {:?}", self.status)
            ).into());
        }
        
        self.status = SwapStatus::Funded;
        self.transactions.insert("counterparty".to_string(), counterparty_tx.to_string());
        
        Ok(())
    }
    
    pub fn redeem(&mut self, secret: [u8; 32], redeem_tx: &str) -> AnyaResult<()>  -> Result<(), Box<dyn Error>> {
        if self.status != SwapStatus::Funded {
            return Err(BitcoinError::CrossChain(
                format!("Cannot redeem swap in state {:?}", self.status)
            ).into());
        }
        
        // Verify secret
        let hash = crate::bitcoin::BitcoinHelper::sha256_hash(&secret);
        if hash != self.secret_hash {
            return Err(BitcoinError::CrossChain("Invalid secret".to_string()).into());
        }
        
        self.status = SwapStatus::Redeemed;
        self.transactions.insert("redeem".to_string(), redeem_tx.to_string());
        self.completed_at = Some(chrono::Utc::now());
        
        Ok(())
    }
    
    pub fn refund(&mut self, refund_tx: &str) -> AnyaResult<()>  -> Result<(), Box<dyn Error>> {
        if self.status != SwapStatus::Funded {
            return Err(BitcoinError::CrossChain(
                format!("Cannot refund swap in state {:?}", self.status)
            ).into());
        }
        
        self.status = SwapStatus::Refunded;
        self.transactions.insert("refund".to_string(), refund_tx.to_string());
        self.completed_at = Some(chrono::Utc::now());
        
        Ok(())
    }
}

impl CrossChainDLC {
    pub fn initialize(&mut self, contracts: HashMap<String, String>) -> AnyaResult<()>  -> Result<(), Box<dyn Error>> {
        if self.status != DLCStatus::Created {
            return Err(BitcoinError::CrossChain(
                format!("Cannot initialize DLC in state {:?}", self.status)
            ).into());
        }
        
        self.status = DLCStatus::Initialized;
        self.contracts = contracts;
        
        Ok(())
    }
    
    pub fn fund(&mut self) -> AnyaResult<()>  -> Result<(), Box<dyn Error>> {
        if self.status != DLCStatus::Initialized {
            return Err(BitcoinError::CrossChain(
                format!("Cannot fund DLC in state {:?}", self.status)
            ).into());
        }
        
        self.status = DLCStatus::Funded;
        
        Ok(())
    }
    
    pub fn execute(&mut self, outcome: &str) -> AnyaResult<()>  -> Result<(), Box<dyn Error>> {
        if self.status != DLCStatus::Funded {
            return Err(BitcoinError::CrossChain(
                format!("Cannot execute DLC in state {:?}", self.status)
            ).into());
        }
        
        self.status = DLCStatus::Executed;
        self.completed_at = Some(chrono::Utc::now());
        
        Ok(())
    }
} 
