# Governance API Reference

This document provides a comprehensive reference for the Anya Core governance API endpoints.

## Overview

The Governance API provides endpoints for interacting with the DAO governance system, including voting, proposals, and token management.

## Authentication

All governance API endpoints require authentication using a valid API key or signed request.

```http
Authorization: Bearer <api_key>
```

## Base URL

```
https://api.anya-core.org/v1/governance
```

## Endpoints

### Proposals

#### GET /proposals

List all governance proposals.

**Parameters:**

- `status` (optional): Filter by proposal status (`active`, `passed`, `rejected`)
- `limit` (optional): Number of results to return (default: 20)
- `offset` (optional): Pagination offset

**Response:**

```json
{
  "proposals": [
    {
      "id": "prop-001",
      "title": "Upgrade Protocol Parameters",
      "description": "Proposal to update protocol parameters...",
      "status": "active",
      "votes_for": 1000,
      "votes_against": 200,
      "created_at": "2025-06-17T10:00:00Z",
      "expires_at": "2025-06-24T10:00:00Z"
    }
  ],
  "total": 1,
  "limit": 20,
  "offset": 0
}
```

#### POST /proposals

Create a new governance proposal.

**Request Body:**

```json
{
  "title": "Proposal Title",
  "description": "Detailed proposal description",
  "type": "parameter_change",
  "parameters": {
    "target_parameter": "new_value"
  }
}
```

#### GET /proposals/{id}

Get details of a specific proposal.

#### POST /proposals/{id}/vote

Cast a vote on a proposal.

**Request Body:**

```json
{
  "vote": "for" | "against",
  "voting_power": 100
}
```

### Voting

#### GET /votes

List votes cast by the authenticated user.

#### GET /votes/{proposal_id}

Get all votes for a specific proposal.

### Token Management

#### GET /tokens/balance

Get the governance token balance for the authenticated user.

**Response:**

```json
{
  "balance": "1000.0",
  "locked": "100.0",
  "available": "900.0"
}
```

#### POST /tokens/delegate

Delegate voting power to another address.

**Request Body:**

```json
{
  "delegate_address": "anya1abc123...",
  "amount": "500.0"
}
```

### DAO Operations

#### GET /dao/status

Get the current status of the DAO.

**Response:**

```json
{
  "active_proposals": 5,
  "total_voting_power": 50000,
  "treasury_balance": "1000000.0",
  "governance_token_supply": "100000000.0"
}
```

#### GET /dao/treasury

Get treasury information.

#### POST /dao/execute/{proposal_id}

Execute a passed proposal (admin only).

## WebSocket API

Real-time updates are available via WebSocket connection.

### Connection

```javascript
const ws = new WebSocket('wss://api.anya-core.org/v1/governance/ws');
```

### Events

- `proposal_created`: New proposal created
- `proposal_updated`: Proposal status changed
- `vote_cast`: New vote cast
- `proposal_executed`: Proposal executed

### Example

```javascript
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  if (data.type === 'vote_cast') {
    console.log(`Vote cast on proposal ${data.proposal_id}`);
  }
};
```

## Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Invalid or missing authentication |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource not found |
| 429 | Too Many Requests - Rate limit exceeded |
| 500 | Internal Server Error |

## Rate Limits

- **Standard endpoints**: 100 requests per minute
- **Voting endpoints**: 10 requests per minute
- **WebSocket**: 1 connection per API key

## SDKs

### JavaScript/TypeScript

```bash
npm install @anya-core/governance-sdk
```

```typescript
import { GovernanceClient } from '@anya-core/governance-sdk';

const client = new GovernanceClient({
  apiKey: 'your-api-key',
  baseUrl: 'https://api.anya-core.org/v1/governance'
});

const proposals = await client.getProposals();
```

### Rust

```toml
[dependencies]
anya-governance = "0.1.0"
```

```rust
use anya_governance::GovernanceClient;

let client = GovernanceClient::new("your-api-key");
let proposals = client.get_proposals().await?;
```

## Examples

### Create and Vote on Proposal

```javascript
// Create proposal
const proposal = await client.createProposal({
  title: "Increase Block Size Limit",
  description: "Proposal to increase the block size limit to improve throughput",
  type: "parameter_change",
  parameters: {
    max_block_size: 4000000
  }
});

// Vote on proposal
await client.vote(proposal.id, {
  vote: "for",
  voting_power: 1000
});
```

## Security Considerations

- Always use HTTPS for API requests
- Store API keys securely
- Implement proper rate limiting
- Validate all input parameters
- Use multi-signature for critical operations

## See Also

- [DAO System Guide](../DAO_SYSTEM_GUIDE.md)
- [Governance Framework](../GOVERNANCE_FRAMEWORK.md)
- [API Authentication](README.md#authentication)

---

*This documentation is part of the Anya Core project. For more information, see the [main documentation index](../index.md).*
