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
use bitcoin::{Address, Network, Transaction, Txid, Script, ScriptBuf};
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
    
    /// Import a PSBT, verify, sign and return
    pub async fn sign_psbt(&self, psbt_base64: &str) -> Result<Psbt> {
        let mut psbt: Psbt = encode::deserialize(&base64::decode(psbt_base64)?)?;
        
        let mut wallet = self.inner.lock().await;
        wallet.sign(&mut psbt, None)?;
        
        Ok(psbt)
    }
    
    /// Broadcast a finalized PSBT
    pub async fn broadcast_psbt(&self, psbt: Psbt) -> Result<Txid> {
        // Extract the transaction
        let tx = psbt.extract_tx();
        
        // Broadcast the transaction
        let txid = self.blockchain.broadcast(&tx)?;
        
        info!("Transaction broadcast: {}", txid);
        Ok(txid)
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_wallet_creation() {
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
        
        let wallet = BitcoinWallet::new(config).await;
        assert!(wallet.is_ok());
    }
    
    #[tokio::test]
    async fn test_address_generation() {
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
        let address = wallet.get_address(AddressIndex::New).await;
        assert!(address.is_ok());
        
        let address_info = address.unwrap();
        assert!(address_info.address.starts_with("tb1p")); // Testnet taproot address
    }
}
