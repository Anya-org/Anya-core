//! Tests for Silent Payments (BIP-353) Implementation
//!
//! This module contains unit tests and integration tests for the
//! Silent Payments implementation, validating conformance to BIP-353.

use bitcoin::secp256k1::{Secp256k1, SecretKey, XOnlyPublicKey};
use bitcoin::network::constants::Network;
use bitcoin::{OutPoint, Transaction, TxOut, Amount, Address};
use bitcoin::hashes::{Hash, sha256};
use crate::Result;
use crate::silent_payments::{
    SilentPaymentAddress, 
    KeyManager, 
    SilentPaymentScanner,
    SilentPaymentSender,
    SilentPaymentInfo,
    derive_shared_secret,
    MAINNET_PREFIX,
    TESTNET_PREFIX,
};
use std::str::FromStr;

#[test]
fn test_key_generation() -> Result<()> {
    // Test creation of a key manager with random keys
    let key_manager = KeyManager::new_random()?;
    
    // Verify that public keys were properly generated
    assert!(key_manager.scan_pubkey().is_valid());
    assert!(key_manager.spend_pubkey().is_valid());
    
    // Verify that the network is set to mainnet by default
    assert_eq!(key_manager.network(), Network::Bitcoin);
    
    Ok(())
}

#[test]
fn test_address_generation() -> Result<()> {
    // Create a key manager
    let key_manager = KeyManager::new_random()?;
    
    // Generate an address
    let address = key_manager.generate_address();
    
    // Verify the address format
    let address_str = address.to_string();
    assert!(address_str.starts_with(MAINNET_PREFIX));
    
    // Parse the address back
    let parsed_address = SilentPaymentAddress::from_str(&address_str)?;
    
    // Verify the parsed address matches the original
    assert_eq!(parsed_address.scan_pubkey, *key_manager.scan_pubkey());
    assert_eq!(parsed_address.spend_pubkey, *key_manager.spend_pubkey());
    assert_eq!(parsed_address.network, Network::Bitcoin);
    
    Ok(())
}

#[test]
fn test_network_specific_addresses() -> Result<()> {
    // Create a key manager
    let mut key_manager = KeyManager::new_random()?;
    
    // Test mainnet address
    key_manager.set_network(Network::Bitcoin);
    let mainnet_address = key_manager.generate_address();
    assert!(mainnet_address.to_string().starts_with(MAINNET_PREFIX));
    
    // Test testnet address
    key_manager.set_network(Network::Testnet);
    let testnet_address = key_manager.generate_address();
    assert!(testnet_address.to_string().starts_with(TESTNET_PREFIX));
    
    Ok(())
}

#[test]
fn test_silent_payment_flow() -> Result<()> {
    // Set up a receiver with key manager
    let receiver_key_manager = KeyManager::new_random()?;
    let receiver_address = receiver_key_manager.generate_address();
    
    // Set up a sender
    let sender = SilentPaymentSender::new();
    let sender_secret = SecretKey::new(&mut bitcoin::secp256k1::rand::thread_rng());
    
    // Create a dummy outpoint
    let outpoint = OutPoint::new(
        bitcoin::Txid::from_slice(&[0x42; 32]).unwrap(),
        0
    );
    
    // Create a payment output
    let amount = Amount::from_sat(50_000);
    let output = sender.create_payment_output(
        &receiver_address,
        &sender_secret,
        &outpoint,
        amount,
    )?;
    
    // Create a dummy transaction with this output
    let mut tx = Transaction {
        version: 2,
        lock_time: 0,
        input: vec![bitcoin::TxIn {
            previous_output: outpoint,
            script_sig: bitcoin::ScriptBuf::new(),
            sequence: 0xffffffff,
            witness: bitcoin::Witness::new(),
        }],
        output: vec![output],
    };
    
    // Create a scanner for the receiver
    let mut scanner = SilentPaymentScanner::new(
        *receiver_key_manager.scan_secret(),
        *receiver_key_manager.spend_pubkey(),
    )?;
    
    // Scan the transaction
    let detected = scanner.scan_transaction(&tx, Some(700_000))?;
    
    // Verify a payment was found
    assert_eq!(detected.len(), 1);
    
    // Verify the payment details
    let payment = &detected[0];
    assert_eq!(payment.txid, tx.txid());
    assert_eq!(payment.vout, 0);
    assert_eq!(payment.amount, amount.to_sat());
    assert_eq!(payment.block_height, Some(700_000));
    assert_eq!(payment.spent, false);
    
    Ok(())
}

