#!/bin/bash
set -e

# Clean up legacy files and documentation
rm -rf docs/legacy/* 
rm -f docs/*.bak
rm -f scripts/deprecated/*

# Create directory structure
mkdir -p docs/{architecture,specs,api}
mkdir -p scripts/{deployment,testing}

# Initialize core documentation
cat > docs/architecture/SYSTEM_MAP.md << 'EOF'
# Anya Core System Architecture
## Bitcoin Development Framework v2.5 Compliant

### Hexagonal Architecture Overview
- Core Domain Logic
- Protocol Adapters 
- External Interfaces
- Monitoring & Metrics

### Component Structure
- Node Communication (P2P)
- Wallet Interface (PSBT/BIP-174) 
- Smart Contract Layer (Miniscript)
- Oracle Integration (DLC)

### Security & Compliance
- BIP 341/342 (Taproot)
- Transaction Validation
- Audit Requirements
EOF

# Set up workspace structure
cat > scripts/deployment/setup_workspace.sh << 'EOF'
#!/bin/bash
cargo new --lib anya-core
cargo new --lib anya-core-pkg
cargo new --bin cli
EOF

chmod +x scripts/deployment/setup_workspace.sh
