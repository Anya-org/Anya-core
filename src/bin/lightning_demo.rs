/// Lightning Network Demo
///
/// This program demonstrates the Lightning Network functionality
/// provided by the anya-core library, including node operations,
/// channel management, and payment processing.
use std::error::Error;

use crate::{AnyaCore, AnyaResult, AnyaError};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    println!("===================================================");
    println!("⚡ Anya Core Lightning Network Demonstration");
    println!("===================================================");
    
    // Initialize with Lightning enabled
    let mut config = crate::config::Config::default();
    config.bitcoin_config.lightning_enabled = true;
    
    // Create the Anya Core instance
    let anya = AnyaCore::new(config)?;
    
    // Get the Bitcoin manager
    let bitcoin_manager = anya.bitcoin_manager.as_ref()
        .ok_or_else(|| AnyaError::Bitcoin("Bitcoin manager not initialized".to_string()))?;
    
    // Get the Lightning node
    let lightning_node = bitcoin_manager.lightning_node()
        .ok_or_else(|| AnyaError::Bitcoin("Lightning node not initialized".to_string()))?;
    
    // Step 1: Get node information
    println!("\n1. Getting Lightning node information...");
    let node_info = lightning_node.get_node_info()?;
    println!("   Node pubkey: {}", node_info.pubkey);
    println!("   Node addresses: {:?}", node_info.addresses);
    println!("   Node alias: {}", node_info.alias.as_deref().unwrap_or("None"));
    
    // Step 2: Connect to a peer
    println!("\n2. Connecting to a peer...");
    let peer_pubkey = "02eec7245d6b7d2ccb30380bfbe2a3648cd7a942653f5aa340edcea1f283686619";
    match lightning_node.connect_peer(peer_pubkey, "127.0.0.1", 9735) {
        Ok(_) => println!("   Connected to peer {}", peer_pubkey),
        Err(e) => println!("   Error connecting to peer: {:?}", e),
    }
    
    // Step 3: List connected peers
    println!("\n3. Listing connected peers...");
    match lightning_node.list_peers() {
        Ok(peers) => {
            println!("   Connected to {} peers:", peers.len());
            for peer in peers {
                println!("   - {}", peer.pubkey);
            }
        },
        Err(e) => println!("   Error listing peers: {:?}", e),
    }
    
    // Step 4: Create a Bitcoin-Lightning bridge
    println!("\n4. Creating Bitcoin-Lightning bridge...");
    let lightning_node_arc = Arc::new(lightning_node.clone());
    let bridge = match crate::layer2::lightning::BitcoinLightningBridge::new(lightning_node_arc) {
        Ok(bridge) => {
            println!("   Bridge created successfully");
            bridge
        },
        Err(e) => {
            println!("   Error creating bridge: {:?}", e);
            return Err(e);
        }
    };
    
    // Step 5: Initialize the bridge
    println!("\n5. Initializing bridge with current block height...");
    let current_height = bitcoin_manager.get_block_height()?;
    match bridge.init(current_height) {
        Ok(_) => println!("   Bridge initialized at block height {}", current_height),
        Err(e) => println!("   Error initializing bridge: {:?}", e),
    }
    
    // Step 6: Create a funding address
    println!("\n6. Creating funding address for channel...");
    match bridge.create_funding_address(peer_pubkey, 100_000, None, false) {
        Ok(address) => {
            println!("   Created funding address: {}", address);
            println!("   Send 100,000 sats to this address to open a channel");
        },
        Err(e) => println!("   Error creating funding address: {:?}", e),
    }
    
    // Step 7: Open a channel directly
    println!("\n7. Opening a direct channel with peer...");
    match lightning_node.open_channel(peer_pubkey, 50_000, Some(10_000 * 1000), false) {
        Ok(channel) => {
            println!("   Channel opened successfully:");
            println!("   Channel ID: {}", channel.channel_id);
            println!("   Capacity: {} sats", channel.capacity);
            println!("   Local balance: {} sats", channel.local_balance);
            println!("   Remote balance: {} sats", channel.remote_balance);
            
            // Register the channel with the bridge
            match bridge.register_channel_transaction(&channel, None) {
                Ok(_) => println!("   Channel registered with bridge"),
                Err(e) => println!("   Error registering channel: {:?}", e),
            }
        },
        Err(e) => println!("   Error opening channel: {:?}", e),
    }
    
    // Step 8: List channels
    println!("\n8. Listing channels...");
    match lightning_node.list_channels() {
        Ok(channels) => {
            println!("   Found {} channels:", channels.len());
            for channel in channels {
                println!("   - Channel ID: {}", channel.channel_id);
                println!("     Remote pubkey: {}", channel.remote_pubkey);
                println!("     Capacity: {} sats", channel.capacity);
                println!("     Local balance: {} sats", channel.local_balance);
                println!("     Active: {}", channel.is_active);
            }
        },
        Err(e) => println!("   Error listing channels: {:?}", e),
    }
    
    // Step 9: Create an invoice
    println!("\n9. Creating an invoice...");
    let mut invoice = None;
    match lightning_node.create_invoice(Some(50_000), "Demo payment", Some(3600)) {
        Ok(inv) => {
            println!("   Created invoice:");
            println!("   BOLT11: {}", inv.bolt11);
            println!("   Payment hash: {}", inv.payment_hash);
            println!("   Amount: {} msats", inv.amount_msat.unwrap_or(0));
            println!("   Description: {}", inv.description);
            invoice = Some(inv);
        },
        Err(e) => println!("   Error creating invoice: {:?}", e),
    }
    
    // Step 10: Decode an invoice
    if let Some(ref inv) = invoice {
        println!("\n10. Decoding invoice...");
        match lightning_node.decode_invoice(&inv.bolt11) {
            Ok(decoded) => {
                println!("   Decoded invoice:");
                println!("   Payment hash: {}", decoded.payment_hash);
                println!("   Description: {}", decoded.description);
                println!("   Amount: {:?} msats", decoded.amount_msat);
            },
            Err(e) => println!("   Error decoding invoice: {:?}", e),
        }
    }
    
    // Step 11: Pay an invoice
    if let Some(ref inv) = invoice {
        println!("\n11. Paying invoice...");
        match lightning_node.pay_invoice(&inv.bolt11, None) {
            Ok(payment) => {
                println!("   Payment successful:");
                println!("   Payment ID: {}", payment.payment_id);
                println!("   Payment hash: {}", payment.payment_hash);
                println!("   Amount: {} msats", payment.amount_msat);
                println!("   Fee: {} msats", payment.fee_msat);
                println!("   Status: {:?}", payment.status);
            },
            Err(e) => println!("   Error paying invoice: {:?}", e),
        }
    }
    
    // Step 12: List payments
    println!("\n12. Listing payments...");
    match lightning_node.list_payments() {
        Ok(payments) => {
            println!("   Found {} payments:", payments.len());
            for payment in payments {
                println!("   - Payment hash: {}", payment.payment_hash);
                println!("     Amount: {} msats", payment.amount_msat);
                println!("     Fee: {} msats", payment.fee_msat);
                println!("     Status: {:?}", payment.status);
            }
        },
        Err(e) => println!("   Error listing payments: {:?}", e),
    }
    
    // Step 13: Channel operations
    println!("\n13. Monitoring for channel updates...");
    println!("   Simulating monitoring for 5 seconds...");
    thread::sleep(Duration::from_secs(5));
    
    // Step 14: Close a channel
    println!("\n14. Closing channel...");
    match lightning_node.list_channels() {
        Ok(channels) => {
            if let Some(channel) = channels.first() {
                match lightning_node.close_channel(&channel.channel_id, false) {
                    Ok(closing_txid) => {
                        println!("   Channel closing initiated:");
                        println!("   Channel ID: {}", channel.channel_id);
                        println!("   Closing txid: {}", closing_txid);
                        
                        // Register the closing transaction
                        match bridge.register_channel_close(&channel.channel_id, closing_txid) {
                            Ok(_) => println!("   Channel close registered with bridge"),
                            Err(e) => println!("   Error registering channel close: {:?}", e),
                        }
                    },
                    Err(e) => println!("   Error closing channel: {:?}", e),
                }
            } else {
                println!("   No channels to close");
            }
        },
        Err(e) => println!("   Error listing channels: {:?}", e),
    }
    
    // Step 15: List channel transactions
    println!("\n15. Listing all channel transactions...");
    match bridge.list_channel_transactions() {
        Ok(txs) => {
            println!("   Found {} channel transactions:", txs.len());
            for tx in txs {
                println!("   - Channel ID: {}", tx.channel_id);
                println!("     Funding txid: {}", tx.funding_txid);
                println!("     Status: {:?}", tx.status);
                if let Some(txid) = tx.closing_txid {
                    println!("     Closing txid: {}", txid);
                }
            }
        },
        Err(e) => println!("   Error listing channel transactions: {:?}", e),
    }
    
    println!("\n===================================================");
    println!("⚡ Lightning Network Demonstration Completed!");
    println!("===================================================");
    
    Ok(())
}
