# API Reference

[AIR-3][AIS-3][BPC-3][RES-3]

**AI Labeling**: This documentation is AI-generated with technical review and validation.

**Date**: July 13, 2025

## Overview

Complete API reference for Anya Core, covering Bitcoin, Web5/DID, DLC, and system endpoints. All APIs follow REST conventions with comprehensive error handling and security. This document represents the canonical API reference for Anya Core.

## Table of Contents

    Authentication
    - System API
    - Bitcoin Wallet API
    - Web5/DID API
    - DLC API
    - Error Handling
    - Rate Limiting

## Authentication

### API Key Authentication

```http
Authorization: Bearer <api_key>
Content-Type: application/json
```


### JWT Token Authentication

```http
Authorization: Bearer <jwt_token>
X-API-Version: v1
```


### Multi-Factor Authentication

For sensitive operations, MFA is required:


```json
{
  "mfa_token": "123456",
  "mfa_method": "totp"
}
```


## System API

### Health Check

Returns the current health status of the API service.

```http
GET /api/v1/health
```


### System Information

Returns information about the system, including version, environment, and configuration.


```http
GET /api/v1/info
```


## Bitcoin Wallet API

### Create Wallet

Creates a new Bitcoin wallet.


```http
POST /api/v1/wallets
Content-Type: application/json

{
  "name": "My Bitcoin Wallet",
  "network": "mainnet",
  "wallet_type": "segwit"
}
```


### Get Wallet

Retrieves information about a specific wallet.


```http
GET /api/v1/wallets/:id
```


### Get Balance

Retrieves the balance for a specific wallet.


```http
GET /api/v1/wallets/:id/balance
```


### Generate Address

Generates a new address for a specific wallet.


```http
POST /api/v1/wallets/:id/address
Content-Type: application/json

{
  "address_type": "p2wpkh"
}
```


### Send Transaction

Creates and sends a new transaction from a specific wallet.


```http
POST /api/v1/wallets/:id/transactions
Content-Type: application/json

{
  "to_address": "bc1q...",
  "amount": 0.001,
  "fee_rate": 5
}
```


### List Transactions

Retrieves transactions for a specific wallet.


```http
GET /api/v1/wallets/:id/transactions
```


## Web5/DID API

### Create Identity

Creates a new decentralized identity (DID).


```http
POST /api/v1/identities
Content-Type: application/json

{
  "method": "key",
  "keyType": "Ed25519"
}
```


### Get Identity

Retrieves information about a specific identity.


```http
GET /api/v1/identities/:id
```


### Create Credential

Creates a new verifiable credential.


```http
POST /api/v1/credentials
Content-Type: application/json

{
  "issuer": "did:key:z6Mk...",
  "subject": "did:key:z6Mk...",
  "type": ["VerifiableCredential", "BTC_Identity"],
  "claims": {
    "name": "Example User",
    "verified": true
  }
}
```


### Get Credential

Retrieves information about a specific credential.


```http
GET /api/v1/credentials/:id
```


### Verify Credential

Verifies the validity of a credential.


```http
POST /api/v1/credentials/verify
Content-Type: application/json

{
  "credential": { /* credential JSON */ }
}
```


## DLC API

### Create Contract

Creates a new Discreet Log Contract (DLC).


```http
POST /api/v1/dlc
Content-Type: application/json

{
  "counterparty": "03a9bd...",
  "oracle": "02abc...",
  "outcomes": [
    {
      "event": "BTC > 50000 USD",
      "payout": { "initiator": 1.0, "counterparty": 0.0 }
    },
    {
      "event": "BTC <= 50000 USD",
      "payout": { "initiator": 0.0, "counterparty": 1.0 }
    }
  ],
  "collateral": 1.0,
  "maturity_time": 1689433200
}
```


### Get Contract

Retrieves information about a specific DLC.


```http
GET /api/v1/dlc/:id
```


### Accept Contract

