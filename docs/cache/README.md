---
title: "Cache Module"
description: "Caching system for Anya Core"
status: "active"
last_updated: "2025-08-06"
---

# Cache Module [AIR-3][AIS-3][BPC-3][RES-3]

This module provides a flexible caching system for improving performance across the Anya Core platform.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Usage](#usage)
- [Configuration](#configuration)
- [Cache Stats](#cache-stats)
- [Examples](#examples)

## Overview

The Cache module implements a high-performance caching system that helps reduce redundant operations and improves response times throughout the Anya Core platform. It provides a standardized interface for storing and retrieving data with configurable expiration policies.

## Features

- In-memory and distributed caching options
- Configurable time-to-live (TTL) for cache entries
- Automatic cache invalidation
- Cache statistics and monitoring
- Thread-safe implementation

## Usage

The core of the module is the `CacheManager`, which provides methods for cache operations:

```rust
// Create a new cache manager
let cache = CacheManager::new(config);

// Store data in cache
cache.set("user:1234".to_string(), user_data).await?;

// Retrieve data from cache
let data = cache.get("user:1234").await?;

// Clean up expired entries
cache.cleanup().await?;
```

## Configuration

The cache system can be configured through the `CacheConfig` struct:

```rust
let config = CacheConfig {
    max_size: 1024 * 1024 * 10, // 10 MB
    ttl: std::time::Duration::from_secs(300), // 5 minutes
    distributed: false,
    redis_url: None,
};

let cache = CacheManager::new(config);
```

## Cache Stats

The cache system provides statistics for monitoring and debugging:

```rust
let stats = cache.get_stats().await?;
println!("Expired entries: {}", stats.expired_entries);
```

## Examples

Example of using the cache in a Bitcoin transaction retrieval scenario:

```rust
// Initialize cache
let cache = CacheManager::new(CacheConfig::default());

// Function that uses cache for transaction retrieval
async fn get_transaction(txid: &str, cache: &CacheManager) -> Result<Transaction, Error> {
    let cache_key = format!("tx:{}", txid);

    // Try to get from cache first
    match cache.get(&cache_key).await {
        Ok(data) => {
            // Deserialize cached data
            return Ok(bincode::deserialize(&data)?);
        },
        Err(_) => {
            // Not in cache, fetch from network
            let tx = bitcoin_client.get_transaction(txid).await?;

            // Cache the result for future requests
            let serialized = bincode::serialize(&tx)?;
            cache.set(cache_key, serialized).await?;

            Ok(tx)
        }
    }
}
```
