# Bitcoin Protocol Adapter Documentation

## Overview

The Bitcoin Protocol Adapter provides a unified interface for Bitcoin protocol interactions with full BIP-342 support. This adapter consolidates previously scattered Bitcoin-related functionality across the codebase into a cohesive, well-structured module.

## Components

### BitcoinAdapter

The main adapter class that implements the ProtocolAdapter trait:

```rust
pub struct BitcoinAdapter {
    network: Network,
    bip342_enabled: bool,
    secp: Secp256k1<bitcoin::secp256k1::All>,
    validator: validation::BitcoinValidator,
}
```

### Validation Module

A consolidated validation system that implements BIP-342 (Tapscript) validation:

```rust
// Create a validator with specific validation standards
let validator = BitcoinValidator::new(
    Network::Testnet, 
    ValidationStandard::Tapscript
);

// Validate a transaction
validator.validate_transaction(&transaction, None)?;

// Validate a block
validator.validate_block(&block)?;
```

### Tapscript Module

Implements Tapscript functionality for BIP-342:

```rust
// Create a tapscript handler
let handler = TapscriptHandler::new(true);

// Create and validate tapscripts
let script = handler.create_script(opcodes, data)?;
handler.verify_script(&script, &witness_data)?;
```

### PSBT Module

Handles Partially Signed Bitcoin Transactions:

```rust
// Create and modify PSBTs
let psbt = PsbtHandler::new();
let tx = psbt.build_transaction(inputs, outputs)?;
```

## BIP-342 Support

The Bitcoin Protocol Adapter fully supports BIP-342 (Tapscript) with the following features:

1. **Taproot Script Validation**: Complete validation of Taproot scripts according to BIP-342 specifications
2. **Leaf Version Checks**: Proper validation of Tapscript leaf versions
3. **Disabled Opcode Detection**: Checking for disallowed opcodes like OP_CHECKMULTISIG
4. **Script Size Limits**: Enforcing the 10,000 byte script size limit

## Usage Examples

### Basic Transaction Validation

```rust
// Create a Bitcoin adapter with BIP-342 support
let adapter = BitcoinAdapter::new(true)?;

// Validate a transaction
if adapter.verify_tapscript(&transaction, leaf_hash)? {
    println!("Transaction is valid!");
}
```

### Block Validation

```rust
// Create a Bitcoin adapter with BIP-342 support
let adapter = BitcoinAdapter::new(true)?;

// Validate a complete block including all transactions
if adapter.validate_block(&block)? {
    println!("Block is valid!");
}
```

### Creating Taproot Outputs

```rust
// Create a Bitcoin adapter with BIP-342 support
let adapter = BitcoinAdapter::new(true)?;

// Create a taproot output from a script and internal key
let output = adapter.create_taproot_output(script, internal_key)?;
```

## Historical Context

This implementation consolidates previously scattered Bitcoin validation logic from:

- `/home/anya/anya-core/anyacore/src/bitcoin_internal/psbt.rs`
- `/home/anya/anya-core/anyacore/src/bitcoin/psbt.rs`
- `/home/anya/anya-core/anyacore/src/tapscript.rs`
- `/home/anya/anya-core/core/src/psbt.rs`

The consolidated module provides a single source of truth for BIP-342 validation, reducing code duplication and improving maintainability.

## MCP Server Integration

The Bitcoin Protocol Adapter integrates with the MCP server through the protocol-adapters package. The MCP server properly initializes the Bitcoin adapter during startup, ensuring that the correct sequence is followed:

1. Update health status to "starting"
2. Create the BitcoinAdapter instance
3. Start the adapter before storing it
4. Only store the adapter after successful initialization
5. Update health status to "running" once everything is properly started
