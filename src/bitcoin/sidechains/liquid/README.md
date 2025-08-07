# Bitcoin Sidechains: Liquid Module

[AIR-3][AIS-3][BPC-3][RES-3]

## Introduction

The Liquid module provides integration with Blockstream's Liquid sidechain, offering fast settlement and confidential transactions for Bitcoin. This module enables working with Liquid assets, transactions, and the peg-in/peg-out process.

## Features

- Liquid asset creation, issuance, and management
- Confidential transaction support
- Federation monitoring and management
- Two-way peg operations (peg-in/peg-out)
- Full Liquid RPC client implementation

## Core Components

### LiquidAsset

Represents a Liquid asset:

```rust
pub struct LiquidAsset {
    /// Asset ID
    pub id: String,

    /// Asset name
    pub name: Option<String>,

    /// Asset ticker
    pub ticker: Option<String>,

    /// Precision (number of decimal places)
    pub precision: u8,

    /// Total issuance
    pub total_issuance: u64,

    /// Issuer
    pub issuer: Option<String>,

    /// Is confidential
    pub is_confidential: bool,
}
```

### LiquidTransaction

Represents a transaction on the Liquid sidechain:

```rust
pub struct LiquidTransaction {
    /// Transaction ID
    pub txid: String,

    /// Transaction version
    pub version: u32,

    /// Transaction inputs
    pub inputs: Vec<LiquidTxInput>,

    /// Transaction outputs
    pub outputs: Vec<LiquidTxOutput>,

    /// Transaction locktime
    pub locktime: u32,
}
```

### LiquidClient

Client for interacting with a Liquid node:

```rust
/// Create a new Liquid client
pub fn new_liquid_client(url: &str, username: &str, password: &str) -> AnyaResult<LiquidClient> {
    // Creates a client connected to a Liquid node
}

/// Get information about a Liquid asset
pub async fn get_asset_info(asset_id: &str) -> AnyaResult<LiquidAsset> {
    // Retrieves asset information
}

/// Create and issue a new Liquid asset
pub async fn issue_asset(name: &str, amount: u64, is_confidential: bool) -> AnyaResult<LiquidAsset> {
    // Issues a new asset
}
```

## Error Handling

The Liquid module defines specific error types:

```rust
pub enum LiquidError {
    /// Network error
    NetworkError(String),

    /// RPC error
    RpcError(String),

    /// Transaction error
    TransactionError(String),
}
```

## Usage Examples

### Working with Liquid Assets

```rust
use anya::bitcoin::sidechains::liquid::{LiquidClient, LiquidAsset};

async fn manage_liquid_assets() -> AnyaResult<()> {
    // Connect to Liquid node
    let client = LiquidClient::new("http://localhost:7041", "user", "password")?;

    // Get information about L-BTC
    let lbtc = client.get_asset_info("6f0279e9ed041c3d710a9f57d0c02928416460c4b722ae3457a11eec381c526d").await?;
    println!("L-BTC precision: {}", lbtc.precision);

    // Issue a new asset
    let new_asset = client.issue_asset("My Token", 1_000_000, true).await?;
    println!("New asset issued: {}", new_asset.id);

    Ok(())
}
```

### Performing a Peg-In Operation

```rust
use anya::bitcoin::sidechains::liquid::{LiquidClient, PegOperation};

async fn perform_peg_in(btc_amount: f64) -> AnyaResult<()> {
    let client = LiquidClient::new("http://localhost:7041", "user", "password")?;

    // Generate peg-in address
    let peg_in_address = client.get_peg_in_address().await?;
    println!("Send BTC to this address: {}", peg_in_address.bitcoin_address);
    println!("Claim code: {}", peg_in_address.claim_script);

    // Monitor peg-in status
    let status = client.get_peg_in_status(&peg_in_address.claim_script).await?;
    println!("Peg-in status: {:?}", status);

    Ok(())
}
```

## Federation Information

The Liquid Federation consists of functionaries that secure the network. This module provides functionality to monitor federation status:

```rust
// Get current federation status
let federation = client.get_federation_status().await?;
println!("Active functionaries: {}/{}", federation.active_members, federation.total_members);
```

## For more information

- [Liquid Documentation](https://docs.blockstream.com/liquid/technical_overview.html)
- See the comprehensive documentation in the [docs/](../../../docs/) directory
