use anya_bitcoin::{
    wallet::{BitcoinWallet, WalletConfig},
    dlc::{
        Oracle, OracleEvent, OracleAttestation, OraclePublicKey,
        DlcContract, ContractInfo, ContractOutcome, ContractBuilder,
        execute_contract, refund_contract, get_contract_status, ContractStatus
    },
    Config,
};

use bitcoin::{Address, Network, Transaction, Txid};
use bitcoin::consensus::encode;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tempfile::tempdir;
use secp256k1_zkp::{rand, Secp256k1, SecretKey, PublicKey, schnorr::Signature, XOnlyPublicKey};

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

// Helper function to create a test oracle
fn setup_test_oracle() -> Oracle {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    
    Oracle::new(
        secret_key,
        "Test Oracle".to_string(),
        "https://example.com/oracle".to_string(),
    )
}

#[tokio::test]
#[ignore] // Requires multiple funded wallets to run end-to-end
async fn test_dlc_contract_workflow() {
    // Setup wallets for Alice and Bob
    let alice_wallet = setup_test_wallet();
    let bob_wallet = setup_test_wallet();
    
    // Setup oracle
    let oracle = setup_test_oracle();
    let oracle_event = OracleEvent::new(
        "BTC-USD price at 2023-12-31".to_string(),
        vec![
            "below 30000".to_string(),
            "30000 to 40000".to_string(),
            "above 40000".to_string()
        ],
    );
    
    println!("Oracle ID: {}", oracle.id());
    println!("Oracle Event: {}", oracle_event.id);
    
    // Create contract
    let contract_builder = ContractBuilder::new()
        .with_oracle(oracle.public_key())
        .with_event(oracle_event.clone())
        .with_collateral(100000, 100000) // 100k sats from each party
        .with_outcomes(vec![
            ("below 30000".to_string(), (0, 200000)),     // All to Alice
            ("30000 to 40000".to_string(), (100000, 100000)), // 50/50 split
            ("above 40000".to_string(), (200000, 0)),     // All to Bob
        ]);
    
    // In a real scenario, Alice and Bob would sign this separately
    // For this test, we're simulating both sides
    
    println!("Contract created successfully!");
    
    // This test can't actually create the full contract without funding,
    // but we can verify the builder works
    assert!(contract_builder.outcomes.len() == 3);
}

#[tokio::test]
async fn test_oracle_attestation() {
    // Setup oracle
    let oracle = setup_test_oracle();
    
    // Create an event
    let oracle_event = OracleEvent::new(
        "BTC-USD price at 2023-12-31".to_string(),
        vec![
            "below 30000".to_string(),
            "30000 to 40000".to_string(),
            "above 40000".to_string()
        ],
    );
    
    // Create an attestation (in real usage, this would be published by the oracle)
    let outcome = "above 40000".to_string();
    let attestation = oracle.attest_to_outcome(&oracle_event, &outcome);
    
    // Verify the attestation
    let is_valid = oracle.verify_attestation(&attestation, &oracle_event, &outcome);
    assert!(is_valid, "Attestation verification failed");
    
    println!("Oracle attestation verified successfully!");
}

#[tokio::test]
async fn test_contract_status() {
    // This is more of a unit test than integration test
    // In a real scenario, we'd need funded wallets and a real blockchain
    
    // Create dummy contract
    let secp = Secp256k1::new();
    let alice_secret = SecretKey::new(&mut rand::thread_rng());
    let bob_secret = SecretKey::new(&mut rand::thread_rng());
    let oracle_secret = SecretKey::new(&mut rand::thread_rng());
    
    let oracle_pubkey = OraclePublicKey::from_secret_key(&secp, &oracle_secret);
    
    let contract = DlcContract {
        id: "test_contract".to_string(),
        info: ContractInfo {
            oracle_pubkey,
            event_id: "test_event".to_string(),
            outcomes: vec![
                "outcome1".to_string(),
                "outcome2".to_string(),
            ],
            // Other fields would be populated here in a real contract
            ..Default::default()
        },
        // Other fields would be populated here in a real contract
        ..Default::default()
    };
    
    // Check initial status - should be pending as no funding tx exists
    assert_eq!(contract.status, ContractStatus::Pending);
    
    println!("Contract status verification successful!");
}
