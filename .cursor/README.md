# Anya-Core MCP Configuration
[AIR-3][AIS-3][AIT-2][AIM-2][AIP-2][AIE-2][BPC-3][AIP-3][PFM-2][SCL-2][RES-2]

Last Updated: 2025-03-10 09:15 UTC+2

## Overview

This directory contains the Model Context Protocol (MCP) configuration for the Cursor AI assistant within the Anya-Core repository. The MCP allows the AI to use specialized Bitcoin development tools according to the Bitcoin Development Framework v2.5.

## MCP Server Implementation

The MCP server is implemented as a Node.js application following the stdio protocol as specified in the [Cursor documentation](https://docs.cursor.com/context/model-context-protocol). The server provides Bitcoin-specific tools that conform to the project's hexagonal architecture requirements:

```
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
```
Validate this Taproot transaction: tr(KEY,{SILENT_LEAF})
```

### 2. Taproot Asset Creator

Creates Taproot assets with proper metadata according to project standards, generating both the asset definition and React Native component code.

Usage example:
```
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
```
Audit this Bitcoin code for security: function verifySignature(signature, message) { ... }
```

### 4. PSBT Generator

Generates Partially Signed Bitcoin Transaction (PSBT) templates that comply with BIP-174 and BIP-370.

Usage example:
```
Generate a PSBT with 2 inputs and 1 output
```

### 5. DLC Verifier

Verifies Discrete Log Contract setups for compliance with project standards.

Usage example:
```
Verify this DLC contract with oracle public key 03abc...
```

## AI Labelling Compliance

All tools implemented in the MCP server follow the AI labelling guidelines specified in `AI_LABELLING.md`. The server has the following ratings:

* **AIR-3**: Full AI-Readiness with structured data and well-documented interfaces
* **AIS-3**: Full AI Security with comprehensive validation and threat modeling
* **AIT-2**: Enhanced AI Testing with unit and integration tests
* **AIM-2**: Enhanced AI Monitoring with metrics and alerting
* **AIP-2**: Enhanced AI Privacy with data minimization and anonymization
* **AIE-2**: Enhanced AI Ethics with ethical guidelines and review process
* **BPC-3**: Full Bitcoin Compliance with all relevant BIPs and comprehensive testing
* **AIP-3**: Full Interoperability with all relevant systems
* **PFM-2**: Enhanced Performance with comprehensive optimizations
* **SCL-2**: Moderately Scalable with horizontal and vertical scaling support
* **RES-2**: Moderately Resilient with comprehensive error handling and failover mechanisms

## Security Considerations

The MCP server processes user requests and executes Bitcoin-related tools locally. All tools follow the project's security guidelines:

1. Input validation for all parameters
2. No direct execution of user-provided code
3. Isolation from production systems
4. Comprehensive error handling
5. No access to sensitive keys or wallets

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

* [Bitcoin Development Framework v2.5](../docs/bitcoin-framework.md)
* [AI Labelling Guidelines](../AI_LABELLING.md)
* [Hexagonal Architecture Requirements](../docs/hexagonal-architecture.md)
* [Cursor MCP Documentation](https://docs.cursor.com/context/model-context-protocol) 