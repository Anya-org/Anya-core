# API Reference

[AIR-3][AIS-3][BPC-3][RES-3]

**AI Labeling**: This documentation is AI-generated with technical review and validation.

**Date**: June 7, 2025

## Overview

Complete API reference for Anya Core, covering Bitcoin, Web5, ML systems, and extension development. All APIs follow REST conventions with comprehensive error handling and security.

## Table of Contents

- [Authentication](#authentication)
- [Bitcoin API](#bitcoin-api)
- [Web5 API](#web5-api)
- [ML API](#ml-api)
- [Extension API](#extension-api)
- [System API](#system-api)
- [Error Handling](#error-handling)
- [Rate Limiting](#rate-limiting)

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

## Bitcoin API

### Wallet Management

#### Create Wallet

```http
POST /api/v1/bitcoin/wallets
```

```json
{
  "name": "main_wallet",
  "network": "mainnet",
  "wallet_type": "segwit",
  "passphrase": "optional_passphrase"
}
```

**Response:**
```json
{
  "wallet_id": "wallet_12345",
  "name": "main_wallet",
  "network": "mainnet",
  "addresses": {
    "receiving": "bc1qxyz...",
    "change": "bc1qabc..."
  },
  "created_at": "2025-05-30T10:00:00Z"
}
```

#### Get Wallet Balance

```http
GET /api/v1/bitcoin/wallets/{wallet_id}/balance
```

**Response:**
```json
{
  "confirmed": 150000000,
  "unconfirmed": 5000000,
  "total": 155000000,
  "currency": "satoshis"
}
```

#### List Transactions

```http
GET /api/v1/bitcoin/wallets/{wallet_id}/transactions?limit=50&offset=0
```

**Response:**
```json
{
  "transactions": [
    {
      "txid": "abc123...",
      "amount": 50000000,
      "confirmations": 6,
      "timestamp": "2025-05-30T09:00:00Z",
      "type": "received"
    }
  ],
  "total": 1250,
  "has_more": true
}
```

### Transaction Management

#### Create Transaction

```http
POST /api/v1/bitcoin/transactions
```

```json
{
  "wallet_id": "wallet_12345",
  "outputs": [
    {
      "address": "bc1qrecipient...",
      "amount": 50000000
    }
  ],
  "fee_rate": 10,
  "rbf": true
}
```

**Response:**
```json
{
  "txid": "def456...",
  "hex": "0100000001...",
  "fee": 2250,
  "size": 225,
  "vsize": 141,
  "status": "signed"
}
```

#### Broadcast Transaction

```http
POST /api/v1/bitcoin/transactions/{txid}/broadcast
```

**Response:**
```json
{
  "txid": "def456...",
  "status": "broadcasted",
  "broadcasted_at": "2025-05-30T10:30:00Z"
}
```

### Lightning Network

#### Open Channel

```http
POST /api/v1/lightning/channels
```

```json
{
  "peer_pubkey": "02abc123...",
  "local_amount": 1000000,
  "push_amount": 500000,
  "private": false
}
```

#### Create Invoice

```http
POST /api/v1/lightning/invoices
```

```json
{
  "amount": 100000,
  "memo": "Payment for services",
  "expiry": 3600
}
```

**Response:**
```json
{
  "payment_request": "lnbc1u1p...",
  "payment_hash": "abc123...",
  "expires_at": "2025-05-30T11:30:00Z"
}
```

## Web5 API

### DID Management

#### Create DID

```http
POST /api/v1/web5/dids
```

```json
{
  "method": "did:key",
  "key_type": "ed25519"
}
```

**Response:**
```json
{
  "did": "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
  "document": {
    "id": "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
    "verificationMethod": [...]
  },
  "private_key": "...",
  "created_at": "2025-05-30T10:00:00Z"
}
```

#### Resolve DID

```http
GET /api/v1/web5/dids/{did}
```

**Response:**
```json
{
  "document": {
    "id": "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
    "verificationMethod": [...],
    "service": [...]
  },
  "metadata": {
    "resolved_at": "2025-05-30T10:00:00Z",
    "resolver": "did:key"
  }
}
```

### Verifiable Credentials

#### Issue Credential

```http
POST /api/v1/web5/credentials
```

```json
{
  "issuer": "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
  "subject": "did:key:z6MkpTHR8VNsBxYAAWHut2Geadd9jSwuBV8xRoAnwWsdvktH",
  "type": ["VerifiableCredential", "BitcoinWalletCredential"],
  "credentialSubject": {
    "walletAddress": "bc1qxyz...",
    "verificationLevel": "kyc_verified"
  },
  "expirationDate": "2026-05-30T10:00:00Z"
}
```

**Response:**
```json
{
  "credential": {
    "@context": [...],
    "type": ["VerifiableCredential", "BitcoinWalletCredential"],
    "issuer": "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
    "issuanceDate": "2025-05-30T10:00:00Z",
    "credentialSubject": {...},
    "proof": {...}
  },
  "credential_id": "cred_12345"
}
```

#### Verify Credential

```http
POST /api/v1/web5/credentials/verify
```

```json
{
  "credential": {...}
}
```

**Response:**
```json
{
  "verified": true,
  "verification_method": "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK#z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
  "verified_at": "2025-05-30T10:00:00Z"
}
```

### Decentralized Web Node (DWN)

#### Store Data

```http
POST /api/v1/web5/dwn/records
```

```json
{
  "owner": "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
  "schema": "https://schema.org/Document",
  "data": {...},
  "published": true
}
```

**Response:**
```json
{
  "record_id": "rec_12345",
  "data_cid": "bafybeiabc123...",
  "created_at": "2025-05-30T10:00:00Z"
}
```

#### Query Records

```http
GET /api/v1/web5/dwn/records?owner={did}&schema={schema}&limit=50
```

**Response:**
```json
{
  "records": [
    {
      "record_id": "rec_12345",
      "owner": "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
      "schema": "https://schema.org/Document",
      "created_at": "2025-05-30T10:00:00Z"
    }
  ],
  "total": 125,
  "has_more": true
}
```

## ML API

### Model Management

#### List Models

```http
GET /api/v1/ml/models
```

**Response:**
```json
{
  "models": [
    {
      "model_id": "price_prediction_v2",
      "name": "Bitcoin Price Prediction",
      "version": "2.1.0",
      "status": "active",
      "accuracy": 0.87,
      "created_at": "2025-05-30T10:00:00Z"
    }
  ]
}
```

#### Load Model

```http
POST /api/v1/ml/models/{model_id}/load
```

**Response:**
```json
{
  "model_id": "price_prediction_v2",
  "status": "loaded",
  "memory_usage": "512MB",
  "loaded_at": "2025-05-30T10:00:00Z"
}
```

### Inference

#### Run Inference

```http
POST /api/v1/ml/inference
```

```json
{
  "model_id": "price_prediction_v2",
  "input": {
    "current_price": 50000.0,
    "volume_24h": 25000000000,
    "market_cap": 950000000000,
    "fear_greed_index": 65
  }
}
```

**Response:**
```json
{
  "prediction": {
    "price_24h": 52500.0,
    "confidence": 0.82,
    "trend": "bullish"
  },
  "inference_time": "125ms",
  "model_version": "2.1.0"
}
```

#### Batch Inference

```http
POST /api/v1/ml/inference/batch
```

```json
{
  "model_id": "price_prediction_v2",
  "inputs": [
    {"current_price": 50000.0, "volume_24h": 25000000000},
    {"current_price": 51000.0, "volume_24h": 26000000000}
  ]
}
```

### Agent System

#### List Agents

```http
GET /api/v1/ml/agents
```

**Response:**
```json
{
  "agents": [
    {
      "agent_id": "trading_agent_v1",
      "name": "Bitcoin Trading Agent",
      "status": "active",
      "success_rate": 0.75,
      "total_trades": 1250
    }
  ]
}
```

#### Execute Agent Task

```http
POST /api/v1/ml/agents/{agent_id}/tasks
```

```json
{
  "task_type": "market_analysis",
  "parameters": {
    "symbol": "BTC",
    "timeframe": "1d",
    "indicators": ["rsi", "macd", "bollinger"]
  }
}
```

**Response:**
```json
{
  "task_id": "task_12345",
  "status": "completed",
  "result": {
    "recommendation": "buy",
    "confidence": 0.78,
    "analysis": {...}
  },
  "execution_time": "2.3s"
}
```

## Extension API

### Extension Management

#### List Extensions

```http
GET /api/v1/extensions
```

**Response:**
```json
{
  "extensions": [
    {
      "extension_id": "bitcoin_custody_v1",
      "name": "Bitcoin Custody Suite",
      "version": "1.2.0",
      "status": "active",
      "author": "Anya Core Team"
    }
  ]
}
```

#### Install Extension

```http
POST /api/v1/extensions
```

```json
{
  "extension_id": "lightning_tools_v2",
  "version": "2.0.1",
  "configuration": {
    "auto_start": true,
    "permissions": ["wallet_read", "network_access"]
  }
}
```

#### Configure Extension

```http
PUT /api/v1/extensions/{extension_id}/config
```

```json
{
  "settings": {
    "api_endpoint": "https://api.example.com",
    "timeout": 30000,
    "retry_attempts": 3
  }
}
```

### Extension Communication

#### Send Message to Extension

```http
POST /api/v1/extensions/{extension_id}/messages
```

```json
{
  "type": "command",
  "payload": {
    "action": "process_transaction",
    "data": {...}
  }
}
```

## System API

### Health Check

```http
GET /api/v1/health
```

**Response:**
```json
{
  "status": "healthy",
  "components": {
    "bitcoin": "healthy",
    "web5": "healthy",
    "ml": "healthy",
    "database": "healthy"
  },
  "timestamp": "2025-05-30T10:00:00Z"
}
```

### System Metrics

```http
GET /api/v1/metrics
```

**Response:**
```json
{
  "performance": {
    "cpu_usage": 45.2,
    "memory_usage": 67.8,
    "disk_usage": 23.1
  },
  "bitcoin": {
    "block_height": 845678,
    "mempool_size": 1250,
    "fee_estimates": {
      "fast": 15,
      "medium": 8,
      "slow": 3
    }
  },
  "ml": {
    "active_models": 3,
    "inference_queue": 25,
    "average_inference_time": "150ms"
  }
}
```

### Configuration

#### Get Configuration

```http
GET /api/v1/config
```

#### Update Configuration

```http
PUT /api/v1/config
```

```json
{
  "bitcoin": {
    "network": "mainnet",
    "fee_strategy": "medium"
  },
  "ml": {
    "max_inference_queue": 1000,
    "model_cache_size": "2GB"
  }
}
```

## Error Handling

### Error Response Format

```json
{
  "error": {
    "code": "INVALID_TRANSACTION",
    "message": "Transaction validation failed",
    "details": {
      "reason": "Insufficient funds",
      "required": 50000000,
      "available": 45000000
    },
    "timestamp": "2025-05-30T10:00:00Z",
    "request_id": "req_12345"
  }
}
```

### Error Codes

| Code | Description | HTTP Status |
|------|-------------|-------------|
| `INVALID_REQUEST` | Request validation failed | 400 |
| `UNAUTHORIZED` | Authentication required | 401 |
| `FORBIDDEN` | Insufficient permissions | 403 |
| `NOT_FOUND` | Resource not found | 404 |
| `RATE_LIMITED` | Rate limit exceeded | 429 |
| `INTERNAL_ERROR` | Internal server error | 500 |

### Bitcoin-Specific Errors

| Code | Description |
|------|-------------|
| `INVALID_TRANSACTION` | Transaction validation failed |
| `INSUFFICIENT_FUNDS` | Wallet balance too low |
| `INVALID_ADDRESS` | Bitcoin address format invalid |
| `NETWORK_ERROR` | Bitcoin network communication failed |

### Web5-Specific Errors

| Code | Description |
|------|-------------|
| `INVALID_DID` | DID format or resolution failed |
| `CREDENTIAL_VERIFICATION_FAILED` | Credential signature invalid |
| `DWN_STORAGE_ERROR` | Decentralized Web Node storage failed |

### ML-Specific Errors

| Code | Description |
|------|-------------|
| `MODEL_NOT_FOUND` | Requested model not available |
| `INFERENCE_FAILED` | Model inference execution failed |
| `INVALID_INPUT` | Input data format invalid |
| `MODEL_OVERLOADED` | Model processing queue full |

## Rate Limiting

### Limits by Endpoint

| Endpoint Category | Requests per Minute | Burst Limit |
|------------------|-------------------|-------------|
| Authentication | 60 | 10 |
| Bitcoin Read | 300 | 50 |
| Bitcoin Write | 60 | 10 |
| Web5 Operations | 200 | 30 |
| ML Inference | 100 | 20 |
| System APIs | 600 | 100 |

### Rate Limit Headers

```http
X-RateLimit-Limit: 300
X-RateLimit-Remaining: 275
X-RateLimit-Reset: 1714426800
Retry-After: 60
```

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

- [API Changelog](../../dependencies/CHANGELOG.md)
- [SDK Documentation](../README.md)
- [Authentication Guide](../integration/authentication.md)
- [Rate Limiting Guide](../security/rate-limiting.md)

## Support

For API support and questions:

- **Documentation Issues**: GitHub Issues
- **API Support**: api-support@anya-core.dev
- **Discord**: Join our developer community

---

This API reference is maintained by the Anya Core team and updated with each release.

