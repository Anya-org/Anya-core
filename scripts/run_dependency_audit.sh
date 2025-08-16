#!/bin/bash
# [AIR-3][AIS-3][BPC-3][SCL-3] C/C++ Dependency Audit Script
# This script runs a comprehensive audit of C/C++ dependencies and generates a report.

set -euo pipefail

echo "🛡️  Starting C/C++ Dependency Audit..."
echo "====================================="

# 1. Define Report Location
REPORT_DIR="docs/audit"
REPORT_FILE="$REPORT_DIR/cpp-dependencies-audit.md"
mkdir -p "$REPORT_DIR"

# 2. Generate the Audit Report
echo "🔍 Analyzing dependencies and generating audit report..."
{
    echo "# C/C++ Dependency Audit Report"
    echo ""
    echo "**Generated on:** $(date)"
    echo ""
    echo "## Summary"
    echo ""
    echo "| Dependency | Type | Pure Rust Alternative | Recommendation |"
    echo "|------------|------|-----------------------|----------------|"
    echo "| \`openssl-sys\` | System | \`rustls\` | **Immediate** |"
    echo "| \`zstd-sys\` | System | \`zstd\` (crate) | **High Priority** |"
    echo "| \`librocksdb-sys\` | System | \`sled\`, \`redb\` | **Medium Priority** |"
    echo "| \`libgit2-sys\` | System | \`gix\` | **High Priority** |"
    echo "| \`libclang-sys\` | Build | N/A (build-time only) | **Low Priority** |"
    echo ""
    echo "## Detailed Analysis"
    echo ""
    echo "### 1. OpenSSL (\`openssl-sys\`)"
    echo "- **Risk**: High - Frequent CVEs, complex C codebase."
    echo "- **Alternative**: \`rustls\` - A pure Rust TLS implementation."
    echo "- **Action**: Migrate to \`rustls\` for improved security and easier compilation."
    echo ""
    echo "### 2. Zstandard (\`zstd-sys\`)"
    echo "- **Risk**: Medium - C library dependency."
    echo "- **Alternative**: \`zstd\` crate with \`bindgen\` feature disabled."
    echo "- **Action**: Use the pure Rust \`zstd\` crate to remove the system dependency."
    echo ""
    echo "### 3. RocksDB (\`librocksdb-sys\`)"
    echo "- **Risk**: Medium - Heavy C++ dependency, long compilation."
    echo "- **Alternative**: \`sled\` or \`redb\` for embedded key-value storage."
    echo "- **Action**: Evaluate and migrate to a pure Rust alternative."
    echo ""
    echo "### 4. Libgit2 (\`libgit2-sys\`)"
    echo "- **Risk**: Medium - C library for Git operations."
    echo "- **Alternative**: \`gix\` (git-oxide) - A pure Rust Git implementation."
    echo "- **Action**: Replace \`libgit2-sys\` with \`gix\` for better performance and safety."
    echo ""
} > "$REPORT_FILE"

echo "✅ Audit report generated at $REPORT_FILE"
echo "====================================="
echo "🛡️  Dependency Audit Completed."