Accepts a DLC as the counterparty.


```http
POST /api/v1/dlc/:id/accept
```


### Finalize Contract

Finalizes a DLC after acceptance.


```http
POST /api/v1/dlc/:id/finalize
```


### Execute Contract

Executes a DLC based on oracle attestation.


```http
POST /api/v1/dlc/:id/execute
Content-Type: application/json

{
  "attestation": "signature_from_oracle"
}
```


## Error Handling

### Error Format

All API errors are returned in a standardized format:


```json
{
  "success": false,
  "message": "Error message describing the issue"
}
```


For more detailed errors:


```json
{
  "success": false,
  "message": "Validation error",
  "errors": {
    "field_name": "Error description for this field"
  }
}
```


### Common HTTP Status Codes

| Status Code | Description |
|-------------|-------------|
| 200 | OK - Request succeeded |
| 201 | Created - Resource was successfully created |
| 400 | Bad Request - Invalid parameters or validation error |
| 401 | Unauthorized - Authentication required |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource not found |
| 409 | Conflict - Resource already exists or state conflict |
| 500 | Internal Server Error - Unexpected server error |

## Rate Limiting

### Headers

Rate limit information is returned in the response headers:


```http
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 99
X-RateLimit-Reset: 1626307200
```


### Limits

| Endpoint | Anonymous | Authenticated |
|----------|-----------|---------------|
| `/api/v1/*` | 60/hour | 1000/hour |
| `/api/v1/wallets/*` | 30/hour | 500/hour |
| `/api/v1/identities/*` | 30/hour | 500/hour |
| `/api/v1/dlc/*` | 30/hour | 500/hour |

## SDK Examples

### Rust SDK

```rust
use anya_sdk::{AnyaClient, BitcoinApi, Web5Api, MlApi};

#[tokio::main]
async fn main() -> Result<()> {
    let client = AnyaClient::new("your_api_key").await?;
    
    // Bitcoin operations
    let wallet = client.bitcoin().create_wallet("main_wallet").await?;
    let balance = client.bitcoin().get_balance(&wallet.id).await?;
    
    // Web5 operations
    let did = client.web5().create_did("did:key").await?;
    let credential = client.web5().issue_credential(&did, credential_data).await?;
    
    // ML operations
    let prediction = client.ml().run_inference("price_model", input_data).await?;
    
    Ok(())
}
```

### JavaScript SDK

```javascript
import { AnyaClient } from '@anya-core/sdk';

const client = new AnyaClient({ apiKey: 'your_api_key' });

// Bitcoin operations
const wallet = await client.bitcoin.createWallet('main_wallet');
const balance = await client.bitcoin.getBalance(wallet.id);

// Web5 operations
const did = await client.web5.createDid('did:key');
const credential = await client.web5.issueCredential(did, credentialData);

// ML operations
const prediction = await client.ml.runInference('price_model', inputData);
```

### Python SDK

```python
from anya_sdk import AnyaClient

async def main():
    client = AnyaClient(api_key='your_api_key')
    
    # Bitcoin operations
    wallet = await client.bitcoin.create_wallet('main_wallet')
    balance = await client.bitcoin.get_balance(wallet.id)
    
    # Web5 operations
    did = await client.web5.create_did('did:key')
    credential = await client.web5.issue_credential(did, credential_data)
    
    # ML operations
    prediction = await client.ml.run_inference('price_model', input_data)
```

## Resources

- [API Changelog](../CHANGELOG.md)
- [SDK Documentation](README.md)
- [Authentication Guide](../integration/authentication.md)
- [Rate Limiting Guide](../security/rate-limiting.md)

## Support

For API support and questions:

- **Documentation Issues**: GitHub Issues
- **API Support**: <api-support@anya-core.dev>
- **Discord**: Join our developer community

---

This API reference is maintained by the Anya Core team and updated with each release. For more information, visit [https://developer.anya-core.com](https://developer.anya-core.com).
