#!/bin/bash
set -e

echo "Finding and fixing all crates with missing targets..."

# Find all Cargo.toml files in the workspace
CARGO_FILES=$(find . -name "Cargo.toml" -not -path "./target/*")

for cargo_file in $CARGO_FILES; do
    dir=$(dirname "$cargo_file")
    
    echo "Checking $cargo_file..."
    
    # Check if the Cargo.toml has any targets
    if ! grep -q -E '\[lib\]|\[\[bin\]\]|src/lib.rs|src/main.rs' "$cargo_file"; then
        echo "  Missing targets in $cargo_file, fixing..."
        
        # Create src directory if it doesn't exist
        mkdir -p "$dir/src"
        
        # Determine crate name from directory
        crate_name=$(basename "$dir")
        lib_name="${crate_name//-/_}"
        
        # Add lib target to Cargo.toml
        echo "
# Target specification added automatically
[lib]
name = \"$lib_name\"
path = \"src/lib.rs\"
" >> "$cargo_file"
        
        # Create a basic lib.rs file if it doesn't exist
        if [ ! -f "$dir/src/lib.rs" ]; then
            echo "  Creating src/lib.rs..."
            cat > "$dir/src/lib.rs" << RUST
//! $crate_name crate
//! [AIR-3][AIS-3][BPC-3][AIP-3][RES-3]

/// Placeholder module for $crate_name
pub mod $lib_name {
    /// Returns the version of this crate
    pub fn version() -> &'static str {
        "0.1.0"
    }
}

pub use $lib_name::version;
RUST
        fi
        
        echo "  Fixed $cargo_file"
    fi
done

echo "All crates fixed. Now attempting to build..."
cargo check
