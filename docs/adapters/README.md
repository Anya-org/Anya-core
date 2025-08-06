# Adapters Module Documentation

**Compliance Tags**: [AIR-3][AIS-3][BPC-3][RES-3]

[AIS-3]: #ais-3 "Application Integration Standard Level 3"
[RES-3]: #res-3 "Resource Efficiency Standard Level 3"

## Overview

The Adapters module provides input and output adapters for integrating Anya Core with external systems and protocols. Adapters implement standardized interfaces for communication, data exchange, and protocol compliance.

## Core Components

### Input Adapters

- **RestApi**: Handles incoming REST API requests and delegates them to command handlers.

### Output Adapters

- **BitcoinNodeClient**: Connects to external Bitcoin nodes via RPC for transaction broadcasting, block retrieval, and network monitoring.

## Usage Example

```rust
use anya_core::adapters::{RestApi, BitcoinNodeClient};

let rest_api = RestApi { handler: my_command_handler };
let btc_client = BitcoinNodeClient { rpc_client: my_rpc_client };
```

## Integration Points

- **Network Module**: For network communication
- **Bitcoin Module**: For Bitcoin node integration
- **Web Module**: For web API handling
- **Test Module**: For mock adapter implementations

## Compliance Standards

### AIR-3

Ensures high availability and integrity by providing robust adapter implementations and error handling.

### AIS-3

Comprehensive APIs for integration with external systems and protocols.

### BPC-3

Implements Bitcoin protocol adapters for full compatibility.

### RES-3

Efficient data exchange and resource management for minimal overhead.