#[test]
fn test_multiple_outputs() -> Result<()> {
    // Set up a receiver with key manager
    let receiver_key_manager = KeyManager::new_random()?;
    let receiver_address = receiver_key_manager.generate_address();
    
    // Set up a sender
    let sender = SilentPaymentSender::new();
    let sender_secret = SecretKey::new(&mut bitcoin::secp256k1::rand::thread_rng());
    
    // Create dummy outpoints
    let outpoints = vec![
        OutPoint::new(bitcoin::Txid::from_slice(&[0x42; 32]).unwrap(), 0),
        OutPoint::new(bitcoin::Txid::from_slice(&[0x43; 32]).unwrap(), 1),
    ];
    
    // Create payment amounts
    let amounts = vec![
        Amount::from_sat(50_000),
        Amount::from_sat(100_000),
    ];
    
    // Create multiple outputs
    let outputs = sender.create_multiple_outputs(
        &receiver_address,
        &sender_secret,
        &outpoints,
        &amounts,
    )?;
    
    // Verify the correct number of outputs
    assert_eq!(outputs.len(), 2);
    
    // Verify the output amounts
    assert_eq!(outputs[0].value, 50_000);
    assert_eq!(outputs[1].value, 100_000);
    
    Ok(())
}

#[test]
fn test_bip32_derivation() -> Result<()> {
    // Create an xpriv from a known seed
    let seed = [0u8; 64]; // Use a fixed seed for deterministic tests
    let secp = Secp256k1::new();
    let xpriv = bitcoin::util::bip32::ExtendedPrivKey::new_master(Network::Bitcoin, &seed)
        .expect("Failed to derive master key");
    
    // Derive keys using default BIP-353 derivation paths
    let key_manager = KeyManager::derive_from_xpriv(
        &xpriv,
        None, // Use default scan path
        None, // Use default spend path
        Network::Bitcoin,
    )?;
    
    // Generate an address
    let address = key_manager.generate_address();
    assert!(address.to_string().starts_with(MAINNET_PREFIX));
    
    Ok(())
}

#[test]
fn test_shared_secret_derivation() -> Result<()> {
    // Create keys
    let secp = Secp256k1::new();
    let scan_secret = SecretKey::new(&mut bitcoin::secp256k1::rand::thread_rng());
    let spend_secret = SecretKey::new(&mut bitcoin::secp256k1::rand::thread_rng());
    let sender_secret = SecretKey::new(&mut bitcoin::secp256k1::rand::thread_rng());
    
    // Derive public keys
    let scan_pubkey = XOnlyPublicKey::from_secret_key(&secp, &scan_secret).0;
    let spend_pubkey = XOnlyPublicKey::from_secret_key(&secp, &spend_secret).0;
    
    // Create outpoint
    let outpoint = OutPoint::new(
        bitcoin::Txid::from_slice(&[0x42; 32]).unwrap(),
        0
    );
    
    // Derive shared secret
    let secret = derive_shared_secret(
        &scan_pubkey,
        &spend_pubkey,
        &sender_secret,
        &outpoint,
    )?;
    
    // Verify the secret is not all zeros or ones
    assert_ne!(secret, [0u8; 32]);
    assert_ne!(secret, [0xffu8; 32]);
    
    // Verify a different outpoint produces a different secret
    let different_outpoint = OutPoint::new(
        bitcoin::Txid::from_slice(&[0x43; 32]).unwrap(),
        1
    );
    
    let different_secret = derive_shared_secret(
        &scan_pubkey,
        &spend_pubkey,
        &sender_secret,
        &different_outpoint,
    )?;
    
    assert_ne!(secret, different_secret);
    
    Ok(())
} 