// SPDX-License-Identifier: MIT
// Copyright (C) 2023-2025 Anya Project Contributors

//! Bitcoin wallet implementation using Bitcoin Dev Kit (BDK)
//! 
//! This module provides secure wallet functionality for Bitcoin operations,
//! including key management, transaction creation, and signing.

use std::str::FromStr;
use std::sync::Arc;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use bitcoin::{Address, Network, Transaction, Txid, Script, ScriptBuf, BlockHash};
use bitcoin::consensus::encode;
use bitcoin::psbt::PartiallySignedTransaction as Psbt;
use bdk::{
    wallet::AddressIndex,
    Wallet, SyncOptions, FeeRate,
    database::SqliteDatabase,
    blockchain::{Blockchain, ElectrumBlockchain},
    electrum_client::Client,
    keys::{
        DerivableKey,
        bip39::{Mnemonic, Language, WordCount},
        ExtendedKey,
    },
    wallet::{tx_builder::TxBuilder, AddressInfo},
};
use bdk_macros::bdkwallet;
use miniscript::Descriptor;
use secp256k1::{rand, Secp256k1};
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use tracing::{info, warn, error, debug};

/// Configuration for a Bitcoin wallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConfig {
    /// Wallet name
    pub name: String,
    
    /// Path to the wallet database file
    pub database_path: PathBuf,
    
    /// Network to use (mainnet, testnet, regtest)
    pub network: Network,
    
    /// Electrum server URL
    pub electrum_url: String,
    
    /// Optional password for wallet encryption
    #[serde(skip_serializing)]
    pub password: Option<String>,
    
    /// Optional existing mnemonic phrase
    #[serde(skip_serializing)]
    pub mnemonic: Option<String>,
    
    /// Whether the wallet should use taproot descriptors
    pub use_taproot: bool,
}

/// Bitcoin wallet implementation
pub struct BitcoinWallet {
    /// Internal BDK wallet
    inner: Mutex<Wallet<SqliteDatabase>>,
    
    /// Wallet configuration
    config: WalletConfig,
    
    /// Electrum blockchain client
    blockchain: Arc<ElectrumBlockchain>,
}

impl BitcoinWallet {
    /// Create a new wallet with the given configuration
    pub async fn new(config: WalletConfig) -> Result<Self> {
        let secp = Secp256k1::new();
        
        // Generate or use existing mnemonic
        let mnemonic = match config.mnemonic {
            Some(ref phrase) => Mnemonic::from_str(phrase)?,
            None => {
                info!("Generating new wallet mnemonic");
                Mnemonic::new(&mut rand::thread_rng(), WordCount::Words12)
            }
        };
        
        // Derive keys from mnemonic
        let extended_key = mnemonic.into_extended_key()?;
        let xprv = extended_key.into_xprv(config.network)?;
        
        // Create descriptor based on configuration
        let descriptor = if config.use_taproot {
            // Taproot descriptor (BIP-386)
            format!("tr({}/*)", xprv)
        } else {
            // Native SegWit descriptor (BIP-84)
            format!("wpkh({}/*)", xprv)
        };
        
        // Create database
        let database_path = config.database_path.to_string_lossy();
        let database = SqliteDatabase::new(&database_path)?;
        
        // Create descriptors (we use the same for external and internal)
        let wallet = Wallet::new(
            &descriptor,
            Some(&descriptor),
            config.network,
            database,
        )?;
        
        // Connect to Electrum server
        let electrum_client = Client::new(&config.electrum_url)?;
        let blockchain = ElectrumBlockchain::from(electrum_client);
        
        // Create our wallet struct
        let bitcoin_wallet = Self {
            inner: Mutex::new(wallet),
            config: config.clone(),
            blockchain: Arc::new(blockchain),
        };
        
        // Initial sync with the blockchain
        bitcoin_wallet.sync().await?;
        
        Ok(bitcoin_wallet)
    }
    
    /// Sync the wallet with the blockchain
    pub async fn sync(&self) -> Result<()> {
        let mut wallet = self.inner.lock().await;
        wallet.sync(&self.blockchain, SyncOptions::default())?;
        
        let balance = wallet.get_balance()?;
        info!(
            "Wallet synced. Balance: {} confirmed, {} unconfirmed",
            balance.confirmed, balance.untrusted_pending
        );
        
        Ok(())
    }
    
    /// Get a new address from the wallet
    pub async fn get_address(&self, address_index: AddressIndex) -> Result<AddressInfo> {
        let wallet = self.inner.lock().await;
        let address = wallet.get_address(address_index)?;
        Ok(address)
    }
    
