#!/bin/bash
# Branch Protection Disable Script
# This script temporarily disables branch protection for administrative tasks
# Usage: ./scripts/disable-branch-protection.sh

set -e

echo "🛡️ Temporarily Disabling Branch Protection Rules"
echo "------------------------------------------------"

# Get current repository information
if [ -z "$GITHUB_TOKEN" ]; then
  echo "❌ GITHUB_TOKEN environment variable is required"
  echo "Please set your Personal Access Token with 'repo' permissions:"
  echo "export GITHUB_TOKEN=your_token_here"
  exit 1
fi

# Extract repo info from git remote
REMOTE_URL=$(git remote get-url origin)
if [[ $REMOTE_URL =~ github\.com[:/]([^/]+)/([^/]+)(\.git)?$ ]]; then
  OWNER=${BASH_REMATCH[1]}
  REPO=${BASH_REMATCH[2]%.git}
else
  echo "❌ Could not determine repository owner and name from remote URL"
  exit 1
fi

echo "📁 Repository: $OWNER/$REPO"

# Disable branch protection for main branch
echo "🔓 Disabling protection for 'main' branch..."
curl -s -X DELETE \
  -H "Authorization: token $GITHUB_TOKEN" \
  -H "Accept: application/vnd.github.v3+json" \
  "https://api.github.com/repos/$OWNER/$REPO/branches/main/protection"

echo "✅ Branch protection for 'main' disabled"
echo ""
echo "⚠️ WARNING: Branch protection is now disabled. This removes:"
echo "  - Required pull request reviews"
echo "  - Required status checks"
echo "  - Required signed commits"
echo "  - Other protection rules"
echo ""
echo "🕒 This is intended for temporary administrative use only."
echo "🔒 Remember to re-enable protection after completing your tasks using:"
echo "    ./scripts/enable-branch-protection.sh"
echo ""

# Save timestamp to temp file for tracking
echo "$(date +%s)" >/tmp/branch_protection_disabled_at

exit 0
