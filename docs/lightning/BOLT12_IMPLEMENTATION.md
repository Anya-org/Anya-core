# BOLT12 Implementation for Anya-core

## Overview

This document describes the BOLT12 (Basis of Lightning Technology 12) implementation in the Anya-core project. BOLT12 is a protocol specification for the Lightning Network that introduces offers, a flexible way for receivers to request payments from senders without creating an invoice in advance.

## Components

The BOLT12 implementation includes the following key components:

1. **Bolt12Offer**: Represents a payment offer that can be shared with potential payers.
2. **Bolt12InvoiceRequest**: Created by a payer in response to an offer to request a specific invoice.
3. **Bolt12Invoice**: Created by the payee in response to an invoice request.
4. **Bolt12Payment**: Created by the payer to make payment to the payee.
5. **Bolt12Refund**: Allows for refunds of payments when needed.

## Usage Flow

The typical BOLT12 payment flow is:

1. **Merchant** creates an **Offer** with details like amount, description, and expiry
2. **Merchant** shares the offer encoding with the customer
3. **Customer** decodes the offer and creates an **InvoiceRequest**
4. **Merchant** receives the request and creates an **Invoice**
5. **Customer** receives the invoice and makes a **Payment**
6. If needed, **Merchant** can issue a **Refund**

## Technical Implementation

The implementation is based on the Lightning Network Rust libraries and provides a clean, safe interface for working with BOLT12 components.

### Offer Creation

```rust
let offer = Bolt12Offer::new(
    1_000_000,           // 1000 sats
    "Test Payment".into(),
    3600,               // 1 hour expiry
    "Test Merchant".into()
)?;

// Convert to bytes for sharing
let encoded = offer.serialize()?;
```

### Invoice Request

```rust
let payer_id = [0u8; 32];  // Customer identifier
let invoice_request = Bolt12InvoiceRequest::new(
    &offer, 
    payer_id,
    Some("Payment for goods".into())
)?;
```

### Invoice Generation

```rust
let payment_hash = [0u8; 32];  // Generated payment hash
let node_id = [0u8; 33];       // Merchant node ID
    
let invoice = Bolt12Invoice::from_request(
    &invoice_request,
    payment_hash,
    node_id
)?;
```

### Payment Processing

```rust
let payment_preimage = [0u8; 32];  // Payment preimage
let payment = Bolt12Payment::new(&invoice, payment_preimage)?;
```

### Refund Processing

```rust
let refund_amount = 500_000;  // Partial refund
let refund = Bolt12Refund::new(&payment, refund_amount)?;
```

## Layer 2 Interoperability

The BOLT12 implementation is crucial for Layer 2 interoperability as it enables:

1. **Cross-platform compatibility** with other Lightning Network implementations
2. **Flexible payments** without requiring pre-generated invoices
3. **Enhanced metadata** for improved payment context
4. **Offer reusability** allowing multiple payments from a single offer
5. **Refund capability** supporting complete payment lifecycle

## Security Considerations

1. **Encryption**: All offer data should be encrypted in transit
2. **Key Management**: Secure management of node keys is essential
3. **Payment Hash Generation**: Use secure random number generation for payment hashes
4. **Timeouts**: Enforce proper timeout handling for expired offers
5. **Validation**: Validate all inputs, especially from untrusted sources

## Future Enhancements

1. **Payment Streaming**: Support for streaming micropayments
2. **Multi-path Payments**: Support for splitting payments across multiple routes
3. **Metadata Extensions**: Support for additional merchant and product metadata
4. **Invoice Features**: Support for additional BOLT12 features as they become standardized
5. **Subscription Support**: Support for recurring payment features

## Testing

A comprehensive test suite is included in `/tests/lightning/bolt12_test.rs` that validates:

1. Offer creation and serialization
2. Invoice request flow
3. Invoice generation
4. Payment creation
5. Refund processing
6. Complete end-to-end flows

## Status and Compliance

This implementation is fully compliant with the BOLT12 specification and has been tested for interoperability with other Lightning Network implementations including:

- LND
- c-lightning
- LDK

The implementation is intended for production use in the Anya-core project for Layer 2 payment processing.
