# API Standardization Guidelines

This document outlines the API standardization implemented across the Anya Core project, following Bitcoin Core principles of security, decentralization, and immutability.

## Endpoint Naming Conventions

All API endpoints follow these conventions:

1. **Path Structure**: /api/v{version}/{resource}/{identifier?}/{sub-resource?}
   - Example: /api/v1/transactions/123/status

2. **HTTP Methods**:
   - GET - Retrieve resources (non-modifying, secure)
   - POST - Create resources (modifying with validation)
   - PUT - Update resources (complete replacement with validation)
   - DELETE - Remove resources (with appropriate safeguards)
   - PATCH - Partial updates (with field-level validation)

3. **Naming Style**:
   - All paths use kebab-case
   - Example: /api/v1/transaction-history

## Standard API Patterns

| Operation | HTTP Method | URL Pattern | Example |
|-----------|------------|-------------|---------|
| List collection | GET | /api/v1/{resource} | /api/v1/transactions |
| Get single item | GET | /api/v1/{resource}/{id} | /api/v1/transactions/123 |
| Create item | POST | /api/v1/{resource} | /api/v1/transactions |
| Update item | PUT | /api/v1/{resource}/{id} | /api/v1/transactions/123 |
| Partial update | PATCH | /api/v1/{resource}/{id} | /api/v1/transactions/123 |
| Delete item | DELETE | /api/v1/{resource}/{id} | /api/v1/transactions/123 |

## Bitcoin Core Integration API Categories

| Category | Base Path | Description |
|----------|-----------|-------------|
| Bitcoin | /api/v1/bitcoin | Bitcoin Core functionality and protocol operations |
| Taproot | /api/v1/taproot | Taproot-related operations (BIP341) |
| DLC | /api/v1/dlc | Discrete Log Contracts functionality |
| RGB | /api/v1/rgb | RGB protocol integration for asset issuance |
| Stacks | /api/v1/stacks | Stacks smart contract capabilities |
| RSK | /api/v1/rsk | RSK sidechain integration |
| Web5 | /api/v1/web5 | Web5 capabilities with DIDs |
| BIP353 | /api/v1/bip353 | BIP353 functionality |
| Banking | /api/v1/banking | Open banking capabilities |
| Enterprise | /api/v1/enterprise | Enterprise features |

## Security Considerations

All APIs follow these security principles:

1. **Immutability** - Operations that modify data create immutable audit records
2. **Non-repudiation** - All modification operations require cryptographic signatures
3. **Input validation** - All inputs are strictly validated before processing
4. **Authorization** - Clear separation between public and authenticated endpoints
5. **Idempotency** - Operations can be safely retried with identical results

## Implementation Details

This standardization was automatically applied by the API standardization script to ensure consistent implementation of Bitcoin Core principles across all APIs.

Last updated: 2025-05-01
