# CodeQL Configuration for Official Bitcoin Improvement Proposals (BIPs)
[AIR-3][AIS-3][BPC-3][AIT-2]

This directory contains CodeQL configuration files for analyzing the Anya Bitcoin implementation according to official Bitcoin Improvement Proposals (BIPs) security standards.

## Configuration Files

- **codeql-config.yml**: Main configuration file that defines paths to analyze, security severity levels, and query filters.
- **bitcoin-protocol-security.qls**: Custom query suite for Bitcoin protocol security analysis.
- **crypto-validation.qls**: Custom query suite for cryptographic algorithm validation.

## Security Focus Areas

The CodeQL configuration focuses on the following key areas:

1. **Bitcoin Protocol Compliance (BPC-3)**
   - Taproot (BIP-341)
   - Schnorr signatures (BIP-340)
   - PSBT validation (BIP-174/370)
   - Lightning Network integration

2. **Cryptographic Security (AIS-3)**
   - Constant-time operations
   - Secure random number generation
   - Key management
   - Side-channel attack prevention

3. **AI Readiness (AIR-3)**
   - Proper labeling
   - Documentation
   - Error handling
   - Resource management

## Integration with GitHub Actions

The CodeQL configuration is integrated with GitHub Actions through the security-scan.yml workflow file. This workflow:

1. Runs on push to main/develop, pull requests to main, and daily
2. Performs CodeQL analysis with our custom configuration
3. Runs additional Bitcoin-specific security scans
4. Reports any issues found through GitHub Security dashboard

## Usage

### GitHub Actions

CodeQL analysis is automatically run as part of the GitHub Actions workflow.

### Local Analysis

To run CodeQL analysis locally:

1. Install the CodeQL CLI: https://github.com/github/codeql-cli-binaries/releases
2. Set up the CodeQL databases:
   ```sh
   codeql database create js-db --language=javascript
   codeql database create rust-db --language=rust
   ```
3. Run the analysis with our custom configuration:
   ```sh
   codeql database analyze js-db .github/codeql/codeql-config.yml --format=sarif-latest --output=js-results.sarif
   codeql database analyze rust-db .github/codeql/codeql-config.yml --format=sarif-latest --output=rust-results.sarif
   ```

## Extending the Configuration

To add new custom queries:

1. Create a query file with the `.ql` extension in the `anya-core/codeql/queries` directory
2. Add the query to the appropriate `.qls` query suite file
3. Update the `codeql-config.yml` file if necessary

## BDF v2.5 Compliance

The CodeQL configuration is designed to validate compliance with official Bitcoin Improvement Proposals (BIPs), focusing on:

- BPC-3 level compliance for Bitcoin protocol features
- AIS-3 level security for cryptographic operations
- AIR-3 level readiness for AI integration

## Related Scripts

Security validation scripts that complement CodeQL analysis:

- `scripts/security/analyze-mcp-server.js`: Analyzes MCP server security
- `scripts/bitcoin/validate-bip-compliance.js`: Validates BIP compliance
- `scripts/security/crypto-validation.js`: Validates cryptographic implementations 