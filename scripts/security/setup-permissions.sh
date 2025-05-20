#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# Setup permissions for security scripts
# [AIR-2][AIS-2]

echo "Setting up permissions for security scripts..."

# Make security scripts executable
chmod +x scripts/security/analyze-mcp-server.js
chmod +x scripts/security/crypto-validation.js
chmod +x scripts/bitcoin/validate-bip-compliance.js

# Create directories if they don't exist
mkdir -p reports
mkdir -p anya-core/codeql/queries

echo "Permissions set successfully!" 