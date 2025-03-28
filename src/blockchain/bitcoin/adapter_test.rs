#![feature(edition2021)]
//! Tests for the Bitcoin blockchain adapter
//!
//! These tests connect to a Bitcoin testnet node to verify functionality of the adapter
//! with real blockchain data. To run these tests, you need access to a Bitcoin testnet node.
//!
//! Run with: `cargo test --test bitcoin_adapter -- --ignored`

use std::collections::HashMap;
use std::env;
use std::time::Duration;

use bitcoin::Network;
use tokio::time::sleep;

use crate::blockchain::{
    BlockchainAdapter, NodePort, WalletPort, SmartContractPort, MetricsPort, SecurityPort,
    AlertComparison, TransactionParams, TxInput,
};
use crate::bitcoin::rpc::BitcoinRpcClient;
use crate::blockchain::bitcoin::adapter::{BitcoinAdapter, BitcoinAdapterConfig};

// Test fixture to set up a Bitcoin testnet adapter
async fn setup_testnet_adapter() -> Option<BitcoinAdapter> {
    // Try to get connection details from environment variables, or use defaults for a local node
    let rpc_url = env::var("BITCOIN_TESTNET_RPC_URL").unwrap_or_else(|_| "http://localhost:18332".to_string());
    let rpc_user = env::var("BITCOIN_TESTNET_RPC_USER").unwrap_or_else(|_| "bitcoin".to_string());
    let rpc_password = env::var("BITCOIN_TESTNET_RPC_PASSWORD").unwrap_or_else(|_| "password".to_string());
    
    // Create a config with shorter intervals for testing
    let config = BitcoinAdapterConfig {
        network: Network::Testnet,
        rpc_url,
        rpc_user,
        rpc_password,
        timeout: 10,
        metrics_interval: 5,
        security_interval: 5,
        mempool_interval: 5,
        fee_estimation_blocks: vec![1, 6, 144],
        enable_security_monitoring: true,
        chain_split_threshold: 3,
        fee_spike_threshold: 2.0,
        max_utxo_cache_size: 100,
        max_block_cache_size: 10,
        max_tx_cache_size: 100,
    };
    
    // Try to create the adapter, but return None if it fails (e.g., if testnet node is not available)
    match BitcoinAdapter::new(config).await {
        Ok(adapter) => Some(adapter),
        Err(e) => {
            eprintln!("Failed to connect to Bitcoin testnet node: {}", e);
            eprintln!("Skipping tests that require a testnet node");
            None
        }
    }
}

// Helper function to check if we should skip a test
fn should_skip_test(adapter: &Option<BitcoinAdapter>) -> bool {
    if adapter.is_none() {
        eprintln!("Skipping test as no testnet node is available");
        return true;
    }
    false
}

// Test the initialization of the adapter
#[tokio::test]
#[ignore] // Ignore by default as it requires a testnet node
async fn test_adapter_initialization() {
    let adapter = setup_testnet_adapter().await;
    if should_skip_test(&adapter) {
        return;
    }
    
    let adapter = adapter.unwrap();
    
    // Initialize the adapter
    let result = adapter.initialize().await;
    assert!(result.is_ok(), "Failed to initialize adapter: {:?}", result);
    
    // Verify the chain ID
    let chain_id = adapter.get_chain_id();
    assert_eq!(chain_id, "bitcoin-testnet", "Chain ID should be bitcoin-testnet");
    
    // Verify we can get blockchain state
    let state = adapter.get_blockchain_state().await;
    assert!(state.is_ok(), "Failed to get blockchain state: {:?}", state);
    
    // Check that the state has a sane block height (testnet should be well above 0)
    let state = state.unwrap();
    assert!(state.best_block_height > 0, "Block height should be greater than 0");
    
    println!("Adapter initialized with state: {:?}", state);
}