    /// Get the current balance
    pub async fn get_balance(&self) -> Result<bdk::Balance> {
        let wallet = self.inner.lock().await;
        let balance = wallet.get_balance()?;
        Ok(balance)
    }
    
    /// Send bitcoin to a specific address
    pub async fn send_to_address(
        &self,
        address: &str,
        amount_sats: u64,
        fee_rate: f32,
    ) -> Result<Txid> {
        let recipient = Address::from_str(address)?
            .require_network(self.config.network)?;
        
        let mut wallet = self.inner.lock().await;
        
        // Create transaction using the TxBuilder
        let (psbt, _details) = {
            wallet.build_tx()
                .add_recipient(recipient.script_pubkey(), amount_sats)
                .enable_rbf()
                .fee_rate(FeeRate::from_sat_per_vb(fee_rate))
                .finish()?
        };
        
        // Sign the transaction
        let finalized = wallet.sign(psbt, None)?;
        
        // Broadcast the transaction
        let raw_tx = finalized.extract_tx();
        let txid = self.blockchain.broadcast(&raw_tx)?;
        
        info!("Transaction sent: {}", txid);
        Ok(txid)
    }
    
    /// Create and sign a PSBT transaction but don't broadcast
    pub async fn create_psbt(
        &self,
        address: &str,
        amount_sats: u64,
        fee_rate: f32,
    ) -> Result<Psbt> {
        let recipient = Address::from_str(address)?
            .require_network(self.config.network)?;
        
        let mut wallet = self.inner.lock().await;
        
        // Create transaction using the TxBuilder
        let (mut psbt, _details) = {
            wallet.build_tx()
                .add_recipient(recipient.script_pubkey(), amount_sats)
                .enable_rbf()
                .fee_rate(FeeRate::from_sat_per_vb(fee_rate))
                .finish()?
        };
        
        // Sign the transaction
        wallet.sign(&mut psbt, None)?;
        
        Ok(psbt)
    }
    
    /// Create a PSBT with multiple outputs
    pub async fn create_multi_output_psbt(
        &self,
        outputs: Vec<(String, u64)>,
        fee_rate: Option<f32>,
    ) -> Result<Psbt> {
        // Set default fee rate if not provided
        let fee_rate = fee_rate.unwrap_or(1.0);
        
        let mut wallet = self.inner.lock().await;
        
        // Start creating transaction
        let mut builder = wallet.build_tx();
        
        // Add all outputs
        for (address, amount) in outputs {
            let recipient = Address::from_str(&address)?
                .require_network(self.config.network)?;
            
            builder.add_recipient(recipient.script_pubkey(), amount);
        }
        
        // Set fee rate and enable RBF
        builder.enable_rbf().fee_rate(FeeRate::from_sat_per_vb(fee_rate));
        
        // Finish building the PSBT
        let (psbt, _details) = builder.finish()?;
        
        debug!("Multi-output PSBT created with {} outputs", outputs.len());
        
        Ok(psbt)
    }
    
    /// Enhance a PSBT with metadata for hardware wallet compatibility
    pub async fn enhance_psbt_for_hardware(&self, psbt: &mut Psbt) -> Result<()> {
        let wallet = self.inner.lock().await;
        
        // Add UTXO information for each input
        for input in psbt.inputs.iter_mut() {
            if input.non_witness_utxo.is_none() && input.witness_utxo.is_none() {
                // Find the originating transaction for this input
                if let Some(utxo) = wallet.get_utxo(input.previous_txid, input.previous_output_index as u32)? {
                    // For SegWit inputs, add witness UTXO
                    if utxo.txout.script_pubkey.is_witness_program() {
                        input.witness_utxo = Some(utxo.txout.clone());
                    } else {
                        // For non-SegWit, add the full transaction
                        let tx = self.get_transaction(&utxo.outpoint.txid).await?;
                        input.non_witness_utxo = Some(tx);
                    }
                }
                
                // Add BIP32 derivation paths
                wallet.fill_signature_info(input)?;
            }
        }
        
        debug!("Enhanced PSBT for hardware wallet compatibility");
        
        Ok(())
    }
    
    /// Sign a PSBT with the wallet's private keys
    pub async fn sign_psbt_internal(&self, psbt: &mut Psbt) -> Result<bool> {
        let mut wallet = self.inner.lock().await;
        let signed = wallet.sign(psbt, None)?;
        
        debug!("PSBT signed: {}", signed);
        
        Ok(signed)
    }
    
    /// Import a PSBT from base64 string
    pub async fn import_psbt(&self, psbt_base64: &str) -> Result<Psbt> {
        let psbt = Psbt::from_str(psbt_base64)?;
        Ok(psbt)
    }
    
