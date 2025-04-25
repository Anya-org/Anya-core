use std::error::Error;
// src/bitcoin/dlc/execution.rs

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use bitcoin::Txid;
use bitcoin::secp256k1::Signature;

use crate::common::error::AnyaResult;
use super::contract::{Contract, ContractState};
use super::oracle::OracleAttestation;

/// Status of a DLC execution
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionStatus {
    /// Execution is pending (waiting for oracle attestation)
    Pending,
    
    /// Execution is in progress
    InProgress,
    
    /// Execution has been successful
    Successful,
    
    /// Execution has failed
    Failed(String),
    
    /// Execution has been refunded
    Refunded,
    
    /// Execution has timed out
    TimedOut,
}

/// Record of a DLC execution
#[derive(Debug, Clone)]
pub struct ExecutionRecord {
    /// Unique identifier
    pub id: String,
    
    /// Contract ID
    pub contract_id: String,
    
    /// Status of the execution
    pub status: ExecutionStatus,
    
    /// Oracle attestation used for execution
    pub attestation: Option<OracleAttestation>,
    
    /// Transaction ID of the execution transaction
    pub execution_txid: Option<Txid>,
    
    /// Timestamp when execution was initiated
    pub initiated_at: DateTime<Utc>,
    
    /// Timestamp when execution was completed
    pub completed_at: Option<DateTime<Utc>>,
    
    /// Outcome value used for execution
    pub outcome: Option<String>,
    
    /// Final distribution to offering party (in satoshis)
    pub final_offer_amount: Option<u64>,
    
    /// Final distribution to accepting party (in satoshis)
    pub final_accept_amount: Option<u64>,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ExecutionRecord {
    /// Creates a new execution record
    pub fn new(contract_id: String) -> Self  -> Result<(), Box<dyn Error>> {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            contract_id,
            status: ExecutionStatus::Pending,
            attestation: None,
            execution_txid: None,
            initiated_at: Utc::now(),
            completed_at: None,
            outcome: None,
            final_offer_amount: None,
            final_accept_amount: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Updates the status of the execution
    pub fn update_status(&mut self, status: ExecutionStatus)  -> Result<(), Box<dyn Error>> {
        self.status = status;
        
        if status == ExecutionStatus::Successful || 
           matches!(status, ExecutionStatus::Failed(_)) || 
           status == ExecutionStatus::Refunded ||
           status == ExecutionStatus::TimedOut {
            self.completed_at = Some(Utc::now());
        }
    }
    
    /// Sets the oracle attestation
    pub fn set_attestation(&mut self, attestation: OracleAttestation)  -> Result<(), Box<dyn Error>> {
        self.attestation = Some(attestation.clone());
        self.outcome = Some(attestation.outcome);
    }
    
    /// Sets the execution transaction ID
    pub fn set_execution_txid(&mut self, txid: Txid)  -> Result<(), Box<dyn Error>> {
        self.execution_txid = Some(txid);
    }
    
    /// Sets the final distribution amounts
    pub fn set_final_amounts(&mut self, offer_amount: u64, accept_amount: u64)  -> Result<(), Box<dyn Error>> {
        self.final_offer_amount = Some(offer_amount);
        self.final_accept_amount = Some(accept_amount);
    }
    
    /// Adds metadata to the execution record
    pub fn add_metadata(&mut self, key: &str, value: &str)  -> Result<(), Box<dyn Error>> {
        self.metadata.insert(key.to_string(), value.to_string());
    }
}

/// Manager for DLC executions
pub struct ExecutionManager {
    /// Map of contract ID to execution records
    executions: HashMap<String, Vec<ExecutionRecord>>,
}

impl ExecutionManager {
    /// Creates a new execution manager
    pub fn new() -> Self  -> Result<(), Box<dyn Error>> {
        Self {
            executions: HashMap::new(),
        }
    }
    
    /// Creates a new execution record for a contract
    pub fn create_execution(&mut self, contract_id: &str) -> AnyaResult<ExecutionRecord>  -> Result<(), Box<dyn Error>> {
        let record = ExecutionRecord::new(contract_id.to_string());
        
        let records = self.executions.entry(contract_id.to_string())
            .or_insert_with(Vec::new);
        
        records.push(record.clone());
        
        Ok(record)
    }
    
    /// Gets all execution records for a contract
    pub fn get_executions(&self, contract_id: &str) -> Vec<ExecutionRecord>  -> Result<(), Box<dyn Error>> {
        self.executions.get(contract_id)
            .map(|records| records.clone())
            .unwrap_or_else(Vec::new)
    }
    
    /// Gets the latest execution record for a contract
    pub fn get_latest_execution(&self, contract_id: &str) -> Option<ExecutionRecord>  -> Result<(), Box<dyn Error>> {
        self.executions.get(contract_id)
            .and_then(|records| records.last().cloned())
    }
    
    /// Gets an execution record by ID
    pub fn get_execution(&self, execution_id: &str) -> Option<ExecutionRecord>  -> Result<(), Box<dyn Error>> {
        for records in self.executions.values() {
            if let Some(record) = records.iter().find(|r| r.id == execution_id) {
                return Some(record.clone());
            }
        }
        
        None
    }
    
    /// Updates an execution record
    pub fn update_execution(&mut self, record: ExecutionRecord) -> AnyaResult<()>  -> Result<(), Box<dyn Error>> {
        if let Some(records) = self.executions.get_mut(&record.contract_id) {
            if let Some(index) = records.iter().position(|r| r.id == record.id) {
                records[index] = record;
                return Ok(());
            }
        }
        
        Err("Execution record not found".into())
    }
    
    /// Executes a contract with an oracle attestation
    pub fn execute_contract(
        &mut self, 
        contract: &mut Contract, 
        attestation: OracleAttestation
    ) -> AnyaResult<ExecutionRecord>  -> Result<(), Box<dyn Error>> {
        // Validate the contract state
        if contract.state != ContractState::Funded {
            return Err(format!("Contract is not in funded state: {:?}", contract.state).into());
        }
        
        // Create a new execution record
        let mut record = self.create_execution(&contract.id)?;
        
        // Set the attestation
        record.set_attestation(attestation);
        
        // Update the status to in progress
        record.update_status(ExecutionStatus::InProgress);
        
        // Update the execution record
        self.update_execution(record.clone())?;
        
        // Update the contract state
        contract.update_state(ContractState::Executed);
        
        Ok(record)
    }
    
    /// Processes a refund for a contract
    pub fn process_refund(&mut self, contract: &mut Contract) -> AnyaResult<ExecutionRecord>  -> Result<(), Box<dyn Error>> {
        // Validate contract state
        if contract.state != ContractState::Funded {
            return Err(format!("Contract is not in funded state: {:?}", contract.state).into());
        }
        
        // Create a new execution record
        let mut record = self.create_execution(&contract.id)?;
        
        // Update the status to refunded
        record.update_status(ExecutionStatus::Refunded);
        
        // Update the execution record
        self.update_execution(record.clone())?;
        
        // Update the contract state
        contract.update_state(ContractState::Refunded);
        
        Ok(record)
    }
} 