// Test the NodePort functionality
#[tokio::test]
#[ignore]
async fn test_node_port() {
    let adapter = setup_testnet_adapter().await;
    if should_skip_test(&adapter) {
        return;
    }
    
    let adapter = adapter.unwrap();
    let _ = adapter.initialize().await.expect("Failed to initialize adapter");
    
    // Get blockchain state
    let state = adapter.get_blockchain_state().await.expect("Failed to get blockchain state");
    println!("Current block height: {}", state.best_block_height);
    
    // Get a block by height - use a recent block
    let height = state.best_block_height - 5; // Go back a few blocks to ensure it's stable
    let block = adapter.get_block_by_height(height).await.expect("Failed to get block by height");
    println!("Block at height {}: {}", height, block.hash);
    
    // Verify we can get the same block by hash
    let block_by_hash = adapter.get_block_by_hash(&block.hash).await.expect("Failed to get block by hash");
    assert_eq!(block.height, block_by_hash.height, "Blocks should have the same height");
    
    // Get raw block
    let raw_block = adapter.get_raw_block(&block.hash).await.expect("Failed to get raw block");
    assert!(!raw_block.is_empty(), "Raw block should not be empty");
    
    // Get a transaction from the block
    if block.tx_count > 0 {
        // Get the transactions in the block
        let transactions = adapter.get_block_by_hash(&block.hash).await.expect("Failed to get block").tx_ids;
        if let Some(txid) = transactions.get(0) {
            // Get the transaction
            let tx = adapter.get_transaction(txid).await.expect("Failed to get transaction");
            println!("Transaction {}: size={}, vsize={}", txid, tx.size, tx.vsize);
            
            // Get raw transaction
            let raw_tx = adapter.get_raw_transaction(txid).await.expect("Failed to get raw transaction");
            assert!(!raw_tx.is_empty(), "Raw transaction should not be empty");
        }
    }
    
    // Get mempool status
    let mempool = adapter.get_mempool_status().await.expect("Failed to get mempool status");
    println!("Mempool: {} transactions, {} bytes", mempool.tx_count, mempool.size);
    
    // Get mempool transactions
    let mempool_txs = adapter.get_mempool_transactions().await.expect("Failed to get mempool transactions");
    println!("Mempool has {} transactions", mempool_txs.len());
    
    // Estimate fee
    let fee = adapter.estimate_fee(6).await.expect("Failed to estimate fee");
    println!("Estimated fee for 6 blocks: {} sat/byte", fee);
    
    // Get peer info
    let peers = adapter.get_peer_info().await.expect("Failed to get peer info");
    println!("Connected to {} peers", peers.len());
    assert!(!peers.is_empty(), "Should be connected to at least one peer");
    
    // Get network hashrate
    let hashrate = adapter.get_network_hashrate().await.expect("Failed to get network hashrate");
    println!("Network hashrate: {} hashes/sec", hashrate);
    assert!(hashrate > 0.0, "Network hashrate should be greater than 0");
}

