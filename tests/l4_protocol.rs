#![feature(edition2021)]
#[tokio::test]
async fn test_private_transaction_flow() {
    let l4 = AnyaL4Protocol::new(Network::Testnet);
    let psbt = create_test_psbt();
    
    let txid = l4.send_private_transaction(psbt)
        .await
        .expect("Failed to send transaction");
    
    // Verify transaction propagation
    let status = l4.rpc_adapter.call("gettxout", &[json!(txid), json!(0)])
        .await
        .expect("RPC call failed");
    
    assert!(status["value"].as_f64().unwrap() > 0.0);
    assert_eq!(txid.len(), 64); // Proper TXID format
} 