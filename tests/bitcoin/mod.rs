use anya_core::bitcoin::config::BitcoinConfig;
use anya_core::bitcoin::interface::BitcoinInterface;
use anya_core::bitcoin::rust::RustBitcoinImplementation;
use bitcoin::Network;

#[tokio::test]
async fn test_rust_bitcoin_implementation() {
    let config = BitcoinConfig {
        network: Network::Testnet.to_string(),
        ..Default::default()
    };
    let implementation = RustBitcoinImplementation::new(&config).unwrap();

    // Test address generation
    let address = implementation
        .generate_address(anya_core::bitcoin::interface::AddressType::P2WPKH)
        .await
        .unwrap();

    // Test transaction creation
    let outputs = vec![
        (
            "tb1qbe99gemjdvde2amfl54s34gkx4nscv9vpx0v2s".to_string(),
            10000,
        ),
        (
            "tb1qpx9gxxqsm97za32zn96sfsc7u0s5wz3y7j8z4n".to_string(),
            20000,
        ),
    ];
    let tx = implementation
        .create_transaction(outputs, 10)
        .await
        .unwrap();
    assert_eq!(tx.output.len(), 2, "Transaction should have 2 outputs");

    // Test broadcasting (mocked)
    let txid = implementation.broadcast_transaction(&tx).await.unwrap();
    assert!(!txid.is_empty(), "Transaction ID should not be empty");

    println!("Completed RustBitcoinImplementation tests successfully");
}
