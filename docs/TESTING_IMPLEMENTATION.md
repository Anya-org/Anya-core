# Anya-Core Testing Implementation Guide

[AIR-3][AIS-3][AIT-3][AIP-3][BPC-3][DAO-4]

This document provides detailed implementation guidelines for the Anya-Core testing framework, complementing the [Testing Strategy](./TESTING_STRATEGY.md).

## Testing Framework Architecture

The Anya-Core testing framework follows a hexagonal architecture pattern:

                      +----------------+
                      |  Test Domain   |
                      |  Logic         |
                      +-------+--------+
                              |
                      +-------v--------+
                      |  Test Ports    |
                      +-------+--------+
                              |
+-----------------+   +-------v--------+   +----------------+
|                 |   |                |   |                | 
```

# Unified Testing System

The Anya-Core project includes a comprehensive unified testing system that allows running all tests across different components with consistent configuration and reporting.

## New Features

### System Prerequisites Check

The unified tester now performs a comprehensive system check before running any tests, ensuring your environment meets the minimum requirements:

- CPU cores and performance
- Available memory
- Disk space
- Network connectivity
- RPC endpoint availability

The system will provide specific recommendations if your environment doesn't meet optimal requirements.

### Real-Time Dashboard

The testing process now includes a real-time dashboard that shows:

- Current operation and status
- Progress bar with percentage complete
- Recent events and details
- Test results as they happen

### Automatic Configuration Updates

After the tests complete successfully, the system can automatically update your `anya.conf` configuration file based on:

- Test results and performance metrics
- System capabilities
- Detected issues and optimizations

This ensures your configuration is always optimized for your specific environment.

## Running the Unified Tests

### Using the CLI

```bash
# Run all tests with the dashboard
cargo run --bin anya-tester -- unified

# Run specific components
cargo run --bin anya-tester -- unified --components bitcoin,dao

# Run with custom Bitcoin RPC endpoint
cargo run --bin anya-tester -- unified --rpc-endpoint https://bitcoin-testnet-rpc.publicnode.com
```

### Using the Helper Script

```bash
# Run all tests with the dashboard
./scripts/run_unified_tests.sh

# Run specific components
./scripts/run_unified_tests.sh --components bitcoin,dao
```

## Configuration Optimization

After successful tests, you'll be prompted to update your configuration. This process:

1. Adjusts cache sizes based on available system memory
2. Optimizes batch sizes based on performance test results
3. Configures the Bitcoin RPC endpoint that performed best
4. Adjusts DAO parameters for optimal governance 
5. Fine-tunes system awareness thresholds

You can always decline the update if you prefer to maintain manual control of your configuration. 