    /// Export a PSBT as base64 string
    pub async fn export_psbt(&self, psbt: &Psbt) -> Result<String> {
        Ok(psbt.to_string())
    }
    
    /// Import a PSBT, verify, sign and return
    pub async fn sign_psbt(&self, psbt_base64: &str) -> Result<Psbt> {
        // Parse the PSBT from base64
        let mut psbt = Psbt::from_str(psbt_base64)?;
        
        // Sign the transaction
        let mut wallet = self.inner.lock().await;
        let finalized = wallet.sign(&mut psbt, None)?;
        
        if finalized {
            debug!("PSBT is finalized and ready for broadcast");
        } else {
            debug!("PSBT signed but may require additional signatures");
        }
        
        Ok(psbt)
    }
    
    /// Finalize a PSBT by extracting the final transaction
    pub async fn finalize_psbt(&self, psbt: &mut Psbt) -> Result<Transaction> {
        // Attempt to finalize inputs that are not yet finalized
        let mut wallet = self.inner.lock().await;
        
        // Try to finalize any inputs that might be ready
        for i in 0..psbt.inputs.len() {
            if psbt.inputs[i].final_script_sig.is_none() && psbt.inputs[i].final_script_witness.is_none() {
                wallet.finalize_psbt(psbt, i)?;
            }
        }
        
        // Extract the transaction
        match psbt.extract_tx() {
            Ok(tx) => {
                debug!("PSBT finalized successfully: {}", tx.txid());
                Ok(tx)
            },
            Err(e) => {
                error!("Failed to finalize PSBT: {}", e);
                Err(anyhow!("PSBT is not fully signed or has incomplete information: {}", e))
            }
        }
    }
    
    /// Broadcast a finalized PSBT
    pub async fn broadcast_psbt(&self, psbt: Psbt) -> Result<Txid> {
        // Extract the final transaction
        let tx = match psbt.extract_tx() {
            Ok(tx) => tx,
            Err(e) => return Err(anyhow!("Cannot extract transaction from PSBT: {}", e)),
        };
        
        // Broadcast the transaction
        let txid = self.blockchain.broadcast(&tx)?;
        
        info!("Transaction broadcast: {}", txid);
        
        Ok(txid)
    }
    
    /// Simulate hardware wallet flow for PSBT
    pub async fn hardware_wallet_flow(&self, outputs: Vec<(String, u64)>, fee_rate: Option<f32>) -> Result<String> {
        // 1. Create PSBT
        let mut psbt = self.create_multi_output_psbt(outputs, fee_rate).await?;
        
        // 2. Enhance PSBT with hardware wallet metadata
        self.enhance_psbt_for_hardware(&mut psbt).await?;
        
        // 3. Export PSBT for hardware wallet (in a real scenario, user would sign this with their device)
        let psbt_base64 = self.export_psbt(&psbt).await?;
        
        // 4. In a real scenario, the user would sign with hardware wallet and return the signed PSBT
        // Here we just return the unsigned PSBT
        debug!("PSBT ready for hardware wallet signing: {}", psbt_base64);
        
        Ok(psbt_base64)
    }
    
    /// Check if a PSBT is fully signed and ready to broadcast
    pub async fn is_psbt_finalized(&self, psbt: &Psbt) -> Result<bool> {
        match psbt.extract_tx() {
            Ok(_) => Ok(true),
            Err(_) => {
                // Count how many inputs are finalized
                let finalized_inputs = psbt.inputs.iter()
                    .filter(|input| input.final_script_sig.is_some() || input.final_script_witness.is_some())
                    .count();
                
                let all_finalized = finalized_inputs == psbt.inputs.len();
                
                debug!("PSBT finalization status: {}/{} inputs finalized", 
                    finalized_inputs, psbt.inputs.len());
                
                Ok(all_finalized)
            }
        }
    }
    
    /// Send an OP_RETURN transaction to the Bitcoin blockchain
    /// 
    /// This is used for anchoring data (like credential hashes) to the blockchain
    /// in a provable way without storing sensitive data directly.
    pub async fn send_op_return(&self, script: &Script, fee_rate: Option<f32>) -> Result<Txid> {
        let mut wallet = self.inner.lock().await;
        
        // Use default fee rate if not specified
        let fee_rate = fee_rate.unwrap_or(1.0);
        
        // Create transaction using the TxBuilder with OP_RETURN output
        let script_buf = ScriptBuf::from_script(script);
        let (psbt, _details) = {
            wallet.build_tx()
                .add_data(script_buf)
                .enable_rbf()
                .fee_rate(FeeRate::from_sat_per_vb(fee_rate))
                .finish()?
        };
        
        // Sign the transaction
        let finalized = wallet.sign(psbt, None)?;
        
        // Broadcast the transaction
        let raw_tx = finalized.extract_tx();
        let txid = self.blockchain.broadcast(&raw_tx)?;
        
        info!("OP_RETURN transaction sent: {}", txid);
        Ok(txid)
    }
    
