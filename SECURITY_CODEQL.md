# Bitcoin MCP Server Security and CodeQL Analysis
[AIR-3][AIS-3][BPC-3][AIT-3][RES-3]

This document describes the security analysis framework for the Bitcoin MCP server and related components, aligned with the Bitcoin Development Framework v2.5 requirements.

## Overview

The security analysis framework consists of:

1. **CodeQL Configuration**: Custom CodeQL configuration for Bitcoin-specific security analysis
2. **Security Scripts**: JavaScript-based security validation scripts
3. **Continuous Integration**: GitHub Actions workflow integration for automated security scanning
4. **Documentation**: Security standards and compliance documentation

All components implement the required AI labeling system for traceability and compliance tracking.

## CodeQL Configuration

The CodeQL configuration is stored in the `.github/codeql` directory and includes:

- `codeql-config.yml`: Main configuration with paths, filters, and severity levels
- `bitcoin-protocol-security.qls`: Bitcoin protocol security query suite
- `crypto-validation.qls`: Cryptographic algorithm validation query suite
- `README.md`: Documentation for the CodeQL configuration

The configuration focuses on:

1. Bitcoin protocol security (BPC-3)
2. Cryptographic algorithm security (AIS-3)
3. AI system readiness (AIR-3)
4. Testing and validation (AIT-3)
5. System resilience (RES-3)

## Security Scripts

Custom security scripts in the `scripts/security` and `scripts/bitcoin` directories:

- `analyze-mcp-server.js`: Analyzes MCP server security
- `crypto-validation.js`: Validates cryptographic implementations
- `validate-bip-compliance.js`: Validates BIP compliance
- `run-codeql-analysis.ps1`: PowerShell script to run CodeQL analysis

These scripts implement checks for:

1. Secure random number generation
2. Constant-time operations for cryptographic comparisons
3. Appropriate key sizes
4. Modern cryptographic algorithms
5. Proper error handling
6. Absence of hardcoded secrets
7. BIP standard compliance (BIP-340, BIP-341, BIP-342, BIP-174, BIP-370, BIP-327)

## GitHub Actions Integration

The security analysis is integrated with GitHub Actions through:

- `security-scan.yml`: Main workflow for running security scans
- Custom reporting for security issues
- Integration with GitHub's security dashboard

## Running Security Analysis

### Automated Analysis

Security analysis runs automatically via GitHub Actions:

- On push to main/develop branches
- On pull requests to main
- Daily at midnight (scheduled)
- On-demand via workflow dispatch

### Manual Analysis

To run security analysis manually:

1. **Using PowerShell Script**:

   ```powershell
   .\scripts\security\run-codeql-analysis.ps1
   ```

2. **Using Individual Scripts**:

   ```bash
   node scripts/security/analyze-mcp-server.js --file=scripts/bitcoin/mcp-server.js
   node scripts/security/crypto-validation.js
   node scripts/bitcoin/validate-bip-compliance.js
   ```

3. **Using CodeQL CLI**:

   ```bash
   codeql database create js-db --language=javascript
   codeql database analyze js-db .github/codeql/codeql-config.yml --format=sarif-latest --output=js-results.sarif
   ```

## Security Compliance

Security analysis is designed to validate compliance with:

1. **Bitcoin Development Framework v2.5**
   - BPC-3 level compliance for Bitcoin protocol
   - Full support for BIP-340, BIP-341, BIP-342, BIP-174, BIP-370

2. **AI Labeling Requirements**
   - AIR-3: AI readiness
   - AIS-3: AI security
   - AIT-3: AI testing
   - RES-3: Resilience

3. **Cryptographic Security Standards**
   - Use of secure random number generation
   - Constant-time operations for cryptographic comparisons
   - Appropriate key sizes (256+ bits)
   - Modern cryptographic algorithms

## Report Generation

All security analysis tools generate detailed reports in JSON format, stored in the `reports` directory:

- `mcp-server-security-report-{timestamp}.json`: MCP server security report
- `crypto-validation-report-{timestamp}.json`: Cryptographic validation report
- `bip-compliance-report-{timestamp}.json`: BIP compliance report
- `js-results.sarif`: JavaScript CodeQL analysis results
- `rust-results.sarif`: Rust CodeQL analysis results

## Status and Maintenance

The security analysis framework is:

- Version: 1.0.0
- Last Updated: March 2025
- Compliance: Bitcoin Development Framework v2.5
- AI Labels: [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]

Maintenance Schedule:

- Monthly review of CodeQL queries
- Quarterly update of security standards
- Annual full security audit

---

## Related Security Documentation

For our general security policy, vulnerability reporting process, and bug bounty program information, please refer to [SECURITY.md](./SECURITY.md).
