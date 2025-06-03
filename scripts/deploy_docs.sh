#!/bin/bash

# [AIR-3][AIS-3][BPC-3][RES-3] Documentation Deployment Script
# This script builds and deploys the Anya Core documentation

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Configuration
DOCS_DIR="$(pwd)/docs"
BUILD_DIR="$(pwd)/site"
GIT_BRANCH="gh-pages"
REMOTE="origin"

# Check if we're in the right directory
if [ ! -f "mkdocs.yml" ]; then
    echo -e "${RED}❌ Error: mkdocs.yml not found. Please run this script from the project root.${NC}"
    exit 1
fi

# Verify documentation first
echo -e "${YELLOW}🔍 Verifying documentation...${NC}"
if ! ./scripts/verify_docs.sh; then
    echo -e "${RED}❌ Documentation verification failed. Please fix the issues before deploying.${NC}"
    exit 1
fi

# Activate virtual environment if it exists
if [ -d "venv" ]; then
    echo -e "${YELLOW}🐍 Activating Python virtual environment...${NC}"
    source venv/bin/activate
else
    echo -e "${YELLOW}⚠  Virtual environment not found. Running setup...${NC}"
    ./scripts/setup_docs.sh
    source venv/bin/activate
fi

# Install dependencies
echo -e "${YELLOW}📦 Installing dependencies...${NC}"
pip install -r requirements-docs.txt

# Build documentation
echo -e "${YELLOW}🏗️  Building documentation...${NC}"
mkdocs build --clean --strict

# Verify build
if [ ! -d "site" ]; then
    echo -e "${RED}❌ Build failed: site directory not created${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Documentation built successfully!${NC}"

# Deployment options
echo -e "\n${YELLOW}🚀 Deployment Options:${NC}"
echo "1) Local preview"
echo "2) Deploy to GitHub Pages"
echo -n "Select an option (1-2): "
read -r option

case $option in
    1)
        echo -e "\n${YELLOW}🌐 Starting local server...${NC}"
        echo -e "${GREEN}✅ Open http://127.0.0.1:8000 in your browser to view the documentation${NC}"
        mkdocs serve
        ;;
    2)
        echo -e "\n${YELLOW}🚀 Deploying to GitHub Pages...${NC}"
        
        # Check for uncommitted changes
        if ! git diff-index --quiet HEAD --; then
            echo -e "${YELLOW}⚠  You have uncommitted changes. Please commit or stash them before deploying.${NC}"
            exit 1
        fi
        
        # Deploy to GitHub Pages
        mkdocs gh-deploy --force
        
        echo -e "\n${GREEN}✅ Documentation deployed to GitHub Pages!${NC}"
        echo -e "📚 Visit: https://[your-github-username].github.io/anya-core/"
        ;;
    *)
        echo -e "${RED}❌ Invalid option${NC}"
        exit 1
        ;;
esac
