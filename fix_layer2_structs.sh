#!/bin/bash

# Fix missing fields in all Layer2 modules

# List of files to fix
files=(
    "src/layer2/dlc/mod.rs"
    "src/layer2/liquid/mod.rs" 
    "src/layer2/rgb/mod.rs"
    "src/layer2/rsk/mod.rs"
    "src/layer2/stacks/mod.rs"
    "src/layer2/state_channels/mod.rs"
    "src/layer2/taproot_assets/mod.rs"
)

for file in "${files[@]}"; do
    echo "Fixing $file..."
    
    # Fix VerificationResult - add missing fields
    sed -i '/Ok(VerificationResult {/,/})/ {
        s/error: None,$/error: None,\
            error_message: None,\
            confidence_score: 1.0,/
    }' "$file"
    
    # Fix FeeEstimate - add missing fields  
    sed -i '/Ok(FeeEstimate {/,/})/ {
        s/confirmation_target: [0-9]*,$/&\
            slow_fee: (estimated_fee as f64 * 0.5) as u64,\
            normal_fee: estimated_fee,\
            fast_fee: (estimated_fee as f64 * 2.0) as u64,\
            estimated_confirmation_time: 6,/
    }' "$file"
    
done

echo "All files fixed!"
