#!/bin/bash

# [AIR-3][AIS-3][BPC-3][RES-3] Documentation Serve Script
# This script builds and serves the Anya Core documentation locally

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🚀 Starting Anya Core Documentation Server${NC}"

# Check if virtual environment exists
if [ ! -d "venv" ]; then
    echo -e "${YELLOW}❌ Virtual environment not found. Running setup first...${NC}"
    ./scripts/setup_docs.sh
fi

# Activate virtual environment
source venv/bin/activate

# Check if mkdocs is installed
if ! command -v mkdocs &> /dev/null; then
    echo -e "${YELLOW}❌ MkDocs is not installed. Installing...${NC}"
    pip install -r requirements-docs.txt
fi

echo -e "${GREEN}✓ Starting MkDocs development server...${NC}
echo -e "${YELLOW}👉 Open http://127.0.0.1:8000 in your browser to view the documentation${NC}"
echo -e "${YELLOW}📝 Edit files in the 'docs' directory and the site will auto-reload${NC}"
echo -e "${YELLOW}🛑 Press Ctrl+C to stop the server${NC}\n"

# Run MkDocs with live reload
mkdocs serve
