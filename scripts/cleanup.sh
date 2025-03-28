#!/bin/bash
# Clean up the project and package for distribution
set -e

# Clean build artifacts
cargo clean

# Remove temporary files
find . -name "*.tmp" -delete
find . -name "*.bak" -delete
find . -name "*.orig" -delete
find . -name "*~" -delete

# Format code if rustfmt is available
if command -v rustfmt &> /dev/null; then
    echo "Formatting code..."
    find core anya-core cli -name "*.rs" -exec rustfmt {} \;
    echo "Code formatted successfully"
fi

# Create a clean directory for the release
RELEASE_DIR="dist/anya-core"
mkdir -p "$RELEASE_DIR"

# Copy only the essential files
echo "Creating release package..."
cp -r core "$RELEASE_DIR/"
cp -r anya-core "$RELEASE_DIR/"
cp -r cli "$RELEASE_DIR/"
cp Cargo.toml Cargo.lock "$RELEASE_DIR/" 2>/dev/null || :

# Create README and LICENSE if they don't exist
if [ ! -f "$RELEASE_DIR/README.md" ]; then
    echo "# Anya Core Layer 4 Bitcoin Protocol" > "$RELEASE_DIR/README.md"
    echo "## Bitcoin Development Framework v2.5 Implementation" >> "$RELEASE_DIR/README.md"
    echo "Created with BIP-341, BIP-342, BIP-174, and BIP-370 support." >> "$RELEASE_DIR/README.md"
fi

if [ ! -f "$RELEASE_DIR/LICENSE" ]; then
    echo "MIT License" > "$RELEASE_DIR/LICENSE"
    echo "" >> "$RELEASE_DIR/LICENSE"
    echo "Copyright (c) 2025 Anya Core" >> "$RELEASE_DIR/LICENSE"
    echo "" >> "$RELEASE_DIR/LICENSE"
    echo "Permission is hereby granted, free of charge, to any person obtaining a copy" >> "$RELEASE_DIR/LICENSE"
    echo "of this software and associated documentation files..." >> "$RELEASE_DIR/LICENSE"
fi

# Package everything
echo "Creating tarball..."
tar -czf dist/anya-core.tar.gz -C dist anya-core

echo "Project cleaned and packaged to dist/anya-core.tar.gz"
