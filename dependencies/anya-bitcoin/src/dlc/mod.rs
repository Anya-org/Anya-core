#![feature(edition2021)]
// SPDX-License-Identifier: MIT
// Copyright (C) 2023-2025 Anya Project Contributors

//! Discrete Log Contracts (DLC) implementation
//!
//! DLCs enable conditional payments based on the outcome of future events,
//! using oracle signatures to determine the outcome without revealing the
//! contract details on the blockchain. This module provides functionality
//! for creating, managing, and executing DLCs while preserving privacy.

mod oracle;
mod contract;
mod adaptor;
mod cet;

pub use oracle::{Oracle, OracleEvent, OracleAttestation, OraclePublicKey};
pub use contract::{DlcContract, ContractInfo, ContractOutcome, ContractBuilder, ContractOutput};
pub use adaptor::{AdaptorSignature, AdaptorSigner};
pub use cet::{ContractExecutionTransaction, CetBuilder};

use std::sync::Arc;

use anyhow::{anyhow, Result};
use bitcoin::{OutPoint, Script, ScriptBuf, Transaction, TxOut, Address, Txid, Network};
use bitcoin::psbt::Psbt;
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use secp256k1_zkp::{schnorr::Signature, XOnlyPublicKey};
use serde::{Serialize, Deserialize};

use crate::wallet::BitcoinWallet;

/// Create a new DLC contract
///
/// This is a convenience function for quickly creating a DLC contract.
/// For more control, use the `ContractBuilder` directly.
pub async fn create_contract(
    wallet: &BitcoinWallet,
    outcomes: Vec<(String, u64)>,
    oracle_public_key: &str,
) -> Result<DlcContract> {
    // Parse the oracle public key
    let oracle_pk = oracle::OraclePublicKey::from_hex(oracle_public_key)?;
    
    // Create a contract builder
    let mut builder = ContractBuilder::new(wallet, oracle_pk);
    
    // Add outcomes
    for (outcome_name, amount) in outcomes {
        builder.add_outcome(outcome_name, amount);
    }
    
    // Build the contract
    builder.build().await
}

/// Execute a DLC contract based on an oracle attestation
///
/// This will create and sign the appropriate Contract Execution Transaction (CET)
/// based on the oracle's attestation of the event outcome.
pub async fn execute_contract(
    wallet: &BitcoinWallet,
    contract: &DlcContract,
    attestation: OracleAttestation,
) -> Result<Txid> {
    // Verify the attestation
    if !attestation.verify(&contract.oracle_public_key) {
        return Err(anyhow!("Invalid oracle attestation"));
    }
    
    // Find the matching outcome
    let outcome = contract.outcomes.iter()
        .find(|o| o.name == attestation.outcome)
        .ok_or_else(|| anyhow!("Unknown outcome: {}", attestation.outcome))?;
    
    // Build the CET
    let cet_builder = CetBuilder::new(contract, outcome, &attestation);
    let (cet, sig) = cet_builder.build(wallet).await?;
    
    // Broadcast the transaction
    let txid = wallet.broadcast_transaction(&cet).await?;
    
    Ok(txid)
}

/// Refund a DLC contract after timeout
///
/// If the oracle never attests to an outcome or if both parties agree
/// to cancel, this function creates and signs a refund transaction.
pub async fn refund_contract(
    wallet: &BitcoinWallet,
    contract: &DlcContract,
) -> Result<Txid> {
    // Check if the timeout has been reached
    let current_height = wallet.get_blockchain_height().await?;
    if current_height < contract.timeout_block_height {
        return Err(anyhow!("Contract timeout has not been reached yet"));
    }
    
    // Create the refund transaction
    let refund_tx = contract::create_refund_transaction(wallet, contract).await?;
    
    // Broadcast the transaction
    let txid = wallet.broadcast_transaction(&refund_tx).await?;
    
    Ok(txid)
}

