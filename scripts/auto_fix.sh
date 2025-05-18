#!/bin/bash
# Auto-fix script for Anya Core compilation issues
# Created: 2025-05-17

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Text formatting
RED='\033[0;31m'
# [AIR-3][AIS-3][RES-3]

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Text formatting
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Progress tracking
TOTAL_STEPS=5
current_step=0

# Function to show progress
progress() {
  current_step=$((current_step + 1))
  echo -e "${BLUE}[${current_step}/${TOTAL_STEPS}]${NC} $1..."
}

# Function to run with output
run_with_output() {
  echo -e "${YELLOW}Running: $1${NC}"
  eval "$1"
  if [ $? -eq 0 ]; then
    echo -e "${GREEN}Success!${NC}"
  else
    echo -e "${RED}Failed!${NC}"
    if [ "$2" == "critical" ]; then
      exit 1
    fi
  fi
}

# Check tools
check_tools() {
  progress "Checking required tools"
  for tool in cargo rustc git; do
    if ! command -v $tool &> /dev/null; then
      echo -e "${RED}Error: $tool is not installed${NC}"
      exit 1
    fi
  done
  echo -e "${GREEN}All required tools are installed${NC}"
}

# Fix documentation errors
fix_doc_errors() {
  progress "Fixing documentation errors"
  echo "Searching for files with documentation errors..."
  
  # Fix for inner doc comments in wrong positions
  find "$PROJECT_ROOT/src" -name "*.rs" -type f -exec grep -l "^use.*\n//!" {} \; | while read -r file; do
    echo "Fixing doc comments in $file"
    sed -i '/^use.*\n\/\/!/!b;n;h;:a;n;/^[^\/]/!{H;ba};x;s/use \(.*\)\n\/\/!\(.*\)/\/\/!\2\n\nuse \1/g' "$file"
  done
}

# Fix syntax errors
fix_syntax_errors() {
  progress "Fixing syntax errors"
  
  # Fix double arrow syntax errors in return types
  find "$PROJECT_ROOT/src" -name "*.rs" -type f -exec grep -l "-> .* -> " {} \; | while read -r file; do
    echo "Fixing double arrow syntax in $file"
    sed -i 's/\(-> [^{]*\) -> [^{]*/\1/g' "$file"
  done
  
  # Fix parameter syntax in layer2 modules
  for file in "$PROJECT_ROOT/src/layer2/mod.rs" "$PROJECT_ROOT/src/layer2/state_channels/mod.rs"; do
    if [ -f "$file" ]; then
      echo "Fixing parameter syntax in $file"
      sed -i 's/(\([^:]*\): \([^)]*\))/(\1: \2)/g' "$file"
      sed -i 's/fn \([a-z_]*\)(\([^)]*\): \([^)]*\))/fn \1(\2: \3)/g' "$file"
    fi
  done
}

# Run cargo check to verify fixes
run_cargo_check() {
  progress "Running cargo check"
  run_with_output "cd \"$PROJECT_ROOT\" && cargo check" "critical"
}

# Create git commit with fixes
create_git_commit() {
  progress "Creating git commit with fixes"
  
  if git -C "$PROJECT_ROOT" diff --quiet; then
    echo -e "${YELLOW}No changes to commit${NC}"
    return
  fi
  
  echo "Changes detected, creating commit..."
  
  if [ "$1" == "--auto-commit" ]; then
    run_with_output "cd \"$PROJECT_ROOT\" && git add . && git commit -m \"fix: resolve compilation errors and syntax issues\"" "non-critical"
  else
    echo -e "${YELLOW}Changes ready to commit. Run these commands to commit:${NC}"
    echo "cd \"$PROJECT_ROOT\" && git add . && git commit -m \"fix: resolve compilation errors and syntax issues\""
  fi
}

# Main execution
main() {
  echo -e "${BLUE}====== Starting Anya Core Auto-Fix ======${NC}"
  
  # Parse arguments
  AUTO_COMMIT=false
  MONITOR=false
  
  for arg in "$@"; do
    case $arg in
      --self-check)
        ;;
      --monitor)
        MONITOR=true
        ;;
      --auto-commit|--yes-all)
        AUTO_COMMIT=true
        ;;
    esac
  done
  
  check_tools
  fix_doc_errors
  fix_syntax_errors
  
  if $MONITOR; then
    echo -e "${BLUE}Starting monitoring mode...${NC}"
    run_with_output "cd \"$PROJECT_ROOT\" && cargo watch -x check" "non-critical"
  else
    run_cargo_check
  fi
  
  if $AUTO_COMMIT; then
    create_git_commit "--auto-commit"
  else
    create_git_commit
  fi
  
  echo -e "${GREEN}====== Auto-Fix Complete ======${NC}"
}

main "$@"
