#![feature(edition2021)]
// SPDX-License-Identifier: MIT
// Copyright (C) 2023-2025 Anya Project Contributors

//! Contract Execution Transaction (CET) implementation for Discrete Log Contracts
//!
//! CETs are pre-signed transactions that execute a DLC based on a specific outcome.
//! Each possible outcome of a DLC has a corresponding CET, which can only be
//! executed when the oracle releases a signature attesting to that outcome.

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use bitcoin::{Address, Network, OutPoint, Script, ScriptBuf, Transaction, TxOut, Txid};
use bitcoin::psbt::Psbt;
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey, Message};
use serde::{Serialize, Deserialize};
use tracing::{debug, error, info, warn};

use super::contract::{DlcContract, ContractOutcome};
use super::oracle::OracleAttestation;
use super::adaptor::{AdaptorSignature, AdaptorSigner, BasicAdaptorSigner};
use crate::wallet::BitcoinWallet;

/// Contract Execution Transaction for DLC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractExecutionTransaction {
    /// The raw transaction
    pub transaction: Transaction,
    
    /// The outcome this CET corresponds to
    pub outcome: String,
    
    /// The oracle attestation used to execute the CET
    pub attestation: Option<OracleAttestation>,
}

impl ContractExecutionTransaction {
    /// Create a new CET
    pub fn new(transaction: Transaction, outcome: String) -> Self {
        Self {
            transaction,
            outcome,
            attestation: None,
        }
    }
    
    /// Set the oracle attestation
    pub fn with_attestation(mut self, attestation: OracleAttestation) -> Self {
        self.attestation = Some(attestation);
        self
    }
    
    /// Get the transaction ID
    pub fn txid(&self) -> Txid {
        self.transaction.txid()
    }
}

/// Builder for Contract Execution Transactions
pub struct CetBuilder<'a> {
    /// Reference to the DLC contract
    contract: &'a DlcContract,
    
    /// The outcome for this CET
    outcome: &'a ContractOutcome,
    
    /// Oracle attestation for the outcome
    attestation: &'a OracleAttestation,
    
    /// Fee rate in sat/vB
    fee_rate: u64,
}

impl<'a> CetBuilder<'a> {
    /// Create a new CET builder
    pub fn new(
        contract: &'a DlcContract,
        outcome: &'a ContractOutcome,
        attestation: &'a OracleAttestation,
    ) -> Self {
        Self {
            contract,
            outcome,
            attestation,
            fee_rate: 2, // Default fee rate
        }
    }
    
    /// Set the fee rate
    pub fn with_fee_rate(mut self, fee_rate: u64) -> Self {
        self.fee_rate = fee_rate;
        self
    }
    
    /// Build the CET and its signature
    pub async fn build(
        &self,
        wallet: &BitcoinWallet,
    ) -> Result<(Transaction, Vec<u8>)> {
        // Create the transaction
        let transaction = self.create_transaction(wallet).await?;
        
        // In a real implementation, we would use the oracle's signature
        // to complete the adaptor signature. For now, we'll just create
        // a dummy signature.
        let signature = vec![0u8; 64];
        
        Ok((transaction, signature))
    }
    
    /// Create the CET transaction
    async fn create_transaction(&self, wallet: &BitcoinWallet) -> Result<Transaction> {
        // Get the contract output
        let contract_outpoint = OutPoint {
            txid: self.contract.funding_txid,
            vout: self.contract.funding_vout,
        };
        
        // Calculate fee (in a real implementation, we would calculate
        // based on transaction size and fee rate)
        let fee = 1000; // Placeholder
        
        // Calculate the payout amount
        let payout_amount = self.outcome.amount.saturating_sub(fee);
        
        // Get the payout address
        let payout_address = wallet.get_address(None).await?;
        
        // Create the transaction
        let transaction = Transaction {
            version: 2,
            lock_time: 0,
            input: vec![bitcoin::TxIn {
                previous_output: contract_outpoint,
                script_sig: Script::new().into(),
                sequence: 0xFFFFFFFF,
                witness: vec![], // Will be filled when signing
            }],
            output: vec![TxOut {
                value: payout_amount,
                script_pubkey: payout_address.script_pubkey,
            }],
        };
        
        Ok(transaction)
    }
}

