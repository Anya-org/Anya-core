#!/bin/bash

# [AIR-3][AIS-3][BPC-3][RES-3] Documentation Setup Script
# This script sets up the documentation environment for Anya Core

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🚀 Setting up Anya Core Documentation System${NC}"

# Check if Python 3.8+ is installed
if ! command -v python3 &> /dev/null; then
    echo -e "${YELLOW}❌ Python 3 is required but not installed. Please install Python 3.8 or higher.${NC}"
    exit 1
fi

# Get Python version components
PYTHON_VERSION=$(python3 -c 'import sys; print("{}.{}".format(sys.version_info.major, sys.version_info.minor))')
PYTHON_MAJOR=$(python3 -c 'import sys; print(sys.version_info.major)')
PYTHON_MINOR=$(python3 -c 'import sys; print(sys.version_info.minor)')

# Check if Python version is 3.8 or higher
if [ $PYTHON_MAJOR -lt 3 ] || { [ $PYTHON_MAJOR -eq 3 ] && [ $PYTHON_MINOR -lt 8 ]; }; then
    echo -e "${YELLOW}❌ Python 3.8 or higher is required. Found Python $PYTHON_VERSION.${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Python $PYTHON_VERSION is installed${NC}"

# Create virtual environment if it doesn't exist
if [ ! -d "venv" ]; then
    echo -e "${YELLOW}Creating Python virtual environment...${NC}"
    python3 -m venv venv
    source venv/bin/activate
    pip install --upgrade pip
    pip install -r requirements-docs.txt
    echo -e "${GREEN}✓ Virtual environment created and dependencies installed${NC}"
else
    echo -e "${GREEN}✓ Using existing virtual environment${NC}"
    source venv/bin/activate
fi

# Install documentation dependencies
echo -e "${YELLOW}Installing documentation dependencies...${NC}"
pip install -r requirements-docs.txt

echo -e "\n${GREEN}✅ Documentation setup complete!${NC}"
echo -e "\nTo build the documentation, run: ${YELLOW}mkdocs build${NC}"
echo -e "To serve the documentation locally, run: ${YELLOW}mkdocs serve${NC}"
echo -e "\nFor more information, visit: https://www.mkdocs.org/\n"
