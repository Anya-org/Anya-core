# Script Directory
[AIR-3][AIS-3][BPC-3][AIT-3][RES-3]

This directory contains scripts for building, testing, and maintaining the Bitcoin MCP Server implementation according to the Bitcoin Development Framework v2.5.

## Directory Structure

- `bitcoin/`: Scripts specific to Bitcoin protocol implementation
  - `validate-security.js`: Basic security validation for Bitcoin components
  - `validate-bip-compliance.js`: Validates compliance with Bitcoin Improvement Proposals (BIPs)
  - `mcp-server.js`: Main MCP server implementation

- `security/`: Security analysis and validation scripts
  - `run-codeql-analysis.ps1`: PowerShell script to run CodeQL analysis
  - `crypto-validation.js`: Validates cryptographic implementations
  - `analyze-mcp-server.js`: Analyzes MCP server security
  - `setup-permissions.sh`: Sets up permissions for security scripts

## Key Scripts

### Security Analysis

The security analysis framework consists of multiple components:

1. **CodeQL Analysis**:
   ```powershell
   .\scripts\security\run-codeql-analysis.ps1
   ```
   
2. **Component-specific Analysis**:
   ```bash
   node scripts/security/analyze-mcp-server.js --file=scripts/bitcoin/mcp-server.js
   node scripts/security/crypto-validation.js
   node scripts/bitcoin/validate-bip-compliance.js
   ```

### Test Scripts

Several scripts are available for testing various components:

```powershell
# Run all tests
.\scripts\run-all-tests.sh

# Run integration tests
.\scripts\run-integration-tests.ps1

# Run module tests
.\scripts\run-module-tests.ps1
```

### Build Scripts

Scripts for building and setting up the development environment:

```bash
# Setup development environment
.\scripts\dev-setup.ps1

# Build the project
.\scripts\build.ps1
```

## Compliance

The scripts in this directory have been updated to comply with the Bitcoin Development Framework v2.5 requirements and include proper AI labeling ([AIR-3][AIS-3][BPC-3][AIT-3][RES-3]).

For more information on security analysis and compliance, see `SECURITY_CODEQL.md` in the project root.

## Recent Updates

- **March 2025**: Cleaned up redundant scripts and standardized the security analysis framework
- **March 2025**: Enhanced BIP compliance validation (BIP-340, BIP-341, BIP-342, BIP-174, BIP-370, BIP-327)
- **March 2025**: Added CodeQL integration for comprehensive security analysis 