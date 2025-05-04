#!/bin/bash
COMMIT_MSG=$(cat "$1")
PATTERN='\[([A-Z]{3}-[0-9])\]\[([A-Z]{3})\]\[([A-Z]{3})\]'

if ! [[ $COMMIT_MSG =~ $PATTERN ]]; then
  echo "Error: Commit message doesn't follow AI labelling format"
  echo "Required format: [Category-Version][Component][Status] Message"
  exit 1
fi

# Skip Bitcoin build/installation tests if not a Bitcoin-related change
if [[ $COMMIT_MSG =~ \[BTC\]|\[TRT\]|\[PSB\] ]]; then
  # Run minimal Bitcoin testnet tests
  echo "Running Bitcoin testnet checks..."
  SKIP_EXPENSIVE_TESTS=1 cargo test bitcoin::protocol --features bip174,bip341 -- --nocapture
else
  echo "Skipping Bitcoin tests for non-Bitcoin commit"
fi 