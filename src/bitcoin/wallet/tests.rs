use super::*;
use bitcoin::Network;
use tokio;

#[tokio::test]
async fn test_wallet_creation() {
    let config = WalletConfig {
        name: "test-wallet".to_string(),
        network: Network::Testnet,
        data_dir: std::path::PathBuf::from("./test-wallets"),
        gap_limit: 20,
        min_confirmations: 1,
        fee_strategy: FeeStrategy::Normal,
        coin_selection: CoinSelectionStrategy::LargestFirst,
        receive_descriptor: "wpkh([d34db33f/84h/1h/0h]tpubDC5FSnBiZDMmhiuCmWAYsLwgLYrrT9rAqvTySfuCCrgsWz8K9BnGMHjbv8uzkzXcS1kxnXg6LBMiqck6Zj1dR1K4ry3CNGTdUwKthXHRR8t/0/*)".to_string(),
        change_descriptor: "wpkh([d34db33f/84h/1h/0h]tpubDC5FSnBiZDMmhiuCmWAYsLwgLYrrT9rAqvTySfuCCrgsWz8K9BnGMHjbv8uzkzXcS1kxnXg6LBMiqck6Zj1dR1K4ry3CNGTdUwKthXHRR8t/1/*)".to_string(),
    };

    let network_config = NetworkConfig {
        testnet_rpc_url: Some("https://bitcoin-testnet-rpc.example.com".to_string()),
        testnet_rpc_user: Some("test".to_string()),
        testnet_rpc_pass: Some("test123".to_string()),
        ..Default::default()
    };

    let wallet = BitcoinWallet::new(config, network_config, None).await;
    assert!(wallet.is_ok(), "Failed to create wallet: {:?}", wallet.err());

    let wallet = wallet.unwrap();
    let init_result = wallet.init().await;
    assert!(init_result.is_ok(), "Failed to initialize wallet: {:?}", init_result.err());

    // Test address generation
    let address = wallet.get_new_address().await;
    assert!(address.is_ok(), "Failed to generate address: {:?}", address.err());
    let address = address.unwrap();
    assert!(address.is_valid_for_network(Network::Testnet), "Address not valid for testnet");

    // Test balance checking
    let balance = wallet.get_balance().await;
    assert!(balance.is_ok(), "Failed to get balance: {:?}", balance.err());
}

#[tokio::test]
async fn test_transaction_creation() {
    let wallet = setup_test_wallet().await;
    
    // Create a test transaction
    let params = TransactionParams {
        recipients: vec![(
            Address::from_str("tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx").unwrap(),
            10000 // 0.0001 BTC in satoshis
        )].into_iter().collect(),
        fee_strategy: Some(FeeStrategy::Normal),
        enable_rbf: true,
        ..Default::default()
    };

    let psbt = wallet.create_transaction(params).await;
    assert!(psbt.is_ok(), "Failed to create transaction: {:?}", psbt.err());
}

#[tokio::test]
async fn test_utxo_management() {
    let wallet = setup_test_wallet().await;
    
    // Test UTXO listing
    let utxos = wallet.list_utxos().await;
    assert!(utxos.is_ok(), "Failed to list UTXOs: {:?}", utxos.err());

    // Test UTXO selection
    if let Ok(utxos) = utxos {
        println!("Found {} UTXOs", utxos.len());
        for utxo in utxos {
            println!("UTXO: {} sat at {}:{}", utxo.txout.value, utxo.outpoint.txid, utxo.outpoint.vout);
        }
    }
}

async fn setup_test_wallet() -> BitcoinWallet {
    let config = WalletConfig {
        name: "test-wallet".to_string(),
        network: Network::Testnet,
        data_dir: std::path::PathBuf::from("./test-wallets"),
        gap_limit: 20,
        min_confirmations: 1,
        fee_strategy: FeeStrategy::Normal,
        coin_selection: CoinSelectionStrategy::LargestFirst,
        receive_descriptor: "wpkh([d34db33f/84h/1h/0h]tpubDC5FSnBiZDMmhiuCmWAYsLwgLYrrT9rAqvTySfuCCrgsWz8K9BnGMHjbv8uzkzXcS1kxnXg6LBMiqck6Zj1dR1K4ry3CNGTdUwKthXHRR8t/0/*)".to_string(),
        change_descriptor: "wpkh([d34db33f/84h/1h/0h]tpubDC5FSnBiZDMmhiuCmWAYsLwgLYrrT9rAqvTySfuCCrgsWz8K9BnGMHjbv8uzkzXcS1kxnXg6LBMiqck6Zj1dR1K4ry3CNGTdUwKthXHRR8t/1/*)".to_string(),
    };

    let network_config = NetworkConfig {
        testnet_rpc_url: Some("https://bitcoin-testnet-rpc.example.com".to_string()),
        testnet_rpc_user: Some("test".to_string()),
        testnet_rpc_pass: Some("test123".to_string()),
        ..Default::default()
    };

    BitcoinWallet::new(config, network_config, None)
        .await
        .expect("Failed to create test wallet")
}