---
title: "API Reference"
description: "Complete API documentation for Anya Core"
last_updated: $(date +%Y-%m-%d)
---

[AIR-3][AIS-3][BPC-3][RES-3]

# API Reference

## Overview

This section provides comprehensive API documentation for all Anya Core modules.

## Generated Documentation

The complete API documentation is available through Rust's built-in documentation system:

```bash
# Generate and open full API documentation
cargo doc --open

# Generate docs with private items
cargo doc --document-private-items --open
```

## Module APIs

The following modules provide public APIs:

- [adapters API](../target/doc/anya_core/adapters/index.html)
- [ai API](../target/doc/anya_core/ai/index.html)
- [alignment API](../target/doc/anya_core/alignment/index.html)
- [api API](../target/doc/anya_core/api/index.html)
- [audit API](../target/doc/anya_core/audit/index.html)
- [backup API](../target/doc/anya_core/backup/index.html)
- [bin API](../target/doc/anya_core/bin/index.html)
- [bip API](../target/doc/anya_core/bip/index.html)
- [bips API](../target/doc/anya_core/bips/index.html)
- [bitcoin API](../target/doc/anya_core/bitcoin/index.html)
- [blockchain API](../target/doc/anya_core/blockchain/index.html)
- [cache API](../target/doc/anya_core/cache/index.html)
- [checkpoint API](../target/doc/anya_core/checkpoint/index.html)
- [compliance API](../target/doc/anya_core/compliance/index.html)
- [components API](../target/doc/anya_core/components/index.html)
- [config API](../target/doc/anya_core/config/index.html)
- [core API](../target/doc/anya_core/core/index.html)
- [crosschain API](../target/doc/anya_core/crosschain/index.html)
- [crypto API](../target/doc/anya_core/crypto/index.html)
- [dao API](../target/doc/anya_core/dao/index.html)
- [dashboard API](../target/doc/anya_core/dashboard/index.html)
- [examples API](../target/doc/anya_core/examples/index.html)
- [extensions API](../target/doc/anya_core/extensions/index.html)
- [gdpr API](../target/doc/anya_core/gdpr/index.html)
- [governance API](../target/doc/anya_core/governance/index.html)
- [handlers API](../target/doc/anya_core/handlers/index.html)
- [hardware API](../target/doc/anya_core/hardware/index.html)
- [infrastructure API](../target/doc/anya_core/infrastructure/index.html)
- [install API](../target/doc/anya_core/install/index.html)
- [layer2 API](../target/doc/anya_core/layer2/index.html)
- [lightning API](../target/doc/anya_core/lightning/index.html)
- [ml API](../target/doc/anya_core/ml/index.html)
- [mobile API](../target/doc/anya_core/mobile/index.html)
- [module API](../target/doc/anya_core/module/index.html)
- [monitoring API](../target/doc/anya_core/monitoring/index.html)
- [network API](../target/doc/anya_core/network/index.html)
- [open_banking API](../target/doc/anya_core/open_banking/index.html)
- [performance API](../target/doc/anya_core/performance/index.html)
- [ports API](../target/doc/anya_core/ports/index.html)
- [protocols API](../target/doc/anya_core/protocols/index.html)
- [resource API](../target/doc/anya_core/resource/index.html)
- [security API](../target/doc/anya_core/security/index.html)
- [storage API](../target/doc/anya_core/storage/index.html)
- [tenant API](../target/doc/anya_core/tenant/index.html)
- [test API](../target/doc/anya_core/test/index.html)
- [testing API](../target/doc/anya_core/testing/index.html)
- [tokenomics API](../target/doc/anya_core/tokenomics/index.html)
- [tools API](../target/doc/anya_core/tools/index.html)
- [types API](../target/doc/anya_core/types/index.html)
- [utils API](../target/doc/anya_core/utils/index.html)
- [web API](../target/doc/anya_core/web/index.html)
- [web5 API](../target/doc/anya_core/web5/index.html)

## Usage Examples

### Basic Library Usage

```rust
use anya_core::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize configuration
    let config = AnyaConfig::default();

    // Create core instance
    let core = AnyaCore::new(config).await?;

    // Use core functionality
    // ... your code here

    Ok(())
}
```

### Feature-Specific Usage

```rust
// Bitcoin functionality
#[cfg(feature = "bitcoin")]
use anya_core::bitcoin::BitcoinAdapter;

// Web5 functionality
#[cfg(feature = "web5")]
use anya_core::web5::Web5Adapter;

// ML functionality
use anya_core::ml::MLSystem;
```

## See Also

- [Getting Started Guide](../getting-started/README.md)
- [Module Documentation](../README.md)
- [Architecture Guide](../architecture/README.md)

*API documentation is automatically generated from source code.*
