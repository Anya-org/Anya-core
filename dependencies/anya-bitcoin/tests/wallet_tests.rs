#![feature(edition2021)]
use anya_bitcoin::{
    wallet::{BitcoinWallet, WalletConfig},
    transaction::TransactionService,
    Config,
};

use bitcoin::{Address, Network, Transaction, Txid};
use bitcoin::consensus::encode;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tempfile::tempdir;

// Helper function to set up a test wallet
fn setup_test_wallet() -> BitcoinWallet {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("wallet.db");
    
    let config = WalletConfig {
        name: "test_wallet".to_string(),
        database_path: db_path,
        network: Network::Regtest,
        electrum_url: "ssl://electrum.blockstream.info:60002".to_string(),
        mnemonic: Some("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_string()),
        passphrase: None,
        gap_limit: 20,
    };
    
    BitcoinWallet::new(config).expect("Failed to create wallet")
}

#[tokio::test]
async fn test_wallet_creation() {
    let wallet = setup_test_wallet();
    
    // Check wallet is initialized properly
    assert_eq!(wallet.config.name, "test_wallet");
    assert_eq!(wallet.config.network, Network::Regtest);
    
    // Test that wallet can be synced (may fail if no internet connection)
    let sync_result = wallet.sync();
    println!("Sync result: {:?}", sync_result);
    
    // Test address generation
    let address_info = wallet.get_address(bdk::wallet::AddressIndex::New)
        .expect("Failed to generate address");
    
    println!("Generated address: {}", address_info.address);
    assert!(address_info.address.to_string().starts_with("bcrt1"));
}

#[tokio::test]
async fn test_address_generation() {
    let wallet = setup_test_wallet();
    
    // Generate multiple addresses and ensure they're different
    let address1 = wallet.get_address(bdk::wallet::AddressIndex::New)
        .expect("Failed to generate address 1");
    
    let address2 = wallet.get_address(bdk::wallet::AddressIndex::New)
        .expect("Failed to generate address 2");
    
    assert_ne!(address1.address.to_string(), address2.address.to_string());
    println!("Address 1: {}", address1.address);
    println!("Address 2: {}", address2.address);
}

#[tokio::test]
async fn test_wallet_balance() {
    let wallet = setup_test_wallet();
    
    // Get initial balance (will be 0 on a fresh wallet)
    let balance = wallet.get_balance().expect("Failed to get balance");
    
    // This is a new wallet so balance should be 0
    assert_eq!(balance.confirmed, 0);
    assert_eq!(balance.trusted_pending, 0);
    assert_eq!(balance.untrusted_pending, 0);
    
    println!("Wallet balance: {} confirmed sats", balance.confirmed);
}

#[tokio::test]
#[ignore] // Requires funds in the wallet to run
async fn test_transaction_creation() {
    let wallet = setup_test_wallet();
    
    // Get a new receiving address
    let address = wallet.get_address(bdk::wallet::AddressIndex::New)
        .expect("Failed to generate address")
        .address;
    
    // Create a transaction (will fail without funds)
    let result = wallet.create_psbt(
        &address.to_string(),
        1000, // 1000 sats
        1.0,  // 1 sat/vB fee rate
    );
    
    match result {
        Ok(psbt) => {
            println!("Created PSBT successfully");
            println!("PSBT: {}", encode::serialize_hex(&psbt));
        },
        Err(e) => {
            println!("Failed to create transaction as expected (no funds): {}", e);
            // This is expected to fail in a test environment
        }
    }
}

#[tokio::test]
async fn test_mnemonic_generation() {
    let mnemonic = BitcoinWallet::generate_mnemonic().expect("Failed to generate mnemonic");
    
    // Check that we got a 12-word mnemonic
    let words: Vec<&str> = mnemonic.split_whitespace().collect();
    assert_eq!(words.len(), 12);
    
    println!("Generated mnemonic: {}", mnemonic);
}
