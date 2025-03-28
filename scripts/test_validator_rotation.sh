#!/bin/bash
# Script to test validator rotation functionality

set -e

echo "========================================"
echo "Validator Address Rotation Test"
echo "========================================"

# Check if config directory exists, if not create it
if [ ! -d "config" ]; then
  mkdir -p config
  echo "Created config directory"
fi

# Define the config path
CONFIG_PATH="config/validators.json"

# Step 1: Initialize validators
echo "1. Initializing validators (2-of-3 multisig)..."
cargo run --bin anya-core -- rotate-validators --init --threshold 2 --validators 3

# Verify the config file exists
if [ -f "$CONFIG_PATH" ]; then
  echo "✅ Config file created at $CONFIG_PATH"
else
  echo "❌ Failed to create config file at $CONFIG_PATH"
  exit 1
fi

# Print the current config
echo ""
echo "Current validator configuration:"
cat "$CONFIG_PATH" | jq .

# Step 2: Check rotation status
echo ""
echo "2. Checking rotation status..."
cargo run --bin anya-core -- rotate-validators --status

# Step 3: Force rotation (for testing)
echo ""
echo "3. Forcing rotation by manipulating config..."
# Extract current config
THRESHOLD=$(cat "$CONFIG_PATH" | jq .threshold)
NETWORK=$(cat "$CONFIG_PATH" | jq -r .network)
VALIDATORS=$(cat "$CONFIG_PATH" | jq .validators)
PREVIOUS=$(cat "$CONFIG_PATH" | jq .previous_validators)
PERIOD=$(cat "$CONFIG_PATH" | jq .rotation_period_days)

# Calculate a timestamp 31 days ago
TIMESTAMP=$(( $(date +%s) - 31*24*60*60 ))

# Build new config with old timestamp
NEW_CONFIG=$(cat <<EOF
{
  "network": "$NETWORK",
  "threshold": $THRESHOLD,
  "rotation_period_days": $PERIOD,
  "last_rotation": $TIMESTAMP,
  "validators": $VALIDATORS,
  "previous_validators": $PREVIOUS
}
EOF
)

# Write new config
echo "$NEW_CONFIG" > "$CONFIG_PATH"
echo "✅ Modified config with old timestamp: $TIMESTAMP"

# Step 4: Check rotation status again
echo ""
echo "4. Checking rotation status after modification..."
cargo run --bin anya-core -- rotate-validators --status

# Step 5: Perform actual rotation
echo ""
echo "5. Performing rotation..."
cargo run --bin anya-core -- rotate-validators

# Step 6: Verify new configuration
echo ""
echo "6. Verifying new configuration..."
cat "$CONFIG_PATH" | jq .

# Step 7: Generate and display multisig addresses
echo ""
echo "7. Generating multisig addresses..."
cargo run --bin anya-core -- rotate-validators --addresses

echo ""
echo "========================================"
echo "Validator Address Rotation Test Complete"
echo "========================================" 