# Core Domain Implementation

This directory contains the core domain logic for the Bitcoin implementation, following hexagonal architecture principles.

## Module Structure

### Consensus

```rust
core::consensus
├── validation.rs   // Block and transaction validation
├── rules.rs       // Consensus rules implementation
└── params.rs      // Network parameters
```

The consensus module handles all consensus-critical code:

- BIP-341 (Taproot) validation
- Block verification
- Transaction validation
- Network parameters

### Mempool

```rust
core::mempool
├── pool.rs        // Transaction pool management
├── policy.rs      // Transaction acceptance policy
└── fees.rs        // Fee estimation
```

The mempool module manages unconfirmed transactions:

- Transaction pool management
- Fee estimation
- Replace-by-fee (RBF)
- Transaction prioritization

### Network

```rust
core::network
├── p2p.rs         // P2P network protocol
├── messages.rs    // Network message handling
└── peers.rs       // Peer management
```

The network module defines P2P protocol behavior:

- Message structures
- Peer management
- Network protocol
- Connection handling

### Script

```rust
core::script
├── interpreter.rs  // Script interpretation
└── standard.rs    // Standard script types
```

The script module handles Bitcoin script:

- Script interpretation
- Standard script types
- Tapscript support
- Witness validation

## Design Principles

1. **Pure Domain Logic**
   - No I/O operations
   - No external dependencies
   - No framework code
   - Pure business logic

2. **Error Handling**
   - Custom error types
   - Clear error messages
   - No panics
   - Proper propagation

3. **Testing**
   - Unit tests
   - Property tests
   - Fuzzing
   - Benchmarks

4. **Documentation**
   - Clear API docs
   - Usage examples
   - Performance notes
   - Security considerations

## Usage Examples

### Consensus Validation

```rust
use core::consensus::{validation, rules, params};

// Validate a block
validation::validate_block_header(&block.header)?;
rules::check_consensus_rules(&block)?;
rules::verify_pow(&block)?;
```

### Mempool Management

```rust
use core::mempool::{pool, policy, fees};

// Add transaction to mempool
let mempool = pool::Mempool::new();
policy.check_transaction(&tx)?;
mempool.add_transaction(tx)?;
```

### Network Protocol

```rust
use core::network::{p2p, messages};

// Handle network message
let handler = messages::MessageHandler::new();
handler.handle_message(message)?;
```

### Script Validation

```rust
use core::script::{interpreter, standard};

// Validate script
let interpreter = interpreter::ScriptInterpreter::new();
interpreter.execute_script(&script, &witness)?;
```

## Security Considerations

1. **Consensus Critical**
   - All consensus code is thoroughly reviewed
   - Extensive test coverage
   - Formal verification where possible
   - Security audits

2. **Memory Safety**
   - No unsafe code
   - Proper bounds checking
   - Resource limits
   - DoS protection

3. **Input Validation**
   - All inputs validated
   - Size limits enforced
   - Format checking
   - Malleability prevention

## Performance

1. **Benchmarks**
   - Critical path benchmarking
   - Memory usage tracking
   - CPU profiling
   - I/O patterns

2. **Optimization**
   - Cache friendly
   - Minimal allocations
   - Efficient algorithms
   - Resource pooling

## Contributing

When contributing to core modules:

1. **Code Review**
   - Security implications
   - Performance impact
   - Memory safety
   - Error handling

2. **Testing**
   - Unit tests required
   - Property tests recommended
   - Benchmarks for critical code
   - Security tests

3. **Documentation**
   - Clear API docs
   - Usage examples
   - Performance notes
   - Security considerations 