    /// Get detailed information about a transaction
    pub async fn get_transaction_info(&self, txid: &Txid) -> Result<TransactionInfo> {
        let tx = self.get_transaction(txid).await?;
        
        // Get blockchain information about this transaction
        let tx_result = self.blockchain.get_tx(txid)?;
        
        let info = match tx_result {
            Some(tx_result) => {
                TransactionInfo {
                    txid: *txid,
                    transaction: tx,
                    confirmations: tx_result.confirmation_time.as_ref().map(|ct| ct.height as u32).unwrap_or(0),
                    block_hash: tx_result.confirmation_time.as_ref().map(|ct| ct.block_hash),
                    block_height: tx_result.confirmation_time.as_ref().map(|ct| ct.height as u32),
                    timestamp: tx_result.confirmation_time.as_ref().map(|ct| ct.timestamp),
                    confirmed: tx_result.confirmation_time.is_some(),
                }
            },
            None => {
                // Transaction exists but is not confirmed
                TransactionInfo {
                    txid: *txid,
                    transaction: tx,
                    confirmations: 0,
                    block_hash: None,
                    block_height: None,
                    timestamp: None,
                    confirmed: false,
                }
            }
        };
        
        Ok(info)
    }
    
    /// Get a transaction by its ID
    pub async fn get_transaction(&self, txid: &Txid) -> Result<Transaction> {
        // First check if it's in our wallet
        let wallet = self.inner.lock().await;
        
        // Try to get from the blockchain directly
        match self.blockchain.get_raw_tx(txid) {
            Ok(Some(tx)) => Ok(tx),
            Ok(None) => Err(anyhow!("Transaction not found: {}", txid)),
            Err(e) => Err(anyhow!("Error retrieving transaction: {}", e)),
        }
    }
}

/// Detailed information about a Bitcoin transaction
#[derive(Debug, Clone)]
pub struct TransactionInfo {
    /// Transaction ID
    pub txid: Txid,
    
    /// The full transaction
    pub transaction: Transaction,
    
    /// Number of confirmations
    pub confirmations: u32,
    
    /// Block hash where the transaction was confirmed (if any)
    pub block_hash: Option<BlockHash>,
    
    /// Block height where the transaction was confirmed (if any)
    pub block_height: Option<u32>,
    
    /// Timestamp of the block where the transaction was confirmed (if any)
    pub timestamp: Option<u64>,
    
    /// Whether the transaction is confirmed
    pub confirmed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_taproot_wallet_creation() {
        let dir = tempdir().unwrap();
        let config = WalletConfig {
            name: "test_wallet".into(),
            database_path: dir.path().join("wallet.db"),
            network: Network::Testnet,
            electrum_url: "tcp://explorerzydxu5ecjrkwceayqybizmpjjznk5izmitf2modhcusuqlid.onion:60001".into(),
            use_taproot: true,
            ..Default::default()
        };
        
        let wallet = Wallet::new(&config).unwrap();
        let address = wallet.get_address(AddressIndex::New).unwrap();
        assert!(address.to_string().starts_with("tb1p"));
    }
    
    #[tokio::test]
    async fn test_psbt_v2_signing() {
        let wallet = create_test_wallet();
        let mut builder = wallet.build_tx();
        builder
            .add_recipient("tb1qsgx55dp6gn53gmyth8k4jnlv3awx2wkd5j43l3".parse().unwrap(), 1000)
            .enable_rbf()
            .version(2);

        let (psbt, _) = builder.finish().unwrap();
        assert_eq!(psbt.version, 2);
    }
}

/// Generate a new mnemonic phrase
pub fn generate_mnemonic() -> Result<String> {
    let mnemonic = Mnemonic::new(&mut rand::thread_rng(), WordCount::Words12);
    Ok(mnemonic.to_string())
}

/// Backup the wallet to a file
pub async fn backup(&self, backup_path: &Path) -> Result<()> {
    let wallet = self.inner.lock().await;
    
    // For now we just backup the database file
    // TODO: Implement more sophisticated backup
    std::fs::copy(&self.config.database_path, backup_path)?;
    
    info!("Wallet backed up to {}", backup_path.display());
    Ok(())
}
