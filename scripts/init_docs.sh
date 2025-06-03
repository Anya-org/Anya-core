#!/bin/bash

# [AIR-3][AIS-3][BPC-3][RES-3] Documentation Initialization Script
# This script sets up the documentation directory structure and initial files

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Documentation directories to create
DOC_DIRS=(
    "docs/api"
    "docs/architecture"
    "docs/assets/images"
    "docs/getting-started"
    "docs/guides"
    "docs/installation"
    "docs/standards"
    "docs/tutorials"
)

# Create documentation directories
echo -e "${YELLOW}📂 Creating documentation directories...${NC}"
for dir in "${DOC_DIRS[@]}"; do
    mkdir -p "$dir"
    echo -e "${GREEN}✓ Created directory: $dir${NC}"
done

# Create README files for each directory
echo -e "\n${YELLOW}📝 Creating README files...${NC}"
for dir in "${DOC_DIRS[@]}"; do
    if [ ! -f "$dir/README.md" ]; then
        title=$(echo "$dir" | sed 's/^docs\///' | sed 's/\// /g' | awk '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) tolower(substr($i,2));}1')
        cat > "$dir/README.md" <<EOL
# $title

[AIR-3][AIS-3][BPC-3][RES-3]

## Overview

This directory contains documentation related to $title.

## Contents

- [Getting Started]($(echo "$dir" | sed 's/^docs\///')/getting-started)
- [Guides]($(echo "$dir" | sed 's/^docs\///')/guides)
- [API Reference]($(echo "$dir" | sed 's/^docs\///')/api)

## See Also

- [Main Documentation](../README.md)
- [Documentation Standards](../standards/MARKDOWN_STYLE_GUIDE.md)
EOL
        echo -e "${GREEN}✓ Created README: $dir/README.md${NC}"
    else
        echo -e "${YELLOW}⚠  README already exists: $dir/README.md${NC}"
    fi
done

# Copy template to docs directory
if [ ! -f "docs/TEMPLATE.md" ]; then
    cp docs/.template.md "docs/TEMPLATE.md"
    echo -e "\n${GREEN}✓ Created documentation template: docs/TEMPLATE.md${NC}"
else
    echo -e "\n${YELLOW}⚠  Documentation template already exists: docs/TEMPLATE.md${NC}"
fi

# Create a basic index file
if [ ! -f "docs/index.md" ]; then
    cat > "docs/index.md" <<EOL
---
title: "Anya Core Documentation"
description: "Comprehensive documentation for the Anya Core project"
---

[AIR-3][AIS-3][BPC-3][RES-3]

# Anya Core Documentation

Welcome to the official documentation for Anya Core, a powerful platform combining Bitcoin/crypto functionality, ML-based analytics, Web5 decentralized data management, and Bitcoin-style DAO governance.

## Getting Started

- [Quick Start Guide](getting-started/quickstart.md) - Get up and running in minutes
- [Installation Guide](installation/README.md) - Detailed installation instructions
- [Tutorials](tutorials/README.md) - Step-by-step guides for common tasks

## Documentation Sections

- [API Reference](api/README.md) - Complete API documentation
- [Architecture](architecture/README.md) - System design and architecture
- [Standards](standards/README.md) - Coding and documentation standards
- [Guides](guides/README.md) - In-depth technical guides

## Resources

- [GitHub Repository](https://github.com/anya-org/anya-core)
- [Issue Tracker](https://github.com/anya-org/anya-core/issues)
- [Contributing Guide](CONTRIBUTING.md)

## License

This documentation is licensed under the [MIT License](../LICENSE).
EOL
    echo -e "\n${GREEN}✓ Created documentation index: docs/index.md${NC}"
else
    echo -e "\n${YELLOW}⚠  Documentation index already exists: docs/index.md${NC}"
fi

echo -e "\n${GREEN}✅ Documentation initialization complete!${NC}"
echo -e "\nNext steps:"
echo -e "1. Review the documentation structure"
echo -e "2. Edit the template files as needed"
echo -e "3. Run './scripts/serve_docs.sh' to preview your documentation\n"
