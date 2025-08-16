#!/bin/bash
# C/C++ Dependencies Quick Fix Script
# Implements immediate build performance improvements

set -e

echo "🔧 Anya-Core C/C++ Dependencies Quick Fix"
echo "=========================================="

# Backup current Cargo.toml
echo "📋 Creating backup of Cargo.toml..."
cp Cargo.toml Cargo.toml.backup.$(date +%Y%m%d_%H%M%S)

# Function to update dependency in Cargo.toml
update_dependency() {
    local dep_name="$1"
    local old_pattern="$2"
    local new_pattern="$3"

    echo "🔄 Updating $dep_name dependency..."
    sed -i.bak "s/$old_pattern/$new_pattern/g" Cargo.toml
}

# 1. Replace reqwest to use rustls-tls
echo "🚀 Phase 1: Replacing OpenSSL with rustls..."
sed -i.bak 's/reqwest = { version = "\([^"]*\)", features = \["json"\]/reqwest = { version = "\1", features = ["json", "rustls-tls"], default-features = false/' Cargo.toml

# 2. Update sqlx to use rustls
sed -i.bak 's/sqlx = { version = "\([^"]*\)", features = \["runtime-tokio-rustls", "postgres"\]/sqlx = { version = "\1", features = ["runtime-tokio-rustls", "postgres", "tls-rustls"], default-features = false/' Cargo.toml

# 3. Add pure rust compression
echo "📦 Adding pure Rust compression support..."
if ! grep -q "zstd.*pure_rust" Cargo.toml; then
    echo 'zstd = { version = "0.13.2", features = ["pure_rust"] }' >> Cargo.toml
fi

# 4. Check for unused curl dependency
echo "🔍 Checking for curl usage..."
if grep -q "curl.*=" Cargo.toml; then
    echo "⚠️ Found curl dependency. Checking if it's actually used..."
    if ! rg "use curl|extern crate curl" src/ --quiet; then
        echo "💡 curl dependency found but not used in source code"
        echo "   Consider removing it manually if it's not needed"
    fi
fi

# 5. Test the changes
echo "🧪 Testing changes..."
echo "Running cargo check to verify the changes work..."

# Set required environment variables for build
export LIBCLANG_PATH="/usr/lib/llvm-18/lib"

# Try to build with new dependencies
if cargo check --no-default-features --features="std" --lib; then
    echo "✅ Success! Dependencies updated successfully"
    echo ""
    echo "📊 Changes made:"
    echo "  - reqwest: Now uses rustls-tls instead of OpenSSL"
    echo "  - sqlx: Now uses tls-rustls instead of OpenSSL"
    echo "  - zstd: Added pure_rust feature"
    echo ""
    echo "🎯 Expected improvements:"
    echo "  - Faster build times (less C++ compilation)"
    echo "  - Better cross-platform compatibility"
    echo "  - Reduced system dependencies"
    echo ""
    echo "💾 Backup saved as: Cargo.toml.backup.*"
    echo "🔧 Run 'cargo clean && cargo build' to see full performance improvement"
else
    echo "❌ Build failed with new dependencies"
    echo "🔄 Restoring backup..."
    mv Cargo.toml.backup.* Cargo.toml
    echo "📝 Please check the migration guide for manual fixes needed"
    exit 1
fi

# 6. Generate a report
echo ""
echo "📋 Quick Performance Test:"
echo "========================="
echo "To measure improvement:"
echo "1. Clean build test: cargo clean && time cargo build --release"
echo "2. Compare with backup: mv Cargo.toml.backup.* Cargo.toml && cargo clean && time cargo build --release"
echo ""
echo "🎯 Next Steps (from migration guide):"
echo "- Phase 2: Consider migrating from rocksdb to redb"
echo "- Phase 3: Evaluate git2 → gix migration"
echo "- Phase 4: Review secp256k1 vs k256 for Bitcoin operations"
echo ""
echo "📖 See docs/audit/cpp-migration-guide.md for detailed instructions"
