# Layer2 Architecture Diagrams

**Date: June 22, 2025**

This document provides updated architectural diagrams for the Layer2 modules, reflecting the dual sync/async API structure.

## Overall Layer2 Architecture

```
┌───────────────────────────────────────────────────────────┐
│                    Client Application                     │
└───────────────┬───────────────────────────┬───────────────┘
                │                           │
                ▼                           ▼
┌───────────────────────────┐   ┌───────────────────────────┐
│      Synchronous API      │   │     Asynchronous API      │
└───────────────┬───────────┘   └───────────────┬───────────┘
                │                               │
                ▼                               ▼
┌───────────────────────────┐   ┌───────────────────────────┐
│      Layer2Manager        │   │    Layer2Manager Async    │
└───────────────┬───────────┘   └───────────────┬───────────┘
                │                               │
                ▼                               ▼
┌───────────────────────────────────────────────────────────┐
│                  Layer2 Protocol Layer                    │
├───────────┬───────────┬───────────┬───────────┬───────────┤
│ BobClient │LightningN.│LiquidMod. │ RskClient │StacksClnt.│
├───────────┼───────────┼───────────┼───────────┼───────────┤
│   State   │ Taproot   │    DLC    │    RGB    │   Other   │
│  Channel  │  Assets   │  Oracle   │  Assets   │ Protocols │
└───────────┴───────────┴───────────┴───────────┴───────────┘
```

## Layer2Manager Structure

```
┌───────────────────────────────────────┐
│           Layer2Manager               │
├───────────────────────────────────────┤
│ - protocol_registry: ProtocolRegistry │
│ - config: Layer2Config                │
├───────────────────────────────────────┤
│ + new()                               │
│ + initialize_all()                    │◄──────┐
│ + get_protocol()                      │◄──┐   │
│ + cross_layer_transfer()              │   │   │
│ + verify_cross_layer_proof()          │   │   │   Synchronous
│                                       │   │   │    Methods
│ + initialize_all_async()              │◄──┼───┼───┐
│ + get_protocol_async()                │◄──┼───┘   │
│ + cross_layer_transfer_async()        │   │       │
│ + verify_cross_layer_proof_async()    │   │       │   Asynchronous
└───────────────────────────────────────┘   │       │    Methods
                  ▲                         │       │
                  │                         │       │
┌─────────────────┴─────────────────────┐   │       │
│        ProtocolRegistry               │   │       │
├───────────────────────────────────────┤   │       │
│ - protocols: Map<Type, Box<Protocol>> │   │       │
├───────────────────────────────────────┤   │       │
│ + register()                          │   │       │
│ + get()                               │───┘       │
│ + get_async()                         │───────────┘
└───────────────────────────────────────┘
```

## Layer2Protocol Interface

```
┌──────────────────────────────────┐     ┌────────────────────────────────┐
│        Layer2Protocol            │     │      Layer2ProtocolAsync       │
├──────────────────────────────────┤     ├────────────────────────────────┤
│ + initialize()                   │     │ + initialize_async()           │
│ + submit_transaction()           │     │ + submit_transaction_async()   │
│ + get_transaction_status()       │     │ + get_transaction_status_async()│
│ + transfer_asset()               │     │ + transfer_asset_async()       │
│ + verify_proof()                 │     │ + verify_proof_async()         │
└──────────────────────────────────┘     └────────────────────────────────┘
              ▲                                        ▲
              │                                        │
              │                                        │
┌─────────────┴────────────────────────────────────────┴────────────────┐
│                        BobClient                                      │
├─────────────────────────────────────────────────────────────────────┤
│ - connection: BobConnection                                           │
│ - config: BobConfig                                                   │
├─────────────────────────────────────────────────────────────────────┤
│ + new(config: BobConfig)                                              │
│                                                                       │
│ // Synchronous Implementation                                         │
│ + initialize()                                                        │
│ + submit_transaction()                                                │
│ + get_transaction_status()                                            │
│ + transfer_asset()                                                    │
│ + verify_proof()                                                      │
│                                                                       │
│ // Asynchronous Implementation                                        │
│ + initialize_async()                                                  │
│ + submit_transaction_async()                                          │
│ + get_transaction_status_async()                                      │
│ + transfer_asset_async()                                              │
│ + verify_proof_async()                                                │
└────────────────────────────────────────────────────────────────────┘
```

## Asynchronous Processing Flow

