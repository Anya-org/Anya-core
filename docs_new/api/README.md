# Anya Core API Reference

**Version:** 1.3.0
**Base URL:** `http://localhost:8080/api/v1`
**Protocol:** REST with WebSocket support

## Overview

The Anya Core API provides comprehensive access to all platform functionality including Bitcoin operations, Web5 protocols, Layer2 solutions, and AI/ML capabilities. The API follows RESTful principles with JSON request/response bodies.

## Authentication

Most API endpoints require authentication using JWT tokens or API keys.

### Getting an Access Token

```bash
POST /api/v1/login
Content-Type: application/json

{
  "username": "your_username",
  "password": "your_password"
}
```

**Response:**

```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

### Using Authentication

Include the token in the Authorization header:

```bash
Authorization: Bearer <access_token>
```

## System Endpoints

### Health Check

Check system status and component health.

```http
GET /api/v1/health
```

**Response:**

```json
{
  "status": "operational",
  "version": "1.3.0",
  "uptime_seconds": 123456,
  "components": {
    "bitcoin": "connected",
    "web5": "operational",
    "ml": "ready",
    "layer2": "active"
  }
}
```

### System Information

Get detailed system information and configuration.

```http
GET /api/v1/info
```

**Response:**

```json
{
  "version": "1.3.0",
  "build_date": "2025-08-06T00:00:00Z",
  "git_commit": "a1b2c3d4",
  "features": ["bitcoin", "web5", "ml", "layer2"],
  "network": "mainnet",
  "node_id": "anya_node_12345"
}
```

## Bitcoin & Wallet API

*Available when `bitcoin` feature is enabled*

### Get Wallet Balance

```http
GET /api/v1/wallet/balance
Authorization: Bearer <token>
```

**Response:**

```json
{
  "confirmed_balance": "0.00123456",
  "unconfirmed_balance": "0.00000000",
  "total_balance": "0.00123456",
  "currency": "BTC"
}
```

### Generate New Address

```http
GET /api/v1/wallet/address
Authorization: Bearer <token>
```

**Response:**

```json
{
  "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
  "address_type": "bech32",
  "derivation_path": "m/84'/0'/0'/0/5"
}
```

### Send Transaction

```http
POST /api/v1/wallet/send
Authorization: Bearer <token>
Content-Type: application/json

{
  "recipient": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
  "amount": "0.001",
  "fee_rate": 10
}
```

**Response:**

```json
{
  "txid": "a1b2c3d4e5f6...",
  "amount_sent": "0.001",
  "fee_paid": "0.00001",
  "confirmations": 0,
  "broadcast_time": "2025-08-06T12:00:00Z"
}
```

### Transaction History

```http
GET /api/v1/wallet/history?limit=10&offset=0
Authorization: Bearer <token>
```

**Response:**

```json
{
  "transactions": [
    {
      "txid": "a1b2c3d4e5f6...",
      "amount": "-0.001",
      "fee": "0.00001",
      "confirmations": 6,
      "timestamp": "2025-08-06T11:30:00Z",
      "type": "send"
    }
  ],
  "total_count": 25,
  "has_more": true
}
```

## Web5 Identity API

### Create Identity

Create a new decentralized identity (DID).

```http
POST /api/v1/identity
Authorization: Bearer <token>
Content-Type: application/json

{
  "method": "web5",
  "options": {
    "key_type": "secp256k1"
  }
}
```

**Response:**

```json
{
  "did": "did:web5:eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NksifQ",
  "keys": {
    "signing": {
      "id": "#key-1",
      "type": "JsonWebKey",
      "public_key": "..."
    }
  },
  "created_at": "2025-08-06T12:00:00Z"
}
```

### Get Identity

```http
GET /api/v1/identity/{did}
Authorization: Bearer <token>
```

### Resolve Identity

```http
POST /api/v1/identity/resolve
Authorization: Bearer <token>
Content-Type: application/json

{
  "did": "did:web5:eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NksifQ"
}
```

### Issue Credential

```http
POST /api/v1/credentials/issue
Authorization: Bearer <token>
Content-Type: application/json

{
  "holder_did": "did:web5:...",
  "credential_type": "IdentityCredential",
  "claims": {
    "name": "John Doe",
    "email": "john@example.com"
  }
}
```

### Verify Credential

```http
POST /api/v1/credentials/verify
Authorization: Bearer <token>
Content-Type: application/json

{
  "credential": "eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NksifQ..."
}
```

## Decentralized Web Node (DWN) API

### List Protocols

```http
GET /api/v1/dwn/protocols
Authorization: Bearer <token>
```

### Create Protocol

```http
POST /api/v1/dwn/protocols
Authorization: Bearer <token>
Content-Type: application/json

{
  "protocol": "https://schemas.xyz/social-media",
  "types": {
    "post": {
      "schema": "https://schemas.xyz/post",
      "data_formats": ["application/json"]
    }
  }
}
```

### Query Records

```http
GET /api/v1/dwn/records?protocol=https://schemas.xyz/social-media
Authorization: Bearer <token>
```

### Create Record

```http
POST /api/v1/dwn/records
Authorization: Bearer <token>
Content-Type: application/json