/// List active DLC contracts
///
/// Returns all active DLC contracts that the wallet is a party to.
pub async fn list_contracts(wallet: &BitcoinWallet) -> Result<Vec<DlcContract>> {
    contract::list_active_contracts(wallet).await
}

/// Get the status of a DLC contract
///
/// Checks the blockchain to determine the current status of a contract.
pub async fn get_contract_status(
    wallet: &BitcoinWallet,
    contract: &DlcContract,
) -> Result<ContractStatus> {
    // Check if the funding transaction is confirmed
    let funding_tx_status = wallet.get_transaction(&contract.funding_txid).await?;
    if funding_tx_status.is_none() || funding_tx_status.unwrap().confirmations < 1 {
        return Ok(ContractStatus::Pending);
    }
    
    // Check if any CET has been executed
    for outcome in &contract.outcomes {
        if let Some(cet_txid) = &outcome.execution_txid {
            let cet_status = wallet.get_transaction(cet_txid).await?;
            if cet_status.is_some() && cet_status.unwrap().confirmations >= 1 {
                return Ok(ContractStatus::Executed {
                    outcome: outcome.name.clone(),
                    txid: *cet_txid,
                });
            }
        }
    }
    
    // Check if refund has been executed
    if let Some(refund_txid) = &contract.refund_txid {
        let refund_status = wallet.get_transaction(refund_txid).await?;
        if refund_status.is_some() && refund_status.unwrap().confirmations >= 1 {
            return Ok(ContractStatus::Refunded {
                txid: *refund_txid,
            });
        }
    }
    
    // Check if timeout has been reached
    let current_height = wallet.get_blockchain_height().await?;
    if current_height >= contract.timeout_block_height {
        return Ok(ContractStatus::TimedOut);
    }
    
    // Contract is active
    Ok(ContractStatus::Active)
}

/// Status of a DLC contract
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContractStatus {
    /// Contract funding transaction is not yet confirmed
    Pending,
    
    /// Contract is active and awaiting outcome
    Active,
    
    /// Contract has been executed with a specific outcome
    Executed {
        /// The outcome that was attested
        outcome: String,
        
        /// Transaction ID of the executed CET
        txid: Txid,
    },
    
    /// Contract has been refunded
    Refunded {
        /// Transaction ID of the refund transaction
        txid: Txid,
    },
    
    /// Contract timeout has been reached but not yet refunded
    TimedOut,
}

/// Tests for DLC functionality
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::wallet::WalletConfig;
    
    #[tokio::test]
    async fn test_dlc_contract_creation() {
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
        let oracle = Oracle::new("Weather Oracle");
        let oracle_pk = oracle.public_key();
        
        // Create outcomes for the DLC
        let outcomes = vec![
            ("sunny".to_string(), 1_000_000), // 1M sats if sunny
            ("rainy".to_string(), 500_000),   // 500K sats if rainy
        ];
        
        // Create the contract
        let contract = create_contract(&wallet, outcomes, &oracle_pk.to_hex()).await.unwrap();
        
        // Verify contract properties
        assert_eq!(contract.oracle_public_key, oracle_pk);
        assert_eq!(contract.outcomes.len(), 2);
        assert_eq!(contract.outcomes[0].name, "sunny");
        assert_eq!(contract.outcomes[0].amount, 1_000_000);
        
        // Attest to an outcome
        let event = OracleEvent {
            id: "weather-2023-04-01".to_string(),
            outcome: "sunny".to_string(),
        };
        
        let attestation = oracle.attest(&event);
        
        // Execute the contract
        let txid = execute_contract(&wallet, &contract, attestation).await.unwrap();
        
        // Check contract status
        let status = get_contract_status(&wallet, &contract).await.unwrap();
        assert!(matches!(status, ContractStatus::Executed { .. }));
        
        if let ContractStatus::Executed { outcome, .. } = status {
            assert_eq!(outcome, "sunny");
        }
    }
}