```
┌────────────┐       ┌────────────────┐       ┌────────────────┐
│  Client    │       │ Layer2Manager  │       │ Protocol       │
│Application │       │    Async       │       │  Instance      │
└─────┬──────┘       └────────┬───────┘       └────────┬───────┘
      │                       │                        │
      │ initialize_all_async()|                        │
      │──────────────────────>│                        │
      │                       │                        │
      │                       │    initialize_async()  │
      │                       │───────────────────────>│
      │                       │                        │
      │                       │<───────────────────────│
      │<──────────────────────│                        │
      │                       │                        │
      │ get_protocol_async()  │                        │
      │──────────────────────>│                        │
      │                       │                        │
      │<──────────────────────│                        │
      │                       │                        │
      │     submit_tx_async() │                        │
      │───────────────────────────────────────────────>│
      │                       │                        │
      │                       │                        │
      │                       │                        │
      │<───────────────────────────────────────────────│
      │                       │                        │
```

## Concurrent Operations Handling

```
┌────────────┐      ┌────────────────┐       ┌────────────────┐
│  Client    │      │ Layer2Protocol │       │  Connection    │
│Application │      │    Async       │       │     Pool       │
└─────┬──────┘      └────────┬───────┘       └────────┬───────┘
      │                      │                        │
      │   Batch of N         │                        │
      │  Transactions        │                        │
      │─────────────────────>│                        │
      │                      │                        │
      │                      │    Get N connections   │
      │                      │───────────────────────>│
      │                      │                        │
      │                      │<───────────────────────│
      │                      │                        │
      │                      │                        │
      │                      │  ┌──────────────────┐  │
      │                      │  │Process TX 1      │  │
      │                      │  │in parallel with  │  │
      │                      │  │other transactions│  │
      │                      │  └──────────────────┘  │
      │                      │                        │
      │                      │  ┌──────────────────┐  │
      │                      │  │ TX N completed   │  │
      │                      │  └──────────────────┘  │
      │                      │                        │
      │                      │Release connections     │
      │                      │───────────────────────>│
      │                      │                        │
      │   Aggregated         │                        │
      │    Results           │                        │
      │<─────────────────────│                        │
      │                      │                        │
```

## Cross-Layer Operations

```
┌────────────┐     ┌────────────────┐    ┌──────────────┐    ┌──────────────┐
│  Client    │     │ Layer2Manager  │    │ Source       │    │ Destination  │
│Application │     │    Async       │    │ Protocol     │    │ Protocol     │
└─────┬──────┘     └────────┬───────┘    └──────┬───────┘    └──────┬───────┘
      │                     │                    │                   │
      │ cross_layer_        │                    │                   │
      │ transfer_async()    │                    │                   │
      │────────────────────>│                    │                   │
      │                     │                    │                   │
      │                     │ withdraw_async()   │                   │
      │                     │───────────────────>│                   │
      │                     │                    │                   │
      │                     │<───────────────────│                   │
      │                     │                    │                   │
      │                     │ create_proof_async()                   │
      │                     │───────────────────>│                   │
      │                     │                    │                   │
      │                     │<───────────────────│                   │
      │                     │                    │                   │
      │                     │ deposit_async()    │                   │
      │                     │──────────────────────────────────────> │
      │                     │                    │                   │
      │                     │<──────────────────────────────────────│
      │                     │                    │                   │
      │                     │ verify_proof_async()                   │
      │                     │──────────────────────────────────────> │
      │                     │                    │                   │
      │                     │<──────────────────────────────────────│
      │<────────────────────│                    │                   │
      │                     │                    │                   │
```

## Async Error Handling

```
┌────────────┐     ┌────────────────┐     ┌─────────────┐
│  Client    │     │ Layer2Protocol │     │ Error       │
│Application │     │    Async       │     │ Handler     │
└─────┬──────┘     └────────┬───────┘     └─────┬───────┘
      │                     │                   │
      │    async operation  │                   │
      │────────────────────>│                   │
      │                     │                   │
      │                     │    Error occurs   │
      │                     │───────────────────│
      │                     │                   │
      │                     │  Process error    │
      │                     │<───────────────────
      │                     │                   │
      │   Result::Err       │                   │
      │<────────────────────│                   │
      │                     │                   │
      │  match error type   │                   │
      │─────┐               │                   │
      │     │               │                   │
      │<────┘               │                   │
      │                     │                   │
      │ retry or handle     │                   │
      │ based on error      │                   │
      │────────────────────>│                   │
      │                     │                   │
```

These architectural diagrams provide a visual representation of the Layer2 module structure with both synchronous and asynchronous APIs. The diagrams illustrate the relationships between components and the flow of operations in the async implementation.
