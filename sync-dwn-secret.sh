#!/bin/bash
# Sync local [web5] dwn_endpoint from config/anya.conf to GitHub Actions secret DWN_NODE_URL
# Usage: ./sync-dwn-secret.sh <github-repo> <github-username>

if [ -z "$1" ] || [ -z "$2" ]; then
  echo "Usage: $0 <github-repo> <github-username>"
  exit 1
fi

DWN_ENDPOINT=$(grep '^dwn_endpoint' config/anya.conf | cut -d'=' -f2 | tr -d '" ')
if [ -z "$DWN_ENDPOINT" ]; then
  echo "Could not find dwn_endpoint in config/anya.conf."
  exit 1
fi

echo "Setting DWN_NODE_URL secret for $2/$1 to: $DWN_ENDPOINT"
gh secret set DWN_NODE_URL -b "$DWN_ENDPOINT" -R "$2/$1"
