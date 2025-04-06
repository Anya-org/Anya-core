use anya_privacy::silent_payments::{KeyManager, SilentPaymentScanner, SilentPaymentSender};
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::{Transaction, OutPoint, TxIn, TxOut, Witness, Amount, Network, Block, BlockHeader};
use bitcoin::hashes::{Hash, sha256d};
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};
use std::thread;
use rand::Rng;

const NUM_BLOCKS: usize = 10;
const TRANSACTIONS_PER_BLOCK: usize = 100;
const NUM_RECEIVERS: usize = 5;
const PAYMENTS_PER_RECEIVER: usize = 10;
const NUM_THREADS: usize = 4;

/// Performance Test for BIP-353 Silent Payments
/// [AIR-3][AIS-3][AIP-3][BPC-3][PFM-3]
fn main() -> anyhow::Result<()> {
    println!("===== BIP-353 Silent Payments Performance Test =====");
    println!("[AIR-3][AIS-3][AIP-3][BPC-3][PFM-3]");
    println!();
    
    // STEP 1: Generate receiver keys
    println!("STEP 1: Generating {} receiver key pairs...", NUM_RECEIVERS);
    let receivers = (0..NUM_RECEIVERS)
        .map(|_| KeyManager::new_random().expect("Failed to generate keys"))
        .collect::<Vec<_>>();
    
    println!("Sample addresses:");
    for (i, receiver) in receivers.iter().take(3).enumerate() {
        println!("  Receiver {}: {}", i, receiver.generate_address());
    }
    println!();
    
    // STEP 2: Generate test blocks with transactions
    println!("STEP 2: Generating {} test blocks with {} transactions each...", 
             NUM_BLOCKS, TRANSACTIONS_PER_BLOCK);
    
    let mut rng = rand::thread_rng();
    let sender = SilentPaymentSender::new();
    
    // Generate sender keys
    let sender_keys: Vec<SecretKey> = (0..50)
        .map(|_| SecretKey::new(&mut bitcoin::secp256k1::rand::thread_rng()))
        .collect();
    
    // Create blocks with transactions
    let blocks = generate_test_blocks(
        &receivers,
        &sender_keys,
        &sender,
        NUM_BLOCKS,
        TRANSACTIONS_PER_BLOCK,
        PAYMENTS_PER_RECEIVER,
    )?;
    
    println!("Generated {} blocks with total {} transactions", 
             blocks.len(), 
             blocks.iter().map(|b| b.txdata.len()).sum::<usize>());
    println!();
    
    // STEP 3: Sequential scanning (baseline)
    println!("STEP 3: Sequential scanning (baseline)...");
    let scanners: Vec<_> = receivers.iter()
        .map(|r| {
            SilentPaymentScanner::new(
                *r.scan_secret(),
                *r.spend_pubkey(),
            ).expect("Failed to create scanner")
        })
        .collect();
    
    let start = Instant::now();
    let mut total_payments = 0;
    
    for (block_idx, block) in blocks.iter().enumerate() {
        for (receiver_idx, scanner) in scanners.iter().enumerate() {
            let mut scanner = scanner.clone();
            for tx in &block.txdata {
                let payments = scanner.scan_transaction(tx, Some(block_idx as u32))?;
                total_payments += payments.len();
            }
        }
    }
    
    let sequential_duration = start.elapsed();
    println!("Sequential scanning complete.");
    println!("  Time: {:?}", sequential_duration);
    println!("  Payments found: {}", total_payments);
    println!("  Transactions/second: {:.2}", 
             (NUM_BLOCKS * TRANSACTIONS_PER_BLOCK * NUM_RECEIVERS) as f64 / 
             sequential_duration.as_secs_f64());
    println!();
    
    // STEP 4: Parallel scanning
    println!("STEP 4: Parallel scanning with {} threads...", NUM_THREADS);
    let start = Instant::now();
    
    // Create thread-safe counter for detected payments
    let payment_counter = Arc::new(Mutex::new(0usize));
    
    // Divide the work among threads
    let blocks_per_thread = (NUM_BLOCKS + NUM_THREADS - 1) / NUM_THREADS;
    let mut handles = vec![];
    
    for thread_id in 0..NUM_THREADS {
        let thread_blocks = blocks.clone();
        let thread_scanners = scanners.clone();
        let thread_counter = payment_counter.clone();
        
        let handle = thread::spawn(move || {
            let start_idx = thread_id * blocks_per_thread;
            let end_idx = std::cmp::min(start_idx + blocks_per_thread, NUM_BLOCKS);
            
            let mut local_counter = 0;
            
            for block_idx in start_idx..end_idx {
                if block_idx >= thread_blocks.len() {
                    break;
                }
                
                let block = &thread_blocks[block_idx];
                
                for scanner in &thread_scanners {
                    let mut scanner = scanner.clone();
                    
                    for tx in &block.txdata {
                        if let Ok(payments) = scanner.scan_transaction(tx, Some(block_idx as u32)) {
                            local_counter += payments.len();
                        }
                    }
                }
            }
            
            // Update the global counter
            let mut counter = thread_counter.lock().unwrap();
            *counter += local_counter;
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let parallel_duration = start.elapsed();
    let total_parallel_payments = *payment_counter.lock().unwrap();
    
    println!("Parallel scanning complete.");
    println!("  Time: {:?}", parallel_duration);
    println!("  Payments found: {}", total_parallel_payments);
    println!("  Transactions/second: {:.2}", 
             (NUM_BLOCKS * TRANSACTIONS_PER_BLOCK * NUM_RECEIVERS) as f64 / 
             parallel_duration.as_secs_f64());
    println!();
    
    // STEP 5: Performance comparison
    println!("STEP 5: Performance comparison");
    let speedup = sequential_duration.as_secs_f64() / parallel_duration.as_secs_f64();
    println!("  Speedup: {:.2}x", speedup);
    println!("  Efficiency: {:.2}%", (speedup / NUM_THREADS as f64) * 100.0);
    println!();
    
    // STEP 6: Summary
    println!("STEP 6: Summary");
    println!("BIP-353 Silent Payments performance test completed successfully.");
    println!("Total transactions processed: {}", NUM_BLOCKS * TRANSACTIONS_PER_BLOCK);
    println!("Total scanning operations: {}", NUM_BLOCKS * TRANSACTIONS_PER_BLOCK * NUM_RECEIVERS);
    
    Ok(())
}

/// Generate test blocks with random transactions and some Silent Payments
fn generate_test_blocks(
    receivers: &[KeyManager],
    sender_keys: &[SecretKey],
    sender: &SilentPaymentSender,
    num_blocks: usize,
    txs_per_block: usize,
    payments_per_receiver: usize,
) -> anyhow::Result<Vec<Block>> {
    let mut blocks = Vec::with_capacity(num_blocks);
    let mut rng = rand::thread_rng();
    let secp = Secp256k1::new();
    
    for block_idx in 0..num_blocks {
        let mut transactions = Vec::with_capacity(txs_per_block);
        
        // Create transactions for this block
        for tx_idx in 0..txs_per_block {
            // Create a random transaction
            let outpoint = OutPoint::new(
                bitcoin::Txid::from_slice(&rand::random::<[u8; 32]>()).unwrap(),
                0,
            );
            
            let mut tx_outputs = Vec::new();
            
            // Add random outputs
            let num_outputs = rng.gen_range(1..5);
            for _ in 0..num_outputs {
                // Create random output
                let value = rng.gen_range(10_000..1_000_000);
                let dummy_pubkey = bitcoin::PublicKey::new(bitcoin::secp256k1::PublicKey::from_secret_key(
                    &secp,
                    &SecretKey::new(&mut bitcoin::secp256k1::rand::thread_rng()),
                ));
                
                let script = bitcoin::ScriptBuf::new_p2pkh(&bitcoin::PubkeyHash::from(
                    dummy_pubkey.pubkey_hash(),
                ));
                
                tx_outputs.push(TxOut {
                    value,
                    script_pubkey: script,
                });
            }
            
            // Decide if this transaction should contain Silent Payments
            let should_add_silent_payments = tx_idx < payments_per_receiver * receivers.len();
            
            if should_add_silent_payments {
                // Determine which receiver to pay to
                let receiver_idx = tx_idx % receivers.len();
                let receiver = &receivers[receiver_idx];
                
                // Pick a random sender key
                let sender_key_idx = rng.gen_range(0..sender_keys.len());
                let sender_key = &sender_keys[sender_key_idx];
                
                // Create a Silent Payment output
                let amount = Amount::from_sat(rng.gen_range(50_000..500_000));
                let output = sender.create_payment_output(
                    &receiver.generate_address(),
                    sender_key,
                    &outpoint,
                    amount,
                )?;
                
                tx_outputs.push(output);
            }
            
            // Create the transaction
            let tx = Transaction {
                version: 2,
                lock_time: 0,
                input: vec![
                    TxIn {
                        previous_output: outpoint,
                        script_sig: bitcoin::ScriptBuf::new(),
                        sequence: 0xFFFFFFFF,
                        witness: Witness::new(),
                    }
                ],
                output: tx_outputs,
            };
            
            transactions.push(tx);
        }
        
        // Create a dummy block header
        let prev_blockhash = if block_idx == 0 {
            bitcoin::BlockHash::all_zeros()
        } else {
            blocks[block_idx - 1].header.block_hash()
        };
        
        let merkle_root = bitcoin::TxMerkleNode::all_zeros();
        
        let header = BlockHeader {
            version: 1,
            prev_blockhash,
            merkle_root,
            time: 1_600_000_000 + (block_idx as u32 * 600),
            bits: 0,
            nonce: 0,
        };
        
        // Create the block
        let block = Block {
            header,
            txdata: transactions,
        };
        
        blocks.push(block);
    }
    
    Ok(blocks)
} 