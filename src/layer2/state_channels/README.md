# layer2/state_channels Module

State Channels protocol implementation for Layer2 Bitcoin scaling

This module provides a comprehensive State Channels protocol implementation following
the Layer2 async architecture patterns and official Bitcoin standards.

## Overview

The `state_channels` module implements a complete state channels protocol that enables off-chain transactions between participants with on-chain settlement guarantees. State channels significantly improve Bitcoin scalability by allowing multiple transactions to occur off-chain while maintaining the security properties of the Bitcoin blockchain.

## Key Components

### StateChannelsProtocol

The main protocol implementation that:

- Manages the lifecycle of state channels
- Handles opening, updating, and closing channels
- Processes off-chain state transitions
- Implements the Layer2Protocol trait for integration with the core system

```rust
let config = StateChannelsConfig {
    network: "testnet".to_string(),
    max_channel_value: 10_000_000, // 0.1 BTC
    channel_timeout: 1008,         // 1 week in blocks
    min_channel_capacity: 100_000, // 0.001 BTC
};

let protocol = StateChannelsProtocol::new(config);
```

### Channel Management

The module provides comprehensive state channel operations:

- **Channel Opening**: Establish new channels between participants
- **State Updates**: Update channel state with new balance distributions
- **Dispute Resolution**: Handle disputes when participants disagree
- **Channel Closing**: Safely close channels and settle on-chain

```rust
// Open a new channel
let channel_id = protocol.open_channel(
    vec!["alice".to_string(), "bob".to_string()],
    HashMap::from([
        ("alice".to_string(), 5_000_000),
        ("bob".to_string(), 5_000_000)
    ])
).await?;

// Update channel state
protocol.update_state(
    &channel_id,
    HashMap::from([
        ("alice".to_string(), 6_000_000),
        ("bob".to_string(), 4_000_000)
    ]),
    HashMap::new() // signatures
).await?;
```

### Protocol Features

- **High Throughput**: Unlimited transactions between channel participants
- **Low Latency**: Instant confirmation of off-chain transactions
- **Privacy**: Transactions only visible to channel participants
- **Security**: On-chain settlement guarantees
- **Smart Contract Support**: Conditional payments and state transitions

### Layer2 Integration

The module implements the Layer2Protocol trait, providing:

- Standard protocol lifecycle management
- Health monitoring and diagnostics
- State synchronization and validation
- Transaction submission and verification
- Fee estimation

## Technical Details

### Channel States

Channels can exist in various states:

- **Opening**: Channel is being established but not yet operational
- **Open**: Channel is active and can process transactions
- **Updating**: Channel state is being updated
- **Disputed**: Participants disagree on channel state
- **Closing**: Channel is in the process of closing
- **Closed**: Channel is permanently closed

### State Updates

Each state update includes:

- New balance distribution among participants
- Incremental state number
- Cryptographic signatures from all participants
- Timestamp for auditability

## For more information

See the comprehensive documentation in the [docs/](/docs/) directory.
