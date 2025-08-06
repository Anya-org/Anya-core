*title: "Api Reference"
description: "Documentation for Api Reference"

[AIR-3][AIS-3][BPC-3][RES-3]

[AIS-3]: https://github.com/Anya-org/Anya-core/docs/standards/AI_LABELING.md#ais-3


<!-- markdownlint-disable MD013 line-length -->

# API Reference

## Table of Contents

    - Section 1
    - Section 2


## Overview

Anya provides a comprehensive REST and WebSocket API for integrating Bitcoin infrastructure into enterprise applications. This reference covers all available endpoints, authentication, error handling, and best practices.

## Authentication

### API Keys


    // Request with API key
    GET /api/v1/transactions
    Authorization: Bearer YOUR_API_KEY


### OAuth2

grant_type=client_credentials
    // OAuth2 token request
    POST /oauth/token
    Content-Type: application/x-www-form-urlencoded
grant_type=client_credentials
    // OAuth2 token request
    POST /oauth/token
    Content-Type: application/x-www-form-urlencoded

    grant_type=client_credentials
    &client_id=YOUR_CLIENT_ID
    &client_secret=YOUR_CLIENT_SECRET
    grant_type=client_credentials
    &client_id=YOUR_CLIENT_ID
    &client_secret=YOUR_CLIENT_SECRET

## REST API

### Transaction Endpoints


#### Create Transaction

    POST /api/v1/transactions

    GET /api/v1/transactions/{txid}

    Content-Type: application/json
    POST /api/v1/transactions

    GET /api/v1/transactions/{txid}

    Content-Type: application/json

    {
        "recipients": [{
            "address": "bc1q...",
            "amount": "0.1"
        }],
        "fee_rate": "5",
        "rbf": true
    }
    {
        "recipients": [{
            "address": "bc1q...",
            "amount": "0.1"
        }],
        "fee_rate": "5",
        "rbf": true
    }

#### Get Transaction

    GET /api/v1/transactions/{txid}
    GET /api/v1/transactions/{txid}

#### List Transactions

    GET /api/v1/transactions?limit=10&offset=0
    GET /api/v1/transactions?limit=10&offset=0

### Wallet Endpoints

#### Create Wallet

    POST /api/v1/wallets
    Content-Type: application/json
    POST /api/v1/wallets
    Content-Type: application/json

    {
        "name": "main",
        "type": "p2wpkh",  # p2wpkh (Pay-to-Witness-Public-Key-Hash) is a valid Bitcoin address type
        "backup_type": "encrypted"
    }


#### Get Wallet

    GET /api/v1/wallets/{wallet_id}
    GET /api/v1/wallets/{wallet_id}

#### List Wallets


    GET /api/v1/wallets?limit=10&offset=0
    GET /api/v1/wallets?limit=10&offset=0

### Contract Endpoints

#### Create Contract

GET /api/v1/transactions/{txid}

    POST /api/v1/contracts
    Content-Type: application/json
GET /api/v1/transactions/{txid}

    POST /api/v1/contracts
    Content-Type: application/json

    {
        "type": "dlc",
        "oracle": "oracle_id",
        "outcomes": ["true", "false"],
        "collateral": "1.0"
    }

    GET /api/v1/transactions/{txid}
    {
        "type": "dlc",
        "oracle": "oracle_id",
        "outcomes": ["true", "false"],
        "collateral": "1.0"
    }

    GET /api/v1/transactions/{txid}

#### Get Contract

GET /api/v1/transactions/{txid}

    GET /api/v1/contracts/{contract_id}
GET /api/v1/transactions/{txid}

    GET /api/v1/contracts/{contract_id}

#### Execute Contract

    PUT /api/v1/contracts/{contract_id}/execute
    Content-Type: application/json
    PUT /api/v1/contracts/{contract_id}/execute
    Content-Type: application/json

    {
        "outcome": "true"
    }
    {
        "outcome": "true"
    }

## WebSocket API

### Connection

    // Connect to WebSocket
    ws://api.anya.com/v1/ws
    // Authentication message
    {
        "type": "auth",
        "api_key": "YOUR_API_KEY"
    }
    // Connect to WebSocket
    ws://api.anya.com/v1/ws
    // Authentication message
    {
        "type": "auth",
        "api_key": "YOUR_API_KEY"
    }

### Subscriptions

#### Transaction Updates

    // Subscribe
    {
        "type": "subscribe",
        "channel": "transactions"
    }
    // Update message
    {
        "type": "transaction",
        "data": {
            "txid": "...",
            "status": "confirmed",
            "block_height": 700000
        }
    }
    // Subscribe
    {
        "type": "subscribe",
        "channel": "transactions"
    }
    // Update message
    {
        "type": "transaction",
        "data": {
            "txid": "...",
            "status": "confirmed",
            "block_height": 700000
        }
    }

#### Block Updates

GET /api/v1/transactions/{txid}

    // Subscribe
    {
        "type": "subscribe",
        "channel": "blocks"
    }
    // Update message
    {
        "type": "block",
        "data": {
            "height": 700000,
            "hash": "...",
            "timestamp": 1631234567
        }
    }
GET /api/v1/transactions/{txid}

    // Subscribe
    {
        "type": "subscribe",
        "channel": "blocks"
    }
    // Update message
    {
        "type": "block",
        "data": {
            "height": 700000,
            "hash": "...",
            "timestamp": 1631234567
        }
    }

