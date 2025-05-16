# Performance Testing

This directory contains performance testing utilities and benchmarks for Anya Core components.

## Table of Contents
- [Running Benchmarks](#running-benchmarks)
- [Writing Benchmarks](#writing-benchmarks)
- [Performance Guidelines](#performance-guidelines)
- [Benchmark Results](#benchmark-results)

## Running Benchmarks

### Prerequisites

```bash
# Install dependencies
cargo install criterion
cargo install cargo-criterion
```

### Running All Benchmarks

```bash
cargo criterion
```

### Running Specific Benchmarks

```bash
# Run a specific benchmark
cargo criterion --bench transaction_benchmark

# Filter benchmarks by name
cargo criterion -- --filter "create_wallet"
```

## Writing Benchmarks

### Basic Benchmark

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use anya_core::crypto::KeyPair;

fn benchmark_key_generation(c: &mut Criterion) {
    c.bench_function("generate_keypair", |b| {
        b.iter(|| {
            let _keypair = KeyPair::generate();
        })
    });
}

criterion_group!(benches, benchmark_key_generation);
criterion_main!(benches);
```

### Benchmark with Setup

```rust
use criterion::{criterion_group, criterion_main, Criterion, BatchSize};
use anya_core::wallet::Wallet;

fn benchmark_wallet_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("wallet_operations");
    
    // Benchmark wallet creation
    group.bench_function("create_wallet", |b| {
        b.iter(|| Wallet::new())
    });
    
    // Benchmark transaction signing with setup
    group.bench_function("sign_transaction", |b| {
        b.iter_batched(
            || (Wallet::new(), create_test_transaction()),
            |(wallet, tx)| wallet.sign(tx),
            BatchSize::SmallInput
        )
    });
    
    group.finish();
}
```

## Performance Guidelines

### General Guidelines

1. **Isolate Tests**: Each benchmark should test a single operation
2. **Use Realistic Data**: Test with realistic input sizes
3. **Warm-up**: Allow for warm-up iterations
4. **Measure Consistently**: Run benchmarks on the same hardware

### Criterion Features

- **Statistical Analysis**: Automatically calculates statistics
- **Comparison**: Compare against previous runs
- **Parameterization**: Test with different input sizes

## Benchmark Results

### Current Benchmarks

| Benchmark | Throughput | Latency (p95) |
|-----------|------------|---------------|
| Key Generation | 12,500 ops/s | 80μs |
| Transaction Signing | 8,200 ops/s | 122μs |
| Block Validation | 45 blocks/s | 22ms |
| Wallet Creation | 15,000 ops/s | 67μs |

### Performance Regression Testing

To detect performance regressions:

```bash
# Save baseline
cargo criterion -- --save-baseline base

# Compare against baseline
cargo criterion -- --baseline base --benchmark
```

## Profiling

### Using perf (Linux)

```bash
# Record profile
perf record --call-graph dwarf -- cargo bench --bench transaction_benchmark -- --profile-time=5

# Generate flamegraph
perf script | stackcollapse-perf.pl | flamegraph.pl > flamegraph.svg
```

### Using Instruments (macOS)

```bash
# Build with debug symbols
RUSTFLAGS=-g cargo build --release --bench transaction_benchmark

# Profile with Instruments
instruments -t "Time Profiler" -D profile.trace ./target/release/transaction_benchmark --bench
```

## Continuous Integration

Performance tests are run on every PR. The CI will fail if:
- Any benchmark shows >10% regression
- Memory usage increases by >5%
- CPU usage increases by >5%

## Troubleshooting

### Common Issues

1. **Noisy Neighbors**
   - Run benchmarks on dedicated hardware
   - Disable CPU frequency scaling
   - Use `taskset` to pin to specific cores

2. **High Variance**
   - Increase sample size
   - Run for longer duration
   - Check for background processes

3. **Memory Leaks**
   - Run with `valgrind --leak-check=full`
   - Check for unbounded collections

## Resources

- [Criterion.rs Book](https://bheisler.github.io/criterion.rs/book/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