{
  "protocol": "https://schemas.xyz/social-media",
  "schema": "https://schemas.xyz/post",
  "data": {
    "content": "Hello, decentralized world!",
    "timestamp": "2025-08-06T12:00:00Z"
  }
}
```

### Get Record

```http
GET /api/v1/dwn/records/{record_id}
Authorization: Bearer <token>
```

### Update Record

```http
PUT /api/v1/dwn/records/{record_id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "data": {
    "content": "Updated content",
    "updated_at": "2025-08-06T13:00:00Z"
  }
}
```

### Delete Record

```http
DELETE /api/v1/dwn/records/{record_id}
Authorization: Bearer <token>
```

## RGB Asset API

*Available when `bitcoin` feature is enabled*

### List Assets

```http
GET /api/v1/rgb/assets
Authorization: Bearer <token>
```

**Response:**

```json
{
  "assets": [
    {
      "asset_id": "rgb1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
      "name": "MyToken",
      "ticker": "MTK",
      "total_supply": "1000000",
      "decimals": 8,
      "created_at": "2025-08-06T10:00:00Z"
    }
  ]
}
```

### Create Asset

```http
POST /api/v1/rgb/assets
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "MyToken",
  "ticker": "MTK",
  "total_supply": "1000000",
  "decimals": 8,
  "metadata": {
    "description": "My test token",
    "website": "https://mytoken.com"
  }
}
```

### Get Asset

```http
GET /api/v1/rgb/assets/{asset_id}
Authorization: Bearer <token>
```

### Transfer Asset

```http
POST /api/v1/rgb/assets/{asset_id}/transfer
Authorization: Bearer <token>
Content-Type: application/json

{
  "recipient": "rgb1recipient...",
  "amount": "100.50"
}
```

### Asset History

```http
GET /api/v1/rgb/assets/{asset_id}/history
Authorization: Bearer <token>
```

## DLC (Discreet Log Contracts) API

### Create Contract

```http
POST /api/v1/dlc
Authorization: Bearer <token>
Content-Type: application/json

{
  "oracle_pubkey": "02abc123...",
  "oracle_event": "weather_2025_08_06",
  "outcomes": [
    {"outcome": "sunny", "payout": 1000000},
    {"outcome": "rainy", "payout": 0}
  ],
  "collateral": 1000000,
  "maturity_time": "2025-08-06T18:00:00Z"
}
```

### Get Contract

```http
GET /api/v1/dlc/{contract_id}
Authorization: Bearer <token>
```

### Accept Contract

```http
POST /api/v1/dlc/{contract_id}/accept
Authorization: Bearer <token>
Content-Type: application/json

{
  "funding_tx": "01000000...",
  "funding_signatures": ["3045022100..."]
}
```

### Finalize Contract

```http
POST /api/v1/dlc/{contract_id}/finalize
Authorization: Bearer <token>
```

### Execute Contract

```http
POST /api/v1/dlc/{contract_id}/execute
Authorization: Bearer <token>
Content-Type: application/json

{
  "oracle_signature": "304502210...",
  "outcome": "sunny"
}
```

## Error Handling

The API uses standard HTTP status codes and returns errors in a consistent format:

```json
{
  "error": {
    "code": "INVALID_REQUEST",
    "message": "The request is invalid or missing required parameters",
    "details": {
      "field": "recipient",
      "issue": "Invalid Bitcoin address format"
    }
  },
  "timestamp": "2025-08-06T12:00:00Z",
  "request_id": "req_123456789"
}
```

### Common Error Codes

- **`UNAUTHORIZED`** (401): Missing or invalid authentication
- **`FORBIDDEN`** (403): Insufficient permissions
- **`NOT_FOUND`** (404): Resource not found
- **`INVALID_REQUEST`** (400): Invalid request format or parameters
- **`RATE_LIMITED`** (429): Rate limit exceeded
- **`INTERNAL_ERROR`** (500): Server error

## Rate Limiting

API endpoints are rate-limited to prevent abuse:

- **Authenticated requests**: 1000 requests per hour
- **Unauthenticated requests**: 100 requests per hour

Rate limit headers are included in responses:

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1691337600
```

## WebSocket API

Real-time events are available via WebSocket connection:

```javascript
const ws = new WebSocket('ws://localhost:8080/api/v1/ws');

// Subscribe to events
ws.send(JSON.stringify({
  type: 'subscribe',
  channels: ['transactions', 'blocks', 'dwn_updates']
}));

// Listen for events
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Event:', data);
};
```

### Available Event Channels

- **`transactions`**: New Bitcoin transactions
- **`blocks`**: New Bitcoin blocks
- **`dwn_updates`**: DWN record changes
- **`system_status`**: System status changes

## SDK Examples

### Rust

```rust
use anya_core::{AnyaCore, AnyaConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AnyaConfig::default();
    let anya = AnyaCore::new(config)?;

    // Use the API
    let status = anya.get_status()?;
    println!("System status: {:?}", status);

    Ok(())
}
```

### JavaScript/TypeScript

```typescript
import { AnyaClient } from '@anya-core/sdk';

const client = new AnyaClient({
  baseUrl: 'http://localhost:8080/api/v1',
  apiKey: 'your_api_key'
});

// Get system health
const health = await client.system.health();
console.log('Health:', health);

// Create identity
const identity = await client.web5.createIdentity();
console.log('New DID:', identity.did);
```

### Python

```python
from anya_core import AnyaClient

client = AnyaClient(
    base_url="http://localhost:8080/api/v1",
    api_key="your_api_key"
)

# Get wallet balance
balance = client.bitcoin.get_balance()
print(f"Balance: {balance['total_balance']} BTC")

# Create DWN record
record = client.dwn.create_record({
    "protocol": "https://schemas.xyz/social-media",
    "data": {"content": "Hello from Python!"}
})
```

## Further Documentation

- **[Bitcoin Integration Guide](../guides/bitcoin-integration.md)** - Detailed Bitcoin usage
- **[Web5 Integration Guide](../guides/web5-integration.md)** - DID and DWN implementation
- **[Layer2 Protocols](../guides/layer2-protocols.md)** - RGB, DLC, Lightning Network
- **[Configuration Reference](../reference/configuration.md)** - API configuration options

---

**Need help?** Check our [troubleshooting guide](../operations/troubleshooting.md) or join the [Discord community](https://discord.gg/anya-core).
