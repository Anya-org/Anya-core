#!/bin/bash

# Validate repository structure, submodules and remotes
# [AIR-3][AIS-3][BPC-3]

set -euo pipefail

# Define colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Expected submodules and remotes as arrays
declare -a SUBMODULE_NAMES=(
  "dash33"
  "anya-bitcoin"
  "anya-web5"
  "anya-enterprise"
  "anya-extensions"
)

declare -a SUBMODULE_URLS=(
  "git@github.com:anya-org/dash33.git"
  "git@github.com:anya-org/anya-bitcoin.git" 
  "git@github.com:anya-org/anya-web5.git"
  "git@github.com:anya-org/anya-enterprise.git"
  "git@github.com:anya-org/anya-extensions.git"
)

declare -a REMOTE_NAMES=(
  "origin"
  "upstream"
)

declare -a REMOTE_URLS=(
  "git@github.com:anya-org/anya-core.git"
  "git@github.com:anya-org/anya-core.git"
)

check_submodule() {
  local name=$1
  local expected_url=$2
  
  if ! git submodule status | grep -q "$name"; then
    echo -e "${RED}❌ Missing submodule: $name${NC}"
    echo "Adding submodule..."
    git submodule add "$expected_url" "$name" || return 1
  fi

  cd "$name" || return 1
  
  local actual_url
  actual_url=$(git remote get-url origin)
  
  if [ "$actual_url" != "$expected_url" ]; then
    echo -e "${RED}❌ Incorrect URL for $name${NC}"
    echo "Expected: $expected_url"
    echo "Actual: $actual_url"
    echo "Fixing URL..."
    git remote set-url origin "$expected_url" || return 1
  fi
  
  cd - > /dev/null || return 1
  return 0
}

check_remote() {
  local name=$1
  local expected_url=$2

  if ! git remote | grep -q "^$name\$"; then
    echo -e "${RED}❌ Missing remote: $name${NC}"
    echo "Adding remote..."
    git remote add "$name" "$expected_url" || return 1
  fi

  local actual_url
  actual_url=$(git remote get-url "$name")
  
  if [ "$actual_url" != "$expected_url" ]; then
    echo -e "${RED}❌ Incorrect URL for remote $name${NC}"
    echo "Expected: $expected_url"
    echo "Actual: $actual_url"
    echo "Fixing URL..."
    git remote set-url "$name" "$expected_url" || return 1
  fi
  
  return 0
}

# Updated main function to use arrays instead of associative arrays
main() {
    echo -e "${YELLOW}Checking submodules...${NC}"
    git submodule init
    git submodule update

    local errors=0
    
    # Check each submodule using arrays
    for i in "${!SUBMODULE_NAMES[@]}"; do
        name="${SUBMODULE_NAMES[$i]}"
        url="${SUBMODULE_URLS[$i]}"
        
        echo -e "\nChecking submodule: $name"
        if ! check_submodule "$name" "$url"; then
            ((errors++))
        else
            echo -e "${GREEN}✓ Submodule $name OK${NC}"
        fi
    done

    echo -e "\n${YELLOW}Checking remotes...${NC}"
  
    # Check each expected remote
    for i in "${!REMOTE_NAMES[@]}"; do
        name="${REMOTE_NAMES[$i]}"
        url="${REMOTE_URLS[$i]}"
        
        echo -e "\nChecking remote: $name"
        if ! check_remote "$name" "$url"; then
            ((errors++))
        else
            echo -e "${GREEN}✓ Remote $name OK${NC}"
        fi
    done

    # Update submodules recursively
    echo -e "\n${YELLOW}Updating submodules...${NC}"
    git submodule update --init --recursive
    
    if [ "$errors" -eq 0 ]; then
        echo -e "\n${GREEN}All submodules and remotes validated successfully!${NC}"
        exit 0
    else
        echo -e "\n${RED}Found $errors error(s) in submodules/remotes configuration${NC}"
        exit 1
    fi
}

main "$@"
