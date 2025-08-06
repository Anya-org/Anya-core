---
title: "Lightning Module"
description: "Lightning Network implementation for Anya Core"
status: "active"
last_updated: "2025-08-06"
---

# Lightning Module

This module provides Lightning Network functionality including channel management, payment routing, and invoice handling.

## Table of Contents

- [Overview](#overview)
- [Components](#components)
- [Channel Management](#channel-management)
- [Payment Routing](#payment-routing)
- [Invoicing](#invoicing)
- [Examples](#examples)

## Overview

The Lightning module implements a complete Lightning Network node compatible with the BOLT specifications, enabling fast, low-cost Bitcoin transactions through payment channels. The implementation focuses on reliability, security, and interoperability with other Lightning implementations.

## Components

The Lightning module consists of several key components:

- **Channel Manager**: Manages Lightning payment channels
- **Router**: Handles payment routing across the network
- **Invoice System**: Creates and processes Lightning invoices
- **Peer Manager**: Manages connections to other Lightning nodes

## Channel Management

The Channel Management system provides functionality for opening, maintaining, and closing Lightning channels:

```rust
// Open a new channel
let channel_point = lightning.open_channel(
    node_pubkey,
    Amount::from_sat(1_000_000), // 1M sats capacity
    Amount::from_sat(500_000)    // 500K sats local funding
).await?;

// Get information about a channel
let channel_info = lightning.get_channel_info(channel_point)?;
println!("Channel capacity: {} sats", channel_info.capacity);

// Close a channel cooperatively
lightning.close_channel(channel_point, false).await?;
```

## Payment Routing

The Router component handles finding efficient payment paths through the Lightning Network:

```rust
// Find a path to the destination
let route = lightning.find_route(
    destination_pubkey,
    Amount::from_sat(50_000),
    route_hints
)?;

// Send a payment along the route
let payment_result = lightning.send_payment(
    route,
    payment_hash,
    payment_secret
).await?;

// Check payment status
if payment_result.status.is_succeeded() {
    println!("Payment succeeded! Preimage: {}", payment_result.preimage);
}
```

## Invoicing

The Invoice system handles creating and paying Lightning invoices:

```rust
// Create an invoice
let invoice = lightning.create_invoice(
    Amount::from_sat(50_000),
    "Coffee payment",
    3600 // Expiry in seconds
)?;

println!("Payment request: {}", invoice.payment_request);

// Decode an invoice
let decoded = lightning.decode_invoice(&payment_request)?;
println!("Amount: {} sats", decoded.amount_msat / 1000);

// Pay an invoice
let payment_result = lightning.pay_invoice(&payment_request).await?;
```

## Examples

Complete example of using the Lightning module:

```rust
// Initialize Lightning with Bitcoin backend
let config = LightningConfig {
    network: Network::Bitcoin,
    node_private_key: private_key,
    listen_addr: "0.0.0.0:9735".to_string(),
};

let lightning = Lightning::new(config, bitcoin_client)?;

// Connect to peers
lightning.connect_peer("02abc...@node.example.com:9735").await?;

// Open a channel
let channel_point = lightning.open_channel(
    "02abc...",
    Amount::from_sat(2_000_000),
    Amount::from_sat(1_000_000)
).await?;

// Create an invoice
let invoice = lightning.create_invoice(
    Amount::from_sat(50_000),
    "Test payment",
    3600
)?;

// Print payment request for recipient
println!("Please pay: {}", invoice.payment_request);

// Wait for invoice to be paid
let paid_notification = lightning.wait_invoice_paid(invoice.payment_hash).await?;
println!("Invoice paid! Received {} sats", paid_notification.amount_received_sat);
```

