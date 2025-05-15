#!/bin/bash
# Fix parameter syntax errors in the codebase

# Function to find and fix incorrect parameter patterns
fix_pattern() {
  local pattern=$1
  local replacement=$2
  echo "Fixing pattern: $pattern -> $replacement"
  
  # Use find to locate all Rust files and perform replacements
  find /home/anya/anyachainlabs/projects/anya-core/src -name "*.rs" -type f -exec sed -i "s/$pattern/$replacement/g" {} \;
}

# Fix the most common parameter patterns
fix_pattern "_key_id: key_id: &strstr" "_key_id: \&str"
fix_pattern "_data: data: &\[u8\]\[u8\]" "_data: \&\[u8\]"
fix_pattern "data: data: &\[u8\]\[u8\]" "data: \&\[u8\]"
fix_pattern "_signature: signature: &\[u8\]\[u8\]" "_signature: \&\[u8\]"
fix_pattern "signature: signature: &\[u8\]\[u8\]" "signature: \&\[u8\]"
fix_pattern "key_id: key_id: &strstr" "key_id: \&str"
fix_pattern "__key_id: key_id: &strstr" "__key_id: \&str"
fix_pattern "__data: data: &\[u8\]\[u8\]" "__data: \&\[u8\]"
fix_pattern "__signature: signature: &\[u8\]\[u8\]" "__signature: \&\[u8\]"
fix_pattern "tx__data: data: &\[u8\]\[u8\]" "tx_data: \&\[u8\]"
fix_pattern "model__data: data: &\[u8\]\[u8\]" "model_data: \&\[u8\]"
fix_pattern "asset_meta_data: data: &\[u8\]\[u8\]" "asset_meta_data: \&\[u8\]"

echo "Fixed parameter syntax errors in the codebase"
