# Anya Core Test System

# [AIR-3][AIS-3][BPC-3][RES-3]

Date: 2025-05-20

## Overview

The Anya Core Test System provides a comprehensive framework for testing all components of the Anya Core project following the Bitcoin Development Framework v2.5 standards. This system is designed to ensure that all code meets the quality, security, and compliance requirements specified in the BDF v2.5.

## Unified Test Framework

The `unified_test_framework.sh` script serves as the primary entry point for all testing activities. It follows the hexagonal architecture principles and provides a consistent interface for running tests across different modules and components.

### Features

- **Multi-level Testing**: Support for minimal, standard, and full test levels
- **Category-based Testing**: Ability to test specific components (core, bitcoin, hsm, web5)
- **Parallel Test Execution**: Optimized performance through parallel test execution
- **Timeout Management**: Configurable timeouts to prevent hanging tests
- **Comprehensive Logging**: Detailed logs for troubleshooting and analysis
- **Consistent Reporting**: Standardized output format for all tests

## Usage

```bash
# Run minimal core tests
./scripts/test/unified_test_framework.sh --level=minimal --category=core

# Run standard tests for all categories
./scripts/test/unified_test_framework.sh --level=standard

# Run full Bitcoin tests
./scripts/test/unified_test_framework.sh --level=full --category=bitcoin

# Run tests with verbose output
./scripts/test/unified_test_framework.sh --verbose

# Set custom timeout
./scripts/test/unified_test_framework.sh --timeout=600
```

## Test Categories

### Core Tests

Tests for the core functionality of the Anya Core project, including:

- Basic system operations
- Configuration management
- Error handling
- Logging and monitoring

### Bitcoin Tests

Tests for Bitcoin-related functionality, including:

- Network configuration
- Transaction handling
- Block verification
- Taproot implementation
- RSK Bitcoin verification with `#[rsk_bind]` annotation

### HSM Tests

Tests for Hardware Security Module functionality, including:

- Key management
- Signing operations
- Secure storage
- Hardware device integration

### Web5 Tests

Tests for Web5-related functionality, including:

- DID management
- Decentralized Web Node (DWN)
- Verifiable Credentials
- Web5 protocol compliance

## Test Levels

### Minimal Tests

Basic tests that verify core functionality with minimal features enabled. These tests are fast and provide a quick sanity check.

### Standard Tests

Comprehensive tests that verify all standard functionality with default features enabled. These tests provide a good balance between coverage and execution time.

### Full Tests

Exhaustive tests that verify all functionality with all features enabled. These tests provide maximum coverage but may take longer to execute.

## Hexagonal Architecture Compliance

The test system follows the hexagonal architecture principles as defined in the Bitcoin Development Framework v2.5:

1. **Core Domain Tests**
   - Tests that verify core business logic
   - Bitcoin-specific functionality tests

2. **Port Tests**
   - Tests for interfaces and APIs
   - Tests for external system integration

3. **Adapter Tests**
   - Tests for OS-specific adapters
   - Tests for hardware-specific adapters
   - Tests for network-specific adapters

## AI Labeling Compliance

All test scripts include proper AI labeling according to the Bitcoin Development Framework v2.5 standards:

- [AIR-3] - AI Responsibility
- [AIS-3] - AI Security
- [BPC-3] - Bitcoin Protocol Compliance
- [RES-3] - Resource Efficiency

## Integration with CI/CD

The test system is designed to integrate seamlessly with CI/CD pipelines:

```yaml
# Example CI/CD configuration
test:
  script:
    - ./scripts/test/unified_test_framework.sh --level=standard
  artifacts:
    paths:
      - logs/tests/
```

## Logs and Reporting

Test logs are stored in the `logs/tests/` directory with timestamps for easy reference. Each test run generates a comprehensive report that includes:

- Test execution time
- Pass/fail status for each test
- Detailed error messages for failed tests
- System resource usage during tests

## Extending the Test System

To add new tests to the system:

1. Create a new test file in the appropriate module directory
2. Ensure the test follows the Bitcoin Development Framework v2.5 standards
3. Add proper AI labeling to the test file
4. Update the unified test framework if necessary

## Conclusion

The Anya Core Test System provides a robust framework for ensuring the quality, security, and compliance of the Anya Core project according to the Bitcoin Development Framework v2.5 standards. By following the hexagonal architecture principles and implementing proper AI labeling, the test system helps maintain a high standard of code quality and reliability.
