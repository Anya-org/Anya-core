# dao/compat/clarity_repl/vm Module

Stacks VM types compatibility layer

This module contains types compatible with Stacks VM/Clarity

## Overview

The `vm` module provides a Stacks Virtual Machine compatibility layer, implementing core Clarity data types and transaction structures needed for smart contract execution within the Anya Core DAO system.

## Key Components

### Principal Data Management

Stacks principal address handling:

- **Address Management**: Handle Stacks blockchain addresses
- **Contract Principals**: Support contract-based principals
- **Principal Parsing**: Parse principal strings with contract names

```rust
use anya_core::dao::compat::clarity_repl::vm::PrincipalData;

// Create standard principal
let principal = PrincipalData::from("ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM".to_string());

// Create contract principal
let contract_principal = PrincipalData::from("ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.my-contract".to_string());
```

### Clarity Value System

Complete Clarity type system implementation:

- **Primitive Types**: Integer, UInt, Bool, String
- **Complex Types**: Principal, Optional, Sequence
- **Type Safety**: Maintain Clarity's type safety guarantees

```rust
use anya_core::dao::compat::clarity_repl::vm::Value;

// Create different value types
let int_val = Value::Int(-42);
let uint_val = Value::UInt(42);
let bool_val = Value::Bool(true);
let string_val = Value::String("hello".to_string());

// Create optional values
let some_val = Value::Some(Box::new(Value::UInt(100)));
let none_val = Value::None;

// Create sequences
let list_val = Value::Sequence(vec![Value::UInt(1), Value::UInt(2), Value::UInt(3)]);
```

### Transaction Management

Stacks transaction structures for contract interaction:

- **Contract Calls**: Structure for calling contract functions
- **Function Arguments**: Type-safe argument passing
- **Sender Management**: Principal-based sender identification

```rust
use anya_core::dao::compat::clarity_repl::vm::{StacksTransaction, Value, PrincipalData};

// Create contract call transaction
let transaction = StacksTransaction {
    contract_call: "my-contract".to_string(),
    function_name: "transfer".to_string(),
    args: vec![
        Value::Principal(PrincipalData::from("ST1...".to_string())),
        Value::UInt(1000),
    ],
    sender: Some(PrincipalData::from("ST2...".to_string())),
};
```

## API Reference

### PrincipalData

- `from(address)`: Create principal from address string
- `address`: Stacks blockchain address
- `contract_name`: Optional contract name for contract principals

### Value

- `Int(i128)`: Signed 128-bit integer
- `UInt(u128)`: Unsigned 128-bit integer
- `Bool(bool)`: Boolean value
- `Principal(PrincipalData)`: Stacks principal
- `String(String)`: UTF-8 string
- `Some(Box<Value>)`: Optional value (present)
- `None`: Optional value (absent)
- `Sequence(Vec<Value>)`: List/sequence of values

### StacksTransaction

- `contract_call`: Target contract identifier
- `function_name`: Contract function to call
- `args`: Function arguments as Clarity values
- `sender`: Transaction sender principal

## For more information

See the comprehensive documentation in the [docs/](/docs/) directory.