// Test the WalletPort functionality
// Note: This test only tests read-only functions as we don't have a wallet
#[tokio::test]
#[ignore]
async fn test_wallet_port_readonly() {
    let adapter = setup_testnet_adapter().await;
    if should_skip_test(&adapter) {
        return;
    }
    
    let adapter = adapter.unwrap();
    let _ = adapter.initialize().await.expect("Failed to initialize adapter");
    
    // Get blockchain state to find a block
    let state = adapter.get_blockchain_state().await.expect("Failed to get blockchain state");
    let height = state.best_block_height - 5; // Go back a few blocks to ensure it's stable
    
    // Get a block
    let block = adapter.get_block_by_height(height).await.expect("Failed to get block by height");
    
    // Find a transaction in the block
    if let Some(txid) = block.tx_ids.get(0) {
        // Analyze the transaction
        let analysis = adapter.analyze_transaction(&txid).await.expect("Failed to analyze transaction");
        println!("Transaction analysis: {} inputs, {} outputs", analysis.inputs.len(), analysis.outputs.len());
        
        // If the transaction has outputs with addresses, test get_address_balance and related functions
        for output in &analysis.outputs {
            if let Some(address) = &output.address {
                // Get address balance
                let balance = adapter.get_address_balance(address).await.expect("Failed to get address balance");
                println!("Address {} balance: {} confirmed, {} unconfirmed", 
                         address, balance.confirmed, balance.unconfirmed);
                
                // Get address transactions
                let txs = adapter.get_address_transactions(address, Some(10)).await.expect("Failed to get address transactions");
                println!("Address {} has {} transactions", address, txs.len());
                
                // Get address UTXOs
                let utxos = adapter.get_address_utxos(address).await.expect("Failed to get address UTXOs");
                println!("Address {} has {} UTXOs", address, utxos.len());
                
                // Only need to test one address
                break;
            }
        }
        
        // Create a raw transaction (unsigned)
        let inputs = vec![
            TxInput {
                txid: txid.clone(),
                vout: 0,
                sequence: None,
            }
        ];
        
        // Create a dummy output
        let mut outputs = HashMap::new();
        outputs.insert("tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(), 0.0001); // Testnet address
        
        // This will fail because we don't have the private keys, but it tests the interface
        let result = adapter.create_raw_transaction(inputs, outputs).await;
        println!("Create raw transaction result: {:?}", result);
    }
}

// Test the SmartContractPort functionality
#[tokio::test]
#[ignore]
async fn test_smart_contract_port() {
    let adapter = setup_testnet_adapter().await;
    if should_skip_test(&adapter) {
        return;
    }
    
    let adapter = adapter.unwrap();
    let _ = adapter.initialize().await.expect("Failed to initialize adapter");
    
    // Test deploy_contract with a simple P2SH script
    // This is a multi-sig script: 1-of-2 multisig with dummy public keys
    let script_hex = "5121030000000000000000000000000000000000000000000000000000000000000001210300000000000000000000000000000000000000000000000000000000000000020252ae";
    
    // This will likely fail in practice without a funded wallet, but it tests the interface
    let result = adapter.deploy_contract(script_hex, "", &["0.0001".to_string()]).await;
    println!("Deploy contract result: {:?}", result);
    
    // Get blockchain state to find a block
    let state = adapter.get_blockchain_state().await.expect("Failed to get blockchain state");
    
    // Find a P2SH address to test call_contract
    // Use a known P2SH address from testnet (replace with a real one if needed)
    let p2sh_address = "2MzQwSSnBHWHqSAqtTVQ6v47XtaisrJa1Vc";
    
    // Test call_contract
    let result = adapter.call_contract(p2sh_address, "", "", &[]).await;
    println!("Call contract result: {:?}", result);
    
    // Test get_contract_balance
    let result = adapter.get_contract_balance(p2sh_address).await;
    println!("Contract balance result: {:?}", result);
    
    // Test get_contract_events
    let result = adapter.get_contract_events(
        p2sh_address, "", "all", 
        Some(state.best_block_height - 100), 
        Some(state.best_block_height)
    ).await;
    println!("Contract events result: {:?}", result);
}

// Test the MetricsPort functionality
#[tokio::test]
#[ignore]
async fn test_metrics_port() {
    let adapter = setup_testnet_adapter().await;
    if should_skip_test(&adapter) {
        return;
    }
    
    let adapter = adapter.unwrap();
    let _ = adapter.initialize().await.expect("Failed to initialize adapter");
    
    // Get metrics
    let metrics = adapter.get_metrics().await.expect("Failed to get metrics");
    println!("Block count: {}, TX count: {}", metrics.block_count, metrics.tx_count);
    assert!(metrics.block_count > 0, "Block count should be greater than 0");
    
    // Get network hashrate
    let hashrate = adapter.get_network_hashrate().await.expect("Failed to get network hashrate");
    println!("Network hashrate: {} hashes/sec", hashrate);
    
    // Get mempool size
    let mempool_size = adapter.get_mempool_size().await.expect("Failed to get mempool size");
    println!("Mempool size: {} transactions", mempool_size);
    
    // Get fee estimates
    let fee_estimates = adapter.get_fee_estimates().await.expect("Failed to get fee estimates");
    println!("Fee estimates: {:?}", fee_estimates);
    assert!(!fee_estimates.is_empty(), "Should have at least one fee estimate");
    
    // Get node version
    let version = adapter.get_node_version().await.expect("Failed to get node version");
    println!("Node version: {}", version);
    assert!(!version.is_empty(), "Node version should not be empty");
    
    // Get transaction volume
    let volume = adapter.get_transaction_volume(10).await.expect("Failed to get transaction volume");
    println!("Transaction volume for last 10 blocks: {} satoshis", volume);
    
    // Get block time average
    let avg_block_time = adapter.get_block_time_average(10).await.expect("Failed to get block time average");
    println!("Average block time for last 10 blocks: {} seconds", avg_block_time);
    assert!(avg_block_time > 0.0, "Average block time should be greater than 0");
    
    // Get difficulty
    let difficulty = adapter.get_difficulty().await.expect("Failed to get difficulty");
    println!("Current difficulty: {}", difficulty);
    assert!(difficulty > 0.0, "Difficulty should be greater than 0");
    
    // Get mempool fee histogram
    let histogram = adapter.get_mempool_fee_histogram().await.expect("Failed to get mempool fee histogram");
    println!("Mempool fee histogram has {} data points", histogram.len());
}

// Test the SecurityPort functionality
#[tokio::test]
#[ignore]
async fn test_security_port() {
    let adapter = setup_testnet_adapter().await;
    if should_skip_test(&adapter) {
        return;
    }
    
    let adapter = adapter.unwrap();
    let _ = adapter.initialize().await.expect("Failed to initialize adapter");
    
    // Check for chain splits
    let chain_split = adapter.check_chain_split().await.expect("Failed to check chain split");
    println!("Chain split detection result: {:?}", chain_split);
    
    // Get blockchain state to find a block
    let state = adapter.get_blockchain_state().await.expect("Failed to get blockchain state");
    let height = state.best_block_height - 5; // Go back a few blocks to ensure it's stable
    
    // Get a block
    let block = adapter.get_block_by_height(height).await.expect("Failed to get block by height");
    
    // Find a transaction to test double spend detection
    if let Some(txid) = block.tx_ids.get(0) {
        // Check for double spends (this should be none for a confirmed transaction)
        let double_spend = adapter.detect_double_spend(txid, 1).await.expect("Failed to detect double spend");
        println!("Double spend detection result: {:?}", double_spend);
        
        // Check transaction malleability
        let malleability = adapter.check_transaction_malleability(txid).await.expect("Failed to check transaction malleability");
        println!("Transaction malleability check result: {:?}", malleability);
    }
    
    // Check for anomalous fees
    let anomalies = adapter.detect_anomalous_fees().await.expect("Failed to detect anomalous fees");
    println!("Detected {} fee anomalies", anomalies.len());
    
    // Monitor large transactions
    let large_txs = adapter.monitor_large_transactions(1.0).await.expect("Failed to monitor large transactions");
    println!("Detected {} large transactions (>1 BTC)", large_txs.len());
    
    // Check reorg depth (should be None for a stable block)
    let reorg = adapter.check_reorg_depth(height).await.expect("Failed to check reorg depth");
    println!("Reorg depth check result: {:?}", reorg);
}

// Test monitoring functionality
#[tokio::test]
#[ignore]
async fn test_monitoring() {
    let adapter = setup_testnet_adapter().await;
    if should_skip_test(&adapter) {
        return;
    }
    
    let adapter = adapter.unwrap();
    let _ = adapter.initialize().await.expect("Failed to initialize adapter");
    
    // Start monitoring
    let result = adapter.start_monitoring().await;
    assert!(result.is_ok(), "Failed to start monitoring: {:?}", result);
    
    // Wait for some monitoring to happen
    println!("Waiting for monitoring to collect data...");
    sleep(Duration::from_secs(10)).await;
    
    // Get unusual transactions
    let unusual_txs = adapter.get_unusual_transactions().await.expect("Failed to get unusual transactions");
    println!("Detected {} unusual transactions", unusual_txs.len());
    
    // Get security alerts
    let alerts = adapter.get_security_alerts().await.expect("Failed to get security alerts");
    println!("Detected {} security alerts", alerts.len());
    
    // Test alert comparison
    let alert = AlertComparison {
        field: "block_height".to_string(),
        comparison: "gt".to_string(),
        value: "1000".to_string(), // Testnet should be well above 1000 blocks
        extra: None,
    };
    
    let alert_result = adapter.compare_with_alert(&alert).await.expect("Failed to compare with alert");
    assert!(alert_result, "Block height should be greater than 1000 on testnet");
    
    // Stop monitoring
    let result = adapter.stop_monitoring().await;
    assert!(result.is_ok(), "Failed to stop monitoring: {:?}", result);
}

// Test transaction creation and broadcasting (read-only version)
#[tokio::test]
#[ignore]
async fn test_transaction_creation_readonly() {
    let adapter = setup_testnet_adapter().await;
    if should_skip_test(&adapter) {
        return;
    }
    
    let adapter = adapter.unwrap();
    let _ = adapter.initialize().await.expect("Failed to initialize adapter");
    
    // Create a transaction params object
    // Note: This will not be broadcast since we're not controlling a wallet
    let params = TransactionParams {
        inputs: None, // Let the wallet select inputs
        outputs: HashMap::from([
            ("tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(), 0.0001)
        ]),
        fee_rate: Some(5.0), // 5 sat/byte
        change_address: None,
        op_return_data: Some(vec![1, 2, 3, 4, 5]),
        rbf: Some(true),
        locktime: None,
    };
    
    // This will fail because we don't have a funded wallet, but it tests the interface
    let result = adapter.create_transaction(params).await;
    println!("Create transaction result: {:?}", result);
}

// A integration test that exercises multiple components together
#[tokio::test]
#[ignore]
async fn test_integration_workflow() {
    let adapter = setup_testnet_adapter().await;
    if should_skip_test(&adapter) {
        return;
    }
    
    let adapter = adapter.unwrap();
    let _ = adapter.initialize().await.expect("Failed to initialize adapter");
    
    // 1. Get the blockchain state
    let state = adapter.get_blockchain_state().await.expect("Failed to get blockchain state");
    println!("Current block height: {}", state.best_block_height);
    
    // 2. Get a recent block
    let height = state.best_block_height - 3;
    let block = adapter.get_block_by_height(height).await.expect("Failed to get block");
    println!("Block at height {}: hash={}, tx_count={}", height, block.hash, block.tx_count);
    
    // 3. Get the first transaction in the block
    if let Some(txid) = block.tx_ids.get(0) {
        // 4. Get transaction details
        let tx = adapter.get_transaction(txid).await.expect("Failed to get transaction");
        println!("Transaction {}: size={}, vsize={}", txid, tx.size, tx.vsize);
        
        // 5. Analyze the transaction
        let analysis = adapter.analyze_transaction(txid).await.expect("Failed to analyze transaction");
        println!("Transaction has {} inputs and {} outputs", analysis.inputs.len(), analysis.outputs.len());
        
        // 6. Check if the transaction is in the mempool (should be false for a confirmed tx)
        let in_mempool = adapter.is_in_mempool(txid).await.expect("Failed to check if tx is in mempool");
        assert!(!in_mempool, "Confirmed transaction should not be in mempool");
        
        // 7. Check for double spends
        let double_spend = adapter.detect_double_spend(txid, 3).await.expect("Failed to detect double spend");
        assert!(double_spend.is_none(), "Confirmed transaction should not have double spends");
        
        // 8. Check transaction malleability
        let malleability = adapter.check_transaction_malleability(txid).await.expect("Failed to check transaction malleability");
        println!("Malleability check: {:?}", malleability);
        
        // 9. Get outputs and check UTXOs
        for (i, output) in analysis.outputs.iter().enumerate() {
            if let Some(address) = &output.address {
                // 10. Get address balance
                let balance = adapter.get_address_balance(address).await.expect("Failed to get address balance");
                println!("Output #{} to {}: {} confirmed, {} unconfirmed", 
                         i, address, balance.confirmed, balance.unconfirmed);
                
                // 11. Get UTXO for this output
                let utxo = adapter.get_utxo(txid, i as u32).await.expect("Failed to get UTXO");
                if let Some(utxo_info) = utxo {
                    println!("UTXO: txid={}, vout={}, amount={}", 
                             utxo_info.txid, utxo_info.vout, utxo_info.amount);
                } else {
                    println!("UTXO was spent or doesn't exist");
                }
                
                // Only check one output
                break;
            }
        }
    }
    
    // 12. Get current fee estimates
    let fee_estimates = adapter.get_fee_estimates().await.expect("Failed to get fee estimates");
    println!("Fee estimates: {:?}", fee_estimates);
    
    // 13. Get metrics
    let metrics = adapter.get_metrics().await.expect("Failed to get metrics");
    println!("Metrics: block_count={}, tx_count={}, difficulty={}", 
             metrics.block_count, metrics.tx_count, metrics.difficulty);
    
    // 14. Check for chain splits
    let chain_split = adapter.check_chain_split().await.expect("Failed to check chain split");
    println!("Chain split detection: {:?}", chain_split);
} 