#### Contract Updates

    // Subscribe
    {
        "type": "subscribe",
        "channel": "contracts"
    }
    // Update message
    {
        "type": "contract",
        "data": {
            "contract_id": "...",
            "status": "executed",
            "outcome": "true"
        }
    }
    // Subscribe
    {
        "type": "subscribe",
        "channel": "contracts"
    }
    // Update message
    {
        "type": "contract",
        "data": {
            "contract_id": "...",
            "status": "executed",
            "outcome": "true"
        }
    }

## Error Handling


### Error Format

    {
        "error": {
            "code": "invalid_request",
            "message": "Invalid transaction parameters",
            "details": {
                "field": "amount",
                "reason": "insufficient_funds"
            }
    {
        "error": {
            "code": "invalid_request",
            "message": "Invalid transaction parameters",
            "details": {
                "field": "amount",
                "reason": "insufficient_funds"
            }
        }
    }

### Common Error Codes

- `invalid_request`: Invalid request parameters
- `unauthorized`: Authentication failed
- `forbidden`: Permission denied
- `not_found`: Resource not found
- `rate_limited`: Too many requests
- `internal_error`: Server error


## Rate Limiting

### Rate Limit Headers

```text

X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1631234567
```

### Limits

- REST API: 1000 requests per minute
- WebSocket: 100 messages per second
- Bulk operations: 10 requests per minute

## Pagination

### Request


    GET /api/v1/transactions?limit=10&offset=0
    GET /api/v1/transactions?limit=10&offset=0

### Response


    {
        "data": [...],
        "pagination": {
            "total": 100,
            "limit": 10,
            "offset": 0,
            "has_more": true
        }
    }
    {
        "data": [...],
        "pagination": {
            "total": 100,
            "limit": 10,
            "offset": 0,
            "has_more": true
        }
    }

## Versioning

### API Versions

- v1: Current stable version
- v2: Beta version (if available)
- v0: Deprecated version


### Headers

```
Accept: application/json; version=1
```

## Examples

### Creating a Transaction

    use anya_sdk::{Client, TransactionBuilder};
    let client = Client::new(api_key);
    let tx = TransactionBuilder::new()
        .add_recipient("bc1q...", "0.1")
        .set_fee_rate(5)
        .enable_rbf()
        .build()?;
    let result = client.send_transaction(tx).await?;
    use anya_sdk::{Client, TransactionBuilder};
    let client = Client::new(api_key);
    let tx = TransactionBuilder::new()
        .add_recipient("bc1q...", "0.1")
        .set_fee_rate(5)
        .enable_rbf()
        .build()?;
    let result = client.send_transaction(tx).await?;

### Managing Contracts

    use anya_sdk::{Client, ContractBuilder};
    let client = Client::new(api_key);
    let contract = ContractBuilder::new()
        .set_type(ContractType::DLC)
        .set_oracle("oracle_id")
        .add_outcomes(vec!["true", "false"])
        .set_collateral("1.0")
        .build()?;
    let result = client.create_contract(contract).await?;
    use anya_sdk::{Client, ContractBuilder};
    let client = Client::new(api_key);
    let contract = ContractBuilder::new()
        .set_type(ContractType::DLC)
        .set_oracle("oracle_id")
        .add_outcomes(vec!["true", "false"])
        .set_collateral("1.0")
        .build()?;
    let result = client.create_contract(contract).await?;

### WebSocket Subscription

    use anya_sdk::{WebSocketClient, Subscription};
    let ws = WebSocketClient::new(api_key);
    ws.subscribe(vec![
        Subscription::Transactions,
        Subscription::Blocks,
        Subscription::Contracts,
    ])?;
    while let Some(msg) = ws.next().await {
        match msg {
            Message::Transaction(tx) => println!("New transaction: {}", tx.txid),
            Message::Block(block) => println!("New block: {}", block.height),
            Message::Contract(contract) => println!("Contract update: {}", contract.id),
            Message::Transaction(tx) => eprintln!("New transaction: {}", tx.txid),
            Message::Block(block) => eprintln!("New block: {}", block.height),
            Message::Contract(contract) => eprintln!("Contract update: {}", contract.id),

## Best Practices

### 1. Error Handling

- Always check error responses
- Implement exponential backoff
- Handle rate limiting
- Log errors appropriately

### 2. Performance

- Use WebSocket for real-time updates
- Implement caching
- Batch operations when possible
- Monitor API usage

### 3. Security

- Secure API keys
- Use HTTPS

- Implement timeouts
- Validate responses

## SDK Support

### Official SDKs

- Rust: `anya-sdk`
- Python: `anya-python`
- JavaScript: `anya-js`
- Go: `anya-go`

### Installation

    ## Rust
    cargo add anya-sdk
    ## Python
    pip install anya-python
    ## JavaScript
    npm install anya-js
    ## Go
    go get github.com/anya/anya-go
    ## Rust
    cargo add anya-sdk
    ## Python
    pip install anya-python
    ## JavaScript
    npm install anya-js
    ## Go
    go get github.com/anya/anya-go

## Support

For API support:

- API documentation
- SDK documentation
- Support channels
- Status page

*Last updated: 2025-06-02*

## See Also

### Related Document

### Related Document

- [Related Document](#related-document)

