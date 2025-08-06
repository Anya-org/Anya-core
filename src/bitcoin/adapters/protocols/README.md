# Bitcoin Protocol Adapters Module

[![AIR-3](https://img.shields.io/badge/AIR-Level%203-green)](../../../../docs/compliance/README.md)
[![AIS-3](https://img.shields.io/badge/AIS-Level%203-green)](../../../../docs/compliance/README.md)
[![BPC-3](https://img.shields.io/badge/BPC-Level%203-green)](../../../../docs/compliance/README.md)
[![AIT-3](https://img.shields.io/badge/AIT-Level%203-green)](../../../../docs/compliance/README.md)
[![RES-3](https://img.shields.io/badge/RES-Level%203-green)](../../../../docs/compliance/README.md)

## Overview

The `bitcoin/adapters/protocols` module implements Bitcoin protocol-specific adapters following the hexagonal architecture pattern. This module serves as an interface layer between the Bitcoin protocol implementations and the core Bitcoin functionality in Anya Core.

## Key Components

### BitcoinProtocolAdapter

This is the primary adapter that implements the `BitcoinPort` interface for interacting with Bitcoin networks. It handles protocol-level operations such as connecting to the Bitcoin network, sending and receiving transactions, and retrieving blockchain information.

```rust
pub struct BitcoinProtocolAdapter {
    /// Bitcoin protocol compliance level
    compliance_level: BPCLevel,
    /// Connection URL
    url: String,
    /// Is the adapter connected
    connected: bool,
}
```

#### Core Methods

- `new(url: &str, compliance_level: BPCLevel) -> Self`: Creates a new protocol adapter with the specified URL and compliance level
- `compliance_level() -> BPCLevel`: Returns the current compliance level of the adapter
- `is_connected() -> bool`: Checks if the adapter is connected to the network
- `verify_transaction(tx: &Transaction) -> Result<bool>`: Verifies a transaction against the current compliance level

### BitcoinPort Implementation

The `BitcoinProtocolAdapter` implements the `BitcoinPort` trait, providing the following functionality:

```rust
impl BitcoinPort for BitcoinProtocolAdapter {
    fn connect(&self) -> Result<()>;
    fn disconnect(&self) -> Result<()>;
    fn send_transaction(&self, tx: &Transaction) -> Result<Txid>;
    fn get_transaction(&self, txid: &Txid) -> Result<Option<Transaction>>;
    fn get_block(&self, hash: &BlockHash) -> Result<Option<Block>>;
    fn get_blockchain_info(&self) -> Result<Value>;
}
```

## Usage Examples

### Creating and Using a Protocol Adapter

```rust
use crate::bitcoin::adapters::protocols::BitcoinProtocolAdapter;
use crate::bitcoin::adapters::BitcoinPort;
use crate::bitcoin::protocol::BPCLevel;
use bitcoin::Transaction;

// Create a new adapter with BPC-3 compliance level
let adapter = BitcoinProtocolAdapter::new("https://bitcoin-node:8332", BPCLevel::Bpc3);

// Connect to the Bitcoin network
adapter.connect()?;

// Send a transaction
let tx = Transaction::from_hex("0200000001...")?;
let txid = adapter.send_transaction(&tx)?;

// Get blockchain info
let blockchain_info = adapter.get_blockchain_info()?;
println!("Current block height: {}", blockchain_info["blocks"]);

// Disconnect when done
adapter.disconnect()?;
```

### Verifying Transaction Compliance

```rust
use crate::bitcoin::adapters::protocols::BitcoinProtocolAdapter;
use crate::bitcoin::protocol::BPCLevel;
use bitcoin::Transaction;

// Create a new adapter with BPC-3 compliance level
let adapter = BitcoinProtocolAdapter::new("https://bitcoin-node:8332", BPCLevel::Bpc3);

// Parse a transaction
let tx = Transaction::from_hex("0200000001...")?;

// Verify the transaction against BPC-3 compliance rules
match adapter.verify_transaction(&tx) {
    Ok(true) => println!("Transaction is compliant with BPC-3"),
    Ok(false) => println!("Transaction is not compliant with BPC-3"),
    Err(e) => println!("Error verifying transaction: {}", e),
}
```

## Error Handling

The module uses `anyhow::Result` for error handling, which provides rich context for errors. Error conditions include:

- Network connectivity issues
- Transaction validation failures
- Invalid block or transaction references

Example error handling:

```rust
let result = adapter.send_transaction(&tx);
match result {
    Ok(txid) => println!("Transaction sent successfully: {}", txid),
    Err(e) => println!("Failed to send transaction: {}", e),
}
```

## Compliance Standards

This module adheres to the following compliance standards:

- **AIR-3**: Architecture Interface Requirements Level 3
- **AIS-3**: Architecture Implementation Standards Level 3
- **BPC-3**: Bitcoin Protocol Compliance Level 3
- **AIT-3**: Architecture Integration Testing Level 3
- **RES-3**: Resiliency Engineering Standards Level 3

## Integration with Other Modules

The Bitcoin Protocol Adapters module integrates with:

- `bitcoin::protocol`: For transaction validation and compliance verification
- `bitcoin::adapters`: For the `BitcoinPort` interface definition
- Core Bitcoin types: For working with transactions, blocks, and blockchain data

## For More Information

See the comprehensive documentation in the [docs/](../../../../docs/) directory.
