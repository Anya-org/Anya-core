# Lightning Network Implementation

## Overview

The Lightning Network implementation in anya-core provides a robust second-layer payment protocol built on top of the Bitcoin blockchain. It allows for fast, low-cost transactions without the need to record every transaction on the blockchain, significantly improving scalability for Bitcoin applications.

## Architecture

The implementation follows the hexagonal architecture pattern with clear separation of concerns:

```
                    +---------------------+
                    |   Bitcoin Network   |
                    +----------+----------+
                               |
                               v
+----------------+   +--------------------+   +----------------+
|                |   |                    |   |                |
| Lightning Node +-->| Bitcoin-Lightning  |<--+ Bitcoin Client |
|                |   |       Bridge       |   |                |
+-------+--------+   +--------------------+   +----------------+
        |
+-------v--------+   +--------------------+   +----------------+
|                |   |                    |   |                |
| Payment Router +-->|  Payment Executor  |<--+ Invoice Manager|
|                |   |                    |   |                |
+----------------+   +--------------------+   +----------------+
```

## Core Components

### LightningNode

The central component responsible for managing Lightning Network functionality:

- Channel management
- Peer connections
- Invoice creation and payment
- Transaction signing

### BitcoinLightningBridge

Handles the interaction between the Bitcoin blockchain and the Lightning Network:

- Funding transactions for channels
- Monitoring blockchain for channel-related transactions
- Managing channel lifecycle events (opening, closing)
- Handling on-chain funds for Lightning operations

### Channel Management

Channels are the core concept in Lightning Network, allowing parties to transact off-chain:

- Channel creation with multi-signature wallets
- Channel state management
- Balance updates via commitment transactions
- Channel closure (cooperative and force-close)

### Payment Management

Components for handling Lightning payments:

- Invoice creation and decoding
- Payment routing
- Payment execution
- Multi-hop payments

## Usage Examples

### Initializing Lightning Components

```rust
use anya_core::{AnyaCore, AnyaConfig};

// Create a configuration with Lightning enabled
let mut config = AnyaConfig::default();
config.bitcoin_config.lightning_enabled = true;

// Initialize the system
let anya = AnyaCore::new(config)?;

// Access the Lightning node through the Bitcoin manager
if let Some(bitcoin_manager) = &anya.bitcoin_manager {
    if let Some(lightning_node) = bitcoin_manager.lightning_node() {
        // Now you can use the Lightning Node
        let node_info = lightning_node.get_node_info()?;
        println!("Lightning node pubkey: {}", node_info.pubkey);
    }
}
```

### Opening a Channel

```rust
// Connect to a peer
lightning_node.connect_peer("02eec7245d6b7d2ccb30380bfbe2a3648cd7a942653f5aa340edcea1f283686619", "127.0.0.1", 9735)?;

// Open a channel with the peer
let channel = lightning_node.open_channel(
    "02eec7245d6b7d2ccb30380bfbe2a3648cd7a942653f5aa340edcea1f283686619", 
    100_000, // 100,000 satoshis capacity
    Some(10_000 * 1000), // 10,000 satoshis initial push to peer (in millisatoshis)
    false // Public channel
)?;

println!("Opened channel with ID: {}", channel.channel_id);
```

### Creating and Paying Invoices

```rust
// Create an invoice
let invoice = lightning_node.create_invoice(
    Some(50_000), // 50,000 millisatoshis
    "Test payment", 
    Some(3600) // 1 hour expiry
)?;

println!("Invoice: {}", invoice.bolt11);

// Pay an invoice
let payment = lightning_node.pay_invoice(&invoice.bolt11, None)?;
println!("Payment sent with ID: {}", payment.payment_id);
```

### Using the Bitcoin-Lightning Bridge

```rust
// Create a Bitcoin-Lightning bridge
let bridge = BitcoinLightningBridge::new(Arc::new(lightning_node))?;

// Initialize with current block height
let current_height = bitcoin_manager.get_block_height()?;
bridge.init(current_height)?;

// Create a funding address for a new channel
let address = bridge.create_funding_address(
    "02eec7245d6b7d2ccb30380bfbe2a3648cd7a942653f5aa340edcea1f283686619",
    200_000, // 200,000 satoshi channel
    None, // No initial push
    false // Public channel
)?;

println!("Send funds to {} to open the channel", address);
```

## Integration with Bitcoin

The Lightning implementation integrates with the Bitcoin functionality through:

1. **On-chain funding**: Using Bitcoin transactions to fund channels
2. **Transaction monitoring**: Watching for channel funding and closing transactions
3. **Blockchain verification**: Ensuring secure channel operations
4. **Key management**: Shared key infrastructure for both on-chain and off-chain operations

## Security Considerations

- **Custody**: Lightning nodes have hot wallets with private keys
- **Channel backups**: Static channel backups to recover funds
- **Watchtowers**: Monitoring for malicious channel closures
- **Transaction verification**: Proper validation of all channel transactions

## Future Enhancements

- **BOLT12 Offers**: Support for more flexible payment requests
- **Splicing**: Adding/removing funds from channels without closing
- **Multi-part payments**: Splitting payments across multiple channels
- **Trampoline routing**: Better privacy and routing reliability

## Reference

- [BOLT Specifications](https://github.com/lightning/bolts)
- [Lightning Development Kit (LDK)](https://lightningdevkit.org/)
- [Lightning Network RFC](https://github.com/lightning/bolts/blob/master/00-introduction.md)

---

_Last updated: 2025-03-01_ 