/// Create CET transactions for all possible outcomes of a contract
pub async fn create_cet_transactions(
    wallet: &BitcoinWallet,
    contract: &DlcContract,
) -> Result<HashMap<String, ContractExecutionTransaction>> {
    let mut cets = HashMap::new();
    
    // In a real implementation, we would create proper CETs
    // for each outcome. For now, we'll create placeholder transactions.
    
    for outcome in &contract.outcomes {
        // Create a dummy transaction
        let transaction = Transaction {
            version: 2,
            lock_time: 0,
            input: vec![bitcoin::TxIn {
                previous_output: OutPoint {
                    txid: contract.funding_txid,
                    vout: contract.funding_vout,
                },
                script_sig: Script::new().into(),
                sequence: 0xFFFFFFFF,
                witness: vec![],
            }],
            output: vec![TxOut {
                value: outcome.amount - 1000, // Subtract fee
                script_pubkey: wallet.get_address(None).await?.script_pubkey,
            }],
        };
        
        let cet = ContractExecutionTransaction::new(transaction, outcome.name.clone());
        cets.insert(outcome.name.clone(), cet);
    }
    
    Ok(cets)
}

/// Sign a CET with an adaptor signature
pub fn sign_cet_with_adaptor(
    cet: &Transaction,
    contract: &DlcContract,
    secret_key: &SecretKey,
    oracle_pubkey: &PublicKey,
) -> Result<AdaptorSignature> {
    // In a real implementation, we would create a proper adaptor signature
    // For now, we'll create a placeholder
    let adaptor_signer = BasicAdaptorSigner::new(secret_key.clone());
    
    // Create a message to sign (transaction hash)
    let txid = cet.txid();
    let message = txid.as_ref();
    
    adaptor_signer.create_adaptor_signature(message, oracle_pubkey)
}

/// Complete a CET signature using an oracle attestation
pub fn complete_cet_signature(
    cet: &Transaction,
    adaptor_signature: &AdaptorSignature,
    oracle_attestation: &OracleAttestation,
    secret_key: &SecretKey,
) -> Result<Vec<u8>> {
    // In a real implementation, we would properly complete the signature
    // For now, we'll create a placeholder
    let adaptor_signer = BasicAdaptorSigner::new(secret_key.clone());
    
    adaptor_signer.complete_signature(adaptor_signature, &oracle_attestation.signature)
}

/// Tests for CET functionality
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::wallet::WalletConfig;
    use super::super::oracle::{Oracle, OracleEvent, OraclePublicKey};
    
    #[tokio::test]
    async fn test_cet_creation() {
        // Create test wallet
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("wallet.db");
        
        let config = WalletConfig {
            name: "test_wallet".to_string(),
            database_path: db_path,
            network: Network::Testnet,
            electrum_url: "ssl://electrum.blockstream.info:60002".to_string(),
            password: None,
            mnemonic: None,
            use_taproot: true,
        };
        
        let wallet = BitcoinWallet::new(config).await.unwrap();
        
        // Create an oracle
        let oracle = Oracle::new("Test Oracle");
        let oracle_pk = oracle.public_key();
        
        // Create a dummy contract
        let contract = DlcContract {
            id: "test-contract".to_string(),
            oracle_public_key: oracle_pk.clone(),
            funding_txid: Txid::all_zeros(),
            funding_vout: 0,
            funding_amount: 1_000_000,
            timeout_block_height: 100000,
            outcomes: vec![
                ContractOutcome {
                    name: "sunny".to_string(),
                    amount: 1_000_000,
                    execution_txid: None,
                    adaptor_signature: None,
                },
                ContractOutcome {
                    name: "rainy".to_string(),
                    amount: 500_000,
                    execution_txid: None,
                    adaptor_signature: None,
                },
            ],
            refund_txid: None,
            maturity_time: 100000,
            local_pubkey: PublicKey::from_slice(&[3; 33]).unwrap(),
            remote_pubkey: PublicKey::from_slice(&[2; 33]).unwrap(),
            contract_script: ScriptBuf::new(),
            contract_address: Address::from_script(&ScriptBuf::new(), Network::Testnet).unwrap(),
            info: None,
        };
        
        // Create CETs for all outcomes
        let cets = create_cet_transactions(&wallet, &contract).await.unwrap();
        
        // Check that we have CETs for both outcomes
        assert_eq!(cets.len(), 2);
        assert!(cets.contains_key("sunny"));
        assert!(cets.contains_key("rainy"));
        
        // Check that the CETs have the correct structure
        let sunny_cet = &cets["sunny"];
        assert_eq!(sunny_cet.outcome, "sunny");
        assert_eq!(sunny_cet.transaction.input.len(), 1);
        assert_eq!(sunny_cet.transaction.output.len(), 1);
        
        // Check that the input references the contract funding
        assert_eq!(sunny_cet.transaction.input[0].previous_output.txid, contract.funding_txid);
        assert_eq!(sunny_cet.transaction.input[0].previous_output.vout, contract.funding_vout);
    }
}
