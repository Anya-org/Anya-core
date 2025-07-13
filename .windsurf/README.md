# Anya-Core MCP Configuration

[AIR-3][AIS-3][AIT-3][BPC-3][RES-3]

Last Updated: 2025-03-10 09:15 UTC+2

## Overview

This directory contains the Model Context Protocol (MCP) configuration for the Cursor AI assistant within the Anya-Core repository. The MCP allows the AI to use specialized Bitcoin development tools according to official Bitcoin Improvement Proposals (BIPs).

## Consolidated MCP Servers

| Server Name          | Purpose                          | Protocols Supported       | Security Level |
|----------------------|----------------------------------|---------------------------|----------------|
| anya-bitcoin-core    | Core protocol operations         | BIP-341, BIP-342, PSBT    | AIS-3          |

## Access Control Matrix

| Tool Category        | Allowed Actions                 | Restricted Operations      |
|----------------------|---------------------------------|----------------------------|
| Protocol Validation  | Read-only analysis              | No network broadcasting    |
| File Operations      | PSBT creation/editing           | No raw transaction signing |

## MCP Server Implementation

The MCP server is implemented as a Node.js application following the stdio protocol as specified in the [Cursor documentation](https://docs.cursor.com/context/model-context-protocol). The server provides Bitcoin-specific tools that conform to the project's hexagonal architecture requirements:

```text
                      +----------------+
                      |  Bitcoin Core  |
                      +-------+--------+
                              |
                      +-------v--------+
                      |  Adapter Layer |
                      +-------+--------+
                              |
+----------------+    +-------v--------+    +----------------+
|   External     |    |   Application  |    |   Monitoring   |
|   Interfaces   <----+   Core Logic    +---->   & Metrics   |
| (APIs, Wallets)|    +-------+--------+    | (Prometheus)   |
+----------------+            |             +----------------+
                      +-------v--------+
                      |   Protocol     |
                      |   Adapters     |
                      +-------+--------+
                              |
                      +-------v--------+
                      |  Blockchain    |
                      |  Network       |
                      +----------------+
```

## Available Tools

The MCP server provides the following Bitcoin development tools:

### 1. Bitcoin Protocol Validator

Validates Bitcoin protocol compliance according to BIP standards:

* BIP-341 (Taproot)
* BIP-342 (Tapscript)
* BIP-174 (PSBT)
* BIP-370 (PSBT Version 2)

Usage example:

```text
Validate this Taproot transaction: tr(KEY,{SILENT_LEAF})
```

### 2. Taproot Asset Creator

Creates Taproot assets with proper metadata according to project standards, generating both the asset definition and React Native component code.

Usage example:

```text
Create a new asset named PrivacyCoin with supply 21000000
```

### 3. Bitcoin Security Audit

Runs security audit on Bitcoin code according to the compliance checklist, checking for:

* Timing vulnerabilities
* Input validation
* Error handling
* BIP-341 compliance
* Memory management issues

Usage example:

```text
Audit this Bitcoin code for security: function verifySignature(signature, message) { ... }
```

### 4. PSBT Generator

Generates Partially Signed Bitcoin Transaction (PSBT) templates that comply with BIP-174 and BIP-370.

Usage example:

```text
Generate a PSBT with 2 inputs and 1 output
```

### 5. DLC Verifier

Verifies Discrete Log Contract setups for compliance with project standards.

Usage example:

```text
Verify this DLC contract with oracle public key 03abc...
```

## AI Labelling Compliance

All tools implemented in the MCP server follow the AI labelling guidelines specified in `docs/standards/AI_LABELING.md`. The server has the following ratings:

* **AIR-3**: Full AI-Readiness with structured data and well-documented interfaces
* **AIS-3**: Full AI Security with comprehensive validation and threat modeling
* **AIT-3**: Enhanced AI Testing with unit and integration tests
* **BPC-3**: Full Bitcoin Compliance with all relevant BIPs and comprehensive testing
* **RES-3**: Moderately Resilient with comprehensive error handling and failover mechanisms

## Security Considerations

The MCP server processes user requests and executes Bitcoin-related tools locally. All tools follow the project's security guidelines:

1. Input validation using BIP-341 regex patterns [AIS-3]
2. Schnorr signature verification with constant-time checks [AIS-3][BPC-3]
3. PSBT validation according to BIP-174/370 [BPC-3]

## Usage in Cursor

To use the MCP tools in Cursor:

1. Ensure you have Cursor installed and updated
2. Open the Anya-Core repository in Cursor
3. The project-specific MCP configuration (.cursor/mcp.json) will be automatically detected
4. Use the AI assistant to interact with the Bitcoin development tools
5. The AI will use the appropriate tool based on your natural language request

## Implementation Notes

The MCP server implements the stdio protocol format as specified in the Cursor documentation. It listens for JSON-formatted requests on stdin and responds with JSON-formatted results on stdout.

## Contributing

When adding new tools to the MCP server:

1. Add tool definition to the TOOLS array in `scripts/bitcoin/mcp-server.js`
2. Implement the tool handler function
3. Update this README with the new tool information
4. Add appropriate AI labelling headers
5. Run tests to ensure the tool functions correctly

## Related Documentation

* [Official Bitcoin Improvement Proposals (BIPs)](../docs/bitcoin-framework.md)
* [AI Labelling Guidelines](../docs/standards/AI_LABELING.md)
* [Hexagonal Architecture Requirements](../docs/hexagonal-architecture.md)
* [Cursor MCP Documentation](https://docs.cursor.com/context/model-context-protocol) 

## Security Audit Results [AIS-3][BPC-3]

| Check                  | Status  | Remediation |
|------------------------|---------|-------------|
| Schnorr Verification   | Fixed   | Implemented constant-time checks |
| Input Validation       | Fixed   | Added BIP-341 regex patterns |
| Error Handling         | Fixed   | Added comprehensive try-catch blocks |
| Taproot Compliance     | Fixed   | SILENT_LEAF implementation |
