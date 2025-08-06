# PSBT v2 (BIP-370) API Usage Examples

This document provides real-world usage examples for PSBT v2 (BIP-370) operations in the Anya Core API.

## Example: Create PSBT v2

```json
POST /api/psbt/v2/create
{
  "inputs": [...],
  "outputs": [...],
  "version": 2
}
```

## Example: Sign PSBT v2

```json
POST /api/psbt/v2/sign
{
  "psbt": "...base64...",
  "key": "..."
}
```

## Example: Finalize PSBT v2

```json
POST /api/psbt/v2/finalize
{
  "psbt": "...base64..."
}
```

## Migration: BIP-174 to BIP-370

- Use `/api/psbt/migrate` endpoint to convert legacy PSBTs to v2 format.
