# Web5 Agent Implementation

## Overview

This directory contains the implementation of AI agents specialized for Web5 in the Anya Core project. The primary focus is on enforcing the Read First Always principle in Web5 operations to ensure data consistency and integrity in decentralized systems.

## Components

### Web5 Agent (`web5_agent.rs`)

The Web5 agent implements the Read First Always principle for Web5 Decentralized Web Node (DWN) operations. Key features include:

1. **ReadFirstDwnManager**: A wrapper around standard DWN operations that enforces reads before writes.
2. **Metrics Tracking**: Collection of detailed metrics on read/write operations and compliance.
3. **Violation Detection**: Automatic detection and logging of principle violations.
4. **Testing**: Comprehensive tests that verify the implementation adheres to the principle.

## Read First Always Principle

The Read First Always principle requires that any operation modifying data (create, update, delete) must first read the current state of that data. This ensures:

1. Data consistency across distributed systems
2. Prevention of race conditions
3. Better conflict detection and resolution
4. Enhanced debugging capabilities

For detailed information about this principle, see the [READ_FIRST_ALWAYS.md](../../../docs/READ_FIRST_ALWAYS.md) document.

## Integration with Bitcoin Anchoring

The Read First principle is particularly important when working with Bitcoin-anchored data:

1. It ensures all operations verify the current blockchain state before modifications
2. It prevents potential conflicts in credential issuance and verification
3. It maintains consistency between on-chain and off-chain data

## Usage Examples

```rust
// Create a ReadFirstDwnManager
let web5_client = get_web5_client();
let manager = ReadFirstDwnManager::new(Arc::new(web5_client));

// Create a record (will automatically query similar records first)
let record = manager.create_record(&CreateRecordOptions {
    data: serde_json::to_string(&data)?,
    schema: "https://schema.org/VerifiableCredential".to_string(),
    data_format: "application/json".to_string(),
})?;

// Update a record (will automatically read the record first)
let updated_record = manager.update_record(&record.id, &UpdateRecordOptions {
    data: serde_json::to_string(&updated_data)?,
    data_format: "application/json".to_string(),
})?;

// Get compliance metrics
let metrics = manager.get_metrics();
println!("Compliance rate: {}%", metrics.compliance_rate());
```

## Testing

The implementation includes comprehensive unit tests that verify:

1. The Read First principle is enforced for all write operations
2. Metrics are correctly tracked and reported
3. Proper error handling for invalid operations
4. Compliance rate calculation

Run tests with:

```bash
cargo test --package anya-core --lib src/ml/agents/web5_agent.rs
```

## Future Improvements

1. **Extended Metrics**: Add more detailed metrics like average operation timing
2. **Recovery Strategies**: Implement automatic recovery from Read First violations
3. **Integration with Tracing**: Add distributed tracing capabilities
4. **Performance Optimizations**: Add caching for frequently accessed records
