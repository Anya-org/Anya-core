# dao/compat/clarity_repl/repl Module

Stacks REPL compatibility layer

This module contains types compatible with Stacks REPL environment

## Overview

The `repl` module provides a Stacks Clarity REPL (Read-Eval-Print Loop) compatibility layer that enables interactive testing and development of Clarity smart contracts within the Anya Core environment.

## Key Components

### Session Management

REPL session management for contract testing:

- **Contract Deployment**: Deploy and manage test contracts
- **State Management**: Maintain session state across interactions
- **Principal Management**: Handle Stacks principal addresses

```rust
use anya_core::dao::compat::clarity_repl::repl::{Session, TestEnvironment};

// Create a new REPL session
let mut session = Session::new();

// Deploy a contract
session.deploy_contract("test-contract", "(define-public (hello) (ok \"world\"))")?;

// Call contract functions
let result = session.call_contract("test-contract", "hello", &[])?;
```

### Test Environment

Comprehensive test environment for Clarity development:

- **Transaction Execution**: Execute contract transactions in test mode
- **Read-Only Calls**: Query contract state without state changes
- **Epoch Management**: Support different Stacks blockchain epochs

```rust
// Initialize test environment
let mut env = TestEnvironment::new();

// Execute transactions
let tx_request = TransactionRequest {
    contract_call: "test-contract".to_string(),
    function_name: "hello".to_string(),
    function_args: vec![],
    sender: PrincipalData::from("ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM".to_string()),
};

let result = env.execute_transaction(tx_request)?;
```

## API Reference

### Session

- `new()`: Create new REPL session
- `deploy_contract(name, code)`: Deploy Clarity contract
- `call_contract(contract, function, args)`: Call contract function

### TestEnvironment

- `new()`: Create test environment
- `execute_transaction(request)`: Execute contract transaction
- `execute_read_only(request)`: Execute read-only contract call

### Request Types

- `TransactionRequest`: State-changing contract call
- `ReadOnlyRequest`: Non-state-changing contract query

## For more information

See the comprehensive documentation in the [docs/](/docs/) directory.
