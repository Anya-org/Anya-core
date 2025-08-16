# testing Module

Testing utilities for Anya-Core

## Overview

The `testing` module provides comprehensive testing infrastructure for the Anya Core system. It includes performance benchmarking, cross-component validation, and specialized testing utilities to ensure the reliability and efficiency of the entire system.

## Key Components

### UnifiedTester

A comprehensive testing framework that:

- Validates Bitcoin protocol implementations
- Verifies DAO compliance
- Monitors AI metrics and performance
- Tests cross-component interactions

```rust
let tester = UnifiedTester::new();
let report = tester.full_system_test()?;
println!("Bitcoin tests: {}", report.bitcoin);
println!("DAO compliance: {}", report.dao);
println!("AI performance: {}", report.ai);
```

### Performance Testing

The `performance` submodule offers robust benchmarking capabilities:

- Configurable test runners with warmup phases
- Consistent measurement methodologies
- Report generation in Markdown format
- Targeted and comprehensive test suites

```rust
// Run a comprehensive performance test suite
run_comprehensive_test_suite(Path::new("./reports"))?;

// Or run a specific test
run_targeted_test("transaction_processing", 10000, Path::new("./reports"))?;
```

### Sectional Test Utilities

The `sectional_test_utils` module allows:

- Breaking large test suites into manageable sections
- Managing dependencies between test sections
- Selective enabling/disabling of test components
- Timeout management for long-running tests

```rust
let mut runner = SectionRunner::new();

runner.add_section(TestSection {
    name: "bitcoin_consensus".to_string(),
    enabled: true,
    timeout_seconds: 180,
    dependencies: vec!["bitcoin_network".to_string()],
});

runner.run_section("bitcoin_consensus")?;
```

### Component-Specific Validators

The module includes specialized validators for different system components:

- `BitcoinValidator`: Validates Bitcoin protocol implementation
- `DaoComplianceCheck`: Verifies DAO compliance with standards
- `AIMetricCollector`: Gathers and validates AI performance metrics

## Performance Test Components

The performance testing framework includes specialized benchmarks for:

- Transaction processing (simple, complex, and fixed scenarios)
- Database operations
- Cache efficiency
- Network throughput
- API response times

## Integration with CI/CD

The testing module is designed to integrate seamlessly with CI/CD pipelines:

- Generates machine-readable outputs
- Supports threshold-based pass/fail criteria
- Provides historical performance comparisons
- Creates comprehensive test reports

## For more information

See the comprehensive documentation in the [docs/](/docs/) directory.

Note: Network-bound tests (e.g., DNS/Bitcoin ports) may fail on developer machines without the required services; treat them as advisory locally and rely on CI or configured environments for definitive results.
