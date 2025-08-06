# Layer2: Lightning Network Module

**Compliance Tags**: [AIR-3][AIS-3][AIM-3][BPC-3][RES-3]

## Introduction

The Lightning Network module provides a comprehensive implementation that enables fast, low-cost Bitcoin transactions through a second-layer protocol built on top of the Bitcoin blockchain. This implementation follows official Bitcoin Improvement Proposals (BIPs) and Lightning Network specifications.

## Features

- Complete Lightning Network node management
- Payment channel creation, management, and monitoring
- Invoice generation and payment processing
- Multi-hop payment routing
- BOLT (Basis of Lightning Technology) compliance
- Channel balancing and liquidity management
- Watchtower integration for security

## Core Components

### LightningConfig

Configuration settings for Lightning Network nodes:

```rust
pub struct LightningConfig {
    /// Network type: mainnet, testnet, regtest
    pub network: String,
    /// Node URL
    pub node_url: String,
    /// Macaroon for authentication (hex encoded)
    pub macaroon: String,
    /// TLS certificate (base64 encoded)
    pub cert: String,
    /// Node alias
    pub alias: String,
    /// Auto-pilot enabled
    pub autopilot: bool,
    /// Channel capacity limits
    pub min_channel_size: u64,
    pub max_channel_size: u64,
}
```

### ChannelInfo

Information about Lightning Network payment channels:

```rust
pub struct ChannelInfo {
    pub channel_id: String,
    pub capacity: u64,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub active: bool,
    pub peer_pubkey: String,
    pub initiator: bool,
    pub private: bool,
}
```

### LightningInvoice

Payment invoice details:

```rust
pub struct LightningInvoice {
    pub payment_request: String,
    pub r_hash: String,
    pub r_preimage: Option<String>,
    pub value: u64,
    pub settled: bool,
    pub creation_date: u64,
    pub expiry: u64,
    pub description: String,
}
```

### LightningProtocol

Main implementation of the Lightning Network protocol:

```rust
pub struct LightningProtocol {
    config: LightningConfig,
    connected: Arc<RwLock<bool>>,
    node_info: Arc<RwLock<Option<NodeInfo>>>,
    channels: Arc<RwLock<HashMap<String, ChannelInfo>>>,
    invoices: Arc<RwLock<HashMap<String, LightningInvoice>>>,
    payments: Arc<RwLock<HashMap<String, LightningPayment>>>,
    transactions: Arc<RwLock<HashMap<String, TransactionResult>>>,
}
```

## Usage Examples

### Node Initialization

```rust
use anya::layer2::lightning::{LightningConfig, LightningProtocol};
use anya::layer2::Layer2Protocol;

async fn initialize_lightning() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let config = LightningConfig {
        network: "testnet".to_string(),
        node_url: "127.0.0.1:10009".to_string(),
        macaroon: "0201036c6e64022f03...".to_string(),
        cert: "LS0tLS1CRUdJTiBDRVJUSUZ...".to_string(),
        alias: "my-lightning-node".to_string(),
        autopilot: true,
        min_channel_size: 200_000,   // 200k sats
        max_channel_size: 50_000_000, // 0.5 BTC
    };

    // Initialize protocol
    let lightning = LightningProtocol::new(config);

    // Connect to the node
    lightning.initialize().await?;

    // Get node information
    let node_info = lightning.get_node_info().await?;
    println!("Connected to node: {}", node_info.alias);
    println!("Public key: {}", node_info.identity_pubkey);
    println!("Active channels: {}", node_info.num_active_channels);

    Ok(())
}
```

### Creating Payment Channels

```rust
use anya::layer2::lightning::LightningProtocol;

async fn open_channel(
    lightning: &LightningProtocol,
    peer_pubkey: &str,
    capacity: u64
) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to peer
    lightning.connect_peer(peer_pubkey, "127.0.0.1:9735").await?;

    // Open channel
    let channel = lightning.open_channel(peer_pubkey, capacity, 0).await?;
    println!("Channel opened: {}", channel.channel_id);
    println!("Capacity: {} sats", channel.capacity);

    Ok(())
}
```

### Processing Payments

```rust
use anya::layer2::lightning::LightningProtocol;

async fn process_payment(
    lightning: &LightningProtocol
) -> Result<(), Box<dyn std::error::Error>> {
    // Create invoice
    let invoice = lightning.create_invoice(50000, "Coffee payment", 3600).await?;
    println!("Payment request: {}", invoice.payment_request);

    // Wait for payment
    let paid_invoice = lightning.wait_for_invoice_payment(&invoice.r_hash, 300).await?;
    if paid_invoice.settled {
        println!("Payment received: {} sats", paid_invoice.value);
    }

    // Make a payment
    let payment_result = lightning.pay_invoice("lnbc5000n1...").await?;
    match payment_result.status {
        PaymentStatus::Succeeded => {
            println!("Payment sent successfully!");
            println!("Preimage: {}", payment_result.payment_preimage.unwrap());
        },
        PaymentStatus::Failed => {
            println!("Payment failed: {}", payment_result.failure_reason.unwrap());
        },
        _ => println!("Payment status: {:?}", payment_result.status),
    }

    Ok(())
}
```

## Security Considerations

1. **Watchtower Integration**: Always use watchtowers to protect against channel breaches when the node is offline
2. **Backup Management**: Regularly back up channel states and SCB (Static Channel Backup) files
3. **Macaroon Permissions**: Use the least-privileged macaroons for different operations
4. **Fund Management**: Don't put more funds in Lightning channels than you're willing to actively manage
5. **Channel Monitoring**: Implement monitoring for force-closes and unilateral channel closures

## Integration with Other Modules

The Lightning Network module integrates with:

- **Bitcoin Core**: For on-chain funding and settlement
- **Key Management**: For secure signing operations
- **Wallet**: For managing on-chain funds and channel balances
- **Monitoring**: For channel health and network status

## For More Information

- [Lightning Network RFC Repository](https://github.com/lightning/bolts)
- [BOLT Specifications](https://github.com/lightning/bolts/blob/master/00-introduction.md)
- Project documentation
