#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3] Simple Documentation Analysis
# Test script to analyze the documentation situation

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "🔍 ANALYZING ANYA CORE DOCUMENTATION"
echo "================================="
echo "Workspace: $WORKSPACE_ROOT"
echo

# Check what we have
echo "📁 CURRENT STRUCTURE:"
echo "Source directory:"
ls -la "$WORKSPACE_ROOT/src/" | head -10

echo
echo "Documentation directory:"
if [[ -d "$WORKSPACE_ROOT/docs" ]]; then
    ls -la "$WORKSPACE_ROOT/docs/" | head -10
else
    echo "No docs directory found"
fi

echo
echo "📊 COUNTS:"
echo "Source modules: $(find "$WORKSPACE_ROOT/src" -maxdepth 1 -type d | wc -l)"
echo "Documentation files: $(find "$WORKSPACE_ROOT/docs" -name "*.md" 2>/dev/null | wc -l || echo 0)"

echo
echo "🎯 KEY MODULES IN SOURCE:"
find "$WORKSPACE_ROOT/src" -maxdepth 1 -type d -name "*" | while read -r dir; do
    module_name=$(basename "$dir")
    if [[ "$module_name" != "src" ]]; then
        rust_files=$(find "$dir" -name "*.rs" 2>/dev/null | wc -l)
        echo "- $module_name: $rust_files Rust files"
    fi
done

echo
echo "✅ Analysis complete!"
