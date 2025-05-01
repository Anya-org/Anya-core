// Bitcoin Transaction Validation Module
// [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
//
// Transaction validation according to Bitcoin Development Framework v2.5 requirements

use anyhow::{Result, Context, bail};
use bitcoin::{Transaction, OutPoint, TxOut, Txid, Amount};
use std::collections::HashMap;

/// Transaction validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Overall validity of transaction
    pub valid: bool,
    
    /// Specific validation checks
    pub checks: HashMap<String, bool>,
    
    /// Error messages for failed checks
    pub errors: HashMap<String, String>,
}

impl ValidationResult {
    /// Create a new validation result
    pub fn new() -> Self {
        Self {
            valid: true,
            checks: HashMap::new(),
            errors: HashMap::new(),
        }
    }
    
    /// Add a validation check result
    pub fn add_check(&mut self, name: &str, valid: bool, error_msg: Option<&str>) {
        self.checks.insert(name.to_string(), valid);
        
        if !valid {
            self.valid = false;
            if let Some(msg) = error_msg {
                self.errors.insert(name.to_string(), msg.to_string());
            }
        }
    }
    
    /// Get failed check names
    pub fn failed_checks(&self) -> Vec<String> {
        self.checks.iter()
            .filter(|(_, &valid)| !valid)
            .map(|(name, _)| name.clone())
            .collect()
    }
}

/// Transaction validator for Bitcoin transactions
pub struct TransactionValidator {
    /// UTXO set for validating inputs
    utxo_set: HashMap<OutPoint, TxOut>,
}

impl TransactionValidator {
    /// Create a new transaction validator
    pub fn new() -> Self {
        Self {
            utxo_set: HashMap::new(),
        }
    }
    
    /// Add a UTXO to the validator's UTXO set
    pub fn add_utxo(&mut self, outpoint: OutPoint, txout: TxOut) {
        self.utxo_set.insert(outpoint, txout);
    }
    
    /// Remove a UTXO from the validator's UTXO set
    pub fn remove_utxo(&mut self, outpoint: &OutPoint) -> Option<TxOut> {
        self.utxo_set.remove(outpoint)
    }
    
    /// Get a UTXO from the validator's UTXO set
    pub fn get_utxo(&self, outpoint: &OutPoint) -> Option<&TxOut> {
        self.utxo_set.get(outpoint)
    }
    
    /// Validate a Bitcoin transaction
    pub fn validate_transaction(&self, tx: &Transaction) -> Result<ValidationResult> {
        let mut result = ValidationResult::new();
        
        // Check 1: Transaction must have at least one input and one output
        let has_inputs_outputs = tx.input.len() > 0 && tx.output.len() > 0;
        result.add_check(
            "has_inputs_outputs",
            has_inputs_outputs,
            Some("Transaction must have at least one input and one output"),
        );
        
        // Check 2: Transaction inputs must reference valid UTXOs
        let mut inputs_exist = true;
        let mut total_input_value = Amount::from_sat(0);
        
        for input in &tx.input {
            let outpoint = &input.previous_output;
            
            if let Some(txout) = self.get_utxo(outpoint) {
                // Add input value
                total_input_value += txout.value;
            } else {
                inputs_exist = false;
                break;
            }
        }
        
        result.add_check(
            "inputs_exist",
            inputs_exist,
            Some("Transaction inputs must reference valid UTXOs"),
        );
        
        // Check 3: Transaction outputs must not be empty
        let outputs_not_empty = tx.output.iter().all(|output| output.value > Amount::ZERO);
        result.add_check(
            "outputs_not_empty",
            outputs_not_empty,
            Some("Transaction outputs must not be empty"),
        );
        
        // Check 4: Transaction must not create new coins (inputs >= outputs)
        let total_output_value = tx.output.iter().fold(Amount::ZERO, |sum, output| sum + output.value);
        let no_new_coins = total_input_value >= total_output_value;
        
        result.add_check(
            "no_new_coins",
            no_new_coins,
            Some("Transaction must not create new coins (inputs >= outputs)"),
        );
        
        // Check 5: Transaction must have reasonable fee
        if inputs_exist && no_new_coins {
            let fee = total_input_value - total_output_value;
            let fee_reasonable = fee <= Amount::from_btc(0.1)?; // 0.1 BTC max fee as a reasonable limit
            
            result.add_check(
                "fee_reasonable",
                fee_reasonable,
                Some("Transaction fee is unreasonably high"),
            );
        }
        
        // Check 6: Transaction weight must be within limits
        let tx_weight = tx.weight().to_wu() as u32;
        let weight_valid = tx_weight <= super::constants::MAX_STANDARD_TX_WEIGHT;
        
        result.add_check(
            "weight_valid",
            weight_valid,
            Some("Transaction weight exceeds maximum standard transaction weight"),
        );
        
        Ok(result)
    }
    
    /// Validate a transaction spends the specified outpoint
    pub fn validate_outpoint_spend(&self, tx: &Transaction, outpoint: &OutPoint) -> Result<bool> {
        for input in &tx.input {
            if input.previous_output == *outpoint {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// Check if transaction has Taproot inputs
    pub fn has_taproot_inputs(&self, tx: &Transaction) -> bool {
        for input in &tx.input {
            if !input.witness.is_empty() && input.witness[0].len() == 64 {
                // Schnorr signatures are 64 bytes
                return true;
            }
        }
        
        false
    }
} 